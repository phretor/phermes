//! Control-protocol wire types (newline-delimited JSON over the UDS).

use crate::config::Flavor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "lowercase")]
pub enum Request {
    List,
    Status {
        #[serde(default)]
        id: Option<String>,
    },
    Activate {
        id: String,
    },
    Stop {
        #[serde(default)]
        id: Option<String>,
    },
    Reload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VmState {
    Defined,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmInfo {
    pub id: String,
    pub flavor: Flavor,
    pub state: VmState,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub qmp: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vnc: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error: Option<ErrorBody>,
}

impl Response {
    #[must_use]
    pub fn ok(data: serde_json::Value) -> Self {
        Self { ok: true, data: Some(data), error: None }
    }

    #[must_use]
    pub fn err(kind: &str, message: &str) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(ErrorBody { kind: kind.to_string(), message: message.to_string() }),
        }
    }
}

/// Serialize a value as a single newline-terminated JSON line.
///
/// # Errors
///
/// Returns `serde_json::Error` if serialization fails.
pub fn encode_line<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut s = serde_json::to_string(value)?;
    s.push('\n');
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Flavor;

    #[test]
    fn request_activate_round_trips() {
        let line = r#"{"cmd":"activate","id":"linux"}"#;
        let req: Request = serde_json::from_str(line).unwrap();
        assert_eq!(req, Request::Activate { id: "linux".to_string() });
    }

    #[test]
    fn request_status_without_id() {
        let req: Request = serde_json::from_str(r#"{"cmd":"status"}"#).unwrap();
        assert_eq!(req, Request::Status { id: None });
    }

    #[test]
    fn ok_response_serializes_with_data_and_no_error() {
        let info = VmInfo {
            id: "linux".to_string(),
            flavor: Flavor::Linux,
            state: VmState::Running,
            pid: Some(4321),
            qmp: Some("/run/phermesd/linux/qmp.sock".into()),
            serial: None,
            vnc: Some("/run/phermesd/linux/vnc.sock".into()),
        };
        let resp = Response::ok(serde_json::to_value(&info).unwrap());
        let s = encode_line(&resp).unwrap();
        assert!(s.ends_with('\n'));
        assert!(s.contains(r#""ok":true"#));
        assert!(!s.contains(r#""error""#));
        assert!(s.contains(r#""state":"running""#));
    }

    #[test]
    fn err_response_has_kind_and_message() {
        let resp = Response::err("already_active", "linux is already running");
        let s = encode_line(&resp).unwrap();
        assert!(s.contains(r#""ok":false"#));
        assert!(s.contains(r#""kind":"already_active""#));
        assert!(!s.contains(r#""data""#));
    }
}
