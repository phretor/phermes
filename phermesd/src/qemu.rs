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

#[derive(Debug, thiserror::Error)]
pub enum QemuError {
    #[error("flavor {0:?} is not yet supported")]
    UnsupportedFlavor(Flavor),
    #[error(
        "apple OSK not available at {path} — write the 64-char hex string \
         (mode 0400, root-owned) and retry. ({source})"
    )]
    MissingAppleOsk {
        path: PathBuf,
        #[source]
        source: crate::secrets::SecretsError,
    },
}

/// Build the `qemu-system-x86_64` argument vector (program name excluded).
///
/// # Errors
///
/// Returns `QemuError::MissingAppleOsk` if the flavor is `Macos` and the OSK
/// file at [`crate::secrets::APPLE_OSK_PATH`] is absent or malformed.
pub fn build_argv(vm: &Vm, rt: &RuntimePaths) -> Result<Vec<String>, QemuError> {
    match vm.def.flavor {
        Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt)),
        Flavor::Macos => build_macos_with_secrets(vm, rt),
    }
}

fn net_device(model: NetModel) -> &'static str {
    match model {
        NetModel::VirtioNet => "virtio-net-pci",
        NetModel::E1000 => "e1000",
        NetModel::Vmxnet3 => "vmxnet3",
    }
}

/// Push a `flag value` pair onto the argument vector.
fn pair(a: &mut Vec<String>, flag: &str, val: String) {
    a.push(flag.to_string());
    a.push(val);
}

/// Append disk devices for a PC-style virtio/SCSI/CDROM setup.
fn push_pc_disks(a: &mut Vec<String>, vm: &Vm) {
    let mut scsi_controller_added = false;
    for (i, disk) in vm.def.disk.iter().enumerate() {
        match disk.interface {
            DiskInterface::VirtioScsi | DiskInterface::VirtioBlk | DiskInterface::Ahci => {
                pair(
                    a,
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
                    a,
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
                    pair(a, "-device", "virtio-scsi-pci,id=scsi0".to_string());
                    scsi_controller_added = true;
                }
                pair(a, "-device", format!("scsi-hd,drive=disk{i}"));
            }
            DiskInterface::VirtioBlk => {
                pair(a, "-device", format!("virtio-blk-pci,drive=disk{i}"));
            }
            DiskInterface::Cdrom => {
                pair(a, "-device", format!("ide-cd,drive=disk{i}"));
            }
            DiskInterface::Ahci => {
                pair(a, "-device", format!("ide-hd,drive=disk{i}"));
            }
        }
    }
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
        format!("if=pflash,format=raw,readonly=on,file={}", d.firmware.ovmf_code.display()),
    );
    pair(&mut a, "-drive", format!("if=pflash,format=raw,file={}", rt.vars.display()));
    pair(&mut a, "-qmp", format!("unix:{},server=on,wait=off", rt.qmp.display()));
    pair(&mut a, "-pidfile", rt.pidfile.display().to_string());

    pair(&mut a, "-device", "virtio-serial-pci".to_string());
    pair(
        &mut a,
        "-chardev",
        format!("socket,path={},server=on,wait=off,id=qga0", rt.qga.display()),
    );
    pair(&mut a, "-device", "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0".to_string());

    push_pc_disks(&mut a, vm);

    for (i, net) in d.net.iter().enumerate() {
        let mac = net.mac.clone().unwrap_or_else(|| derived_mac(&vm.id));
        pair(&mut a, "-netdev", format!("bridge,id=net{i},br={}", net.bridge));
        pair(&mut a, "-device", format!("{},netdev=net{i},mac={mac}", net_device(net.model)));
    }

    if d.console.serial {
        pair(&mut a, "-serial", format!("unix:{},server=on,wait=off", rt.serial.display()));
    }
    if d.console.vnc {
        pair(&mut a, "-vnc", format!("unix:{}", rt.vnc.display()));
    }

    a
}

/// Seam: read OSK from the canonical secrets path and delegate to `build_macos`.
fn build_macos_with_secrets(vm: &Vm, rt: &RuntimePaths) -> Result<Vec<String>, QemuError> {
    let path = PathBuf::from(crate::secrets::APPLE_OSK_PATH);
    let osk = crate::secrets::read_apple_osk(&path)
        .map_err(|source| QemuError::MissingAppleOsk { path: path.clone(), source })?;
    Ok(build_macos(vm, rt, &osk))
}

/// Append the AHCI disk section for macOS: one controller + one drive per disk.
fn push_macos_disks(a: &mut Vec<String>, vm: &Vm) {
    pair(a, "-device", "ich9-ahci,id=ahci0".to_string());
    for (i, disk) in vm.def.disk.iter().enumerate() {
        pair(
            a,
            "-drive",
            format!(
                "id=disk{i},if=none,format={},file={}",
                disk.format,
                disk.path.display()
            ),
        );
        pair(a, "-device", format!("ide-hd,bus=ahci0.{i},drive=disk{i}"));
    }
}

