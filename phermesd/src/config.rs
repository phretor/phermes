//! VM definition model: typed TOML at /etc/phermes/vms/<id>.toml.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Flavor {
    Linux,
    Windows,
    Macos,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskInterface {
    #[default]
    VirtioScsi,
    VirtioBlk,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetModel {
    #[default]
    VirtioNet,
    E1000,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resources {
    pub memory_mib: u32,
    pub vcpus: u16,
    #[serde(default = "default_cpu")]
    pub cpu: String,
}

fn default_cpu() -> String {
    "host".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Firmware {
    pub ovmf_code: PathBuf,
    pub ovmf_vars_template: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Disk {
    pub path: PathBuf,
    pub format: String,
    #[serde(default)]
    pub interface: DiskInterface,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Net {
    pub bridge: String,
    #[serde(default)]
    pub model: NetModel,
    #[serde(default)]
    pub mac: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Console {
    #[serde(default)]
    pub serial: bool,
    #[serde(default)]
    pub vnc: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VmDef {
    pub flavor: Flavor,
    pub resources: Resources,
    pub firmware: Firmware,
    pub disk: Vec<Disk>,
    pub net: Vec<Net>,
    #[serde(default)]
    pub console: Console,
}

use std::path::Path;

/// A definition plus its id (the file stem).
#[derive(Debug, Clone)]
pub struct Vm {
    pub id: String,
    pub def: VmDef,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parsing {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("invalid definition {id}: {reason}")]
    Invalid { id: String, reason: String },
}

/// Stable locally-administered MAC from the VM id (FNV-1a, fixed for reproducibility).
#[must_use]
pub fn derived_mac(id: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in id.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let b = hash.to_le_bytes();
    format!("52:54:00:{:02x}:{:02x}:{:02x}", b[0], b[1], b[2])
}

fn validate(id: &str, def: &VmDef) -> Result<(), ConfigError> {
    let invalid = |reason: &str| ConfigError::Invalid {
        id: id.to_string(),
        reason: reason.to_string(),
    };
    if def.resources.memory_mib == 0 {
        return Err(invalid("memory_mib must be > 0"));
    }
    if def.resources.vcpus == 0 {
        return Err(invalid("vcpus must be > 0"));
    }
    if def.disk.is_empty() {
        return Err(invalid("at least one [[disk]] is required"));
    }
    Ok(())
}

/// Load and validate a single VM definition; the id is the file stem.
///
/// # Errors
///
/// Returns `ConfigError::Io` if the file cannot be read, `ConfigError::Parse` if
/// the TOML is malformed, or `ConfigError::Invalid` if validation fails.
pub fn load_file(path: &Path) -> Result<Vm, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let def: VmDef = toml::from_str(&text).map_err(|source| ConfigError::Parse {
        path: path.to_path_buf(),
        source,
    })?;
    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();
    validate(&id, &def)?;
    Ok(Vm { id, def })
}

/// Load every `*.toml` in `dir`, sorted by id. Non-`.toml` files are ignored.
///
/// # Errors
///
/// Returns `ConfigError::Io` if the directory cannot be read or an entry fails,
/// or any error propagated from `load_file` for individual files.
pub fn load_dir(dir: &Path) -> Result<Vec<Vm>, ConfigError> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let read = std::fs::read_dir(dir).map_err(|source| ConfigError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in read {
        let entry = entry.map_err(|source| ConfigError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            entries.push(path);
        }
    }
    entries.sort();
    let mut vms = Vec::with_capacity(entries.len());
    for path in entries {
        vms.push(load_file(&path)?);
    }
    Ok(vms)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = r#"
flavor = "linux"
[resources]
memory_mib = 2048
vcpus = 2
[firmware]
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"
[[disk]]
path = "/var/lib/phermes/images/linux-node.qcow2"
format = "qcow2"
[[net]]
bridge = "vmbr0"
[console]
serial = true
vnc = true
"#;

    #[test]
    fn parses_full_def_with_defaults() {
        let def: VmDef = toml::from_str(FULL).unwrap();
        assert_eq!(def.flavor, Flavor::Linux);
        assert_eq!(def.resources.memory_mib, 2048);
        assert_eq!(def.resources.vcpus, 2);
        assert_eq!(def.resources.cpu, "host"); // defaulted
        assert_eq!(def.disk.len(), 1);
        assert_eq!(def.disk[0].interface, DiskInterface::VirtioScsi); // defaulted
        assert_eq!(def.net[0].model, NetModel::VirtioNet); // defaulted
        assert!(def.console.serial && def.console.vnc);
    }

    #[test]
    fn console_defaults_false_when_absent() {
        let def: VmDef = toml::from_str(
            "flavor=\"linux\"\n[resources]\nmemory_mib=512\nvcpus=1\n\
             [firmware]\novmf_code=\"/a\"\novmf_vars_template=\"/b\"\n\
             [[disk]]\npath=\"/c\"\nformat=\"raw\"\n[[net]]\nbridge=\"vmbr0\"\n",
        )
        .unwrap();
        assert!(!def.console.serial && !def.console.vnc);
    }

    #[test]
    fn derived_mac_is_stable_and_well_formed() {
        let a = derived_mac("linux");
        let b = derived_mac("linux");
        assert_eq!(a, b);
        assert!(a.starts_with("52:54:00:"));
        assert_eq!(a.len(), 17);
        assert_ne!(derived_mac("linux"), derived_mac("windows"));
    }

    #[test]
    fn load_file_uses_filename_stem_as_id() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("linux.toml");
        std::fs::write(&p, FULL).unwrap();
        let vm = load_file(&p).unwrap();
        assert_eq!(vm.id, "linux");
        assert_eq!(vm.def.flavor, Flavor::Linux);
    }

    #[test]
    fn validate_rejects_zero_resources_and_empty_disk() {
        let bad = "flavor=\"linux\"\n[resources]\nmemory_mib=0\nvcpus=1\n\
                   [firmware]\novmf_code=\"/a\"\novmf_vars_template=\"/b\"\n\
                   [[disk]]\npath=\"/c\"\nformat=\"raw\"\n[[net]]\nbridge=\"vmbr0\"\n";
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("bad.toml");
        std::fs::write(&p, bad).unwrap();
        assert!(matches!(load_file(&p), Err(ConfigError::Invalid { .. })));
    }

    #[test]
    fn load_dir_returns_all_defs_sorted() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("linux.toml"), FULL).unwrap();
        std::fs::write(dir.path().join("windows.toml"), FULL).unwrap();
        std::fs::write(dir.path().join("notes.txt"), "ignore me").unwrap();
        let vms = load_dir(dir.path()).unwrap();
        let ids: Vec<&str> = vms.iter().map(|v| v.id.as_str()).collect();
        assert_eq!(ids, vec!["linux", "windows"]);
    }
}
