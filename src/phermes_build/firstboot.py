import json
import os

MOTD_TEMPLATE = """\
 ____  _   _
|  _ \\| | | | ___ _ __ _ __ ___   ___  ___
| |_) | |_| |/ _ \\ '__| '_ ` _ \\ / _ \\/ __|
|  __/|  _  |  __/ |  | | | | | |  __/\\__ \\
|_|   |_| |_|\\___|_|  |_| |_| |_|\\___|___/

 [ENCRYPTED]  [KVM-ISOLATED]  [SELF-HOSTED]  [PORTABLE]

 PHermes — Connect from any browser on this network:
   https://{hostname}.local
   https://{ip_hint}

"""


def write_firstboot_flag(data_mount: str) -> None:
    flag_path = os.path.join(data_mount, "firstboot.flag")
    with open(flag_path, "w") as f:
        json.dump({"status": "pending"}, f)


def write_motd(chroot_mount: str, hostname: str, ip_hint: str) -> None:
    etc_path = os.path.join(chroot_mount, "etc")
    os.makedirs(etc_path, exist_ok=True)
    issue_path = os.path.join(etc_path, "issue")
    with open(issue_path, "w") as f:
        f.write(MOTD_TEMPLATE.format(hostname=hostname, ip_hint=ip_hint))
