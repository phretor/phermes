#!/usr/bin/env bash
# Mount every PHermes partition and assert expected content.
#
# Runs inside the phermes-build container (privileged, host /dev shared).
# Reports a pass/fail per check and exits non-zero if any check fails. All
# checks are guarded so one failure never aborts the rest; the trap tears down
# any LUKS/LVM/mount state this script created.
set -euo pipefail

DISK="${1:?usage: smoke-verify.sh <disk>}"
LVM_NOUDEV="activation { udev_sync = 0 udev_rules = 0 }"
PASSPHRASE="phermes-change-me"
pass=0
fail=0

ok() {
  echo "  ✓ $1"
  pass=$((pass + 1))
}

no() {
  echo "  ✗ $1"
  fail=$((fail + 1))
}

cleanup() {
  umount /mnt/efi /mnt/boot /mnt/root /mnt/data 2>/dev/null || true
  vgchange -an --config "$LVM_NOUDEV" pve >/dev/null 2>&1 || true
  cryptsetup luksClose phermes_luks 2>/dev/null || true
}
trap cleanup EXIT

mkdir -p /mnt/efi /mnt/boot /mnt/root /mnt/data

echo "EFI system partition (${DISK}p1):"
if mount -o ro "${DISK}p1" /mnt/efi 2>/dev/null; then
  if [ -f /mnt/efi/EFI/BOOT/BOOTX64.EFI ]; then
    ok "removable GRUB present (EFI/BOOT/BOOTX64.EFI)"
  else
    no "removable GRUB missing"
  fi
else
  no "could not mount EFI partition"
fi

echo "Boot partition (${DISK}p2):"
if mount -o ro "${DISK}p2" /mnt/boot 2>/dev/null; then
  if compgen -G "/mnt/boot/vmlinuz*" >/dev/null; then
    ok "kernel present (vmlinuz)"
  else
    no "kernel missing"
  fi
  if [ -f /mnt/boot/grub/grub.cfg ]; then
    ok "grub.cfg present"
  else
    no "grub.cfg missing"
  fi
else
  no "could not mount boot partition"
fi

echo "LUKS + LVM (${DISK}p3):"
if [ -e /dev/mapper/phermes_luks ]; then
  ok "LUKS mapping already open"
elif printf '%s' "$PASSPHRASE" | cryptsetup luksOpen --key-file - "${DISK}p3" phermes_luks 2>/dev/null; then
  ok "LUKS opens with build passphrase"
else
  no "LUKS did not open"
fi

if [ -e /dev/mapper/phermes_luks ]; then
  vgchange -ay --config "$LVM_NOUDEV" pve >/dev/null 2>&1 || true

  if mount -o ro /dev/pve/root /mnt/root 2>/dev/null; then
    if grep -q phermes_luks /mnt/root/etc/crypttab 2>/dev/null; then
      ok "/etc/crypttab references phermes_luks"
    else
      no "/etc/crypttab missing or wrong"
    fi
    if grep -q 'LABEL=boot' /mnt/root/etc/fstab 2>/dev/null; then
      ok "/etc/fstab has boot entry"
    else
      no "/etc/fstab missing or wrong"
    fi
    if [ -d /mnt/root/usr/bin ]; then
      ok "Debian base system present"
    else
      no "Debian base missing"
    fi
  else
    no "could not mount root LV"
  fi

  if mount -o ro /dev/pve/btrfs-data /mnt/data 2>/dev/null; then
    for sv in @overlay @phermes @snapshots; do
      if btrfs subvolume list /mnt/data 2>/dev/null | grep -q "path ${sv}\$"; then
        ok "btrfs subvolume ${sv} present"
      else
        no "btrfs subvolume ${sv} missing"
      fi
    done
  else
    no "could not mount btrfs-data"
  fi
fi

echo
echo "Passed: ${pass}   Failed: ${fail}"
[ "$fail" -eq 0 ]
