//! Drives the real `QapiQga` against a hand-rolled guest-agent server (no guest).

use phermesd::qga::{QapiQga, QgaControl};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

/// Minimal QGA server: answer guest-sync (echo id), guest-ping, fsfreeze-freeze (2), thaw (2).
async fn fake_qga_server(path: PathBuf) -> std::io::Result<()> {
    let listener = UnixListener::bind(&path)?;
    let (stream, _) = listener.accept().await?;
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    while let Some(line) = lines.next_line().await? {
        if line.contains("guest-sync") {
            let id: i64 = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            write
                .write_all(format!("{{\"return\":{id}}}\n").as_bytes())
                .await?;
        } else if line.contains("guest-ping") {
            write.write_all(b"{\"return\":{}}\n").await?;
        } else if line.contains("guest-fsfreeze-freeze") || line.contains("guest-fsfreeze-thaw") {
            write.write_all(b"{\"return\":2}\n").await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn connects_pings_freezes_and_thaws() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("qga.sock");
    let server = tokio::spawn(fake_qga_server(sock.clone()));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let qga = QapiQga::connect(&sock).await.unwrap();
    qga.ping().await.unwrap();
    assert_eq!(qga.freeze().await.unwrap(), 2);
    assert_eq!(qga.thaw().await.unwrap(), 2);

    server.abort();
}
