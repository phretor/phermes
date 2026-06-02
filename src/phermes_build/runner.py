import subprocess
import sys
from collections.abc import Sequence

_VERBOSE = False


def set_verbose(value: bool) -> None:
    """Toggle live streaming of command output for all subsequent run_cmd calls."""
    global _VERBOSE
    _VERBOSE = value


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
    if _VERBOSE:
        return _run_streaming(cmd, input=input, check=check)
    result = subprocess.run(
        list(cmd),
        capture_output=True,
        text=True,
        input=input,
    )
    if check and result.returncode != 0:
        raise CommandError(list(cmd), result.returncode, result.stderr)
    return result.stdout.strip()


def _run_streaming(
    cmd: Sequence[str], *, input: str | None, check: bool  # noqa: A002
) -> str:
    """Run a command echoing its output live while still capturing it.

    stderr is merged into stdout so the full transcript appears in order. The
    captured text is returned so callers that parse output keep working.
    """
    cmd = list(cmd)
    print(f"$ {' '.join(cmd)}", file=sys.stderr, flush=True)
    proc = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE if input is not None else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if input is not None and proc.stdin is not None:
        proc.stdin.write(input)
        proc.stdin.close()
    captured: list[str] = []
    if proc.stdout is not None:
        for line in proc.stdout:
            sys.stdout.write(line)
            sys.stdout.flush()
            captured.append(line)
    proc.wait()
    output = "".join(captured)
    if check and proc.returncode != 0:
        raise CommandError(cmd, proc.returncode, output)
    return output.strip()
