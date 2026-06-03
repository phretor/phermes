# PHermes Threat Model (STRIDE)

Status: living document. Reflects the architecture as designed and the Phase 1 build as
implemented. It states what PHermes protects, what it explicitly does **not**, and where
the trust boundaries sit — so the security posture is a decision, not an accident.

## What PHermes is, for threat-modeling purposes

A portable, LUKS2-encrypted SSD that boots `EFI → GRUB → Proxmox VE host → guest VM →
Hermes Agent`. By design it can boot on **arbitrary KVM-capable hardware** (the "portable"
model). The agent and its data live inside the guest; the host stays minimal.

### Assets

| # | Asset | Why it matters |
|---|---|---|
| A1 | Encrypted data partition (Hermes profiles/tasks, knowledge base, documents) | The user's accumulated work; confidential. |
| A2 | LLM API keys / provider credentials | Stored in `@phermes/config.yaml`, injected into the guest. |
| A3 | The LUKS passphrase | Single secret guarding A1/A2 at rest. |
| A4 | Boot-chain integrity (GRUB, kernel, initramfs) | Tampering here defeats everything downstream. |
| A5 | Host (Proxmox) integrity | Hosts the VMs; compromise = control of all guests. |
| A6 | The running agent and its capabilities (code execution, computer use, network) | A compromised/injected agent acts with the guest's full authority. |

### Trust anchors (what we trust, and do not verify)

- **TA1 — Platform firmware**: UEFI/BIOS, and below it SMM, the Intel ME / AMD PSP, and
  microcode. We do not measure or attest any of it; we inherit whatever root of trust the
  OEM provides (Boot Guard / PSP verified boot, if present). **In the portable model this
  means trusting *each host's* firmware**, which may be hostile.
- **TA2 — The plaintext boot chain**: the ESP and `/boot` (GRUB, kernel, initramfs) are
  **unencrypted and unsigned**. No Secure Boot, no measured boot, no TPM. The firmware
  loads an unverified bootloader.
- **TA3 — The operator**: supplies the LUKS passphrase and does not boot the device on
  hardware they consider hostile.
- **TA4 — Upstream sources**: Debian + Proxmox community repos, the Python/uv ecosystem,
  and (dev only) the bundled cloud image, fetched over TLS at build time.

### Trust boundaries

- **TB1** Host hardware/firmware ↔ PHermes (you trust the machine you boot on).
- **TB2** Plaintext boot chain ↔ encrypted root — the unlock moment.
- **TB3** Proxmox host ↔ guest VMs — hypervisor isolation.
- **TB4** Guest VM (Hermes) ↔ external LLM/providers — network egress.
- **TB5** Appliance ↔ LAN — PHermes web UI (443), mDNS (`phermes.local`).
- **TB6** `phermes-build` ↔ the produced disk — supply chain.
- **TB7** Persistent data ↔ VM-flavor switches — data reused across guest contexts.

## Deployment models

The posture differs sharply by how PHermes is deployed:

- **Portable** (the default goal): boots on arbitrary machines. TA1/TA2 are effectively
  unenforceable — you cannot pin to a platform you don't own. Boot-integrity and
  host-firmware threats are largely **out of scope to defend**; the LUKS passphrase and
  physical control carry the weight.
- **Fixed install** (a future, opt-in mode): deployed to one owned machine. Enables the
  standard boot-integrity defenses (Secure Boot + UKI, TPM2 measured boot with PCR-sealed
  unlock), which are impractical for a roaming device. This is the only model in which
  signing the initramfs (via a Unified Kernel Image) is meaningful.

## STRIDE analysis

Legend — **Status**: ✅ mitigated · 🟡 partial / by design · ⛔ accepted (out of scope) ·
🔭 roadmap.

### Spoofing

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| Evil-maid fake passphrase prompt captures the LUKS passphrase | TB2 | None in portable. Fixed-install: TPM-sealed anti-evil-maid (boot displays a sealed secret / unlock gated on PCRs) | ⛔ portable · 🔭 fixed |
| Rogue `phermes.local` (mDNS) lures the user to an attacker UI on the LAN | TB5 | LAN assumed semi-trusted; mitigate with pinned TLS + a UI shown only on first boot | 🟡 |
| MITM of an LLM provider endpoint | TB4 | Providers reached over TLS; system CA trust | 🟡 |
| SSH host-identity spoof when reaching the host/node | TB5 | Dev access is key-based; production locks root → no root SSH | ✅ prod · 🟡 dev |