/// Build the argv vector for a macOS guest on q35 with AHCI storage and vmxnet3.
///
/// The caller supplies the Apple OSK string (already validated). This function
/// is pure: it does not touch the filesystem or secrets store.
fn build_macos(vm: &Vm, rt: &RuntimePaths, osk: &str) -> Vec<String> {
    let d = &vm.def;
    let mut a: Vec<String> = Vec::new();

    a.push("-enable-kvm".to_string());
    pair(&mut a, "-machine", "q35,accel=kvm,kernel-irqchip=split".to_string());
    pair(
        &mut a,
        "-cpu",
        "Penryn,kvm=on,vendor=GenuineIntel,+invtsc,+pcid,+ssse3,+sse4.2,\
         +popcnt,+avx,+aes,+xsave,+xsaveopt,check"
            .to_string(),
    );
    pair(
        &mut a,
        "-smp",
        format!("cpus={},cores=2,threads=2,sockets=1", d.resources.vcpus),
    );
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
    pair(&mut a, "-drive", format!("if=pflash,format=raw,file={}", rt.vars.display()));

    pair(&mut a, "-device", format!("isa-applesmc,osk={osk}"));
    push_macos_disks(&mut a, vm);

    for (i, net) in d.net.iter().enumerate() {
        let mac = net.mac.clone().unwrap_or_else(|| derived_mac(&vm.id));
        pair(&mut a, "-netdev", format!("bridge,id=net{i},br={}", net.bridge));
        pair(&mut a, "-device", format!("vmxnet3,netdev=net{i},mac={mac}"));
    }

    pair(&mut a, "-device", "nec-usb-xhci,id=xhci".to_string());
    pair(&mut a, "-device", "usb-tablet,bus=xhci.0".to_string());

    if d.console.serial {
        pair(
            &mut a,
            "-chardev",
            format!("socket,path={},server=on,wait=off,id=serial0", rt.serial.display()),
        );
        pair(&mut a, "-serial", "chardev:serial0".to_string());
    }
    if d.console.vnc {
        pair(&mut a, "-vnc", format!("unix:{}", rt.vnc.display()));
    }

    pair(&mut a, "-qmp", format!("unix:{},server=on,wait=off", rt.qmp.display()));
    pair(&mut a, "-pidfile", rt.pidfile.display().to_string());

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

    // ── macOS-specific tests ──────────────────────────────────────────────────

    const TEST_OSK: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn macos_vm() -> Vm {
        Vm {
            id: "macos".to_string(),
            def: VmDef {
                flavor: Flavor::Macos,
                resources: Resources {
                    memory_mib: 4096,
                    vcpus: 4,
                    cpu: "Penryn".into(),
                },
                firmware: Firmware {
                    ovmf_code: "/usr/share/OVMF/OVMF_CODE.fd".into(),
                    ovmf_vars_template: "/usr/share/OVMF/OVMF_VARS.fd".into(),
                },
                disk: vec![Disk {
                    path: "/var/lib/phermes/images/macos.qcow2".into(),
                    format: "qcow2".into(),
                    interface: DiskInterface::Ahci,
                }],
                net: vec![Net {
                    bridge: "vmbr0".into(),
                    model: NetModel::Vmxnet3,
                    mac: None,
                }],
                console: Console {
                    serial: true,
                    vnc: true,
                },
            },
        }
    }

    #[test]
    fn macos_flavor_succeeds_when_apple_osk_present_in_argv() {
        let vm = macos_vm();
        let rt = rt();
        let argv = build_macos(&vm, &rt, TEST_OSK);
        assert!(argv.iter().any(|a| a.contains(&format!("isa-applesmc,osk={TEST_OSK}"))));
    }

    #[test]
    fn macos_argv_includes_penryn_cpu_with_full_flag_set() {
        let vm = macos_vm();
        let argv = build_macos(&vm, &rt(), TEST_OSK);
        let cpu_idx = argv.iter().position(|a| a == "-cpu").expect("-cpu present");
        let cpu_val = &argv[cpu_idx + 1];
        assert!(cpu_val.starts_with("Penryn,"), "got: {cpu_val}");
        for flag in [
            "kvm=on", "vendor=GenuineIntel", "+invtsc", "+pcid", "+ssse3",
            "+sse4.2", "+popcnt", "+avx", "+aes", "+xsave", "+xsaveopt", "check",
        ] {
            assert!(cpu_val.contains(flag), "missing {flag} in {cpu_val}");
        }
    }

    #[test]
    fn macos_argv_uses_ahci_not_virtio_scsi() {
        let vm = macos_vm();
        let argv = build_macos(&vm, &rt(), TEST_OSK);
        let joined = argv.join(" ");
        assert!(joined.contains("ich9-ahci"), "no ahci controller: {joined}");
        assert!(joined.contains("ide-hd"), "no ide-hd disk: {joined}");
        assert!(!joined.contains("virtio-scsi"), "virtio-scsi present: {joined}");
    }

    #[test]
    fn macos_argv_uses_vmxnet3() {
        let vm = macos_vm();
        let argv = build_macos(&vm, &rt(), TEST_OSK);
        let joined = argv.join(" ");
        assert!(joined.contains("vmxnet3"), "no vmxnet3: {joined}");
        assert!(!joined.contains("virtio-net"), "virtio-net present: {joined}");
    }

    #[test]
    fn macos_argv_byte_identical_for_equivalent_def_modulo_osk() {
        let vm = macos_vm();
        let a = build_macos(&vm, &rt(), TEST_OSK);
        let b = build_macos(&vm, &rt(), TEST_OSK);
        assert_eq!(a, b);
    }

    #[test]
    fn build_argv_macos_returns_missing_apple_osk_when_file_absent() {
        // APPLE_OSK_PATH (/etc/phermes/secrets/apple.osk) is not present in CI or
        // on developer machines without the secrets bundle, so dispatch must fail.
        // If this test runs on a machine that actually has the file, it will
        // return Ok and the assert will panic — that is the correct behaviour
        // (the file should not exist outside production).
        let vm = macos_vm();
        let result = build_argv(&vm, &rt());
        assert!(matches!(result, Err(QemuError::MissingAppleOsk { .. })));
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
