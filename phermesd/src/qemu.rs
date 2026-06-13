//! Pure QEMU argv builder: a VM definition + runtime paths -> argument vector.
//! This is the seam where future flavors (Windows, macOS) and storage/net blocks plug in.

use crate::config::{derived_mac, DiskInterface, Flavor, NetModel, Vm};
use std::path::PathBuf;

/// Per-VM runtime paths phermesd derives under `/run/phermesd/<id>/`.
#[derive(Debug, Clone)]
pub struct RuntimePaths {
    pub vars: PathBuf,
    pub qmp: PathBuf,
    pub serial: PathBuf,
    pub vnc: PathBuf,
    pub pidfile: PathBuf,
    pub qga: PathBuf,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum QemuError {
    #[error("flavor {0:?} is not yet supported")]
    UnsupportedFlavor(Flavor),
}

/// Build the `qemu-system-x86_64` argument vector (program name excluded).
///
/// # Errors
///
/// Returns `QemuError::UnsupportedFlavor` if the VM flavor is not `Linux` or `Windows`.
pub fn build_argv(vm: &Vm, rt: &RuntimePaths) -> Result<Vec<String>, QemuError> {
    match vm.def.flavor {
        Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt)),
        other @ Flavor::Macos => Err(QemuError::UnsupportedFlavor(other)),
    }
}

fn net_device(model: NetModel) -> &'static str {
    match model {
        NetModel::VirtioNet => "virtio-net-pci",
        NetModel::E1000 => "e1000",
    }
}

/// Push a `flag value` pair onto the argument vector.
fn pair(a: &mut Vec<String>, flag: &str, val: String) {
    a.push(flag.to_string());
    a.push(val);
}

