from phermes_build.runner import run_cmd

LUKS_NAME = "phermes_luks"


def mapper_path(name: str) -> str:
    return f"/dev/mapper/{name}"


def format_luks(device: str, passphrase: str, name: str = LUKS_NAME) -> None:
    # name is accepted for API symmetry with open_luks but unused by luksFormat
    run_cmd(
        ["cryptsetup", "luksFormat", "--type", "luks2", "--batch-mode", "--key-file", "-", device],
        input=passphrase,
    )


def open_luks(device: str, name: str, passphrase: str) -> str:
    run_cmd(
        ["cryptsetup", "luksOpen", "--key-file", "-", device, name],
        input=passphrase,
    )
    return mapper_path(name)


def close_luks(name: str) -> None:
    run_cmd(["cryptsetup", "luksClose", name])


def add_passphrase(device: str, old_passphrase: str, new_passphrase: str) -> None:
    run_cmd(
        ["cryptsetup", "luksAddKey", "--key-file", "-", device],
        input=f"{old_passphrase}\n{new_passphrase}",
    )


def remove_passphrase(device: str, passphrase: str) -> None:
    run_cmd(
        ["cryptsetup", "luksRemoveKey", "--key-file", "-", device],
        input=passphrase,
    )
