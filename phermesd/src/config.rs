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
}