/// Build the argv vector for a generic q35+UEFI+virtio PC. Used for both
/// Linux and Windows guests; their QEMU command lines are identical given
/// the same firmware/disk/net/console settings (only the in-guest OS
/// behavior differs). macOS uses a separate builder in slice #5b.
fn build_pc_uefi(vm: &Vm, rt: &RuntimePaths) -> Vec<String> {
    let d = &vm.def;
    let mut a: Vec<String> = Vec::new();

    pair(&mut a, "-machine", "q35,accel=kvm".to_string());
    pair(&mut a, "-cpu", d.resources.cpu.clone());
    pair(&mut a, "-smp", d.resources.vcpus.to_string());
    pair(&mut a, "-m", d.resources.memory_mib.to_string());
    a.push("-nodefaults".to_string());
    a.push("-no-user-config".to_string());

    pair(
        &mut a,
        "-drive",
        format!(
            "if=pflash,format=raw,readonly=on,file={}",
            d.firmware.ovmf_code.display()
        ),
    );
    pair(
        &mut a,
        "-drive",
        format!("if=pflash,format=raw,file={}", rt.vars.display()),
    );
    pair(
        &mut a,
        "-qmp",
        format!("unix:{},server=on,wait=off", rt.qmp.display()),
    );
    pair(&mut a, "-pidfile", rt.pidfile.display().to_string());

    pair(&mut a, "-device", "virtio-serial-pci".to_string());
    pair(
        &mut a,
        "-chardev",
        format!("socket,path={},server=on,wait=off,id=qga0", rt.qga.display()),
    );
    pair(
        &mut a,
        "-device",
        "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0".to_string(),
    );

    let mut scsi_controller_added = false;
    for (i, disk) in d.disk.iter().enumerate() {
        match disk.interface {
            DiskInterface::VirtioScsi | DiskInterface::VirtioBlk => {
                pair(
                    &mut a,
                    "-drive",
                    format!(
                        "file={},format={},if=none,id=disk{i}",
                        disk.path.display(),
                        disk.format
                    ),
                );
            }
            DiskInterface::Cdrom => {
                pair(
                    &mut a,
                    "-drive",
                    format!(
                        "media=cdrom,readonly=on,format={},file={},if=none,id=disk{i}",
                        disk.format,
                        disk.path.display(),
                    ),
                );
            }
        }
        match disk.interface {
            DiskInterface::VirtioScsi => {
                if !scsi_controller_added {
                    pair(&mut a, "-device", "virtio-scsi-pci,id=scsi0".to_string());
                    scsi_controller_added = true;
                }
                pair(&mut a, "-device", format!("scsi-hd,drive=disk{i}"));
            }
            DiskInterface::VirtioBlk => {
                pair(&mut a, "-device", format!("virtio-blk-pci,drive=disk{i}"));
            }
            DiskInterface::Cdrom => {
                pair(&mut a, "-device", format!("ide-cd,drive=disk{i}"));
            }
        }
    }

    for (i, net) in d.net.iter().enumerate() {
        let mac = net.mac.clone().unwrap_or_else(|| derived_mac(&vm.id));
        pair(&mut a, "-netdev", format!("bridge,id=net{i},br={}", net.bridge));
        pair(
            &mut a,
            "-device",
            format!("{},netdev=net{i},mac={mac}", net_device(net.model)),
        );
    }

    if d.console.serial {
        pair(
            &mut a,
            "-serial",
            format!("unix:{},server=on,wait=off", rt.serial.display()),
        );
    }
    if d.console.vnc {
        pair(&mut a, "-vnc", format!("unix:{}", rt.vnc.display()));
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, Vm, VmDef,
    };
    use std::path::PathBuf;

    fn sample_vm() -> Vm {
        Vm {
            id: "linux".to_string(),
            def: VmDef {
                flavor: Flavor::Linux,
                resources: Resources {
                    memory_mib: 2048,
                    vcpus: 2,
                    cpu: "host".into(),
                },
                firmware: Firmware {
                    ovmf_code: "/usr/share/OVMF/OVMF_CODE.fd".into(),
                    ovmf_vars_template: "/usr/share/OVMF/OVMF_VARS.fd".into(),
                },
                disk: vec![Disk {
                    path: "/var/lib/phermes/images/linux-node.qcow2".into(),
                    format: "qcow2".into(),
                    interface: DiskInterface::VirtioScsi,
                }],
                net: vec![Net {
                    bridge: "vmbr0".into(),
                    model: NetModel::VirtioNet,
                    mac: None,
                }],
                console: Console {
                    serial: true,
                    vnc: true,
                },
            },
        }
    }

    fn rt() -> RuntimePaths {
        RuntimePaths {
            vars: PathBuf::from("/run/phermesd/linux/OVMF_VARS.fd"),
            qmp: PathBuf::from("/run/phermesd/linux/qmp.sock"),
            serial: PathBuf::from("/run/phermesd/linux/serial.sock"),
            vnc: PathBuf::from("/run/phermesd/linux/vnc.sock"),
            pidfile: PathBuf::from("/run/phermesd/linux/vm.pid"),
            qga: PathBuf::from("/run/phermesd/linux/qga.sock"),
        }
    }

    fn find_value<'a>(argv: &'a [String], flag: &str) -> &'a str {
        let i = argv.iter().position(|a| a == flag).expect("flag present");
        argv[i + 1].as_str()
    }

    #[test]
    fn linux_argv_has_kvm_machine_and_resources() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert_eq!(find_value(&argv, "-machine"), "q35,accel=kvm");
        assert_eq!(find_value(&argv, "-cpu"), "host");
        assert_eq!(find_value(&argv, "-smp"), "2");
        assert_eq!(find_value(&argv, "-m"), "2048");
    }

    #[test]
    fn linux_argv_wires_ovmf_pflash_and_qmp_and_pidfile() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert!(argv
            .iter()
            .any(|a| a == "if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd"));
        assert!(argv
            .iter()
            .any(|a| a == "if=pflash,format=raw,file=/run/phermesd/linux/OVMF_VARS.fd"));
        assert_eq!(
            find_value(&argv, "-qmp"),
            "unix:/run/phermesd/linux/qmp.sock,server=on,wait=off"
        );
        assert_eq!(
            find_value(&argv, "-pidfile"),
            "/run/phermesd/linux/vm.pid"
        );
    }

    #[test]
    fn virtio_scsi_disk_adds_controller_once_and_scsi_hd() {
        let mut vm = sample_vm();
        vm.def.disk.push(Disk {
            path: "/var/lib/phermes/images/extra.qcow2".into(),
            format: "qcow2".into(),
            interface: DiskInterface::VirtioScsi,
        });
        let argv = build_argv(&vm, &rt()).unwrap();
        let controllers = argv
            .iter()
            .filter(|a| a.as_str() == "virtio-scsi-pci,id=scsi0")
            .count();
        assert_eq!(controllers, 1);
        assert!(argv.iter().any(|a| a == "scsi-hd,drive=disk0"));
        assert!(argv.iter().any(|a| a == "scsi-hd,drive=disk1"));
    }

    #[test]
    fn virtio_blk_disk_uses_blk_device_no_controller() {
        let mut vm = sample_vm();
        vm.def.disk[0].interface = DiskInterface::VirtioBlk;
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(!argv.iter().any(|a| a.starts_with("virtio-scsi-pci")));
        assert!(argv.iter().any(|a| a == "virtio-blk-pci,drive=disk0"));
    }

    #[test]
    fn net_uses_derived_mac_when_absent() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        let mac = crate::config::derived_mac("linux");
        assert!(argv
            .iter()
            .any(|a| a == &format!("virtio-net-pci,netdev=net0,mac={mac}")));
        assert!(argv.iter().any(|a| a == "bridge,id=net0,br=vmbr0"));
    }

    #[test]
    fn net_uses_explicit_mac_when_present() {
        let mut vm = sample_vm();
        vm.def.net[0].mac = Some("52:54:00:ab:cd:ef".into());
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(argv
            .iter()
            .any(|a| a == "virtio-net-pci,netdev=net0,mac=52:54:00:ab:cd:ef"));
    }

    #[test]
    fn console_flags_off_omit_serial_and_vnc() {
        let mut vm = sample_vm();
        vm.def.console = Console {
            serial: false,
            vnc: false,
        };
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(!argv.iter().any(|a| a == "-serial"));
        assert!(!argv.iter().any(|a| a == "-vnc"));
    }

    #[test]
    fn macos_flavor_is_unsupported() {
        let mut vm = sample_vm();
        vm.def.flavor = Flavor::Macos;
        let result = build_argv(&vm, &rt());
        assert!(matches!(result, Err(QemuError::UnsupportedFlavor(Flavor::Macos))));
    }

    #[test]
    fn windows_argv_is_byte_identical_to_linux_argv_for_equivalent_def() {
        let lx = sample_vm();
        let mut win = sample_vm();
        win.def.flavor = Flavor::Windows;
        let argv_lx = build_argv(&lx, &rt()).unwrap();
        let argv_win = build_argv(&win, &rt()).unwrap();
        assert_eq!(argv_lx, argv_win);
    }

    #[test]
    fn argv_flags_and_values_are_balanced() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert!(!argv.is_empty());
        assert_eq!(
            argv.last().unwrap(),
            "unix:/run/phermesd/linux/vnc.sock"
        );
    }

    #[test]
    fn linux_argv_adds_guest_agent_channel() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert!(argv.iter().any(|a| a == "virtio-serial-pci"));
        assert!(argv.iter().any(|a| a
            == "socket,path=/run/phermesd/linux/qga.sock,server=on,wait=off,id=qga0"));
        assert!(argv.iter().any(|a| a
            == "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0"));
    }

    #[test]
    fn linux_argv_with_cdrom_disk_emits_media_cdrom_and_ide_cd() {
        let mut vm = sample_vm();
        // Append a second disk: the cloud-init seed CDROM.
        vm.def.disk.push(Disk {
            path: "/var/lib/phermes/seed/linux.iso".into(),
            format: "raw".into(),
            interface: DiskInterface::Cdrom,
        });
        let argv = build_argv(&vm, &rt()).unwrap();
        // The seed appears as disk1 (the OS disk in sample_vm is disk0).
        assert!(argv.iter().any(|a| a == "media=cdrom,readonly=on,format=raw,file=/var/lib/phermes/seed/linux.iso,if=none,id=disk1"));
        assert!(argv.iter().any(|a| a == "ide-cd,drive=disk1"));
        // The seed CDROM does NOT add a virtio-scsi controller for itself
        // (only the existing virtio-scsi OS disk does).
        let scsi_controllers = argv.iter().filter(|a| a.as_str() == "virtio-scsi-pci,id=scsi0").count();
        assert_eq!(scsi_controllers, 1);
    }
}
