import subprocess
from collections.abc import Sequence


class CommandError(Exception):
    def __init__(self, cmd: list[str], returncode: int, stderr: str) -> None:
        self.cmd = cmd
        self.returncode = returncode
        self.stderr = stderr
        super().__init__(
            f"Command {cmd[0]!r} failed (exit {returncode}): {stderr.strip()}"
        )


def run_cmd(
    cmd: Sequence[str], *, input: str | None = None, check: bool = True
) -> str:  # noqa: A002
    result = subprocess.run(
        list(cmd),
        capture_output=True,
        text=True,
        input=input,
    )
    if check and result.returncode != 0:
        raise CommandError(list(cmd), result.returncode, result.stderr)
    return result.stdout.strip()
