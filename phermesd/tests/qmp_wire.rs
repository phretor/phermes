//! Drives the real `QapiQmp` against a hand-rolled QMP server (no QEMU).

use phermesd::qmp::{QapiQmp, QmpControl};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

const GREETING: &str =
    r#"{"QMP":{"version":{"qemu":{"micro":0,"minor":0,"major":9},"package":""},"capabilities":[]}}"#;

async fn fake_qmp_server(path: PathBuf) -> std::io::Result<()> {
    let listener = UnixListener::bind(&path)?;
    let (stream, _) = listener.accept().await?;
    let (read, mut write) = stream.into_split();
    write
        .write_all(format!("{GREETING}\n").as_bytes())
        .await?;
    let mut lines = BufReader::new(read).lines();
    while let Some(line) = lines.next_line().await? {
        if line.contains("qmp_capabilities") {
            write.write_all(b"{\"return\":{}}\n").await?;
        } else if line.contains("query-status") {
            write
                .write_all(
                    b"{\"return\":{\"running\":true,\"singlestep\":false,\"status\":\"running\"}}\n",
                )
                .await?;
        } else if line.contains("system_powerdown") {
            write.write_all(b"{\"return\":{}}\n").await?;
            write
                .write_all(b"{\"event\":\"SHUTDOWN\",\"timestamp\":{\"seconds\":0,\"microseconds\":0},\"data\":{\"guest\":true,\"reason\":\"guest-shutdown\"}}\n")
                .await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn connects_queries_powerdown_and_observes_shutdown() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("qmp.sock");
    let server = tokio::spawn(fake_qmp_server(sock.clone()));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut client = QapiQmp::connect(&sock).await.unwrap();
    assert!(client.is_running().await.unwrap());
    client.powerdown().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), client.wait_shutdown())
        .await
        .expect("wait_shutdown timed out")
        .unwrap();

    server.abort();
}
