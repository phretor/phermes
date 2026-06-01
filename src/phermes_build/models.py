from enum import StrEnum

from pydantic import BaseModel, field_validator, model_validator


class VMFlavor(StrEnum):
    MACOS = "macos"
    WINDOWS = "windows"
    LINUX = "linux"


class AcquisitionMode(StrEnum):
    DOWNLOAD = "download"
    IMPORT = "import"
    SKIP = "skip"


class VMConfig(BaseModel):
    flavor: VMFlavor
    mode: AcquisitionMode
    image_path: str | None = None

    @model_validator(mode="after")
    def import_requires_path(self) -> "VMConfig":
        if self.mode == AcquisitionMode.IMPORT and self.image_path is None:
            raise ValueError("image_path required when mode is 'import'")
        return self


class DiskLayout(BaseModel):
    disk: str
    disk_size_gb: int
    efi_mb: int = 512
    boot_mb: int = 1024
    swap_gb: int = 16
    lvm_gb: int
    data_gb: int
    share_gb: int = 0
    share_encrypted: bool = False


class BuildConfig(BaseModel):
    disk: str
    share_size_gb: int = 250
    share_encrypted: bool = False
    vms: list[VMConfig] = []
    temp_luks_passphrase: str = "phermes-change-me"

    @field_validator("disk")
    @classmethod
    def disk_must_be_block_device(cls, v: str) -> str:
        if not v.startswith("/dev/"):
            raise ValueError(f"disk must be a /dev/ path, got {v!r}")
        return v