### Tampering

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| Modify GRUB/kernel/initramfs on plaintext `/boot` to backdoor the unlock (evil-maid) | TB2 | **None** (no Secure Boot / measured boot). Encrypting `/boot` does not help — it relocates the attack to the still-plaintext GRUB and forces GRUB's weaker PBKDF2 KDF | ⛔ portable · 🔭 fixed (UKI + Secure Boot + TPM) |
| Firmware/SMM/ME-PSP implant | TB1 | Trusted (TA1); not verified | ⛔ |
| Bit-flipping ciphertext at rest (dm-crypt is malleable; no authenticated encryption) | A1 | LUKS2 confidentiality only; meaningful tampering needs the key, and corruption surfaces as fs errors. Optional `dm-integrity` (AEAD) not enabled — perf cost | 🟡 · 🔭 |
| A compromised host alters VM images / the data partition | TB3 | Host kept minimal and not network-exposed (see EoP); data on encrypted partition | 🟡 |

### Repudiation

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| No record of who unlocked / accessed the device | — | Single-user appliance; `journald` + Proxmox task logs provide local audit | 🟡 by design |
| Agent actions are unattributable | TB4/A6 | Out of PHermes scope; relies on Hermes' own logging | 🟡 |

### Information Disclosure

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| Device/drive theft exposes data and keys (A1, A2) | TB1 | **LUKS2 full-disk encryption (Argon2id)** — the primary security claim. Encrypted swap inside the container | ✅ |
| Hostile host firmware/DMA reads RAM after unlock (keys, plaintext data) | TB1 | None — post-unlock RAM is exposed to a malicious platform | ⛔ portable |
| `PHERMES_SHARE` (exFAT) is plaintext by default for cross-OS drag-and-drop | A1 | Documented tradeoff; `--share-encrypted` places it inside LUKS | 🟡 by design |
| Samba / Proxmox UI exposed beyond intent | TB5 | Samba bound to `vmbr0` only; Proxmox UI (8006) firewalled to localhost; only the PHermes UI (443) faces the LAN | ✅ |
| Known credentials leak from a shipped build | TB6 | Production locks root and **requires an operator LUKS passphrase**; all known/test credentials gated behind `--dev-credentials` | ✅ |

### Denial of Service

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| Lost/forgotten passphrase → data unrecoverable | A3 | No backdoor / key escrow — intentional. User's responsibility (optional second LUKS keyslot for backup) | 🟡 by design |
| Bad update or failed VM switch bricks the system | TB7 | Snapshot-before-change: LVM-thin (VM disk) + Btrfs (`@overlay`) snapshots; one-command rollback | ✅ |
| A guest exhausts host resources | TB3 | Proxmox/cgroup limits per VM | 🟡 |
| LLM provider outage / rate limiting degrades the agent | TB4 | External dependency; local model backends as fallback | ⛔ external |

### Elevation of Privilege

| Threat | Boundary | Mitigation | Status |
|---|---|---|---|
| Guest VM escapes to the host (QEMU/KVM vuln) | TB3 | Host minimal and trusted; agent runs **in the guest, never on the host**; `phermes update` keeps QEMU/KVM patched | 🟡 |
| Host compromise via the management plane | TB5 | Proxmox UI localhost-only; restricted `PHermesUser` RBAC role; the PHermes UI is the sole exposed interface — itself attack surface (Phase 2 hardening) | 🟡 · 🔭 |
| Dev root SSH abused | TB6 | Dev-only, key-gated; production locks root and keeps default `prohibit-password` | ✅ prod |
| Prompt-injected or malicious agent acts with full guest authority (code exec, computer use, LAN, egress) | A6/TB4 | Blast radius confined to the guest VM by hypervisor isolation; host stays out of reach | 🟡 — the guest *is* the trust boundary |
| Compromised builder or upstream injects into the image | TB6 | Signed `phermes-build`; official upstream repos; no bundled binaries; pinned deps | 🟡 |

## Key residual risks (priority order)

1. **Boot-chain integrity / evil-maid (TB2)** — unaddressed in the portable model and not
   fully addressable there. *Roadmap:* a Fixed-install mode with Secure Boot + signed UKI +
   TPM2 PCR-sealed unlock.
2. **Hostile host firmware on untrusted hardware (TB1)** — inherent to "boot on anything";
   mitigated only by operator discipline (don't boot on machines you don't trust).
3. **The agent as its own blast radius (A6)** — a compromised guest agent has broad
   capability; isolation keeps it off the host but not off the LAN or the data partition.
4. **PHermes web UI attack surface (TB5)** — built in Phase 2; must ship with TLS, auth,
   and a tight exposure model.
5. **At-rest integrity (Tampering/A1)** — confidentiality without authenticated encryption;
   `dm-integrity` is the option if integrity-at-rest becomes a requirement.

## Out of scope (explicitly)

Platform firmware/SMM/ME-PSP implants; physical RAM extraction (cold-boot, DMA) on a
hostile host while unlocked; supply-chain compromise of Debian/Proxmox/PyPI upstreams;
and the internal security of Hermes Agent itself and the LLM providers it talks to.
