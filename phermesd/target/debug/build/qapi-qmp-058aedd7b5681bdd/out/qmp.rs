
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QapiErrorClass {
	#[serde(rename = "GenericError")] GenericError,
	#[serde(rename = "CommandNotFound")] CommandNotFound,
	#[serde(rename = "DeviceNotActive")] DeviceNotActive,
	#[serde(rename = "DeviceNotFound")] DeviceNotFound,
	#[serde(rename = "KVMMissingCap")] KVMMissingCap,
}

impl ::core::str::FromStr for QapiErrorClass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QapiErrorClass {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

QapiErrorClass::GenericError,
QapiErrorClass::CommandNotFound,
QapiErrorClass::DeviceNotActive,
QapiErrorClass::DeviceNotFound,
QapiErrorClass::KVMMissingCap,

    ];
    const NAMES: &'static [&'static str] = &[

"GenericError",
"CommandNotFound",
"DeviceNotActive",
"DeviceNotFound",
"KVMMissingCap",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IoOperationType {
	#[serde(rename = "read")] read,
	#[serde(rename = "write")] write,
}

impl ::core::str::FromStr for IoOperationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for IoOperationType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

IoOperationType::read,
IoOperationType::write,

    ];
    const NAMES: &'static [&'static str] = &[

"read",
"write",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnOffAuto {
	#[serde(rename = "auto")] auto,
	#[serde(rename = "on")] on,
	#[serde(rename = "off")] off,
}

impl ::core::str::FromStr for OnOffAuto {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for OnOffAuto {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

OnOffAuto::auto,
OnOffAuto::on,
OnOffAuto::off,

    ];
    const NAMES: &'static [&'static str] = &[

"auto",
"on",
"off",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnOffSplit {
	#[serde(rename = "on")] on,
	#[serde(rename = "off")] off,
	#[serde(rename = "split")] split,
}

impl ::core::str::FromStr for OnOffSplit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for OnOffSplit {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

OnOffSplit::on,
OnOffSplit::off,
OnOffSplit::split,

    ];
    const NAMES: &'static [&'static str] = &[

"on",
"off",
"split",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrOrNull {
	#[serde(rename = "n")] n(()),
	#[serde(rename = "s")] s(::std::string::String),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OffAutoPCIBAR {
	#[serde(rename = "off")] off,
	#[serde(rename = "auto")] auto,
	#[serde(rename = "bar0")] bar0,
	#[serde(rename = "bar1")] bar1,
	#[serde(rename = "bar2")] bar2,
	#[serde(rename = "bar3")] bar3,
	#[serde(rename = "bar4")] bar4,
	#[serde(rename = "bar5")] bar5,
}

impl ::core::str::FromStr for OffAutoPCIBAR {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for OffAutoPCIBAR {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 8;
    const VARIANTS: &'static [Self] = &[

OffAutoPCIBAR::off,
OffAutoPCIBAR::auto,
OffAutoPCIBAR::bar0,
OffAutoPCIBAR::bar1,
OffAutoPCIBAR::bar2,
OffAutoPCIBAR::bar3,
OffAutoPCIBAR::bar4,
OffAutoPCIBAR::bar5,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"auto",
"bar0",
"bar1",
"bar2",
"bar3",
"bar4",
"bar5",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PCIELinkSpeed {
	#[serde(rename = "2_5")] _2_5,
	#[serde(rename = "5")] _5,
	#[serde(rename = "8")] _8,
	#[serde(rename = "16")] _16,
	#[serde(rename = "32")] _32,
	#[serde(rename = "64")] _64,
}

impl ::core::str::FromStr for PCIELinkSpeed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for PCIELinkSpeed {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

PCIELinkSpeed::_2_5,
PCIELinkSpeed::_5,
PCIELinkSpeed::_8,
PCIELinkSpeed::_16,
PCIELinkSpeed::_32,
PCIELinkSpeed::_64,

    ];
    const NAMES: &'static [&'static str] = &[

"2_5",
"5",
"8",
"16",
"32",
"64",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PCIELinkWidth {
	#[serde(rename = "1")] _1,
	#[serde(rename = "2")] _2,
	#[serde(rename = "4")] _4,
	#[serde(rename = "8")] _8,
	#[serde(rename = "12")] _12,
	#[serde(rename = "16")] _16,
	#[serde(rename = "32")] _32,
}

impl ::core::str::FromStr for PCIELinkWidth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for PCIELinkWidth {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

PCIELinkWidth::_1,
PCIELinkWidth::_2,
PCIELinkWidth::_4,
PCIELinkWidth::_8,
PCIELinkWidth::_12,
PCIELinkWidth::_16,
PCIELinkWidth::_32,

    ];
    const NAMES: &'static [&'static str] = &[

"1",
"2",
"4",
"8",
"12",
"16",
"32",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HostMemPolicy {
	#[serde(rename = "default")] default,
	#[serde(rename = "preferred")] preferred,
	#[serde(rename = "bind")] bind,
	#[serde(rename = "interleave")] interleave,
}

impl ::core::str::FromStr for HostMemPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HostMemPolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

HostMemPolicy::default,
HostMemPolicy::preferred,
HostMemPolicy::bind,
HostMemPolicy::interleave,

    ];
    const NAMES: &'static [&'static str] = &[

"default",
"preferred",
"bind",
"interleave",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetFilterDirection {
	#[serde(rename = "all")] all,
	#[serde(rename = "rx")] rx,
	#[serde(rename = "tx")] tx,
}

impl ::core::str::FromStr for NetFilterDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NetFilterDirection {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

NetFilterDirection::all,
NetFilterDirection::rx,
NetFilterDirection::tx,

    ];
    const NAMES: &'static [&'static str] = &[

"all",
"rx",
"tx",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrabToggleKeys {
	#[serde(rename = "ctrl-ctrl")] ctrl_ctrl,
	#[serde(rename = "alt-alt")] alt_alt,
	#[serde(rename = "shift-shift")] shift_shift,
	#[serde(rename = "meta-meta")] meta_meta,
	#[serde(rename = "scrolllock")] scrolllock,
	#[serde(rename = "ctrl-scrolllock")] ctrl_scrolllock,
}

impl ::core::str::FromStr for GrabToggleKeys {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for GrabToggleKeys {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

GrabToggleKeys::ctrl_ctrl,
GrabToggleKeys::alt_alt,
GrabToggleKeys::shift_shift,
GrabToggleKeys::meta_meta,
GrabToggleKeys::scrolllock,
GrabToggleKeys::ctrl_scrolllock,

    ];
    const NAMES: &'static [&'static str] = &[

"ctrl-ctrl",
"alt-alt",
"shift-shift",
"meta-meta",
"scrolllock",
"ctrl-scrolllock",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetworkAddressFamily {
	#[serde(rename = "ipv4")] ipv4,
	#[serde(rename = "ipv6")] ipv6,
	#[serde(rename = "unix")] unix,
	#[serde(rename = "vsock")] vsock,
	#[serde(rename = "unknown")] unknown,
}

impl ::core::str::FromStr for NetworkAddressFamily {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NetworkAddressFamily {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

NetworkAddressFamily::ipv4,
NetworkAddressFamily::ipv6,
NetworkAddressFamily::unix,
NetworkAddressFamily::vsock,
NetworkAddressFamily::unknown,

    ];
    const NAMES: &'static [&'static str] = &[

"ipv4",
"ipv6",
"unix",
"vsock",
"unknown",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketAddressType {
	#[serde(rename = "inet")] inet,
	#[serde(rename = "unix")] unix,
	#[serde(rename = "vsock")] vsock,
	#[serde(rename = "fd")] fd,
}

impl ::core::str::FromStr for SocketAddressType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SocketAddressType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

SocketAddressType::inet,
SocketAddressType::unix,
SocketAddressType::vsock,
SocketAddressType::fd,

    ];
    const NAMES: &'static [&'static str] = &[

"inet",
"unix",
"vsock",
"fd",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RunState {
	#[serde(rename = "debug")] debug,
	#[serde(rename = "inmigrate")] inmigrate,
	#[serde(rename = "internal-error")] internal_error,
	#[serde(rename = "io-error")] io_error,
	#[serde(rename = "paused")] paused,
	#[serde(rename = "postmigrate")] postmigrate,
	#[serde(rename = "prelaunch")] prelaunch,
	#[serde(rename = "finish-migrate")] finish_migrate,
	#[serde(rename = "restore-vm")] restore_vm,
	#[serde(rename = "running")] running,
	#[serde(rename = "save-vm")] save_vm,
	#[serde(rename = "shutdown")] shutdown,
	#[serde(rename = "suspended")] suspended,
	#[serde(rename = "watchdog")] watchdog,
	#[serde(rename = "guest-panicked")] guest_panicked,
	#[serde(rename = "colo")] colo,
}

impl ::core::str::FromStr for RunState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RunState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 16;
    const VARIANTS: &'static [Self] = &[

RunState::debug,
RunState::inmigrate,
RunState::internal_error,
RunState::io_error,
RunState::paused,
RunState::postmigrate,
RunState::prelaunch,
RunState::finish_migrate,
RunState::restore_vm,
RunState::running,
RunState::save_vm,
RunState::shutdown,
RunState::suspended,
RunState::watchdog,
RunState::guest_panicked,
RunState::colo,

    ];
    const NAMES: &'static [&'static str] = &[

"debug",
"inmigrate",
"internal-error",
"io-error",
"paused",
"postmigrate",
"prelaunch",
"finish-migrate",
"restore-vm",
"running",
"save-vm",
"shutdown",
"suspended",
"watchdog",
"guest-panicked",
"colo",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShutdownCause {
	#[serde(rename = "none")] none,
	#[serde(rename = "host-error")] host_error,
	#[serde(rename = "host-qmp-quit")] host_qmp_quit,
	#[serde(rename = "host-qmp-system-reset")] host_qmp_system_reset,
	#[serde(rename = "host-signal")] host_signal,
	#[serde(rename = "host-ui")] host_ui,
	#[serde(rename = "guest-shutdown")] guest_shutdown,
	#[serde(rename = "guest-reset")] guest_reset,
	#[serde(rename = "guest-panic")] guest_panic,
	#[serde(rename = "subsystem-reset")] subsystem_reset,
	#[serde(rename = "snapshot-load")] snapshot_load,
}

impl ::core::str::FromStr for ShutdownCause {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ShutdownCause {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 11;
    const VARIANTS: &'static [Self] = &[

ShutdownCause::none,
ShutdownCause::host_error,
ShutdownCause::host_qmp_quit,
ShutdownCause::host_qmp_system_reset,
ShutdownCause::host_signal,
ShutdownCause::host_ui,
ShutdownCause::guest_shutdown,
ShutdownCause::guest_reset,
ShutdownCause::guest_panic,
ShutdownCause::subsystem_reset,
ShutdownCause::snapshot_load,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"host-error",
"host-qmp-quit",
"host-qmp-system-reset",
"host-signal",
"host-ui",
"guest-shutdown",
"guest-reset",
"guest-panic",
"subsystem-reset",
"snapshot-load",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_status {
}

impl crate::QmpCommand for query_status { }
impl ::qapi_spec::Command for query_status {
    const NAME: &'static str = "query-status";
    const ALLOW_OOB: bool = false;

    type Ok = StatusInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHUTDOWN {
#[serde(rename = "guest")]
pub guest: bool,
#[serde(rename = "reason")]
pub reason: ShutdownCause,
}

impl ::qapi_spec::Event for SHUTDOWN {
    const NAME: &'static str = "SHUTDOWN";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct POWERDOWN {
}

impl ::qapi_spec::Event for POWERDOWN {
    const NAME: &'static str = "POWERDOWN";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RESET {
#[serde(rename = "guest")]
pub guest: bool,
#[serde(rename = "reason")]
pub reason: ShutdownCause,
}

impl ::qapi_spec::Event for RESET {
    const NAME: &'static str = "RESET";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct STOP {
}

impl ::qapi_spec::Event for STOP {
    const NAME: &'static str = "STOP";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RESUME {
}

impl ::qapi_spec::Event for RESUME {
    const NAME: &'static str = "RESUME";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SUSPEND {
}

impl ::qapi_spec::Event for SUSPEND {
    const NAME: &'static str = "SUSPEND";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SUSPEND_DISK {
}

impl ::qapi_spec::Event for SUSPEND_DISK {
    const NAME: &'static str = "SUSPEND_DISK";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WAKEUP {
}

impl ::qapi_spec::Event for WAKEUP {
    const NAME: &'static str = "WAKEUP";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WATCHDOG {
#[serde(rename = "action")]
pub action: WatchdogAction,
}

impl ::qapi_spec::Event for WATCHDOG {
    const NAME: &'static str = "WATCHDOG";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WatchdogAction {
	#[serde(rename = "reset")] reset,
	#[serde(rename = "shutdown")] shutdown,
	#[serde(rename = "poweroff")] poweroff,
	#[serde(rename = "pause")] pause,
	#[serde(rename = "debug")] debug,
	#[serde(rename = "none")] none,
	#[serde(rename = "inject-nmi")] inject_nmi,
}

impl ::core::str::FromStr for WatchdogAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for WatchdogAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

WatchdogAction::reset,
WatchdogAction::shutdown,
WatchdogAction::poweroff,
WatchdogAction::pause,
WatchdogAction::debug,
WatchdogAction::none,
WatchdogAction::inject_nmi,

    ];
    const NAMES: &'static [&'static str] = &[

"reset",
"shutdown",
"poweroff",
"pause",
"debug",
"none",
"inject-nmi",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RebootAction {
	#[serde(rename = "reset")] reset,
	#[serde(rename = "shutdown")] shutdown,
}

impl ::core::str::FromStr for RebootAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RebootAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

RebootAction::reset,
RebootAction::shutdown,

    ];
    const NAMES: &'static [&'static str] = &[

"reset",
"shutdown",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShutdownAction {
	#[serde(rename = "poweroff")] poweroff,
	#[serde(rename = "pause")] pause,
}

impl ::core::str::FromStr for ShutdownAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ShutdownAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

ShutdownAction::poweroff,
ShutdownAction::pause,

    ];
    const NAMES: &'static [&'static str] = &[

"poweroff",
"pause",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanicAction {
	#[serde(rename = "pause")] pause,
	#[serde(rename = "shutdown")] shutdown,
	#[serde(rename = "exit-failure")] exit_failure,
	#[serde(rename = "none")] none,
}

impl ::core::str::FromStr for PanicAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for PanicAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

PanicAction::pause,
PanicAction::shutdown,
PanicAction::exit_failure,
PanicAction::none,

    ];
    const NAMES: &'static [&'static str] = &[

"pause",
"shutdown",
"exit-failure",
"none",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct watchdog_set_action {
	#[serde(rename = "action")]
pub action: WatchdogAction,
}

impl crate::QmpCommand for watchdog_set_action { }
impl ::qapi_spec::Command for watchdog_set_action {
    const NAME: &'static str = "watchdog-set-action";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_action {
	#[serde(rename = "panic", default, skip_serializing_if = "Option::is_none")]
pub panic: Option<PanicAction>,
	#[serde(rename = "reboot", default, skip_serializing_if = "Option::is_none")]
pub reboot: Option<RebootAction>,
	#[serde(rename = "shutdown", default, skip_serializing_if = "Option::is_none")]
pub shutdown: Option<ShutdownAction>,
	#[serde(rename = "watchdog", default, skip_serializing_if = "Option::is_none")]
pub watchdog: Option<WatchdogAction>,
}

impl crate::QmpCommand for set_action { }
impl ::qapi_spec::Command for set_action {
    const NAME: &'static str = "set-action";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUEST_PANICKED {
#[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
pub info: Option<GuestPanicInformation>,
#[serde(rename = "action")]
pub action: GuestPanicAction,
}

impl ::qapi_spec::Event for GUEST_PANICKED {
    const NAME: &'static str = "GUEST_PANICKED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUEST_CRASHLOADED {
#[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
pub info: Option<GuestPanicInformation>,
#[serde(rename = "action")]
pub action: GuestPanicAction,
}

impl ::qapi_spec::Event for GUEST_CRASHLOADED {
    const NAME: &'static str = "GUEST_CRASHLOADED";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GUEST_PVSHUTDOWN {
}

impl ::qapi_spec::Event for GUEST_PVSHUTDOWN {
    const NAME: &'static str = "GUEST_PVSHUTDOWN";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GuestPanicAction {
	#[serde(rename = "pause")] pause,
	#[serde(rename = "poweroff")] poweroff,
	#[serde(rename = "run")] run,
}

impl ::core::str::FromStr for GuestPanicAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for GuestPanicAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

GuestPanicAction::pause,
GuestPanicAction::poweroff,
GuestPanicAction::run,

    ];
    const NAMES: &'static [&'static str] = &[

"pause",
"poweroff",
"run",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GuestPanicInformationType {
	#[serde(rename = "hyper-v")] hyper_v,
	#[serde(rename = "s390")] s390,
}

impl ::core::str::FromStr for GuestPanicInformationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for GuestPanicInformationType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

GuestPanicInformationType::hyper_v,
GuestPanicInformationType::s390,

    ];
    const NAMES: &'static [&'static str] = &[

"hyper-v",
"s390",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S390CrashReason {
	#[serde(rename = "unknown")] unknown,
	#[serde(rename = "disabled-wait")] disabled_wait,
	#[serde(rename = "extint-loop")] extint_loop,
	#[serde(rename = "pgmint-loop")] pgmint_loop,
	#[serde(rename = "opint-loop")] opint_loop,
}

impl ::core::str::FromStr for S390CrashReason {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for S390CrashReason {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

S390CrashReason::unknown,
S390CrashReason::disabled_wait,
S390CrashReason::extint_loop,
S390CrashReason::pgmint_loop,
S390CrashReason::opint_loop,

    ];
    const NAMES: &'static [&'static str] = &[

"unknown",
"disabled-wait",
"extint-loop",
"pgmint-loop",
"opint-loop",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MEMORY_FAILURE {
#[serde(rename = "action")]
pub action: MemoryFailureAction,
#[serde(rename = "flags")]
pub flags: MemoryFailureFlags,
#[serde(rename = "recipient")]
pub recipient: MemoryFailureRecipient,
}

impl ::qapi_spec::Event for MEMORY_FAILURE {
    const NAME: &'static str = "MEMORY_FAILURE";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryFailureRecipient {
	#[serde(rename = "hypervisor")] hypervisor,
	#[serde(rename = "guest")] guest,
}

impl ::core::str::FromStr for MemoryFailureRecipient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MemoryFailureRecipient {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

MemoryFailureRecipient::hypervisor,
MemoryFailureRecipient::guest,

    ];
    const NAMES: &'static [&'static str] = &[

"hypervisor",
"guest",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryFailureAction {
	#[serde(rename = "ignore")] ignore,
	#[serde(rename = "inject")] inject,
	#[serde(rename = "fatal")] fatal,
	#[serde(rename = "reset")] reset,
}

impl ::core::str::FromStr for MemoryFailureAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MemoryFailureAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

MemoryFailureAction::ignore,
MemoryFailureAction::inject,
MemoryFailureAction::fatal,
MemoryFailureAction::reset,

    ];
    const NAMES: &'static [&'static str] = &[

"ignore",
"inject",
"fatal",
"reset",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotifyVmexitOption {
	#[serde(rename = "run")] run,
	#[serde(rename = "internal-error")] internal_error,
	#[serde(rename = "disable")] disable,
}

impl ::core::str::FromStr for NotifyVmexitOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NotifyVmexitOption {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

NotifyVmexitOption::run,
NotifyVmexitOption::internal_error,
NotifyVmexitOption::disable,

    ];
    const NAMES: &'static [&'static str] = &[

"run",
"internal-error",
"disable",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoTLSCredsEndpoint {
	#[serde(rename = "client")] client,
	#[serde(rename = "server")] server,
}

impl ::core::str::FromStr for QCryptoTLSCredsEndpoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoTLSCredsEndpoint {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoTLSCredsEndpoint::client,
QCryptoTLSCredsEndpoint::server,

    ];
    const NAMES: &'static [&'static str] = &[

"client",
"server",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoSecretFormat {
	#[serde(rename = "raw")] raw,
	#[serde(rename = "base64")] base64,
}

impl ::core::str::FromStr for QCryptoSecretFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoSecretFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoSecretFormat::raw,
QCryptoSecretFormat::base64,

    ];
    const NAMES: &'static [&'static str] = &[

"raw",
"base64",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoHashAlgorithm {
	#[serde(rename = "md5")] md5,
	#[serde(rename = "sha1")] sha1,
	#[serde(rename = "sha224")] sha224,
	#[serde(rename = "sha256")] sha256,
	#[serde(rename = "sha384")] sha384,
	#[serde(rename = "sha512")] sha512,
	#[serde(rename = "ripemd160")] ripemd160,
}

impl ::core::str::FromStr for QCryptoHashAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoHashAlgorithm {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

QCryptoHashAlgorithm::md5,
QCryptoHashAlgorithm::sha1,
QCryptoHashAlgorithm::sha224,
QCryptoHashAlgorithm::sha256,
QCryptoHashAlgorithm::sha384,
QCryptoHashAlgorithm::sha512,
QCryptoHashAlgorithm::ripemd160,

    ];
    const NAMES: &'static [&'static str] = &[

"md5",
"sha1",
"sha224",
"sha256",
"sha384",
"sha512",
"ripemd160",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoCipherAlgorithm {
	#[serde(rename = "aes-128")] aes_128,
	#[serde(rename = "aes-192")] aes_192,
	#[serde(rename = "aes-256")] aes_256,
	#[serde(rename = "des")] des,
	#[serde(rename = "3des")] _3des,
	#[serde(rename = "cast5-128")] cast5_128,
	#[serde(rename = "serpent-128")] serpent_128,
	#[serde(rename = "serpent-192")] serpent_192,
	#[serde(rename = "serpent-256")] serpent_256,
	#[serde(rename = "twofish-128")] twofish_128,
	#[serde(rename = "twofish-192")] twofish_192,
	#[serde(rename = "twofish-256")] twofish_256,
	#[serde(rename = "sm4")] sm4,
}

impl ::core::str::FromStr for QCryptoCipherAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoCipherAlgorithm {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 13;
    const VARIANTS: &'static [Self] = &[

QCryptoCipherAlgorithm::aes_128,
QCryptoCipherAlgorithm::aes_192,
QCryptoCipherAlgorithm::aes_256,
QCryptoCipherAlgorithm::des,
QCryptoCipherAlgorithm::_3des,
QCryptoCipherAlgorithm::cast5_128,
QCryptoCipherAlgorithm::serpent_128,
QCryptoCipherAlgorithm::serpent_192,
QCryptoCipherAlgorithm::serpent_256,
QCryptoCipherAlgorithm::twofish_128,
QCryptoCipherAlgorithm::twofish_192,
QCryptoCipherAlgorithm::twofish_256,
QCryptoCipherAlgorithm::sm4,

    ];
    const NAMES: &'static [&'static str] = &[

"aes-128",
"aes-192",
"aes-256",
"des",
"3des",
"cast5-128",
"serpent-128",
"serpent-192",
"serpent-256",
"twofish-128",
"twofish-192",
"twofish-256",
"sm4",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoCipherMode {
	#[serde(rename = "ecb")] ecb,
	#[serde(rename = "cbc")] cbc,
	#[serde(rename = "xts")] xts,
	#[serde(rename = "ctr")] ctr,
}

impl ::core::str::FromStr for QCryptoCipherMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoCipherMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

QCryptoCipherMode::ecb,
QCryptoCipherMode::cbc,
QCryptoCipherMode::xts,
QCryptoCipherMode::ctr,

    ];
    const NAMES: &'static [&'static str] = &[

"ecb",
"cbc",
"xts",
"ctr",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoIVGenAlgorithm {
	#[serde(rename = "plain")] plain,
	#[serde(rename = "plain64")] plain64,
	#[serde(rename = "essiv")] essiv,
}

impl ::core::str::FromStr for QCryptoIVGenAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoIVGenAlgorithm {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

QCryptoIVGenAlgorithm::plain,
QCryptoIVGenAlgorithm::plain64,
QCryptoIVGenAlgorithm::essiv,

    ];
    const NAMES: &'static [&'static str] = &[

"plain",
"plain64",
"essiv",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoBlockFormat {
	#[serde(rename = "qcow")] qcow,
	#[serde(rename = "luks")] luks,
}

impl ::core::str::FromStr for QCryptoBlockFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoBlockFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoBlockFormat::qcow,
QCryptoBlockFormat::luks,

    ];
    const NAMES: &'static [&'static str] = &[

"qcow",
"luks",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoBlockLUKSKeyslotState {
	#[serde(rename = "active")] active,
	#[serde(rename = "inactive")] inactive,
}

impl ::core::str::FromStr for QCryptoBlockLUKSKeyslotState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoBlockLUKSKeyslotState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoBlockLUKSKeyslotState::active,
QCryptoBlockLUKSKeyslotState::inactive,

    ];
    const NAMES: &'static [&'static str] = &[

"active",
"inactive",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoAkCipherAlgorithm {
	#[serde(rename = "rsa")] rsa,
}

impl ::core::str::FromStr for QCryptoAkCipherAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoAkCipherAlgorithm {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

QCryptoAkCipherAlgorithm::rsa,

    ];
    const NAMES: &'static [&'static str] = &[

"rsa",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoAkCipherKeyType {
	#[serde(rename = "public")] public,
	#[serde(rename = "private")] private,
}

impl ::core::str::FromStr for QCryptoAkCipherKeyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoAkCipherKeyType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoAkCipherKeyType::public,
QCryptoAkCipherKeyType::private,

    ];
    const NAMES: &'static [&'static str] = &[

"public",
"private",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptoRSAPaddingAlgorithm {
	#[serde(rename = "raw")] raw,
	#[serde(rename = "pkcs1")] pkcs1,
}

impl ::core::str::FromStr for QCryptoRSAPaddingAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptoRSAPaddingAlgorithm {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptoRSAPaddingAlgorithm::raw,
QCryptoRSAPaddingAlgorithm::pkcs1,

    ];
    const NAMES: &'static [&'static str] = &[

"raw",
"pkcs1",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobType {
	#[serde(rename = "commit")] commit,
	#[serde(rename = "stream")] stream,
	#[serde(rename = "mirror")] mirror,
	#[serde(rename = "backup")] backup,
	#[serde(rename = "create")] create,
	#[serde(rename = "amend")] amend,
	#[serde(rename = "snapshot-load")] snapshot_load,
	#[serde(rename = "snapshot-save")] snapshot_save,
	#[serde(rename = "snapshot-delete")] snapshot_delete,
}

impl ::core::str::FromStr for JobType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for JobType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 9;
    const VARIANTS: &'static [Self] = &[

JobType::commit,
JobType::stream,
JobType::mirror,
JobType::backup,
JobType::create,
JobType::amend,
JobType::snapshot_load,
JobType::snapshot_save,
JobType::snapshot_delete,

    ];
    const NAMES: &'static [&'static str] = &[

"commit",
"stream",
"mirror",
"backup",
"create",
"amend",
"snapshot-load",
"snapshot-save",
"snapshot-delete",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobStatus {
	#[serde(rename = "undefined")] undefined,
	#[serde(rename = "created")] created,
	#[serde(rename = "running")] running,
	#[serde(rename = "paused")] paused,
	#[serde(rename = "ready")] ready,
	#[serde(rename = "standby")] standby,
	#[serde(rename = "waiting")] waiting,
	#[serde(rename = "pending")] pending,
	#[serde(rename = "aborting")] aborting,
	#[serde(rename = "concluded")] concluded,
	#[serde(rename = "null")] null,
}

impl ::core::str::FromStr for JobStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for JobStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 11;
    const VARIANTS: &'static [Self] = &[

JobStatus::undefined,
JobStatus::created,
JobStatus::running,
JobStatus::paused,
JobStatus::ready,
JobStatus::standby,
JobStatus::waiting,
JobStatus::pending,
JobStatus::aborting,
JobStatus::concluded,
JobStatus::null,

    ];
    const NAMES: &'static [&'static str] = &[

"undefined",
"created",
"running",
"paused",
"ready",
"standby",
"waiting",
"pending",
"aborting",
"concluded",
"null",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobVerb {
	#[serde(rename = "cancel")] cancel,
	#[serde(rename = "pause")] pause,
	#[serde(rename = "resume")] resume,
	#[serde(rename = "set-speed")] set_speed,
	#[serde(rename = "complete")] complete,
	#[serde(rename = "dismiss")] dismiss,
	#[serde(rename = "finalize")] finalize,
	#[serde(rename = "change")] change,
}

impl ::core::str::FromStr for JobVerb {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for JobVerb {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 8;
    const VARIANTS: &'static [Self] = &[

JobVerb::cancel,
JobVerb::pause,
JobVerb::resume,
JobVerb::set_speed,
JobVerb::complete,
JobVerb::dismiss,
JobVerb::finalize,
JobVerb::change,

    ];
    const NAMES: &'static [&'static str] = &[

"cancel",
"pause",
"resume",
"set-speed",
"complete",
"dismiss",
"finalize",
"change",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JOB_STATUS_CHANGE {
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "status")]
pub status: JobStatus,
}

impl ::qapi_spec::Event for JOB_STATUS_CHANGE {
    const NAME: &'static str = "JOB_STATUS_CHANGE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_pause {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_pause { }
impl ::qapi_spec::Command for job_pause {
    const NAME: &'static str = "job-pause";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_resume {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_resume { }
impl ::qapi_spec::Command for job_resume {
    const NAME: &'static str = "job-resume";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_cancel {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_cancel { }
impl ::qapi_spec::Command for job_cancel {
    const NAME: &'static str = "job-cancel";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_complete {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_complete { }
impl ::qapi_spec::Command for job_complete {
    const NAME: &'static str = "job-complete";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_dismiss {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_dismiss { }
impl ::qapi_spec::Command for job_dismiss {
    const NAME: &'static str = "job-dismiss";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct job_finalize {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for job_finalize { }
impl ::qapi_spec::Command for job_finalize {
    const NAME: &'static str = "job-finalize";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_jobs {
}

impl crate::QmpCommand for query_jobs { }
impl ::qapi_spec::Command for query_jobs {
    const NAME: &'static str = "query-jobs";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<JobInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BiosAtaTranslation {
	#[serde(rename = "auto")] auto,
	#[serde(rename = "none")] none,
	#[serde(rename = "lba")] lba,
	#[serde(rename = "large")] large,
	#[serde(rename = "rechs")] rechs,
}

impl ::core::str::FromStr for BiosAtaTranslation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BiosAtaTranslation {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

BiosAtaTranslation::auto,
BiosAtaTranslation::none,
BiosAtaTranslation::lba,
BiosAtaTranslation::large,
BiosAtaTranslation::rechs,

    ];
    const NAMES: &'static [&'static str] = &[

"auto",
"none",
"lba",
"large",
"rechs",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FloppyDriveType {
	#[serde(rename = "144")] _144,
	#[serde(rename = "288")] _288,
	#[serde(rename = "120")] _120,
	#[serde(rename = "none")] none,
	#[serde(rename = "auto")] auto,
}

impl ::core::str::FromStr for FloppyDriveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for FloppyDriveType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

FloppyDriveType::_144,
FloppyDriveType::_288,
FloppyDriveType::_120,
FloppyDriveType::none,
FloppyDriveType::auto,

    ];
    const NAMES: &'static [&'static str] = &[

"144",
"288",
"120",
"none",
"auto",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_pr_managers {
}

impl crate::QmpCommand for query_pr_managers { }
impl ::qapi_spec::Command for query_pr_managers {
    const NAME: &'static str = "query-pr-managers";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<PRManagerInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct eject {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub device: Option<::std::string::String>,
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
}

impl crate::QmpCommand for eject { }
impl ::qapi_spec::Command for eject {
    const NAME: &'static str = "eject";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_open_tray {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub device: Option<::std::string::String>,
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
}

impl crate::QmpCommand for blockdev_open_tray { }
impl ::qapi_spec::Command for blockdev_open_tray {
    const NAME: &'static str = "blockdev-open-tray";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_close_tray {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub device: Option<::std::string::String>,
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
}

impl crate::QmpCommand for blockdev_close_tray { }
impl ::qapi_spec::Command for blockdev_close_tray {
    const NAME: &'static str = "blockdev-close-tray";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_remove_medium {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for blockdev_remove_medium { }
impl ::qapi_spec::Command for blockdev_remove_medium {
    const NAME: &'static str = "blockdev-remove-medium";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_insert_medium {
	#[serde(rename = "id")]
pub id: ::std::string::String,
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

impl crate::QmpCommand for blockdev_insert_medium { }
impl ::qapi_spec::Command for blockdev_insert_medium {
    const NAME: &'static str = "blockdev-insert-medium";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevChangeReadOnlyMode {
	#[serde(rename = "retain")] retain,
	#[serde(rename = "read-only")] read_only,
	#[serde(rename = "read-write")] read_write,
}

impl ::core::str::FromStr for BlockdevChangeReadOnlyMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevChangeReadOnlyMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BlockdevChangeReadOnlyMode::retain,
BlockdevChangeReadOnlyMode::read_only,
BlockdevChangeReadOnlyMode::read_write,

    ];
    const NAMES: &'static [&'static str] = &[

"retain",
"read-only",
"read-write",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_change_medium {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub device: Option<::std::string::String>,
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<::std::string::String>,
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
	#[serde(rename = "read-only-mode", default, skip_serializing_if = "Option::is_none")]
pub read_only_mode: Option<BlockdevChangeReadOnlyMode>,
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for blockdev_change_medium { }
impl ::qapi_spec::Command for blockdev_change_medium {
    const NAME: &'static str = "blockdev-change-medium";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEVICE_TRAY_MOVED {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "tray-open")]
pub tray_open: bool,
}

impl ::qapi_spec::Event for DEVICE_TRAY_MOVED {
    const NAME: &'static str = "DEVICE_TRAY_MOVED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PR_MANAGER_STATUS_CHANGED {
#[serde(rename = "connected")]
pub connected: bool,
#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl ::qapi_spec::Event for PR_MANAGER_STATUS_CHANGED {
    const NAME: &'static str = "PR_MANAGER_STATUS_CHANGED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_set_io_throttle(pub BlockIOThrottle);

impl From<BlockIOThrottle> for block_set_io_throttle {
    fn from(val: BlockIOThrottle) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_set_io_throttle { }
impl ::qapi_spec::Command for block_set_io_throttle {
    const NAME: &'static str = "block_set_io_throttle";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_latency_histogram_set {
	#[serde(rename = "boundaries", default, skip_serializing_if = "Option::is_none")]
pub boundaries: Option<Vec<u64>>,
	#[serde(rename = "boundaries-flush", default, skip_serializing_if = "Option::is_none")]
pub boundaries_flush: Option<Vec<u64>>,
	#[serde(rename = "boundaries-read", default, skip_serializing_if = "Option::is_none")]
pub boundaries_read: Option<Vec<u64>>,
	#[serde(rename = "boundaries-write", default, skip_serializing_if = "Option::is_none")]
pub boundaries_write: Option<Vec<u64>>,
	#[serde(rename = "boundaries-zap", default, skip_serializing_if = "Option::is_none")]
pub boundaries_zap: Option<Vec<u64>>,
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for block_latency_histogram_set { }
impl ::qapi_spec::Command for block_latency_histogram_set {
    const NAME: &'static str = "block-latency-histogram-set";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageInfoSpecificKind {
	#[serde(rename = "qcow2")] qcow2,
	#[serde(rename = "vmdk")] vmdk,
	#[serde(rename = "luks")] luks,
	#[serde(rename = "rbd")] rbd,
	#[serde(rename = "file")] file,
}

impl ::core::str::FromStr for ImageInfoSpecificKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ImageInfoSpecificKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

ImageInfoSpecificKind::qcow2,
ImageInfoSpecificKind::vmdk,
ImageInfoSpecificKind::luks,
ImageInfoSpecificKind::rbd,
ImageInfoSpecificKind::file,

    ];
    const NAMES: &'static [&'static str] = &[

"qcow2",
"vmdk",
"luks",
"rbd",
"file",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockDeviceIoStatus {
	#[serde(rename = "ok")] ok,
	#[serde(rename = "failed")] failed,
	#[serde(rename = "nospace")] nospace,
}

impl ::core::str::FromStr for BlockDeviceIoStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockDeviceIoStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BlockDeviceIoStatus::ok,
BlockDeviceIoStatus::failed,
BlockDeviceIoStatus::nospace,

    ];
    const NAMES: &'static [&'static str] = &[

"ok",
"failed",
"nospace",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Qcow2BitmapInfoFlags {
	#[serde(rename = "in-use")] in_use,
	#[serde(rename = "auto")] auto,
}

impl ::core::str::FromStr for Qcow2BitmapInfoFlags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for Qcow2BitmapInfoFlags {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

Qcow2BitmapInfoFlags::in_use,
Qcow2BitmapInfoFlags::auto,

    ];
    const NAMES: &'static [&'static str] = &[

"in-use",
"auto",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_block {
}

impl crate::QmpCommand for query_block { }
impl ::qapi_spec::Command for query_block {
    const NAME: &'static str = "query-block";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<BlockInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_blockstats {
	#[serde(rename = "query-nodes", default, skip_serializing_if = "Option::is_none")]
pub query_nodes: Option<bool>,
}

impl crate::QmpCommand for query_blockstats { }
impl ::qapi_spec::Command for query_blockstats {
    const NAME: &'static str = "query-blockstats";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<BlockStats>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevOnError {
	#[serde(rename = "report")] report,
	#[serde(rename = "ignore")] ignore,
	#[serde(rename = "enospc")] enospc,
	#[serde(rename = "stop")] stop,
	#[serde(rename = "auto")] auto,
}

impl ::core::str::FromStr for BlockdevOnError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevOnError {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

BlockdevOnError::report,
BlockdevOnError::ignore,
BlockdevOnError::enospc,
BlockdevOnError::stop,
BlockdevOnError::auto,

    ];
    const NAMES: &'static [&'static str] = &[

"report",
"ignore",
"enospc",
"stop",
"auto",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MirrorSyncMode {
	#[serde(rename = "top")] top,
	#[serde(rename = "full")] full,
	#[serde(rename = "none")] none,
	#[serde(rename = "incremental")] incremental,
	#[serde(rename = "bitmap")] bitmap,
}

impl ::core::str::FromStr for MirrorSyncMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MirrorSyncMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

MirrorSyncMode::top,
MirrorSyncMode::full,
MirrorSyncMode::none,
MirrorSyncMode::incremental,
MirrorSyncMode::bitmap,

    ];
    const NAMES: &'static [&'static str] = &[

"top",
"full",
"none",
"incremental",
"bitmap",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitmapSyncMode {
	#[serde(rename = "on-success")] on_success,
	#[serde(rename = "never")] never,
	#[serde(rename = "always")] always,
}

impl ::core::str::FromStr for BitmapSyncMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BitmapSyncMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BitmapSyncMode::on_success,
BitmapSyncMode::never,
BitmapSyncMode::always,

    ];
    const NAMES: &'static [&'static str] = &[

"on-success",
"never",
"always",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MirrorCopyMode {
	#[serde(rename = "background")] background,
	#[serde(rename = "write-blocking")] write_blocking,
}

impl ::core::str::FromStr for MirrorCopyMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MirrorCopyMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

MirrorCopyMode::background,
MirrorCopyMode::write_blocking,

    ];
    const NAMES: &'static [&'static str] = &[

"background",
"write-blocking",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_block_jobs {
}

impl crate::QmpCommand for query_block_jobs { }
impl ::qapi_spec::Command for query_block_jobs {
    const NAME: &'static str = "query-block-jobs";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<BlockJobInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_resize {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
	#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
	#[serde(rename = "size")]
pub size: i64,
}

impl crate::QmpCommand for block_resize { }
impl ::qapi_spec::Command for block_resize {
    const NAME: &'static str = "block_resize";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NewImageMode {
	#[serde(rename = "existing")] existing,
	#[serde(rename = "absolute-paths")] absolute_paths,
}

impl ::core::str::FromStr for NewImageMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NewImageMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

NewImageMode::existing,
NewImageMode::absolute_paths,

    ];
    const NAMES: &'static [&'static str] = &[

"existing",
"absolute-paths",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_snapshot_sync(pub BlockdevSnapshotSync);

impl From<BlockdevSnapshotSync> for blockdev_snapshot_sync {
    fn from(val: BlockdevSnapshotSync) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for blockdev_snapshot_sync { }
impl ::qapi_spec::Command for blockdev_snapshot_sync {
    const NAME: &'static str = "blockdev-snapshot-sync";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_snapshot(pub BlockdevSnapshot);

impl From<BlockdevSnapshot> for blockdev_snapshot {
    fn from(val: BlockdevSnapshot) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for blockdev_snapshot { }
impl ::qapi_spec::Command for blockdev_snapshot {
    const NAME: &'static str = "blockdev-snapshot";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct change_backing_file {
	#[serde(rename = "backing-file")]
pub backing_file: ::std::string::String,
	#[serde(rename = "device")]
pub device: ::std::string::String,
	#[serde(rename = "image-node-name")]
pub image_node_name: ::std::string::String,
}

impl crate::QmpCommand for change_backing_file { }
impl ::qapi_spec::Command for change_backing_file {
    const NAME: &'static str = "change-backing-file";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_commit {
	#[serde(rename = "auto-dismiss", default, skip_serializing_if = "Option::is_none")]
pub auto_dismiss: Option<bool>,
	#[serde(rename = "auto-finalize", default, skip_serializing_if = "Option::is_none")]
pub auto_finalize: Option<bool>,
	#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
	#[serde(rename = "backing-mask-protocol", default, skip_serializing_if = "Option::is_none")]
pub backing_mask_protocol: Option<bool>,
	#[serde(rename = "base", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub base: Option<::std::string::String>,
	#[serde(rename = "base-node", default, skip_serializing_if = "Option::is_none")]
pub base_node: Option<::std::string::String>,
	#[serde(rename = "filter-node-name", default, skip_serializing_if = "Option::is_none")]
pub filter_node_name: Option<::std::string::String>,
	#[serde(rename = "job-id", default, skip_serializing_if = "Option::is_none")]
pub job_id: Option<::std::string::String>,
	#[serde(rename = "on-error", default, skip_serializing_if = "Option::is_none")]
pub on_error: Option<BlockdevOnError>,
	#[serde(rename = "speed", default, skip_serializing_if = "Option::is_none")]
pub speed: Option<i64>,
	#[serde(rename = "top", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub top: Option<::std::string::String>,
	#[serde(rename = "top-node", default, skip_serializing_if = "Option::is_none")]
pub top_node: Option<::std::string::String>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_commit { }
impl ::qapi_spec::Command for block_commit {
    const NAME: &'static str = "block-commit";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)] #[deprecated]
pub struct drive_backup(pub DriveBackup);

impl From<DriveBackup> for drive_backup {
    fn from(val: DriveBackup) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for drive_backup { }
impl ::qapi_spec::Command for drive_backup {
    const NAME: &'static str = "drive-backup";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_backup(pub BlockdevBackup);

impl From<BlockdevBackup> for blockdev_backup {
    fn from(val: BlockdevBackup) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for blockdev_backup { }
impl ::qapi_spec::Command for blockdev_backup {
    const NAME: &'static str = "blockdev-backup";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_named_block_nodes {
	#[serde(rename = "flat", default, skip_serializing_if = "Option::is_none")]
pub flat: Option<bool>,
}

impl crate::QmpCommand for query_named_block_nodes { }
impl ::qapi_spec::Command for query_named_block_nodes {
    const NAME: &'static str = "query-named-block-nodes";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<BlockDeviceInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum XDbgBlockGraphNodeType {
	#[serde(rename = "block-backend")] block_backend,
	#[serde(rename = "block-job")] block_job,
	#[serde(rename = "block-driver")] block_driver,
}

impl ::core::str::FromStr for XDbgBlockGraphNodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for XDbgBlockGraphNodeType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

XDbgBlockGraphNodeType::block_backend,
XDbgBlockGraphNodeType::block_job,
XDbgBlockGraphNodeType::block_driver,

    ];
    const NAMES: &'static [&'static str] = &[

"block-backend",
"block-job",
"block-driver",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockPermission {
	#[serde(rename = "consistent-read")] consistent_read,
	#[serde(rename = "write")] write,
	#[serde(rename = "write-unchanged")] write_unchanged,
	#[serde(rename = "resize")] resize,
}

impl ::core::str::FromStr for BlockPermission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockPermission {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

BlockPermission::consistent_read,
BlockPermission::write,
BlockPermission::write_unchanged,
BlockPermission::resize,

    ];
    const NAMES: &'static [&'static str] = &[

"consistent-read",
"write",
"write-unchanged",
"resize",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_debug_query_block_graph {
}

impl crate::QmpCommand for x_debug_query_block_graph { }
impl ::qapi_spec::Command for x_debug_query_block_graph {
    const NAME: &'static str = "x-debug-query-block-graph";
    const ALLOW_OOB: bool = false;

    type Ok = XDbgBlockGraph;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct drive_mirror(pub DriveMirror);

impl From<DriveMirror> for drive_mirror {
    fn from(val: DriveMirror) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for drive_mirror { }
impl ::qapi_spec::Command for drive_mirror {
    const NAME: &'static str = "drive-mirror";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockDirtyBitmapOrStr {
	#[serde(rename = "external")] external(BlockDirtyBitmap),
	#[serde(rename = "local")] local(::std::string::String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_add(pub BlockDirtyBitmapAdd);

impl From<BlockDirtyBitmapAdd> for block_dirty_bitmap_add {
    fn from(val: BlockDirtyBitmapAdd) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_add { }
impl ::qapi_spec::Command for block_dirty_bitmap_add {
    const NAME: &'static str = "block-dirty-bitmap-add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_remove(pub BlockDirtyBitmap);

impl From<BlockDirtyBitmap> for block_dirty_bitmap_remove {
    fn from(val: BlockDirtyBitmap) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_remove { }
impl ::qapi_spec::Command for block_dirty_bitmap_remove {
    const NAME: &'static str = "block-dirty-bitmap-remove";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_clear(pub BlockDirtyBitmap);

impl From<BlockDirtyBitmap> for block_dirty_bitmap_clear {
    fn from(val: BlockDirtyBitmap) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_clear { }
impl ::qapi_spec::Command for block_dirty_bitmap_clear {
    const NAME: &'static str = "block-dirty-bitmap-clear";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_enable(pub BlockDirtyBitmap);

impl From<BlockDirtyBitmap> for block_dirty_bitmap_enable {
    fn from(val: BlockDirtyBitmap) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_enable { }
impl ::qapi_spec::Command for block_dirty_bitmap_enable {
    const NAME: &'static str = "block-dirty-bitmap-enable";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_disable(pub BlockDirtyBitmap);

impl From<BlockDirtyBitmap> for block_dirty_bitmap_disable {
    fn from(val: BlockDirtyBitmap) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_disable { }
impl ::qapi_spec::Command for block_dirty_bitmap_disable {
    const NAME: &'static str = "block-dirty-bitmap-disable";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_dirty_bitmap_merge(pub BlockDirtyBitmapMerge);

impl From<BlockDirtyBitmapMerge> for block_dirty_bitmap_merge {
    fn from(val: BlockDirtyBitmapMerge) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_dirty_bitmap_merge { }
impl ::qapi_spec::Command for block_dirty_bitmap_merge {
    const NAME: &'static str = "block-dirty-bitmap-merge";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_debug_block_dirty_bitmap_sha256(pub BlockDirtyBitmap);

impl From<BlockDirtyBitmap> for x_debug_block_dirty_bitmap_sha256 {
    fn from(val: BlockDirtyBitmap) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for x_debug_block_dirty_bitmap_sha256 { }
impl ::qapi_spec::Command for x_debug_block_dirty_bitmap_sha256 {
    const NAME: &'static str = "x-debug-block-dirty-bitmap-sha256";
    const ALLOW_OOB: bool = false;

    type Ok = BlockDirtyBitmapSha256;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_mirror {
	#[serde(rename = "auto-dismiss", default, skip_serializing_if = "Option::is_none")]
pub auto_dismiss: Option<bool>,
	#[serde(rename = "auto-finalize", default, skip_serializing_if = "Option::is_none")]
pub auto_finalize: Option<bool>,
	#[serde(rename = "buf-size", default, skip_serializing_if = "Option::is_none")]
pub buf_size: Option<i64>,
	#[serde(rename = "copy-mode", default, skip_serializing_if = "Option::is_none")]
pub copy_mode: Option<MirrorCopyMode>,
	#[serde(rename = "filter-node-name", default, skip_serializing_if = "Option::is_none")]
pub filter_node_name: Option<::std::string::String>,
	#[serde(rename = "granularity", default, skip_serializing_if = "Option::is_none")]
pub granularity: Option<u32>,
	#[serde(rename = "job-id", default, skip_serializing_if = "Option::is_none")]
pub job_id: Option<::std::string::String>,
	#[serde(rename = "on-source-error", default, skip_serializing_if = "Option::is_none")]
pub on_source_error: Option<BlockdevOnError>,
	#[serde(rename = "on-target-error", default, skip_serializing_if = "Option::is_none")]
pub on_target_error: Option<BlockdevOnError>,
	#[serde(rename = "replaces", default, skip_serializing_if = "Option::is_none")]
pub replaces: Option<::std::string::String>,
	#[serde(rename = "speed", default, skip_serializing_if = "Option::is_none")]
pub speed: Option<i64>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
	#[serde(rename = "sync")]
pub sync: MirrorSyncMode,
	#[serde(rename = "target")]
pub target: ::std::string::String,
}

impl crate::QmpCommand for blockdev_mirror { }
impl ::qapi_spec::Command for blockdev_mirror {
    const NAME: &'static str = "blockdev-mirror";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_stream {
	#[serde(rename = "auto-dismiss", default, skip_serializing_if = "Option::is_none")]
pub auto_dismiss: Option<bool>,
	#[serde(rename = "auto-finalize", default, skip_serializing_if = "Option::is_none")]
pub auto_finalize: Option<bool>,
	#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
	#[serde(rename = "backing-mask-protocol", default, skip_serializing_if = "Option::is_none")]
pub backing_mask_protocol: Option<bool>,
	#[serde(rename = "base", default, skip_serializing_if = "Option::is_none")]
pub base: Option<::std::string::String>,
	#[serde(rename = "base-node", default, skip_serializing_if = "Option::is_none")]
pub base_node: Option<::std::string::String>,
	#[serde(rename = "bottom", default, skip_serializing_if = "Option::is_none")]
pub bottom: Option<::std::string::String>,
	#[serde(rename = "filter-node-name", default, skip_serializing_if = "Option::is_none")]
pub filter_node_name: Option<::std::string::String>,
	#[serde(rename = "job-id", default, skip_serializing_if = "Option::is_none")]
pub job_id: Option<::std::string::String>,
	#[serde(rename = "on-error", default, skip_serializing_if = "Option::is_none")]
pub on_error: Option<BlockdevOnError>,
	#[serde(rename = "speed", default, skip_serializing_if = "Option::is_none")]
pub speed: Option<i64>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_stream { }
impl ::qapi_spec::Command for block_stream {
    const NAME: &'static str = "block-stream";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_set_speed {
	#[serde(rename = "device")]
pub device: ::std::string::String,
	#[serde(rename = "speed")]
pub speed: i64,
}

impl crate::QmpCommand for block_job_set_speed { }
impl ::qapi_spec::Command for block_job_set_speed {
    const NAME: &'static str = "block-job-set-speed";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_cancel {
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_job_cancel { }
impl ::qapi_spec::Command for block_job_cancel {
    const NAME: &'static str = "block-job-cancel";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_pause {
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_job_pause { }
impl ::qapi_spec::Command for block_job_pause {
    const NAME: &'static str = "block-job-pause";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_resume {
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_job_resume { }
impl ::qapi_spec::Command for block_job_resume {
    const NAME: &'static str = "block-job-resume";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_complete {
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for block_job_complete { }
impl ::qapi_spec::Command for block_job_complete {
    const NAME: &'static str = "block-job-complete";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_dismiss {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for block_job_dismiss { }
impl ::qapi_spec::Command for block_job_dismiss {
    const NAME: &'static str = "block-job-dismiss";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_finalize {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for block_job_finalize { }
impl ::qapi_spec::Command for block_job_finalize {
    const NAME: &'static str = "block-job-finalize";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_job_change(pub BlockJobChangeOptions);

impl From<BlockJobChangeOptions> for block_job_change {
    fn from(val: BlockJobChangeOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_job_change { }
impl ::qapi_spec::Command for block_job_change {
    const NAME: &'static str = "block-job-change";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevDiscardOptions {
	#[serde(rename = "ignore")] ignore,
	#[serde(rename = "unmap")] unmap,
}

impl ::core::str::FromStr for BlockdevDiscardOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevDiscardOptions {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockdevDiscardOptions::ignore,
BlockdevDiscardOptions::unmap,

    ];
    const NAMES: &'static [&'static str] = &[

"ignore",
"unmap",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevDetectZeroesOptions {
	#[serde(rename = "off")] off,
	#[serde(rename = "on")] on,
	#[serde(rename = "unmap")] unmap,
}

impl ::core::str::FromStr for BlockdevDetectZeroesOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevDetectZeroesOptions {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BlockdevDetectZeroesOptions::off,
BlockdevDetectZeroesOptions::on,
BlockdevDetectZeroesOptions::unmap,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"on",
"unmap",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevAioOptions {
	#[serde(rename = "threads")] threads,
	#[serde(rename = "native")] native,
	#[serde(rename = "io_uring")] io_uring,
}

impl ::core::str::FromStr for BlockdevAioOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevAioOptions {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BlockdevAioOptions::threads,
BlockdevAioOptions::native,
BlockdevAioOptions::io_uring,

    ];
    const NAMES: &'static [&'static str] = &[

"threads",
"native",
"io_uring",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevDriver {
	#[serde(rename = "blkdebug")] blkdebug,
	#[serde(rename = "blklogwrites")] blklogwrites,
	#[serde(rename = "blkreplay")] blkreplay,
	#[serde(rename = "blkverify")] blkverify,
	#[serde(rename = "bochs")] bochs,
	#[serde(rename = "cloop")] cloop,
	#[serde(rename = "compress")] compress,
	#[serde(rename = "copy-before-write")] copy_before_write,
	#[serde(rename = "copy-on-read")] copy_on_read,
	#[serde(rename = "dmg")] dmg,
	#[serde(rename = "file")] file,
	#[serde(rename = "snapshot-access")] snapshot_access,
	#[serde(rename = "ftp")] ftp,
	#[serde(rename = "ftps")] ftps,
	#[serde(rename = "gluster")] gluster,
	#[serde(rename = "host_cdrom")] host_cdrom,
	#[serde(rename = "host_device")] host_device,
	#[serde(rename = "http")] http,
	#[serde(rename = "https")] https,
	#[serde(rename = "io_uring")] io_uring,
	#[serde(rename = "iscsi")] iscsi,
	#[serde(rename = "luks")] luks,
	#[serde(rename = "nbd")] nbd,
	#[serde(rename = "nfs")] nfs,
	#[serde(rename = "null-aio")] null_aio,
	#[serde(rename = "null-co")] null_co,
	#[serde(rename = "nvme")] nvme,
	#[serde(rename = "nvme-io_uring")] nvme_io_uring,
	#[serde(rename = "parallels")] parallels,
	#[serde(rename = "preallocate")] preallocate,
	#[serde(rename = "qcow")] qcow,
	#[serde(rename = "qcow2")] qcow2,
	#[serde(rename = "qed")] qed,
	#[serde(rename = "quorum")] quorum,
	#[serde(rename = "raw")] raw,
	#[serde(rename = "rbd")] rbd,
	#[serde(rename = "replication")] replication,
	#[serde(rename = "ssh")] ssh,
	#[serde(rename = "throttle")] throttle,
	#[serde(rename = "vdi")] vdi,
	#[serde(rename = "vhdx")] vhdx,
	#[serde(rename = "virtio-blk-vfio-pci")] virtio_blk_vfio_pci,
	#[serde(rename = "virtio-blk-vhost-user")] virtio_blk_vhost_user,
	#[serde(rename = "virtio-blk-vhost-vdpa")] virtio_blk_vhost_vdpa,
	#[serde(rename = "vmdk")] vmdk,
	#[serde(rename = "vpc")] vpc,
	#[serde(rename = "vvfat")] vvfat,
}

impl ::core::str::FromStr for BlockdevDriver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevDriver {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 47;
    const VARIANTS: &'static [Self] = &[

BlockdevDriver::blkdebug,
BlockdevDriver::blklogwrites,
BlockdevDriver::blkreplay,
BlockdevDriver::blkverify,
BlockdevDriver::bochs,
BlockdevDriver::cloop,
BlockdevDriver::compress,
BlockdevDriver::copy_before_write,
BlockdevDriver::copy_on_read,
BlockdevDriver::dmg,
BlockdevDriver::file,
BlockdevDriver::snapshot_access,
BlockdevDriver::ftp,
BlockdevDriver::ftps,
BlockdevDriver::gluster,
BlockdevDriver::host_cdrom,
BlockdevDriver::host_device,
BlockdevDriver::http,
BlockdevDriver::https,
BlockdevDriver::io_uring,
BlockdevDriver::iscsi,
BlockdevDriver::luks,
BlockdevDriver::nbd,
BlockdevDriver::nfs,
BlockdevDriver::null_aio,
BlockdevDriver::null_co,
BlockdevDriver::nvme,
BlockdevDriver::nvme_io_uring,
BlockdevDriver::parallels,
BlockdevDriver::preallocate,
BlockdevDriver::qcow,
BlockdevDriver::qcow2,
BlockdevDriver::qed,
BlockdevDriver::quorum,
BlockdevDriver::raw,
BlockdevDriver::rbd,
BlockdevDriver::replication,
BlockdevDriver::ssh,
BlockdevDriver::throttle,
BlockdevDriver::vdi,
BlockdevDriver::vhdx,
BlockdevDriver::virtio_blk_vfio_pci,
BlockdevDriver::virtio_blk_vhost_user,
BlockdevDriver::virtio_blk_vhost_vdpa,
BlockdevDriver::vmdk,
BlockdevDriver::vpc,
BlockdevDriver::vvfat,

    ];
    const NAMES: &'static [&'static str] = &[

"blkdebug",
"blklogwrites",
"blkreplay",
"blkverify",
"bochs",
"cloop",
"compress",
"copy-before-write",
"copy-on-read",
"dmg",
"file",
"snapshot-access",
"ftp",
"ftps",
"gluster",
"host_cdrom",
"host_device",
"http",
"https",
"io_uring",
"iscsi",
"luks",
"nbd",
"nfs",
"null-aio",
"null-co",
"nvme",
"nvme-io_uring",
"parallels",
"preallocate",
"qcow",
"qcow2",
"qed",
"quorum",
"raw",
"rbd",
"replication",
"ssh",
"throttle",
"vdi",
"vhdx",
"virtio-blk-vfio-pci",
"virtio-blk-vhost-user",
"virtio-blk-vhost-vdpa",
"vmdk",
"vpc",
"vvfat",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Qcow2OverlapCheckMode {
	#[serde(rename = "none")] none,
	#[serde(rename = "constant")] constant,
	#[serde(rename = "cached")] cached,
	#[serde(rename = "all")] all,
}

impl ::core::str::FromStr for Qcow2OverlapCheckMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for Qcow2OverlapCheckMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

Qcow2OverlapCheckMode::none,
Qcow2OverlapCheckMode::constant,
Qcow2OverlapCheckMode::cached,
Qcow2OverlapCheckMode::all,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"constant",
"cached",
"all",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Qcow2OverlapChecks {
	#[serde(rename = "flags")] flags(Qcow2OverlapCheckFlags),
	#[serde(rename = "mode")] mode(Qcow2OverlapCheckMode),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevQcowEncryptionFormat {
	#[serde(rename = "aes")] aes,
}

impl ::core::str::FromStr for BlockdevQcowEncryptionFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevQcowEncryptionFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

BlockdevQcowEncryptionFormat::aes,

    ];
    const NAMES: &'static [&'static str] = &[

"aes",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevQcow2EncryptionFormat {
	#[serde(rename = "aes")] aes,
	#[serde(rename = "luks")] luks,
}

impl ::core::str::FromStr for BlockdevQcow2EncryptionFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevQcow2EncryptionFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockdevQcow2EncryptionFormat::aes,
BlockdevQcow2EncryptionFormat::luks,

    ];
    const NAMES: &'static [&'static str] = &[

"aes",
"luks",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SshHostKeyCheckMode {
	#[serde(rename = "none")] none,
	#[serde(rename = "hash")] hash,
	#[serde(rename = "known_hosts")] known_hosts,
}

impl ::core::str::FromStr for SshHostKeyCheckMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SshHostKeyCheckMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

SshHostKeyCheckMode::none,
SshHostKeyCheckMode::hash,
SshHostKeyCheckMode::known_hosts,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"hash",
"known_hosts",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SshHostKeyCheckHashType {
	#[serde(rename = "md5")] md5,
	#[serde(rename = "sha1")] sha1,
	#[serde(rename = "sha256")] sha256,
}

impl ::core::str::FromStr for SshHostKeyCheckHashType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SshHostKeyCheckHashType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

SshHostKeyCheckHashType::md5,
SshHostKeyCheckHashType::sha1,
SshHostKeyCheckHashType::sha256,

    ];
    const NAMES: &'static [&'static str] = &[

"md5",
"sha1",
"sha256",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlkdebugEvent {
	#[serde(rename = "l1_update")] l1_update,
	#[serde(rename = "l1_grow_alloc_table")] l1_grow_alloc_table,
	#[serde(rename = "l1_grow_write_table")] l1_grow_write_table,
	#[serde(rename = "l1_grow_activate_table")] l1_grow_activate_table,
	#[serde(rename = "l2_load")] l2_load,
	#[serde(rename = "l2_update")] l2_update,
	#[serde(rename = "l2_update_compressed")] l2_update_compressed,
	#[serde(rename = "l2_alloc_cow_read")] l2_alloc_cow_read,
	#[serde(rename = "l2_alloc_write")] l2_alloc_write,
	#[serde(rename = "read_aio")] read_aio,
	#[serde(rename = "read_backing_aio")] read_backing_aio,
	#[serde(rename = "read_compressed")] read_compressed,
	#[serde(rename = "write_aio")] write_aio,
	#[serde(rename = "write_compressed")] write_compressed,
	#[serde(rename = "vmstate_load")] vmstate_load,
	#[serde(rename = "vmstate_save")] vmstate_save,
	#[serde(rename = "cow_read")] cow_read,
	#[serde(rename = "cow_write")] cow_write,
	#[serde(rename = "reftable_load")] reftable_load,
	#[serde(rename = "reftable_grow")] reftable_grow,
	#[serde(rename = "reftable_update")] reftable_update,
	#[serde(rename = "refblock_load")] refblock_load,
	#[serde(rename = "refblock_update")] refblock_update,
	#[serde(rename = "refblock_update_part")] refblock_update_part,
	#[serde(rename = "refblock_alloc")] refblock_alloc,
	#[serde(rename = "refblock_alloc_hookup")] refblock_alloc_hookup,
	#[serde(rename = "refblock_alloc_write")] refblock_alloc_write,
	#[serde(rename = "refblock_alloc_write_blocks")] refblock_alloc_write_blocks,
	#[serde(rename = "refblock_alloc_write_table")] refblock_alloc_write_table,
	#[serde(rename = "refblock_alloc_switch_table")] refblock_alloc_switch_table,
	#[serde(rename = "cluster_alloc")] cluster_alloc,
	#[serde(rename = "cluster_alloc_bytes")] cluster_alloc_bytes,
	#[serde(rename = "cluster_free")] cluster_free,
	#[serde(rename = "flush_to_os")] flush_to_os,
	#[serde(rename = "flush_to_disk")] flush_to_disk,
	#[serde(rename = "pwritev_rmw_head")] pwritev_rmw_head,
	#[serde(rename = "pwritev_rmw_after_head")] pwritev_rmw_after_head,
	#[serde(rename = "pwritev_rmw_tail")] pwritev_rmw_tail,
	#[serde(rename = "pwritev_rmw_after_tail")] pwritev_rmw_after_tail,
	#[serde(rename = "pwritev")] pwritev,
	#[serde(rename = "pwritev_zero")] pwritev_zero,
	#[serde(rename = "pwritev_done")] pwritev_done,
	#[serde(rename = "empty_image_prepare")] empty_image_prepare,
	#[serde(rename = "l1_shrink_write_table")] l1_shrink_write_table,
	#[serde(rename = "l1_shrink_free_l2_clusters")] l1_shrink_free_l2_clusters,
	#[serde(rename = "cor_write")] cor_write,
	#[serde(rename = "cluster_alloc_space")] cluster_alloc_space,
	#[serde(rename = "none")] none,
}

impl ::core::str::FromStr for BlkdebugEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlkdebugEvent {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 48;
    const VARIANTS: &'static [Self] = &[

BlkdebugEvent::l1_update,
BlkdebugEvent::l1_grow_alloc_table,
BlkdebugEvent::l1_grow_write_table,
BlkdebugEvent::l1_grow_activate_table,
BlkdebugEvent::l2_load,
BlkdebugEvent::l2_update,
BlkdebugEvent::l2_update_compressed,
BlkdebugEvent::l2_alloc_cow_read,
BlkdebugEvent::l2_alloc_write,
BlkdebugEvent::read_aio,
BlkdebugEvent::read_backing_aio,
BlkdebugEvent::read_compressed,
BlkdebugEvent::write_aio,
BlkdebugEvent::write_compressed,
BlkdebugEvent::vmstate_load,
BlkdebugEvent::vmstate_save,
BlkdebugEvent::cow_read,
BlkdebugEvent::cow_write,
BlkdebugEvent::reftable_load,
BlkdebugEvent::reftable_grow,
BlkdebugEvent::reftable_update,
BlkdebugEvent::refblock_load,
BlkdebugEvent::refblock_update,
BlkdebugEvent::refblock_update_part,
BlkdebugEvent::refblock_alloc,
BlkdebugEvent::refblock_alloc_hookup,
BlkdebugEvent::refblock_alloc_write,
BlkdebugEvent::refblock_alloc_write_blocks,
BlkdebugEvent::refblock_alloc_write_table,
BlkdebugEvent::refblock_alloc_switch_table,
BlkdebugEvent::cluster_alloc,
BlkdebugEvent::cluster_alloc_bytes,
BlkdebugEvent::cluster_free,
BlkdebugEvent::flush_to_os,
BlkdebugEvent::flush_to_disk,
BlkdebugEvent::pwritev_rmw_head,
BlkdebugEvent::pwritev_rmw_after_head,
BlkdebugEvent::pwritev_rmw_tail,
BlkdebugEvent::pwritev_rmw_after_tail,
BlkdebugEvent::pwritev,
BlkdebugEvent::pwritev_zero,
BlkdebugEvent::pwritev_done,
BlkdebugEvent::empty_image_prepare,
BlkdebugEvent::l1_shrink_write_table,
BlkdebugEvent::l1_shrink_free_l2_clusters,
BlkdebugEvent::cor_write,
BlkdebugEvent::cluster_alloc_space,
BlkdebugEvent::none,

    ];
    const NAMES: &'static [&'static str] = &[

"l1_update",
"l1_grow_alloc_table",
"l1_grow_write_table",
"l1_grow_activate_table",
"l2_load",
"l2_update",
"l2_update_compressed",
"l2_alloc_cow_read",
"l2_alloc_write",
"read_aio",
"read_backing_aio",
"read_compressed",
"write_aio",
"write_compressed",
"vmstate_load",
"vmstate_save",
"cow_read",
"cow_write",
"reftable_load",
"reftable_grow",
"reftable_update",
"refblock_load",
"refblock_update",
"refblock_update_part",
"refblock_alloc",
"refblock_alloc_hookup",
"refblock_alloc_write",
"refblock_alloc_write_blocks",
"refblock_alloc_write_table",
"refblock_alloc_switch_table",
"cluster_alloc",
"cluster_alloc_bytes",
"cluster_free",
"flush_to_os",
"flush_to_disk",
"pwritev_rmw_head",
"pwritev_rmw_after_head",
"pwritev_rmw_tail",
"pwritev_rmw_after_tail",
"pwritev",
"pwritev_zero",
"pwritev_done",
"empty_image_prepare",
"l1_shrink_write_table",
"l1_shrink_free_l2_clusters",
"cor_write",
"cluster_alloc_space",
"none",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlkdebugIOType {
	#[serde(rename = "read")] read,
	#[serde(rename = "write")] write,
	#[serde(rename = "write-zeroes")] write_zeroes,
	#[serde(rename = "discard")] discard,
	#[serde(rename = "flush")] flush,
	#[serde(rename = "block-status")] block_status,
}

impl ::core::str::FromStr for BlkdebugIOType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlkdebugIOType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

BlkdebugIOType::read,
BlkdebugIOType::write,
BlkdebugIOType::write_zeroes,
BlkdebugIOType::discard,
BlkdebugIOType::flush,
BlkdebugIOType::block_status,

    ];
    const NAMES: &'static [&'static str] = &[

"read",
"write",
"write-zeroes",
"discard",
"flush",
"block-status",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuorumReadPattern {
	#[serde(rename = "quorum")] quorum,
	#[serde(rename = "fifo")] fifo,
}

impl ::core::str::FromStr for QuorumReadPattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QuorumReadPattern {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QuorumReadPattern::quorum,
QuorumReadPattern::fifo,

    ];
    const NAMES: &'static [&'static str] = &[

"quorum",
"fifo",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IscsiTransport {
	#[serde(rename = "tcp")] tcp,
	#[serde(rename = "iser")] iser,
}

impl ::core::str::FromStr for IscsiTransport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for IscsiTransport {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

IscsiTransport::tcp,
IscsiTransport::iser,

    ];
    const NAMES: &'static [&'static str] = &[

"tcp",
"iser",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IscsiHeaderDigest {
	#[serde(rename = "crc32c")] crc32c,
	#[serde(rename = "none")] none,
	#[serde(rename = "crc32c-none")] crc32c_none,
	#[serde(rename = "none-crc32c")] none_crc32c,
}

impl ::core::str::FromStr for IscsiHeaderDigest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for IscsiHeaderDigest {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

IscsiHeaderDigest::crc32c,
IscsiHeaderDigest::none,
IscsiHeaderDigest::crc32c_none,
IscsiHeaderDigest::none_crc32c,

    ];
    const NAMES: &'static [&'static str] = &[

"crc32c",
"none",
"crc32c-none",
"none-crc32c",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RbdAuthMode {
	#[serde(rename = "cephx")] cephx,
	#[serde(rename = "none")] none,
}

impl ::core::str::FromStr for RbdAuthMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RbdAuthMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

RbdAuthMode::cephx,
RbdAuthMode::none,

    ];
    const NAMES: &'static [&'static str] = &[

"cephx",
"none",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RbdImageEncryptionFormat {
	#[serde(rename = "luks")] luks,
	#[serde(rename = "luks2")] luks2,
	#[serde(rename = "luks-any")] luks_any,
}

impl ::core::str::FromStr for RbdImageEncryptionFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RbdImageEncryptionFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

RbdImageEncryptionFormat::luks,
RbdImageEncryptionFormat::luks2,
RbdImageEncryptionFormat::luks_any,

    ];
    const NAMES: &'static [&'static str] = &[

"luks",
"luks2",
"luks-any",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReplicationMode {
	#[serde(rename = "primary")] primary,
	#[serde(rename = "secondary")] secondary,
}

impl ::core::str::FromStr for ReplicationMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ReplicationMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

ReplicationMode::primary,
ReplicationMode::secondary,

    ];
    const NAMES: &'static [&'static str] = &[

"primary",
"secondary",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NFSTransport {
	#[serde(rename = "inet")] inet,
}

impl ::core::str::FromStr for NFSTransport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NFSTransport {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

NFSTransport::inet,

    ];
    const NAMES: &'static [&'static str] = &[

"inet",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnCbwError {
	#[serde(rename = "break-guest-write")] break_guest_write,
	#[serde(rename = "break-snapshot")] break_snapshot,
}

impl ::core::str::FromStr for OnCbwError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for OnCbwError {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

OnCbwError::break_guest_write,
OnCbwError::break_snapshot,

    ];
    const NAMES: &'static [&'static str] = &[

"break-guest-write",
"break-snapshot",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockdevRef {
	#[serde(rename = "definition")] definition(Box<BlockdevOptions>),
	#[serde(rename = "reference")] reference(::std::string::String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockdevRefOrNull {
	#[serde(rename = "definition")] definition(Box<BlockdevOptions>),
	#[serde(rename = "null")] null(()),
	#[serde(rename = "reference")] reference(::std::string::String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_add(pub BlockdevOptions);

impl From<BlockdevOptions> for blockdev_add {
    fn from(val: BlockdevOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for blockdev_add { }
impl ::qapi_spec::Command for blockdev_add {
    const NAME: &'static str = "blockdev-add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_reopen {
	#[serde(rename = "options")]
pub options: Vec<BlockdevOptions>,
}

impl crate::QmpCommand for blockdev_reopen { }
impl ::qapi_spec::Command for blockdev_reopen {
    const NAME: &'static str = "blockdev-reopen";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_del {
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

impl crate::QmpCommand for blockdev_del { }
impl ::qapi_spec::Command for blockdev_del {
    const NAME: &'static str = "blockdev-del";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevQcow2Version {
	#[serde(rename = "v2")] v2,
	#[serde(rename = "v3")] v3,
}

impl ::core::str::FromStr for BlockdevQcow2Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevQcow2Version {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockdevQcow2Version::v2,
BlockdevQcow2Version::v3,

    ];
    const NAMES: &'static [&'static str] = &[

"v2",
"v3",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Qcow2CompressionType {
	#[serde(rename = "zlib")] zlib,
	#[serde(rename = "zstd")] zstd,
}

impl ::core::str::FromStr for Qcow2CompressionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for Qcow2CompressionType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

Qcow2CompressionType::zlib,
Qcow2CompressionType::zstd,

    ];
    const NAMES: &'static [&'static str] = &[

"zlib",
"zstd",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevVmdkSubformat {
	#[serde(rename = "monolithicSparse")] monolithicSparse,
	#[serde(rename = "monolithicFlat")] monolithicFlat,
	#[serde(rename = "twoGbMaxExtentSparse")] twoGbMaxExtentSparse,
	#[serde(rename = "twoGbMaxExtentFlat")] twoGbMaxExtentFlat,
	#[serde(rename = "streamOptimized")] streamOptimized,
}

impl ::core::str::FromStr for BlockdevVmdkSubformat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevVmdkSubformat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

BlockdevVmdkSubformat::monolithicSparse,
BlockdevVmdkSubformat::monolithicFlat,
BlockdevVmdkSubformat::twoGbMaxExtentSparse,
BlockdevVmdkSubformat::twoGbMaxExtentFlat,
BlockdevVmdkSubformat::streamOptimized,

    ];
    const NAMES: &'static [&'static str] = &[

"monolithicSparse",
"monolithicFlat",
"twoGbMaxExtentSparse",
"twoGbMaxExtentFlat",
"streamOptimized",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevVmdkAdapterType {
	#[serde(rename = "ide")] ide,
	#[serde(rename = "buslogic")] buslogic,
	#[serde(rename = "lsilogic")] lsilogic,
	#[serde(rename = "legacyESX")] legacyESX,
}

impl ::core::str::FromStr for BlockdevVmdkAdapterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevVmdkAdapterType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

BlockdevVmdkAdapterType::ide,
BlockdevVmdkAdapterType::buslogic,
BlockdevVmdkAdapterType::lsilogic,
BlockdevVmdkAdapterType::legacyESX,

    ];
    const NAMES: &'static [&'static str] = &[

"ide",
"buslogic",
"lsilogic",
"legacyESX",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevVhdxSubformat {
	#[serde(rename = "dynamic")] dynamic,
	#[serde(rename = "fixed")] fixed,
}

impl ::core::str::FromStr for BlockdevVhdxSubformat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevVhdxSubformat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockdevVhdxSubformat::dynamic,
BlockdevVhdxSubformat::fixed,

    ];
    const NAMES: &'static [&'static str] = &[

"dynamic",
"fixed",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockdevVpcSubformat {
	#[serde(rename = "dynamic")] dynamic,
	#[serde(rename = "fixed")] fixed,
}

impl ::core::str::FromStr for BlockdevVpcSubformat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockdevVpcSubformat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockdevVpcSubformat::dynamic,
BlockdevVpcSubformat::fixed,

    ];
    const NAMES: &'static [&'static str] = &[

"dynamic",
"fixed",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_create {
	#[serde(rename = "job-id")]
pub job_id: ::std::string::String,
	#[serde(rename = "options")]
pub options: BlockdevCreateOptions,
}

impl crate::QmpCommand for blockdev_create { }
impl ::qapi_spec::Command for blockdev_create {
    const NAME: &'static str = "blockdev-create";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_blockdev_amend {
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "job-id")]
pub job_id: ::std::string::String,
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
	#[serde(rename = "options")]
pub options: BlockdevAmendOptions,
}

impl crate::QmpCommand for x_blockdev_amend { }
impl ::qapi_spec::Command for x_blockdev_amend {
    const NAME: &'static str = "x-blockdev-amend";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockErrorAction {
	#[serde(rename = "ignore")] ignore,
	#[serde(rename = "report")] report,
	#[serde(rename = "stop")] stop,
}

impl ::core::str::FromStr for BlockErrorAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockErrorAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

BlockErrorAction::ignore,
BlockErrorAction::report,
BlockErrorAction::stop,

    ];
    const NAMES: &'static [&'static str] = &[

"ignore",
"report",
"stop",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_IMAGE_CORRUPTED {
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
pub offset: Option<i64>,
#[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
pub size: Option<i64>,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "fatal")]
pub fatal: bool,
#[serde(rename = "msg")]
pub msg: ::std::string::String,
}

impl ::qapi_spec::Event for BLOCK_IMAGE_CORRUPTED {
    const NAME: &'static str = "BLOCK_IMAGE_CORRUPTED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_IO_ERROR {
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "nospace", default, skip_serializing_if = "Option::is_none")]
pub nospace: Option<bool>,
#[serde(rename = "action")]
pub action: BlockErrorAction,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "operation")]
pub operation: IoOperationType,
#[serde(rename = "reason")]
pub reason: ::std::string::String,
}

impl ::qapi_spec::Event for BLOCK_IO_ERROR {
    const NAME: &'static str = "BLOCK_IO_ERROR";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_JOB_COMPLETED {
#[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
pub error: Option<::std::string::String>,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "len")]
pub len: i64,
#[serde(rename = "offset")]
pub offset: i64,
#[serde(rename = "speed")]
pub speed: i64,
#[serde(rename = "type")]
pub type_: JobType,
}

impl ::qapi_spec::Event for BLOCK_JOB_COMPLETED {
    const NAME: &'static str = "BLOCK_JOB_COMPLETED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_JOB_CANCELLED {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "len")]
pub len: i64,
#[serde(rename = "offset")]
pub offset: i64,
#[serde(rename = "speed")]
pub speed: i64,
#[serde(rename = "type")]
pub type_: JobType,
}

impl ::qapi_spec::Event for BLOCK_JOB_CANCELLED {
    const NAME: &'static str = "BLOCK_JOB_CANCELLED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_JOB_ERROR {
#[serde(rename = "action")]
pub action: BlockErrorAction,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "operation")]
pub operation: IoOperationType,
}

impl ::qapi_spec::Event for BLOCK_JOB_ERROR {
    const NAME: &'static str = "BLOCK_JOB_ERROR";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_JOB_READY {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "len")]
pub len: i64,
#[serde(rename = "offset")]
pub offset: i64,
#[serde(rename = "speed")]
pub speed: i64,
#[serde(rename = "type")]
pub type_: JobType,
}

impl ::qapi_spec::Event for BLOCK_JOB_READY {
    const NAME: &'static str = "BLOCK_JOB_READY";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_JOB_PENDING {
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "type")]
pub type_: JobType,
}

impl ::qapi_spec::Event for BLOCK_JOB_PENDING {
    const NAME: &'static str = "BLOCK_JOB_PENDING";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PreallocMode {
	#[serde(rename = "off")] off,
	#[serde(rename = "metadata")] metadata,
	#[serde(rename = "falloc")] falloc,
	#[serde(rename = "full")] full,
}

impl ::core::str::FromStr for PreallocMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for PreallocMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

PreallocMode::off,
PreallocMode::metadata,
PreallocMode::falloc,
PreallocMode::full,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"metadata",
"falloc",
"full",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_WRITE_THRESHOLD {
#[serde(rename = "amount-exceeded")]
pub amount_exceeded: u64,
#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
#[serde(rename = "write-threshold")]
pub write_threshold: u64,
}

impl ::qapi_spec::Event for BLOCK_WRITE_THRESHOLD {
    const NAME: &'static str = "BLOCK_WRITE_THRESHOLD";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_set_write_threshold {
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
	#[serde(rename = "write-threshold")]
pub write_threshold: u64,
}

impl crate::QmpCommand for block_set_write_threshold { }
impl ::qapi_spec::Command for block_set_write_threshold {
    const NAME: &'static str = "block-set-write-threshold";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_blockdev_change {
	#[serde(rename = "child", default, skip_serializing_if = "Option::is_none")]
pub child: Option<::std::string::String>,
	#[serde(rename = "node", default, skip_serializing_if = "Option::is_none")]
pub node: Option<::std::string::String>,
	#[serde(rename = "parent")]
pub parent: ::std::string::String,
}

impl crate::QmpCommand for x_blockdev_change { }
impl ::qapi_spec::Command for x_blockdev_change {
    const NAME: &'static str = "x-blockdev-change";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_blockdev_set_iothread {
	#[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
pub force: Option<bool>,
	#[serde(rename = "iothread")]
pub iothread: StrOrNull,
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

impl crate::QmpCommand for x_blockdev_set_iothread { }
impl ::qapi_spec::Command for x_blockdev_set_iothread {
    const NAME: &'static str = "x-blockdev-set-iothread";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuorumOpType {
	#[serde(rename = "read")] read,
	#[serde(rename = "write")] write,
	#[serde(rename = "flush")] flush,
}

impl ::core::str::FromStr for QuorumOpType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QuorumOpType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

QuorumOpType::read,
QuorumOpType::write,
QuorumOpType::flush,

    ];
    const NAMES: &'static [&'static str] = &[

"read",
"write",
"flush",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUORUM_FAILURE {
#[serde(rename = "reference")]
pub reference: ::std::string::String,
#[serde(rename = "sector-num")]
pub sector_num: i64,
#[serde(rename = "sectors-count")]
pub sectors_count: i64,
}

impl ::qapi_spec::Event for QUORUM_FAILURE {
    const NAME: &'static str = "QUORUM_FAILURE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUORUM_REPORT_BAD {
#[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
pub error: Option<::std::string::String>,
#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
#[serde(rename = "sector-num")]
pub sector_num: i64,
#[serde(rename = "sectors-count")]
pub sectors_count: i64,
#[serde(rename = "type")]
pub type_: QuorumOpType,
}

impl ::qapi_spec::Event for QUORUM_REPORT_BAD {
    const NAME: &'static str = "QUORUM_REPORT_BAD";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_snapshot_internal_sync(pub BlockdevSnapshotInternal);

impl From<BlockdevSnapshotInternal> for blockdev_snapshot_internal_sync {
    fn from(val: BlockdevSnapshotInternal) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for blockdev_snapshot_internal_sync { }
impl ::qapi_spec::Command for blockdev_snapshot_internal_sync {
    const NAME: &'static str = "blockdev-snapshot-internal-sync";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_snapshot_delete_internal_sync {
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
	#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for blockdev_snapshot_delete_internal_sync { }
impl ::qapi_spec::Command for blockdev_snapshot_delete_internal_sync {
    const NAME: &'static str = "blockdev-snapshot-delete-internal-sync";
    const ALLOW_OOB: bool = false;

    type Ok = SnapshotInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct nbd_server_start {
	#[serde(rename = "max-connections", default, skip_serializing_if = "Option::is_none")]
pub max_connections: Option<u32>,
	#[serde(rename = "tls-authz", default, skip_serializing_if = "Option::is_none")]
pub tls_authz: Option<::std::string::String>,
	#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<::std::string::String>,
	#[serde(rename = "addr")]
pub addr: SocketAddressLegacy,
}

impl crate::QmpCommand for nbd_server_start { }
impl ::qapi_spec::Command for nbd_server_start {
    const NAME: &'static str = "nbd-server-start";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FuseExportAllowOther {
	#[serde(rename = "off")] off,
	#[serde(rename = "on")] on,
	#[serde(rename = "auto")] auto,
}

impl ::core::str::FromStr for FuseExportAllowOther {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for FuseExportAllowOther {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

FuseExportAllowOther::off,
FuseExportAllowOther::on,
FuseExportAllowOther::auto,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"on",
"auto",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)] #[deprecated]
pub struct nbd_server_add(pub NbdServerAddOptions);

impl From<NbdServerAddOptions> for nbd_server_add {
    fn from(val: NbdServerAddOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for nbd_server_add { }
impl ::qapi_spec::Command for nbd_server_add {
    const NAME: &'static str = "nbd-server-add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockExportRemoveMode {
	#[serde(rename = "safe")] safe,
	#[serde(rename = "hard")] hard,
}

impl ::core::str::FromStr for BlockExportRemoveMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockExportRemoveMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

BlockExportRemoveMode::safe,
BlockExportRemoveMode::hard,

    ];
    const NAMES: &'static [&'static str] = &[

"safe",
"hard",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)] #[deprecated]
pub struct nbd_server_remove {
	#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<BlockExportRemoveMode>,
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for nbd_server_remove { }
impl ::qapi_spec::Command for nbd_server_remove {
    const NAME: &'static str = "nbd-server-remove";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct nbd_server_stop {
}

impl crate::QmpCommand for nbd_server_stop { }
impl ::qapi_spec::Command for nbd_server_stop {
    const NAME: &'static str = "nbd-server-stop";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockExportType {
	#[serde(rename = "nbd")] nbd,
	#[serde(rename = "vhost-user-blk")] vhost_user_blk,
	#[serde(rename = "fuse")] fuse,
	#[serde(rename = "vduse-blk")] vduse_blk,
}

impl ::core::str::FromStr for BlockExportType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for BlockExportType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

BlockExportType::nbd,
BlockExportType::vhost_user_blk,
BlockExportType::fuse,
BlockExportType::vduse_blk,

    ];
    const NAMES: &'static [&'static str] = &[

"nbd",
"vhost-user-blk",
"fuse",
"vduse-blk",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_export_add(pub BlockExportOptions);

impl From<BlockExportOptions> for block_export_add {
    fn from(val: BlockExportOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for block_export_add { }
impl ::qapi_spec::Command for block_export_add {
    const NAME: &'static str = "block-export-add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct block_export_del {
	#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<BlockExportRemoveMode>,
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for block_export_del { }
impl ::qapi_spec::Command for block_export_del {
    const NAME: &'static str = "block-export-del";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLOCK_EXPORT_DELETED {
#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl ::qapi_spec::Event for BLOCK_EXPORT_DELETED {
    const NAME: &'static str = "BLOCK_EXPORT_DELETED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_block_exports {
}

impl crate::QmpCommand for query_block_exports { }
impl ::qapi_spec::Command for query_block_exports {
    const NAME: &'static str = "query-block-exports";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<BlockExportInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_chardev {
}

impl crate::QmpCommand for query_chardev { }
impl ::qapi_spec::Command for query_chardev {
    const NAME: &'static str = "query-chardev";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ChardevInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_chardev_backends {
}

impl crate::QmpCommand for query_chardev_backends { }
impl ::qapi_spec::Command for query_chardev_backends {
    const NAME: &'static str = "query-chardev-backends";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ChardevBackendInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataFormat {
	#[serde(rename = "utf8")] utf8,
	#[serde(rename = "base64")] base64,
}

impl ::core::str::FromStr for DataFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DataFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

DataFormat::utf8,
DataFormat::base64,

    ];
    const NAMES: &'static [&'static str] = &[

"utf8",
"base64",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ringbuf_write {
	#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<DataFormat>,
	#[serde(rename = "data")]
pub data: ::std::string::String,
	#[serde(rename = "device")]
pub device: ::std::string::String,
}

impl crate::QmpCommand for ringbuf_write { }
impl ::qapi_spec::Command for ringbuf_write {
    const NAME: &'static str = "ringbuf-write";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ringbuf_read {
	#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<DataFormat>,
	#[serde(rename = "device")]
pub device: ::std::string::String,
	#[serde(rename = "size")]
pub size: i64,
}

impl crate::QmpCommand for ringbuf_read { }
impl ::qapi_spec::Command for ringbuf_read {
    const NAME: &'static str = "ringbuf-read";
    const ALLOW_OOB: bool = false;

    type Ok = ::std::string::String;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChardevBackendKind {
	#[serde(rename = "file")] file,
	#[serde(rename = "serial")] serial,
	#[serde(rename = "parallel")] parallel,
	#[serde(rename = "pipe")] pipe,
	#[serde(rename = "socket")] socket,
	#[serde(rename = "udp")] udp,
	#[serde(rename = "pty")] pty,
	#[serde(rename = "null")] null,
	#[serde(rename = "mux")] mux,
	#[serde(rename = "msmouse")] msmouse,
	#[serde(rename = "wctablet")] wctablet,
	#[serde(rename = "braille")] braille,
	#[serde(rename = "testdev")] testdev,
	#[serde(rename = "stdio")] stdio,
	#[serde(rename = "console")] console,
	#[serde(rename = "spicevmc")] spicevmc,
	#[serde(rename = "spiceport")] spiceport,
	#[serde(rename = "qemu-vdagent")] qemu_vdagent,
	#[serde(rename = "dbus")] dbus,
	#[serde(rename = "vc")] vc,
	#[serde(rename = "ringbuf")] ringbuf,
	#[serde(rename = "memory")] memory,
}

impl ::core::str::FromStr for ChardevBackendKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ChardevBackendKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 22;
    const VARIANTS: &'static [Self] = &[

ChardevBackendKind::file,
ChardevBackendKind::serial,
ChardevBackendKind::parallel,
ChardevBackendKind::pipe,
ChardevBackendKind::socket,
ChardevBackendKind::udp,
ChardevBackendKind::pty,
ChardevBackendKind::null,
ChardevBackendKind::mux,
ChardevBackendKind::msmouse,
ChardevBackendKind::wctablet,
ChardevBackendKind::braille,
ChardevBackendKind::testdev,
ChardevBackendKind::stdio,
ChardevBackendKind::console,
ChardevBackendKind::spicevmc,
ChardevBackendKind::spiceport,
ChardevBackendKind::qemu_vdagent,
ChardevBackendKind::dbus,
ChardevBackendKind::vc,
ChardevBackendKind::ringbuf,
ChardevBackendKind::memory,

    ];
    const NAMES: &'static [&'static str] = &[

"file",
"serial",
"parallel",
"pipe",
"socket",
"udp",
"pty",
"null",
"mux",
"msmouse",
"wctablet",
"braille",
"testdev",
"stdio",
"console",
"spicevmc",
"spiceport",
"qemu-vdagent",
"dbus",
"vc",
"ringbuf",
"memory",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct chardev_add {
	#[serde(rename = "backend")]
pub backend: ChardevBackend,
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for chardev_add { }
impl ::qapi_spec::Command for chardev_add {
    const NAME: &'static str = "chardev-add";
    const ALLOW_OOB: bool = false;

    type Ok = ChardevReturn;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct chardev_change {
	#[serde(rename = "backend")]
pub backend: ChardevBackend,
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for chardev_change { }
impl ::qapi_spec::Command for chardev_change {
    const NAME: &'static str = "chardev-change";
    const ALLOW_OOB: bool = false;

    type Ok = ChardevReturn;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct chardev_remove {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for chardev_remove { }
impl ::qapi_spec::Command for chardev_remove {
    const NAME: &'static str = "chardev-remove";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct chardev_send_break {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for chardev_send_break { }
impl ::qapi_spec::Command for chardev_send_break {
    const NAME: &'static str = "chardev-send-break";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSERPORT_CHANGE {
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "open")]
pub open: bool,
}

impl ::qapi_spec::Event for VSERPORT_CHANGE {
    const NAME: &'static str = "VSERPORT_CHANGE";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DumpGuestMemoryFormat {
	#[serde(rename = "elf")] elf,
	#[serde(rename = "kdump-zlib")] kdump_zlib,
	#[serde(rename = "kdump-lzo")] kdump_lzo,
	#[serde(rename = "kdump-snappy")] kdump_snappy,
	#[serde(rename = "kdump-raw-zlib")] kdump_raw_zlib,
	#[serde(rename = "kdump-raw-lzo")] kdump_raw_lzo,
	#[serde(rename = "kdump-raw-snappy")] kdump_raw_snappy,
	#[serde(rename = "win-dmp")] win_dmp,
}

impl ::core::str::FromStr for DumpGuestMemoryFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DumpGuestMemoryFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 8;
    const VARIANTS: &'static [Self] = &[

DumpGuestMemoryFormat::elf,
DumpGuestMemoryFormat::kdump_zlib,
DumpGuestMemoryFormat::kdump_lzo,
DumpGuestMemoryFormat::kdump_snappy,
DumpGuestMemoryFormat::kdump_raw_zlib,
DumpGuestMemoryFormat::kdump_raw_lzo,
DumpGuestMemoryFormat::kdump_raw_snappy,
DumpGuestMemoryFormat::win_dmp,

    ];
    const NAMES: &'static [&'static str] = &[

"elf",
"kdump-zlib",
"kdump-lzo",
"kdump-snappy",
"kdump-raw-zlib",
"kdump-raw-lzo",
"kdump-raw-snappy",
"win-dmp",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct dump_guest_memory {
	#[serde(rename = "begin", default, skip_serializing_if = "Option::is_none")]
pub begin: Option<i64>,
	#[serde(rename = "detach", default, skip_serializing_if = "Option::is_none")]
pub detach: Option<bool>,
	#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<DumpGuestMemoryFormat>,
	#[serde(rename = "length", default, skip_serializing_if = "Option::is_none")]
pub length: Option<i64>,
	#[serde(rename = "paging")]
pub paging: bool,
	#[serde(rename = "protocol")]
pub protocol: ::std::string::String,
}

impl crate::QmpCommand for dump_guest_memory { }
impl ::qapi_spec::Command for dump_guest_memory {
    const NAME: &'static str = "dump-guest-memory";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DumpStatus {
	#[serde(rename = "none")] none,
	#[serde(rename = "active")] active,
	#[serde(rename = "completed")] completed,
	#[serde(rename = "failed")] failed,
}

impl ::core::str::FromStr for DumpStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DumpStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

DumpStatus::none,
DumpStatus::active,
DumpStatus::completed,
DumpStatus::failed,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"active",
"completed",
"failed",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_dump {
}

impl crate::QmpCommand for query_dump { }
impl ::qapi_spec::Command for query_dump {
    const NAME: &'static str = "query-dump";
    const ALLOW_OOB: bool = false;

    type Ok = DumpQueryResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DUMP_COMPLETED {
#[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
pub error: Option<::std::string::String>,
#[serde(rename = "result")]
pub result: DumpQueryResult,
}

impl ::qapi_spec::Event for DUMP_COMPLETED {
    const NAME: &'static str = "DUMP_COMPLETED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_dump_guest_memory_capability {
}

impl crate::QmpCommand for query_dump_guest_memory_capability { }
impl ::qapi_spec::Command for query_dump_guest_memory_capability {
    const NAME: &'static str = "query-dump-guest-memory-capability";
    const ALLOW_OOB: bool = false;

    type Ok = DumpGuestMemoryCapability;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_link {
	#[serde(rename = "name")]
pub name: ::std::string::String,
	#[serde(rename = "up")]
pub up: bool,
}

impl crate::QmpCommand for set_link { }
impl ::qapi_spec::Command for set_link {
    const NAME: &'static str = "set_link";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct netdev_add(pub Netdev);

impl From<Netdev> for netdev_add {
    fn from(val: Netdev) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for netdev_add { }
impl ::qapi_spec::Command for netdev_add {
    const NAME: &'static str = "netdev_add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct netdev_del {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for netdev_del { }
impl ::qapi_spec::Command for netdev_del {
    const NAME: &'static str = "netdev_del";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AFXDPMode {
	#[serde(rename = "native")] native,
	#[serde(rename = "skb")] skb,
}

impl ::core::str::FromStr for AFXDPMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for AFXDPMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

AFXDPMode::native,
AFXDPMode::skb,

    ];
    const NAMES: &'static [&'static str] = &[

"native",
"skb",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetClientDriver {
	#[serde(rename = "none")] none,
	#[serde(rename = "nic")] nic,
	#[serde(rename = "user")] user,
	#[serde(rename = "tap")] tap,
	#[serde(rename = "l2tpv3")] l2tpv3,
	#[serde(rename = "socket")] socket,
	#[serde(rename = "stream")] stream,
	#[serde(rename = "dgram")] dgram,
	#[serde(rename = "vde")] vde,
	#[serde(rename = "bridge")] bridge,
	#[serde(rename = "hubport")] hubport,
	#[serde(rename = "netmap")] netmap,
	#[serde(rename = "vhost-user")] vhost_user,
	#[serde(rename = "vhost-vdpa")] vhost_vdpa,
	#[serde(rename = "af-xdp")] af_xdp,
	#[serde(rename = "vmnet-host")] vmnet_host,
	#[serde(rename = "vmnet-shared")] vmnet_shared,
	#[serde(rename = "vmnet-bridged")] vmnet_bridged,
}

impl ::core::str::FromStr for NetClientDriver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NetClientDriver {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 18;
    const VARIANTS: &'static [Self] = &[

NetClientDriver::none,
NetClientDriver::nic,
NetClientDriver::user,
NetClientDriver::tap,
NetClientDriver::l2tpv3,
NetClientDriver::socket,
NetClientDriver::stream,
NetClientDriver::dgram,
NetClientDriver::vde,
NetClientDriver::bridge,
NetClientDriver::hubport,
NetClientDriver::netmap,
NetClientDriver::vhost_user,
NetClientDriver::vhost_vdpa,
NetClientDriver::af_xdp,
NetClientDriver::vmnet_host,
NetClientDriver::vmnet_shared,
NetClientDriver::vmnet_bridged,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"nic",
"user",
"tap",
"l2tpv3",
"socket",
"stream",
"dgram",
"vde",
"bridge",
"hubport",
"netmap",
"vhost-user",
"vhost-vdpa",
"af-xdp",
"vmnet-host",
"vmnet-shared",
"vmnet-bridged",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RxState {
	#[serde(rename = "normal")] normal,
	#[serde(rename = "none")] none,
	#[serde(rename = "all")] all,
}

impl ::core::str::FromStr for RxState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RxState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

RxState::normal,
RxState::none,
RxState::all,

    ];
    const NAMES: &'static [&'static str] = &[

"normal",
"none",
"all",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_rx_filter {
	#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
}

impl crate::QmpCommand for query_rx_filter { }
impl ::qapi_spec::Command for query_rx_filter {
    const NAME: &'static str = "query-rx-filter";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<RxFilterInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NIC_RX_FILTER_CHANGED {
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl ::qapi_spec::Event for NIC_RX_FILTER_CHANGED {
    const NAME: &'static str = "NIC_RX_FILTER_CHANGED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct announce_self(pub AnnounceParameters);

impl From<AnnounceParameters> for announce_self {
    fn from(val: AnnounceParameters) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for announce_self { }
impl ::qapi_spec::Command for announce_self {
    const NAME: &'static str = "announce-self";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAILOVER_NEGOTIATED {
#[serde(rename = "device-id")]
pub device_id: ::std::string::String,
}

impl ::qapi_spec::Event for FAILOVER_NEGOTIATED {
    const NAME: &'static str = "FAILOVER_NEGOTIATED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NETDEV_STREAM_CONNECTED {
#[serde(rename = "addr")]
pub addr: SocketAddress,
#[serde(rename = "netdev-id")]
pub netdev_id: ::std::string::String,
}

impl ::qapi_spec::Event for NETDEV_STREAM_CONNECTED {
    const NAME: &'static str = "NETDEV_STREAM_CONNECTED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NETDEV_STREAM_DISCONNECTED {
#[serde(rename = "netdev-id")]
pub netdev_id: ::std::string::String,
}

impl ::qapi_spec::Event for NETDEV_STREAM_DISCONNECTED {
    const NAME: &'static str = "NETDEV_STREAM_DISCONNECTED";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EbpfProgramID {
	#[serde(rename = "rss")] rss,
}

impl ::core::str::FromStr for EbpfProgramID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for EbpfProgramID {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

EbpfProgramID::rss,

    ];
    const NAMES: &'static [&'static str] = &[

"rss",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct request_ebpf {
	#[serde(rename = "id")]
pub id: EbpfProgramID,
}

impl crate::QmpCommand for request_ebpf { }
impl ::qapi_spec::Command for request_ebpf {
    const NAME: &'static str = "request-ebpf";
    const ALLOW_OOB: bool = false;

    type Ok = EbpfObject;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_rocker {
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for query_rocker { }
impl ::qapi_spec::Command for query_rocker {
    const NAME: &'static str = "query-rocker";
    const ALLOW_OOB: bool = false;

    type Ok = RockerSwitch;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RockerPortDuplex {
	#[serde(rename = "half")] half,
	#[serde(rename = "full")] full,
}

impl ::core::str::FromStr for RockerPortDuplex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RockerPortDuplex {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

RockerPortDuplex::half,
RockerPortDuplex::full,

    ];
    const NAMES: &'static [&'static str] = &[

"half",
"full",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RockerPortAutoneg {
	#[serde(rename = "off")] off,
	#[serde(rename = "on")] on,
}

impl ::core::str::FromStr for RockerPortAutoneg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for RockerPortAutoneg {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

RockerPortAutoneg::off,
RockerPortAutoneg::on,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"on",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_rocker_ports {
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for query_rocker_ports { }
impl ::qapi_spec::Command for query_rocker_ports {
    const NAME: &'static str = "query-rocker-ports";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<RockerPort>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_rocker_of_dpa_flows {
	#[serde(rename = "tbl-id", default, skip_serializing_if = "Option::is_none")]
pub tbl_id: Option<u32>,
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for query_rocker_of_dpa_flows { }
impl ::qapi_spec::Command for query_rocker_of_dpa_flows {
    const NAME: &'static str = "query-rocker-of-dpa-flows";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<RockerOfDpaFlow>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_rocker_of_dpa_groups {
	#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
pub type_: Option<u8>,
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for query_rocker_of_dpa_groups { }
impl ::qapi_spec::Command for query_rocker_of_dpa_groups {
    const NAME: &'static str = "query-rocker-of-dpa-groups";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<RockerOfDpaGroup>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TpmModel {
	#[serde(rename = "tpm-tis")] tpm_tis,
	#[serde(rename = "tpm-crb")] tpm_crb,
	#[serde(rename = "tpm-spapr")] tpm_spapr,
}

impl ::core::str::FromStr for TpmModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for TpmModel {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

TpmModel::tpm_tis,
TpmModel::tpm_crb,
TpmModel::tpm_spapr,

    ];
    const NAMES: &'static [&'static str] = &[

"tpm-tis",
"tpm-crb",
"tpm-spapr",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_tpm_models {
}

impl crate::QmpCommand for query_tpm_models { }
impl ::qapi_spec::Command for query_tpm_models {
    const NAME: &'static str = "query-tpm-models";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<TpmModel>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TpmType {
	#[serde(rename = "passthrough")] passthrough,
	#[serde(rename = "emulator")] emulator,
}

impl ::core::str::FromStr for TpmType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for TpmType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

TpmType::passthrough,
TpmType::emulator,

    ];
    const NAMES: &'static [&'static str] = &[

"passthrough",
"emulator",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_tpm_types {
}

impl crate::QmpCommand for query_tpm_types { }
impl ::qapi_spec::Command for query_tpm_types {
    const NAME: &'static str = "query-tpm-types";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<TpmType>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_tpm {
}

impl crate::QmpCommand for query_tpm { }
impl ::qapi_spec::Command for query_tpm {
    const NAME: &'static str = "query-tpm";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<TPMInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayProtocol {
	#[serde(rename = "vnc")] vnc,
	#[serde(rename = "spice")] spice,
}

impl ::core::str::FromStr for DisplayProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DisplayProtocol {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

DisplayProtocol::vnc,
DisplayProtocol::spice,

    ];
    const NAMES: &'static [&'static str] = &[

"vnc",
"spice",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SetPasswordAction {
	#[serde(rename = "keep")] keep,
	#[serde(rename = "fail")] fail,
	#[serde(rename = "disconnect")] disconnect,
}

impl ::core::str::FromStr for SetPasswordAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SetPasswordAction {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

SetPasswordAction::keep,
SetPasswordAction::fail,
SetPasswordAction::disconnect,

    ];
    const NAMES: &'static [&'static str] = &[

"keep",
"fail",
"disconnect",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_password(pub SetPasswordOptions);

impl From<SetPasswordOptions> for set_password {
    fn from(val: SetPasswordOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for set_password { }
impl ::qapi_spec::Command for set_password {
    const NAME: &'static str = "set_password";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct expire_password(pub ExpirePasswordOptions);

impl From<ExpirePasswordOptions> for expire_password {
    fn from(val: ExpirePasswordOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for expire_password { }
impl ::qapi_spec::Command for expire_password {
    const NAME: &'static str = "expire_password";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
	#[serde(rename = "ppm")] ppm,
	#[serde(rename = "png")] png,
}

impl ::core::str::FromStr for ImageFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ImageFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

ImageFormat::ppm,
ImageFormat::png,

    ];
    const NAMES: &'static [&'static str] = &[

"ppm",
"png",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct screendump {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
	#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<ImageFormat>,
	#[serde(rename = "head", default, skip_serializing_if = "Option::is_none")]
pub head: Option<i64>,
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for screendump { }
impl ::qapi_spec::Command for screendump {
    const NAME: &'static str = "screendump";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpiceQueryMouseMode {
	#[serde(rename = "client")] client,
	#[serde(rename = "server")] server,
	#[serde(rename = "unknown")] unknown,
}

impl ::core::str::FromStr for SpiceQueryMouseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SpiceQueryMouseMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

SpiceQueryMouseMode::client,
SpiceQueryMouseMode::server,
SpiceQueryMouseMode::unknown,

    ];
    const NAMES: &'static [&'static str] = &[

"client",
"server",
"unknown",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_spice {
}

impl crate::QmpCommand for query_spice { }
impl ::qapi_spec::Command for query_spice {
    const NAME: &'static str = "query-spice";
    const ALLOW_OOB: bool = false;

    type Ok = SpiceInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPICE_CONNECTED {
#[serde(rename = "client")]
pub client: SpiceBasicInfo,
#[serde(rename = "server")]
pub server: SpiceBasicInfo,
}

impl ::qapi_spec::Event for SPICE_CONNECTED {
    const NAME: &'static str = "SPICE_CONNECTED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPICE_INITIALIZED {
#[serde(rename = "client")]
pub client: SpiceChannel,
#[serde(rename = "server")]
pub server: SpiceServerInfo,
}

impl ::qapi_spec::Event for SPICE_INITIALIZED {
    const NAME: &'static str = "SPICE_INITIALIZED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPICE_DISCONNECTED {
#[serde(rename = "client")]
pub client: SpiceBasicInfo,
#[serde(rename = "server")]
pub server: SpiceBasicInfo,
}

impl ::qapi_spec::Event for SPICE_DISCONNECTED {
    const NAME: &'static str = "SPICE_DISCONNECTED";
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SPICE_MIGRATE_COMPLETED {
}

impl ::qapi_spec::Event for SPICE_MIGRATE_COMPLETED {
    const NAME: &'static str = "SPICE_MIGRATE_COMPLETED";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VncPrimaryAuth {
	#[serde(rename = "none")] none,
	#[serde(rename = "vnc")] vnc,
	#[serde(rename = "ra2")] ra2,
	#[serde(rename = "ra2ne")] ra2ne,
	#[serde(rename = "tight")] tight,
	#[serde(rename = "ultra")] ultra,
	#[serde(rename = "tls")] tls,
	#[serde(rename = "vencrypt")] vencrypt,
	#[serde(rename = "sasl")] sasl,
}

impl ::core::str::FromStr for VncPrimaryAuth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for VncPrimaryAuth {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 9;
    const VARIANTS: &'static [Self] = &[

VncPrimaryAuth::none,
VncPrimaryAuth::vnc,
VncPrimaryAuth::ra2,
VncPrimaryAuth::ra2ne,
VncPrimaryAuth::tight,
VncPrimaryAuth::ultra,
VncPrimaryAuth::tls,
VncPrimaryAuth::vencrypt,
VncPrimaryAuth::sasl,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"vnc",
"ra2",
"ra2ne",
"tight",
"ultra",
"tls",
"vencrypt",
"sasl",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VncVencryptSubAuth {
	#[serde(rename = "plain")] plain,
	#[serde(rename = "tls-none")] tls_none,
	#[serde(rename = "x509-none")] x509_none,
	#[serde(rename = "tls-vnc")] tls_vnc,
	#[serde(rename = "x509-vnc")] x509_vnc,
	#[serde(rename = "tls-plain")] tls_plain,
	#[serde(rename = "x509-plain")] x509_plain,
	#[serde(rename = "tls-sasl")] tls_sasl,
	#[serde(rename = "x509-sasl")] x509_sasl,
}

impl ::core::str::FromStr for VncVencryptSubAuth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for VncVencryptSubAuth {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 9;
    const VARIANTS: &'static [Self] = &[

VncVencryptSubAuth::plain,
VncVencryptSubAuth::tls_none,
VncVencryptSubAuth::x509_none,
VncVencryptSubAuth::tls_vnc,
VncVencryptSubAuth::x509_vnc,
VncVencryptSubAuth::tls_plain,
VncVencryptSubAuth::x509_plain,
VncVencryptSubAuth::tls_sasl,
VncVencryptSubAuth::x509_sasl,

    ];
    const NAMES: &'static [&'static str] = &[

"plain",
"tls-none",
"x509-none",
"tls-vnc",
"x509-vnc",
"tls-plain",
"x509-plain",
"tls-sasl",
"x509-sasl",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_vnc {
}

impl crate::QmpCommand for query_vnc { }
impl ::qapi_spec::Command for query_vnc {
    const NAME: &'static str = "query-vnc";
    const ALLOW_OOB: bool = false;

    type Ok = VncInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_vnc_servers {
}

impl crate::QmpCommand for query_vnc_servers { }
impl ::qapi_spec::Command for query_vnc_servers {
    const NAME: &'static str = "query-vnc-servers";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<VncInfo2>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct change_vnc_password {
	#[serde(rename = "password")]
pub password: ::std::string::String,
}

impl crate::QmpCommand for change_vnc_password { }
impl ::qapi_spec::Command for change_vnc_password {
    const NAME: &'static str = "change-vnc-password";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNC_CONNECTED {
#[serde(rename = "client")]
pub client: VncBasicInfo,
#[serde(rename = "server")]
pub server: VncServerInfo,
}

impl ::qapi_spec::Event for VNC_CONNECTED {
    const NAME: &'static str = "VNC_CONNECTED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNC_INITIALIZED {
#[serde(rename = "client")]
pub client: VncClientInfo,
#[serde(rename = "server")]
pub server: VncServerInfo,
}

impl ::qapi_spec::Event for VNC_INITIALIZED {
    const NAME: &'static str = "VNC_INITIALIZED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNC_DISCONNECTED {
#[serde(rename = "client")]
pub client: VncClientInfo,
#[serde(rename = "server")]
pub server: VncServerInfo,
}

impl ::qapi_spec::Event for VNC_DISCONNECTED {
    const NAME: &'static str = "VNC_DISCONNECTED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_mice {
}

impl crate::QmpCommand for query_mice { }
impl ::qapi_spec::Command for query_mice {
    const NAME: &'static str = "query-mice";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<MouseInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QKeyCode {
	#[serde(rename = "unmapped")] unmapped,
	#[serde(rename = "shift")] shift,
	#[serde(rename = "shift_r")] shift_r,
	#[serde(rename = "alt")] alt,
	#[serde(rename = "alt_r")] alt_r,
	#[serde(rename = "ctrl")] ctrl,
	#[serde(rename = "ctrl_r")] ctrl_r,
	#[serde(rename = "menu")] menu,
	#[serde(rename = "esc")] esc,
	#[serde(rename = "1")] _1,
	#[serde(rename = "2")] _2,
	#[serde(rename = "3")] _3,
	#[serde(rename = "4")] _4,
	#[serde(rename = "5")] _5,
	#[serde(rename = "6")] _6,
	#[serde(rename = "7")] _7,
	#[serde(rename = "8")] _8,
	#[serde(rename = "9")] _9,
	#[serde(rename = "0")] _0,
	#[serde(rename = "minus")] minus,
	#[serde(rename = "equal")] equal,
	#[serde(rename = "backspace")] backspace,
	#[serde(rename = "tab")] tab,
	#[serde(rename = "q")] q,
	#[serde(rename = "w")] w,
	#[serde(rename = "e")] e,
	#[serde(rename = "r")] r,
	#[serde(rename = "t")] t,
	#[serde(rename = "y")] y,
	#[serde(rename = "u")] u,
	#[serde(rename = "i")] i,
	#[serde(rename = "o")] o,
	#[serde(rename = "p")] p,
	#[serde(rename = "bracket_left")] bracket_left,
	#[serde(rename = "bracket_right")] bracket_right,
	#[serde(rename = "ret")] ret,
	#[serde(rename = "a")] a,
	#[serde(rename = "s")] s,
	#[serde(rename = "d")] d,
	#[serde(rename = "f")] f,
	#[serde(rename = "g")] g,
	#[serde(rename = "h")] h,
	#[serde(rename = "j")] j,
	#[serde(rename = "k")] k,
	#[serde(rename = "l")] l,
	#[serde(rename = "semicolon")] semicolon,
	#[serde(rename = "apostrophe")] apostrophe,
	#[serde(rename = "grave_accent")] grave_accent,
	#[serde(rename = "backslash")] backslash,
	#[serde(rename = "z")] z,
	#[serde(rename = "x")] x,
	#[serde(rename = "c")] c,
	#[serde(rename = "v")] v,
	#[serde(rename = "b")] b,
	#[serde(rename = "n")] n,
	#[serde(rename = "m")] m,
	#[serde(rename = "comma")] comma,
	#[serde(rename = "dot")] dot,
	#[serde(rename = "slash")] slash,
	#[serde(rename = "asterisk")] asterisk,
	#[serde(rename = "spc")] spc,
	#[serde(rename = "caps_lock")] caps_lock,
	#[serde(rename = "f1")] f1,
	#[serde(rename = "f2")] f2,
	#[serde(rename = "f3")] f3,
	#[serde(rename = "f4")] f4,
	#[serde(rename = "f5")] f5,
	#[serde(rename = "f6")] f6,
	#[serde(rename = "f7")] f7,
	#[serde(rename = "f8")] f8,
	#[serde(rename = "f9")] f9,
	#[serde(rename = "f10")] f10,
	#[serde(rename = "num_lock")] num_lock,
	#[serde(rename = "scroll_lock")] scroll_lock,
	#[serde(rename = "kp_divide")] kp_divide,
	#[serde(rename = "kp_multiply")] kp_multiply,
	#[serde(rename = "kp_subtract")] kp_subtract,
	#[serde(rename = "kp_add")] kp_add,
	#[serde(rename = "kp_enter")] kp_enter,
	#[serde(rename = "kp_decimal")] kp_decimal,
	#[serde(rename = "sysrq")] sysrq,
	#[serde(rename = "kp_0")] kp_0,
	#[serde(rename = "kp_1")] kp_1,
	#[serde(rename = "kp_2")] kp_2,
	#[serde(rename = "kp_3")] kp_3,
	#[serde(rename = "kp_4")] kp_4,
	#[serde(rename = "kp_5")] kp_5,
	#[serde(rename = "kp_6")] kp_6,
	#[serde(rename = "kp_7")] kp_7,
	#[serde(rename = "kp_8")] kp_8,
	#[serde(rename = "kp_9")] kp_9,
	#[serde(rename = "less")] less,
	#[serde(rename = "f11")] f11,
	#[serde(rename = "f12")] f12,
	#[serde(rename = "print")] print,
	#[serde(rename = "home")] home,
	#[serde(rename = "pgup")] pgup,
	#[serde(rename = "pgdn")] pgdn,
	#[serde(rename = "end")] end,
	#[serde(rename = "left")] left,
	#[serde(rename = "up")] up,
	#[serde(rename = "down")] down,
	#[serde(rename = "right")] right,
	#[serde(rename = "insert")] insert,
	#[serde(rename = "delete")] delete,
	#[serde(rename = "stop")] stop,
	#[serde(rename = "again")] again,
	#[serde(rename = "props")] props,
	#[serde(rename = "undo")] undo,
	#[serde(rename = "front")] front,
	#[serde(rename = "copy")] copy,
	#[serde(rename = "open")] open,
	#[serde(rename = "paste")] paste,
	#[serde(rename = "find")] find,
	#[serde(rename = "cut")] cut,
	#[serde(rename = "lf")] lf,
	#[serde(rename = "help")] help,
	#[serde(rename = "meta_l")] meta_l,
	#[serde(rename = "meta_r")] meta_r,
	#[serde(rename = "compose")] compose,
	#[serde(rename = "pause")] pause,
	#[serde(rename = "ro")] ro,
	#[serde(rename = "hiragana")] hiragana,
	#[serde(rename = "henkan")] henkan,
	#[serde(rename = "yen")] yen,
	#[serde(rename = "muhenkan")] muhenkan,
	#[serde(rename = "katakanahiragana")] katakanahiragana,
	#[serde(rename = "kp_comma")] kp_comma,
	#[serde(rename = "kp_equals")] kp_equals,
	#[serde(rename = "power")] power,
	#[serde(rename = "sleep")] sleep,
	#[serde(rename = "wake")] wake,
	#[serde(rename = "audionext")] audionext,
	#[serde(rename = "audioprev")] audioprev,
	#[serde(rename = "audiostop")] audiostop,
	#[serde(rename = "audioplay")] audioplay,
	#[serde(rename = "audiomute")] audiomute,
	#[serde(rename = "volumeup")] volumeup,
	#[serde(rename = "volumedown")] volumedown,
	#[serde(rename = "mediaselect")] mediaselect,
	#[serde(rename = "mail")] mail,
	#[serde(rename = "calculator")] calculator,
	#[serde(rename = "computer")] computer,
	#[serde(rename = "ac_home")] ac_home,
	#[serde(rename = "ac_back")] ac_back,
	#[serde(rename = "ac_forward")] ac_forward,
	#[serde(rename = "ac_refresh")] ac_refresh,
	#[serde(rename = "ac_bookmarks")] ac_bookmarks,
	#[serde(rename = "lang1")] lang1,
	#[serde(rename = "lang2")] lang2,
	#[serde(rename = "f13")] f13,
	#[serde(rename = "f14")] f14,
	#[serde(rename = "f15")] f15,
	#[serde(rename = "f16")] f16,
	#[serde(rename = "f17")] f17,
	#[serde(rename = "f18")] f18,
	#[serde(rename = "f19")] f19,
	#[serde(rename = "f20")] f20,
	#[serde(rename = "f21")] f21,
	#[serde(rename = "f22")] f22,
	#[serde(rename = "f23")] f23,
	#[serde(rename = "f24")] f24,
}

impl ::core::str::FromStr for QKeyCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QKeyCode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 162;
    const VARIANTS: &'static [Self] = &[

QKeyCode::unmapped,
QKeyCode::shift,
QKeyCode::shift_r,
QKeyCode::alt,
QKeyCode::alt_r,
QKeyCode::ctrl,
QKeyCode::ctrl_r,
QKeyCode::menu,
QKeyCode::esc,
QKeyCode::_1,
QKeyCode::_2,
QKeyCode::_3,
QKeyCode::_4,
QKeyCode::_5,
QKeyCode::_6,
QKeyCode::_7,
QKeyCode::_8,
QKeyCode::_9,
QKeyCode::_0,
QKeyCode::minus,
QKeyCode::equal,
QKeyCode::backspace,
QKeyCode::tab,
QKeyCode::q,
QKeyCode::w,
QKeyCode::e,
QKeyCode::r,
QKeyCode::t,
QKeyCode::y,
QKeyCode::u,
QKeyCode::i,
QKeyCode::o,
QKeyCode::p,
QKeyCode::bracket_left,
QKeyCode::bracket_right,
QKeyCode::ret,
QKeyCode::a,
QKeyCode::s,
QKeyCode::d,
QKeyCode::f,
QKeyCode::g,
QKeyCode::h,
QKeyCode::j,
QKeyCode::k,
QKeyCode::l,
QKeyCode::semicolon,
QKeyCode::apostrophe,
QKeyCode::grave_accent,
QKeyCode::backslash,
QKeyCode::z,
QKeyCode::x,
QKeyCode::c,
QKeyCode::v,
QKeyCode::b,
QKeyCode::n,
QKeyCode::m,
QKeyCode::comma,
QKeyCode::dot,
QKeyCode::slash,
QKeyCode::asterisk,
QKeyCode::spc,
QKeyCode::caps_lock,
QKeyCode::f1,
QKeyCode::f2,
QKeyCode::f3,
QKeyCode::f4,
QKeyCode::f5,
QKeyCode::f6,
QKeyCode::f7,
QKeyCode::f8,
QKeyCode::f9,
QKeyCode::f10,
QKeyCode::num_lock,
QKeyCode::scroll_lock,
QKeyCode::kp_divide,
QKeyCode::kp_multiply,
QKeyCode::kp_subtract,
QKeyCode::kp_add,
QKeyCode::kp_enter,
QKeyCode::kp_decimal,
QKeyCode::sysrq,
QKeyCode::kp_0,
QKeyCode::kp_1,
QKeyCode::kp_2,
QKeyCode::kp_3,
QKeyCode::kp_4,
QKeyCode::kp_5,
QKeyCode::kp_6,
QKeyCode::kp_7,
QKeyCode::kp_8,
QKeyCode::kp_9,
QKeyCode::less,
QKeyCode::f11,
QKeyCode::f12,
QKeyCode::print,
QKeyCode::home,
QKeyCode::pgup,
QKeyCode::pgdn,
QKeyCode::end,
QKeyCode::left,
QKeyCode::up,
QKeyCode::down,
QKeyCode::right,
QKeyCode::insert,
QKeyCode::delete,
QKeyCode::stop,
QKeyCode::again,
QKeyCode::props,
QKeyCode::undo,
QKeyCode::front,
QKeyCode::copy,
QKeyCode::open,
QKeyCode::paste,
QKeyCode::find,
QKeyCode::cut,
QKeyCode::lf,
QKeyCode::help,
QKeyCode::meta_l,
QKeyCode::meta_r,
QKeyCode::compose,
QKeyCode::pause,
QKeyCode::ro,
QKeyCode::hiragana,
QKeyCode::henkan,
QKeyCode::yen,
QKeyCode::muhenkan,
QKeyCode::katakanahiragana,
QKeyCode::kp_comma,
QKeyCode::kp_equals,
QKeyCode::power,
QKeyCode::sleep,
QKeyCode::wake,
QKeyCode::audionext,
QKeyCode::audioprev,
QKeyCode::audiostop,
QKeyCode::audioplay,
QKeyCode::audiomute,
QKeyCode::volumeup,
QKeyCode::volumedown,
QKeyCode::mediaselect,
QKeyCode::mail,
QKeyCode::calculator,
QKeyCode::computer,
QKeyCode::ac_home,
QKeyCode::ac_back,
QKeyCode::ac_forward,
QKeyCode::ac_refresh,
QKeyCode::ac_bookmarks,
QKeyCode::lang1,
QKeyCode::lang2,
QKeyCode::f13,
QKeyCode::f14,
QKeyCode::f15,
QKeyCode::f16,
QKeyCode::f17,
QKeyCode::f18,
QKeyCode::f19,
QKeyCode::f20,
QKeyCode::f21,
QKeyCode::f22,
QKeyCode::f23,
QKeyCode::f24,

    ];
    const NAMES: &'static [&'static str] = &[

"unmapped",
"shift",
"shift_r",
"alt",
"alt_r",
"ctrl",
"ctrl_r",
"menu",
"esc",
"1",
"2",
"3",
"4",
"5",
"6",
"7",
"8",
"9",
"0",
"minus",
"equal",
"backspace",
"tab",
"q",
"w",
"e",
"r",
"t",
"y",
"u",
"i",
"o",
"p",
"bracket_left",
"bracket_right",
"ret",
"a",
"s",
"d",
"f",
"g",
"h",
"j",
"k",
"l",
"semicolon",
"apostrophe",
"grave_accent",
"backslash",
"z",
"x",
"c",
"v",
"b",
"n",
"m",
"comma",
"dot",
"slash",
"asterisk",
"spc",
"caps_lock",
"f1",
"f2",
"f3",
"f4",
"f5",
"f6",
"f7",
"f8",
"f9",
"f10",
"num_lock",
"scroll_lock",
"kp_divide",
"kp_multiply",
"kp_subtract",
"kp_add",
"kp_enter",
"kp_decimal",
"sysrq",
"kp_0",
"kp_1",
"kp_2",
"kp_3",
"kp_4",
"kp_5",
"kp_6",
"kp_7",
"kp_8",
"kp_9",
"less",
"f11",
"f12",
"print",
"home",
"pgup",
"pgdn",
"end",
"left",
"up",
"down",
"right",
"insert",
"delete",
"stop",
"again",
"props",
"undo",
"front",
"copy",
"open",
"paste",
"find",
"cut",
"lf",
"help",
"meta_l",
"meta_r",
"compose",
"pause",
"ro",
"hiragana",
"henkan",
"yen",
"muhenkan",
"katakanahiragana",
"kp_comma",
"kp_equals",
"power",
"sleep",
"wake",
"audionext",
"audioprev",
"audiostop",
"audioplay",
"audiomute",
"volumeup",
"volumedown",
"mediaselect",
"mail",
"calculator",
"computer",
"ac_home",
"ac_back",
"ac_forward",
"ac_refresh",
"ac_bookmarks",
"lang1",
"lang2",
"f13",
"f14",
"f15",
"f16",
"f17",
"f18",
"f19",
"f20",
"f21",
"f22",
"f23",
"f24",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyValueKind {
	#[serde(rename = "number")] number,
	#[serde(rename = "qcode")] qcode,
}

impl ::core::str::FromStr for KeyValueKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for KeyValueKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

KeyValueKind::number,
KeyValueKind::qcode,

    ];
    const NAMES: &'static [&'static str] = &[

"number",
"qcode",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct send_key {
	#[serde(rename = "hold-time", default, skip_serializing_if = "Option::is_none")]
pub hold_time: Option<i64>,
	#[serde(rename = "keys")]
pub keys: Vec<KeyValue>,
}

impl crate::QmpCommand for send_key { }
impl ::qapi_spec::Command for send_key {
    const NAME: &'static str = "send-key";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputButton {
	#[serde(rename = "left")] left,
	#[serde(rename = "middle")] middle,
	#[serde(rename = "right")] right,
	#[serde(rename = "wheel-up")] wheel_up,
	#[serde(rename = "wheel-down")] wheel_down,
	#[serde(rename = "side")] side,
	#[serde(rename = "extra")] extra,
	#[serde(rename = "wheel-left")] wheel_left,
	#[serde(rename = "wheel-right")] wheel_right,
	#[serde(rename = "touch")] touch,
}

impl ::core::str::FromStr for InputButton {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for InputButton {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 10;
    const VARIANTS: &'static [Self] = &[

InputButton::left,
InputButton::middle,
InputButton::right,
InputButton::wheel_up,
InputButton::wheel_down,
InputButton::side,
InputButton::extra,
InputButton::wheel_left,
InputButton::wheel_right,
InputButton::touch,

    ];
    const NAMES: &'static [&'static str] = &[

"left",
"middle",
"right",
"wheel-up",
"wheel-down",
"side",
"extra",
"wheel-left",
"wheel-right",
"touch",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputAxis {
	#[serde(rename = "x")] x,
	#[serde(rename = "y")] y,
}

impl ::core::str::FromStr for InputAxis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for InputAxis {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

InputAxis::x,
InputAxis::y,

    ];
    const NAMES: &'static [&'static str] = &[

"x",
"y",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputMultiTouchType {
	#[serde(rename = "begin")] begin,
	#[serde(rename = "update")] update,
	#[serde(rename = "end")] end,
	#[serde(rename = "cancel")] cancel,
	#[serde(rename = "data")] data,
}

impl ::core::str::FromStr for InputMultiTouchType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for InputMultiTouchType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

InputMultiTouchType::begin,
InputMultiTouchType::update,
InputMultiTouchType::end,
InputMultiTouchType::cancel,
InputMultiTouchType::data,

    ];
    const NAMES: &'static [&'static str] = &[

"begin",
"update",
"end",
"cancel",
"data",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputEventKind {
	#[serde(rename = "key")] key,
	#[serde(rename = "btn")] btn,
	#[serde(rename = "rel")] rel,
	#[serde(rename = "abs")] abs,
	#[serde(rename = "mtt")] mtt,
}

impl ::core::str::FromStr for InputEventKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for InputEventKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

InputEventKind::key,
InputEventKind::btn,
InputEventKind::rel,
InputEventKind::abs,
InputEventKind::mtt,

    ];
    const NAMES: &'static [&'static str] = &[

"key",
"btn",
"rel",
"abs",
"mtt",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct input_send_event {
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
	#[serde(rename = "head", default, skip_serializing_if = "Option::is_none")]
pub head: Option<i64>,
	#[serde(rename = "events")]
pub events: Vec<InputEvent>,
}

impl crate::QmpCommand for input_send_event { }
impl ::qapi_spec::Command for input_send_event {
    const NAME: &'static str = "input-send-event";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayGLMode {
	#[serde(rename = "off")] off,
	#[serde(rename = "on")] on,
	#[serde(rename = "core")] core,
	#[serde(rename = "es")] es,
}

impl ::core::str::FromStr for DisplayGLMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DisplayGLMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

DisplayGLMode::off,
DisplayGLMode::on,
DisplayGLMode::core,
DisplayGLMode::es,

    ];
    const NAMES: &'static [&'static str] = &[

"off",
"on",
"core",
"es",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HotKeyMod {
	#[serde(rename = "lctrl-lalt")] lctrl_lalt,
	#[serde(rename = "lshift-lctrl-lalt")] lshift_lctrl_lalt,
	#[serde(rename = "rctrl")] rctrl,
}

impl ::core::str::FromStr for HotKeyMod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HotKeyMod {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

HotKeyMod::lctrl_lalt,
HotKeyMod::lshift_lctrl_lalt,
HotKeyMod::rctrl,

    ];
    const NAMES: &'static [&'static str] = &[

"lctrl-lalt",
"lshift-lctrl-lalt",
"rctrl",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayType {
	#[serde(rename = "default")] default,
	#[serde(rename = "none")] none,
	#[serde(rename = "gtk")] gtk,
	#[serde(rename = "sdl")] sdl,
	#[serde(rename = "egl-headless")] egl_headless,
	#[serde(rename = "curses")] curses,
	#[serde(rename = "cocoa")] cocoa,
	#[serde(rename = "spice-app")] spice_app,
	#[serde(rename = "dbus")] dbus,
}

impl ::core::str::FromStr for DisplayType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DisplayType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 9;
    const VARIANTS: &'static [Self] = &[

DisplayType::default,
DisplayType::none,
DisplayType::gtk,
DisplayType::sdl,
DisplayType::egl_headless,
DisplayType::curses,
DisplayType::cocoa,
DisplayType::spice_app,
DisplayType::dbus,

    ];
    const NAMES: &'static [&'static str] = &[

"default",
"none",
"gtk",
"sdl",
"egl-headless",
"curses",
"cocoa",
"spice-app",
"dbus",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_display_options {
}

impl crate::QmpCommand for query_display_options { }
impl ::qapi_spec::Command for query_display_options {
    const NAME: &'static str = "query-display-options";
    const ALLOW_OOB: bool = false;

    type Ok = DisplayOptions;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayReloadType {
	#[serde(rename = "vnc")] vnc,
}

impl ::core::str::FromStr for DisplayReloadType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DisplayReloadType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

DisplayReloadType::vnc,

    ];
    const NAMES: &'static [&'static str] = &[

"vnc",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct display_reload(pub DisplayReloadOptions);

impl From<DisplayReloadOptions> for display_reload {
    fn from(val: DisplayReloadOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for display_reload { }
impl ::qapi_spec::Command for display_reload {
    const NAME: &'static str = "display-reload";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayUpdateType {
	#[serde(rename = "vnc")] vnc,
}

impl ::core::str::FromStr for DisplayUpdateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DisplayUpdateType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

DisplayUpdateType::vnc,

    ];
    const NAMES: &'static [&'static str] = &[

"vnc",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct display_update(pub DisplayUpdateOptions);

impl From<DisplayUpdateOptions> for display_update {
    fn from(val: DisplayUpdateOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for display_update { }
impl ::qapi_spec::Command for display_update {
    const NAME: &'static str = "display-update";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct client_migrate_info {
	#[serde(rename = "cert-subject", default, skip_serializing_if = "Option::is_none")]
pub cert_subject: Option<::std::string::String>,
	#[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
pub port: Option<i64>,
	#[serde(rename = "tls-port", default, skip_serializing_if = "Option::is_none")]
pub tls_port: Option<i64>,
	#[serde(rename = "hostname")]
pub hostname: ::std::string::String,
	#[serde(rename = "protocol")]
pub protocol: ::std::string::String,
}

impl crate::QmpCommand for client_migrate_info { }
impl ::qapi_spec::Command for client_migrate_info {
    const NAME: &'static str = "client_migrate_info";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QAuthZListPolicy {
	#[serde(rename = "deny")] deny,
	#[serde(rename = "allow")] allow,
}

impl ::core::str::FromStr for QAuthZListPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QAuthZListPolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QAuthZListPolicy::deny,
QAuthZListPolicy::allow,

    ];
    const NAMES: &'static [&'static str] = &[

"deny",
"allow",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QAuthZListFormat {
	#[serde(rename = "exact")] exact,
	#[serde(rename = "glob")] glob,
}

impl ::core::str::FromStr for QAuthZListFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QAuthZListFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QAuthZListFormat::exact,
QAuthZListFormat::glob,

    ];
    const NAMES: &'static [&'static str] = &[

"exact",
"glob",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationStatus {
	#[serde(rename = "none")] none,
	#[serde(rename = "setup")] setup,
	#[serde(rename = "cancelling")] cancelling,
	#[serde(rename = "cancelled")] cancelled,
	#[serde(rename = "active")] active,
	#[serde(rename = "postcopy-active")] postcopy_active,
	#[serde(rename = "postcopy-paused")] postcopy_paused,
	#[serde(rename = "postcopy-recover-setup")] postcopy_recover_setup,
	#[serde(rename = "postcopy-recover")] postcopy_recover,
	#[serde(rename = "completed")] completed,
	#[serde(rename = "failed")] failed,
	#[serde(rename = "colo")] colo,
	#[serde(rename = "pre-switchover")] pre_switchover,
	#[serde(rename = "device")] device,
	#[serde(rename = "wait-unplug")] wait_unplug,
}

impl ::core::str::FromStr for MigrationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigrationStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 15;
    const VARIANTS: &'static [Self] = &[

MigrationStatus::none,
MigrationStatus::setup,
MigrationStatus::cancelling,
MigrationStatus::cancelled,
MigrationStatus::active,
MigrationStatus::postcopy_active,
MigrationStatus::postcopy_paused,
MigrationStatus::postcopy_recover_setup,
MigrationStatus::postcopy_recover,
MigrationStatus::completed,
MigrationStatus::failed,
MigrationStatus::colo,
MigrationStatus::pre_switchover,
MigrationStatus::device,
MigrationStatus::wait_unplug,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"setup",
"cancelling",
"cancelled",
"active",
"postcopy-active",
"postcopy-paused",
"postcopy-recover-setup",
"postcopy-recover",
"completed",
"failed",
"colo",
"pre-switchover",
"device",
"wait-unplug",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_migrate {
}

impl crate::QmpCommand for query_migrate { }
impl ::qapi_spec::Command for query_migrate {
    const NAME: &'static str = "query-migrate";
    const ALLOW_OOB: bool = false;

    type Ok = MigrationInfo;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationCapability {
	#[serde(rename = "xbzrle")] xbzrle,
	#[serde(rename = "rdma-pin-all")] rdma_pin_all,
	#[serde(rename = "auto-converge")] auto_converge,
	#[serde(rename = "zero-blocks")] zero_blocks,
	#[serde(rename = "events")] events,
	#[serde(rename = "postcopy-ram")] postcopy_ram,
	#[serde(rename = "x-colo")] x_colo,
	#[serde(rename = "release-ram")] release_ram,
	#[serde(rename = "return-path")] return_path,
	#[serde(rename = "pause-before-switchover")] pause_before_switchover,
	#[serde(rename = "multifd")] multifd,
	#[serde(rename = "dirty-bitmaps")] dirty_bitmaps,
	#[serde(rename = "postcopy-blocktime")] postcopy_blocktime,
	#[serde(rename = "late-block-activate")] late_block_activate,
	#[serde(rename = "x-ignore-shared")] x_ignore_shared,
	#[serde(rename = "validate-uuid")] validate_uuid,
	#[serde(rename = "background-snapshot")] background_snapshot,
	#[serde(rename = "zero-copy-send")] zero_copy_send,
	#[serde(rename = "postcopy-preempt")] postcopy_preempt,
	#[serde(rename = "switchover-ack")] switchover_ack,
	#[serde(rename = "dirty-limit")] dirty_limit,
	#[serde(rename = "mapped-ram")] mapped_ram,
}

impl ::core::str::FromStr for MigrationCapability {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigrationCapability {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 22;
    const VARIANTS: &'static [Self] = &[

MigrationCapability::xbzrle,
MigrationCapability::rdma_pin_all,
MigrationCapability::auto_converge,
MigrationCapability::zero_blocks,
MigrationCapability::events,
MigrationCapability::postcopy_ram,
MigrationCapability::x_colo,
MigrationCapability::release_ram,
MigrationCapability::return_path,
MigrationCapability::pause_before_switchover,
MigrationCapability::multifd,
MigrationCapability::dirty_bitmaps,
MigrationCapability::postcopy_blocktime,
MigrationCapability::late_block_activate,
MigrationCapability::x_ignore_shared,
MigrationCapability::validate_uuid,
MigrationCapability::background_snapshot,
MigrationCapability::zero_copy_send,
MigrationCapability::postcopy_preempt,
MigrationCapability::switchover_ack,
MigrationCapability::dirty_limit,
MigrationCapability::mapped_ram,

    ];
    const NAMES: &'static [&'static str] = &[

"xbzrle",
"rdma-pin-all",
"auto-converge",
"zero-blocks",
"events",
"postcopy-ram",
"x-colo",
"release-ram",
"return-path",
"pause-before-switchover",
"multifd",
"dirty-bitmaps",
"postcopy-blocktime",
"late-block-activate",
"x-ignore-shared",
"validate-uuid",
"background-snapshot",
"zero-copy-send",
"postcopy-preempt",
"switchover-ack",
"dirty-limit",
"mapped-ram",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_set_capabilities {
	#[serde(rename = "capabilities")]
pub capabilities: Vec<MigrationCapabilityStatus>,
}

impl crate::QmpCommand for migrate_set_capabilities { }
impl ::qapi_spec::Command for migrate_set_capabilities {
    const NAME: &'static str = "migrate-set-capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_migrate_capabilities {
}

impl crate::QmpCommand for query_migrate_capabilities { }
impl ::qapi_spec::Command for query_migrate_capabilities {
    const NAME: &'static str = "query-migrate-capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<MigrationCapabilityStatus>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MultiFDCompression {
	#[serde(rename = "none")] none,
	#[serde(rename = "zlib")] zlib,
	#[serde(rename = "zstd")] zstd,
	#[serde(rename = "qpl")] qpl,
	#[serde(rename = "uadk")] uadk,
}

impl ::core::str::FromStr for MultiFDCompression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MultiFDCompression {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

MultiFDCompression::none,
MultiFDCompression::zlib,
MultiFDCompression::zstd,
MultiFDCompression::qpl,
MultiFDCompression::uadk,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"zlib",
"zstd",
"qpl",
"uadk",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigMode {
	#[serde(rename = "normal")] normal,
	#[serde(rename = "cpr-reboot")] cpr_reboot,
}

impl ::core::str::FromStr for MigMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

MigMode::normal,
MigMode::cpr_reboot,

    ];
    const NAMES: &'static [&'static str] = &[

"normal",
"cpr-reboot",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZeroPageDetection {
	#[serde(rename = "none")] none,
	#[serde(rename = "legacy")] legacy,
	#[serde(rename = "multifd")] multifd,
}

impl ::core::str::FromStr for ZeroPageDetection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ZeroPageDetection {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

ZeroPageDetection::none,
ZeroPageDetection::legacy,
ZeroPageDetection::multifd,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"legacy",
"multifd",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationParameter {
	#[serde(rename = "announce-initial")] announce_initial,
	#[serde(rename = "announce-max")] announce_max,
	#[serde(rename = "announce-rounds")] announce_rounds,
	#[serde(rename = "announce-step")] announce_step,
	#[serde(rename = "throttle-trigger-threshold")] throttle_trigger_threshold,
	#[serde(rename = "cpu-throttle-initial")] cpu_throttle_initial,
	#[serde(rename = "cpu-throttle-increment")] cpu_throttle_increment,
	#[serde(rename = "cpu-throttle-tailslow")] cpu_throttle_tailslow,
	#[serde(rename = "tls-creds")] tls_creds,
	#[serde(rename = "tls-hostname")] tls_hostname,
	#[serde(rename = "tls-authz")] tls_authz,
	#[serde(rename = "max-bandwidth")] max_bandwidth,
	#[serde(rename = "avail-switchover-bandwidth")] avail_switchover_bandwidth,
	#[serde(rename = "downtime-limit")] downtime_limit,
	#[serde(rename = "x-checkpoint-delay")] x_checkpoint_delay,
	#[serde(rename = "multifd-channels")] multifd_channels,
	#[serde(rename = "xbzrle-cache-size")] xbzrle_cache_size,
	#[serde(rename = "max-postcopy-bandwidth")] max_postcopy_bandwidth,
	#[serde(rename = "max-cpu-throttle")] max_cpu_throttle,
	#[serde(rename = "multifd-compression")] multifd_compression,
	#[serde(rename = "multifd-zlib-level")] multifd_zlib_level,
	#[serde(rename = "multifd-zstd-level")] multifd_zstd_level,
	#[serde(rename = "block-bitmap-mapping")] block_bitmap_mapping,
	#[serde(rename = "x-vcpu-dirty-limit-period")] x_vcpu_dirty_limit_period,
	#[serde(rename = "vcpu-dirty-limit")] vcpu_dirty_limit,
	#[serde(rename = "mode")] mode,
	#[serde(rename = "zero-page-detection")] zero_page_detection,
	#[serde(rename = "direct-io")] direct_io,
}

impl ::core::str::FromStr for MigrationParameter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigrationParameter {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 28;
    const VARIANTS: &'static [Self] = &[

MigrationParameter::announce_initial,
MigrationParameter::announce_max,
MigrationParameter::announce_rounds,
MigrationParameter::announce_step,
MigrationParameter::throttle_trigger_threshold,
MigrationParameter::cpu_throttle_initial,
MigrationParameter::cpu_throttle_increment,
MigrationParameter::cpu_throttle_tailslow,
MigrationParameter::tls_creds,
MigrationParameter::tls_hostname,
MigrationParameter::tls_authz,
MigrationParameter::max_bandwidth,
MigrationParameter::avail_switchover_bandwidth,
MigrationParameter::downtime_limit,
MigrationParameter::x_checkpoint_delay,
MigrationParameter::multifd_channels,
MigrationParameter::xbzrle_cache_size,
MigrationParameter::max_postcopy_bandwidth,
MigrationParameter::max_cpu_throttle,
MigrationParameter::multifd_compression,
MigrationParameter::multifd_zlib_level,
MigrationParameter::multifd_zstd_level,
MigrationParameter::block_bitmap_mapping,
MigrationParameter::x_vcpu_dirty_limit_period,
MigrationParameter::vcpu_dirty_limit,
MigrationParameter::mode,
MigrationParameter::zero_page_detection,
MigrationParameter::direct_io,

    ];
    const NAMES: &'static [&'static str] = &[

"announce-initial",
"announce-max",
"announce-rounds",
"announce-step",
"throttle-trigger-threshold",
"cpu-throttle-initial",
"cpu-throttle-increment",
"cpu-throttle-tailslow",
"tls-creds",
"tls-hostname",
"tls-authz",
"max-bandwidth",
"avail-switchover-bandwidth",
"downtime-limit",
"x-checkpoint-delay",
"multifd-channels",
"xbzrle-cache-size",
"max-postcopy-bandwidth",
"max-cpu-throttle",
"multifd-compression",
"multifd-zlib-level",
"multifd-zstd-level",
"block-bitmap-mapping",
"x-vcpu-dirty-limit-period",
"vcpu-dirty-limit",
"mode",
"zero-page-detection",
"direct-io",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_set_parameters(pub MigrateSetParameters);

impl From<MigrateSetParameters> for migrate_set_parameters {
    fn from(val: MigrateSetParameters) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for migrate_set_parameters { }
impl ::qapi_spec::Command for migrate_set_parameters {
    const NAME: &'static str = "migrate-set-parameters";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_migrate_parameters {
}

impl crate::QmpCommand for query_migrate_parameters { }
impl ::qapi_spec::Command for query_migrate_parameters {
    const NAME: &'static str = "query-migrate-parameters";
    const ALLOW_OOB: bool = false;

    type Ok = MigrationParameters;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_start_postcopy {
}

impl crate::QmpCommand for migrate_start_postcopy { }
impl ::qapi_spec::Command for migrate_start_postcopy {
    const NAME: &'static str = "migrate-start-postcopy";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIGRATION {
#[serde(rename = "status")]
pub status: MigrationStatus,
}

impl ::qapi_spec::Event for MIGRATION {
    const NAME: &'static str = "MIGRATION";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIGRATION_PASS {
#[serde(rename = "pass")]
pub pass: i64,
}

impl ::qapi_spec::Event for MIGRATION_PASS {
    const NAME: &'static str = "MIGRATION_PASS";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum COLOMessage {
	#[serde(rename = "checkpoint-ready")] checkpoint_ready,
	#[serde(rename = "checkpoint-request")] checkpoint_request,
	#[serde(rename = "checkpoint-reply")] checkpoint_reply,
	#[serde(rename = "vmstate-send")] vmstate_send,
	#[serde(rename = "vmstate-size")] vmstate_size,
	#[serde(rename = "vmstate-received")] vmstate_received,
	#[serde(rename = "vmstate-loaded")] vmstate_loaded,
}

impl ::core::str::FromStr for COLOMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for COLOMessage {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

COLOMessage::checkpoint_ready,
COLOMessage::checkpoint_request,
COLOMessage::checkpoint_reply,
COLOMessage::vmstate_send,
COLOMessage::vmstate_size,
COLOMessage::vmstate_received,
COLOMessage::vmstate_loaded,

    ];
    const NAMES: &'static [&'static str] = &[

"checkpoint-ready",
"checkpoint-request",
"checkpoint-reply",
"vmstate-send",
"vmstate-size",
"vmstate-received",
"vmstate-loaded",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum COLOMode {
	#[serde(rename = "none")] none,
	#[serde(rename = "primary")] primary,
	#[serde(rename = "secondary")] secondary,
}

impl ::core::str::FromStr for COLOMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for COLOMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

COLOMode::none,
COLOMode::primary,
COLOMode::secondary,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"primary",
"secondary",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FailoverStatus {
	#[serde(rename = "none")] none,
	#[serde(rename = "require")] require,
	#[serde(rename = "active")] active,
	#[serde(rename = "completed")] completed,
	#[serde(rename = "relaunch")] relaunch,
}

impl ::core::str::FromStr for FailoverStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for FailoverStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

FailoverStatus::none,
FailoverStatus::require,
FailoverStatus::active,
FailoverStatus::completed,
FailoverStatus::relaunch,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"require",
"active",
"completed",
"relaunch",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct COLO_EXIT {
#[serde(rename = "mode")]
pub mode: COLOMode,
#[serde(rename = "reason")]
pub reason: COLOExitReason,
}

impl ::qapi_spec::Event for COLO_EXIT {
    const NAME: &'static str = "COLO_EXIT";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum COLOExitReason {
	#[serde(rename = "none")] none,
	#[serde(rename = "request")] request,
	#[serde(rename = "error")] error,
	#[serde(rename = "processing")] processing,
}

impl ::core::str::FromStr for COLOExitReason {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for COLOExitReason {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

COLOExitReason::none,
COLOExitReason::request,
COLOExitReason::error,
COLOExitReason::processing,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"request",
"error",
"processing",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_colo_lost_heartbeat {
}

impl crate::QmpCommand for x_colo_lost_heartbeat { }
impl ::qapi_spec::Command for x_colo_lost_heartbeat {
    const NAME: &'static str = "x-colo-lost-heartbeat";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_cancel {
}

impl crate::QmpCommand for migrate_cancel { }
impl ::qapi_spec::Command for migrate_cancel {
    const NAME: &'static str = "migrate_cancel";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_continue {
	#[serde(rename = "state")]
pub state: MigrationStatus,
}

impl crate::QmpCommand for migrate_continue { }
impl ::qapi_spec::Command for migrate_continue {
    const NAME: &'static str = "migrate-continue";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationAddressType {
	#[serde(rename = "socket")] socket,
	#[serde(rename = "exec")] exec,
	#[serde(rename = "rdma")] rdma,
	#[serde(rename = "file")] file,
}

impl ::core::str::FromStr for MigrationAddressType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigrationAddressType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

MigrationAddressType::socket,
MigrationAddressType::exec,
MigrationAddressType::rdma,
MigrationAddressType::file,

    ];
    const NAMES: &'static [&'static str] = &[

"socket",
"exec",
"rdma",
"file",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationChannelType {
	#[serde(rename = "main")] main,
}

impl ::core::str::FromStr for MigrationChannelType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MigrationChannelType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

MigrationChannelType::main,

    ];
    const NAMES: &'static [&'static str] = &[

"main",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate {
	#[serde(rename = "channels", default, skip_serializing_if = "Option::is_none")]
pub channels: Option<Vec<MigrationChannel>>,
	#[serde(rename = "detach", default, skip_serializing_if = "Option::is_none")]
pub detach: Option<bool>,
	#[serde(rename = "resume", default, skip_serializing_if = "Option::is_none")]
pub resume: Option<bool>,
	#[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
pub uri: Option<::std::string::String>,
}

impl crate::QmpCommand for migrate { }
impl ::qapi_spec::Command for migrate {
    const NAME: &'static str = "migrate";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_incoming {
	#[serde(rename = "channels", default, skip_serializing_if = "Option::is_none")]
pub channels: Option<Vec<MigrationChannel>>,
	#[serde(rename = "exit-on-error", default, skip_serializing_if = "Option::is_none")]
pub exit_on_error: Option<bool>,
	#[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
pub uri: Option<::std::string::String>,
}

impl crate::QmpCommand for migrate_incoming { }
impl ::qapi_spec::Command for migrate_incoming {
    const NAME: &'static str = "migrate-incoming";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_save_devices_state {
	#[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
pub live: Option<bool>,
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for xen_save_devices_state { }
impl ::qapi_spec::Command for xen_save_devices_state {
    const NAME: &'static str = "xen-save-devices-state";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_set_global_dirty_log {
	#[serde(rename = "enable")]
pub enable: bool,
}

impl crate::QmpCommand for xen_set_global_dirty_log { }
impl ::qapi_spec::Command for xen_set_global_dirty_log {
    const NAME: &'static str = "xen-set-global-dirty-log";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_load_devices_state {
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for xen_load_devices_state { }
impl ::qapi_spec::Command for xen_load_devices_state {
    const NAME: &'static str = "xen-load-devices-state";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_set_replication {
	#[serde(rename = "failover", default, skip_serializing_if = "Option::is_none")]
pub failover: Option<bool>,
	#[serde(rename = "enable")]
pub enable: bool,
	#[serde(rename = "primary")]
pub primary: bool,
}

impl crate::QmpCommand for xen_set_replication { }
impl ::qapi_spec::Command for xen_set_replication {
    const NAME: &'static str = "xen-set-replication";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_xen_replication_status {
}

impl crate::QmpCommand for query_xen_replication_status { }
impl ::qapi_spec::Command for query_xen_replication_status {
    const NAME: &'static str = "query-xen-replication-status";
    const ALLOW_OOB: bool = false;

    type Ok = ReplicationStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_colo_do_checkpoint {
}

impl crate::QmpCommand for xen_colo_do_checkpoint { }
impl ::qapi_spec::Command for xen_colo_do_checkpoint {
    const NAME: &'static str = "xen-colo-do-checkpoint";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_colo_status {
}

impl crate::QmpCommand for query_colo_status { }
impl ::qapi_spec::Command for query_colo_status {
    const NAME: &'static str = "query-colo-status";
    const ALLOW_OOB: bool = false;

    type Ok = COLOStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_recover {
	#[serde(rename = "uri")]
pub uri: ::std::string::String,
}

impl crate::QmpCommand for migrate_recover { }
impl ::qapi_spec::Command for migrate_recover {
    const NAME: &'static str = "migrate-recover";
    const ALLOW_OOB: bool = true;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct migrate_pause {
}

impl crate::QmpCommand for migrate_pause { }
impl ::qapi_spec::Command for migrate_pause {
    const NAME: &'static str = "migrate-pause";
    const ALLOW_OOB: bool = true;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UNPLUG_PRIMARY {
#[serde(rename = "device-id")]
pub device_id: ::std::string::String,
}

impl ::qapi_spec::Event for UNPLUG_PRIMARY {
    const NAME: &'static str = "UNPLUG_PRIMARY";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirtyRateStatus {
	#[serde(rename = "unstarted")] unstarted,
	#[serde(rename = "measuring")] measuring,
	#[serde(rename = "measured")] measured,
}

impl ::core::str::FromStr for DirtyRateStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DirtyRateStatus {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

DirtyRateStatus::unstarted,
DirtyRateStatus::measuring,
DirtyRateStatus::measured,

    ];
    const NAMES: &'static [&'static str] = &[

"unstarted",
"measuring",
"measured",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirtyRateMeasureMode {
	#[serde(rename = "page-sampling")] page_sampling,
	#[serde(rename = "dirty-ring")] dirty_ring,
	#[serde(rename = "dirty-bitmap")] dirty_bitmap,
}

impl ::core::str::FromStr for DirtyRateMeasureMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for DirtyRateMeasureMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

DirtyRateMeasureMode::page_sampling,
DirtyRateMeasureMode::dirty_ring,
DirtyRateMeasureMode::dirty_bitmap,

    ];
    const NAMES: &'static [&'static str] = &[

"page-sampling",
"dirty-ring",
"dirty-bitmap",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeUnit {
	#[serde(rename = "second")] second,
	#[serde(rename = "millisecond")] millisecond,
}

impl ::core::str::FromStr for TimeUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for TimeUnit {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

TimeUnit::second,
TimeUnit::millisecond,

    ];
    const NAMES: &'static [&'static str] = &[

"second",
"millisecond",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct calc_dirty_rate {
	#[serde(rename = "calc-time-unit", default, skip_serializing_if = "Option::is_none")]
pub calc_time_unit: Option<TimeUnit>,
	#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<DirtyRateMeasureMode>,
	#[serde(rename = "sample-pages", default, skip_serializing_if = "Option::is_none")]
pub sample_pages: Option<i64>,
	#[serde(rename = "calc-time")]
pub calc_time: i64,
}

impl crate::QmpCommand for calc_dirty_rate { }
impl ::qapi_spec::Command for calc_dirty_rate {
    const NAME: &'static str = "calc-dirty-rate";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_dirty_rate {
	#[serde(rename = "calc-time-unit", default, skip_serializing_if = "Option::is_none")]
pub calc_time_unit: Option<TimeUnit>,
}

impl crate::QmpCommand for query_dirty_rate { }
impl ::qapi_spec::Command for query_dirty_rate {
    const NAME: &'static str = "query-dirty-rate";
    const ALLOW_OOB: bool = false;

    type Ok = DirtyRateInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_vcpu_dirty_limit {
	#[serde(rename = "cpu-index", default, skip_serializing_if = "Option::is_none")]
pub cpu_index: Option<i64>,
	#[serde(rename = "dirty-rate")]
pub dirty_rate: u64,
}

impl crate::QmpCommand for set_vcpu_dirty_limit { }
impl ::qapi_spec::Command for set_vcpu_dirty_limit {
    const NAME: &'static str = "set-vcpu-dirty-limit";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cancel_vcpu_dirty_limit {
	#[serde(rename = "cpu-index", default, skip_serializing_if = "Option::is_none")]
pub cpu_index: Option<i64>,
}

impl crate::QmpCommand for cancel_vcpu_dirty_limit { }
impl ::qapi_spec::Command for cancel_vcpu_dirty_limit {
    const NAME: &'static str = "cancel-vcpu-dirty-limit";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_vcpu_dirty_limit {
}

impl crate::QmpCommand for query_vcpu_dirty_limit { }
impl ::qapi_spec::Command for query_vcpu_dirty_limit {
    const NAME: &'static str = "query-vcpu-dirty-limit";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<DirtyLimitInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_migrationthreads {
}

impl crate::QmpCommand for query_migrationthreads { }
impl ::qapi_spec::Command for query_migrationthreads {
    const NAME: &'static str = "query-migrationthreads";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<MigrationThreadInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct snapshot_save {
	#[serde(rename = "devices")]
pub devices: Vec<::std::string::String>,
	#[serde(rename = "job-id")]
pub job_id: ::std::string::String,
	#[serde(rename = "tag")]
pub tag: ::std::string::String,
	#[serde(rename = "vmstate")]
pub vmstate: ::std::string::String,
}

impl crate::QmpCommand for snapshot_save { }
impl ::qapi_spec::Command for snapshot_save {
    const NAME: &'static str = "snapshot-save";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct snapshot_load {
	#[serde(rename = "devices")]
pub devices: Vec<::std::string::String>,
	#[serde(rename = "job-id")]
pub job_id: ::std::string::String,
	#[serde(rename = "tag")]
pub tag: ::std::string::String,
	#[serde(rename = "vmstate")]
pub vmstate: ::std::string::String,
}

impl crate::QmpCommand for snapshot_load { }
impl ::qapi_spec::Command for snapshot_load {
    const NAME: &'static str = "snapshot-load";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct snapshot_delete {
	#[serde(rename = "devices")]
pub devices: Vec<::std::string::String>,
	#[serde(rename = "job-id")]
pub job_id: ::std::string::String,
	#[serde(rename = "tag")]
pub tag: ::std::string::String,
}

impl crate::QmpCommand for snapshot_delete { }
impl ::qapi_spec::Command for snapshot_delete {
    const NAME: &'static str = "snapshot-delete";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionCompletionMode {
	#[serde(rename = "individual")] individual,
	#[serde(rename = "grouped")] grouped,
}

impl ::core::str::FromStr for ActionCompletionMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ActionCompletionMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

ActionCompletionMode::individual,
ActionCompletionMode::grouped,

    ];
    const NAMES: &'static [&'static str] = &[

"individual",
"grouped",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransactionActionKind {
	#[serde(rename = "abort")] abort,
	#[serde(rename = "block-dirty-bitmap-add")] block_dirty_bitmap_add,
	#[serde(rename = "block-dirty-bitmap-remove")] block_dirty_bitmap_remove,
	#[serde(rename = "block-dirty-bitmap-clear")] block_dirty_bitmap_clear,
	#[serde(rename = "block-dirty-bitmap-enable")] block_dirty_bitmap_enable,
	#[serde(rename = "block-dirty-bitmap-disable")] block_dirty_bitmap_disable,
	#[serde(rename = "block-dirty-bitmap-merge")] block_dirty_bitmap_merge,
	#[serde(rename = "blockdev-backup")] blockdev_backup,
	#[serde(rename = "blockdev-snapshot")] blockdev_snapshot,
	#[serde(rename = "blockdev-snapshot-internal-sync")] blockdev_snapshot_internal_sync,
	#[serde(rename = "blockdev-snapshot-sync")] blockdev_snapshot_sync,
	#[serde(rename = "drive-backup")] drive_backup,
}

impl ::core::str::FromStr for TransactionActionKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for TransactionActionKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 12;
    const VARIANTS: &'static [Self] = &[

TransactionActionKind::abort,
TransactionActionKind::block_dirty_bitmap_add,
TransactionActionKind::block_dirty_bitmap_remove,
TransactionActionKind::block_dirty_bitmap_clear,
TransactionActionKind::block_dirty_bitmap_enable,
TransactionActionKind::block_dirty_bitmap_disable,
TransactionActionKind::block_dirty_bitmap_merge,
TransactionActionKind::blockdev_backup,
TransactionActionKind::blockdev_snapshot,
TransactionActionKind::blockdev_snapshot_internal_sync,
TransactionActionKind::blockdev_snapshot_sync,
TransactionActionKind::drive_backup,

    ];
    const NAMES: &'static [&'static str] = &[

"abort",
"block-dirty-bitmap-add",
"block-dirty-bitmap-remove",
"block-dirty-bitmap-clear",
"block-dirty-bitmap-enable",
"block-dirty-bitmap-disable",
"block-dirty-bitmap-merge",
"blockdev-backup",
"blockdev-snapshot",
"blockdev-snapshot-internal-sync",
"blockdev-snapshot-sync",
"drive-backup",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct transaction {
	#[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
pub properties: Option<TransactionProperties>,
	#[serde(rename = "actions")]
pub actions: Vec<TransactionAction>,
}

impl crate::QmpCommand for transaction { }
impl ::qapi_spec::Command for transaction {
    const NAME: &'static str = "transaction";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraceEventState {
	#[serde(rename = "unavailable")] unavailable,
	#[serde(rename = "disabled")] disabled,
	#[serde(rename = "enabled")] enabled,
}

impl ::core::str::FromStr for TraceEventState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for TraceEventState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

TraceEventState::unavailable,
TraceEventState::disabled,
TraceEventState::enabled,

    ];
    const NAMES: &'static [&'static str] = &[

"unavailable",
"disabled",
"enabled",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct trace_event_get_state {
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for trace_event_get_state { }
impl ::qapi_spec::Command for trace_event_get_state {
    const NAME: &'static str = "trace-event-get-state";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<TraceEventInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct trace_event_set_state {
	#[serde(rename = "ignore-unavailable", default, skip_serializing_if = "Option::is_none")]
pub ignore_unavailable: Option<bool>,
	#[serde(rename = "enable")]
pub enable: bool,
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl crate::QmpCommand for trace_event_set_state { }
impl ::qapi_spec::Command for trace_event_set_state {
    const NAME: &'static str = "trace-event-set-state";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompatPolicyInput {
	#[serde(rename = "accept")] accept,
	#[serde(rename = "reject")] reject,
	#[serde(rename = "crash")] crash,
}

impl ::core::str::FromStr for CompatPolicyInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CompatPolicyInput {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

CompatPolicyInput::accept,
CompatPolicyInput::reject,
CompatPolicyInput::crash,

    ];
    const NAMES: &'static [&'static str] = &[

"accept",
"reject",
"crash",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompatPolicyOutput {
	#[serde(rename = "accept")] accept,
	#[serde(rename = "hide")] hide,
}

impl ::core::str::FromStr for CompatPolicyOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CompatPolicyOutput {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

CompatPolicyOutput::accept,
CompatPolicyOutput::hide,

    ];
    const NAMES: &'static [&'static str] = &[

"accept",
"hide",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qmp_capabilities {
	#[serde(rename = "enable", default, skip_serializing_if = "Option::is_none")]
pub enable: Option<Vec<QMPCapability>>,
}

impl crate::QmpCommand for qmp_capabilities { }
impl ::qapi_spec::Command for qmp_capabilities {
    const NAME: &'static str = "qmp_capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QMPCapability {
	#[serde(rename = "oob")] oob,
}

impl ::core::str::FromStr for QMPCapability {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QMPCapability {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 1;
    const VARIANTS: &'static [Self] = &[

QMPCapability::oob,

    ];
    const NAMES: &'static [&'static str] = &[

"oob",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_version {
}

impl crate::QmpCommand for query_version { }
impl ::qapi_spec::Command for query_version {
    const NAME: &'static str = "query-version";
    const ALLOW_OOB: bool = false;

    type Ok = VersionInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_commands {
}

impl crate::QmpCommand for query_commands { }
impl ::qapi_spec::Command for query_commands {
    const NAME: &'static str = "query-commands";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<CommandInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct quit {
}

impl crate::QmpCommand for quit { }
impl ::qapi_spec::Command for quit {
    const NAME: &'static str = "quit";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonitorMode {
	#[serde(rename = "readline")] readline,
	#[serde(rename = "control")] control,
}

impl ::core::str::FromStr for MonitorMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MonitorMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

MonitorMode::readline,
MonitorMode::control,

    ];
    const NAMES: &'static [&'static str] = &[

"readline",
"control",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_qmp_schema {
}

impl crate::QmpCommand for query_qmp_schema { }
impl ::qapi_spec::Command for query_qmp_schema {
    const NAME: &'static str = "query-qmp-schema";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<SchemaInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaMetaType {
	#[serde(rename = "builtin")] builtin,
	#[serde(rename = "enum")] enum_,
	#[serde(rename = "array")] array,
	#[serde(rename = "object")] object,
	#[serde(rename = "alternate")] alternate,
	#[serde(rename = "command")] command,
	#[serde(rename = "event")] event,
}

impl ::core::str::FromStr for SchemaMetaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SchemaMetaType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

SchemaMetaType::builtin,
SchemaMetaType::enum_,
SchemaMetaType::array,
SchemaMetaType::object,
SchemaMetaType::alternate,
SchemaMetaType::command,
SchemaMetaType::event,

    ];
    const NAMES: &'static [&'static str] = &[

"builtin",
"enum",
"array",
"object",
"alternate",
"command",
"event",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JSONType {
	#[serde(rename = "string")] string,
	#[serde(rename = "number")] number,
	#[serde(rename = "int")] int,
	#[serde(rename = "boolean")] boolean,
	#[serde(rename = "null")] null,
	#[serde(rename = "object")] object,
	#[serde(rename = "array")] array,
	#[serde(rename = "value")] value,
}

impl ::core::str::FromStr for JSONType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for JSONType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 8;
    const VARIANTS: &'static [Self] = &[

JSONType::string,
JSONType::number,
JSONType::int,
JSONType::boolean,
JSONType::null,
JSONType::object,
JSONType::array,
JSONType::value,

    ];
    const NAMES: &'static [&'static str] = &[

"string",
"number",
"int",
"boolean",
"null",
"object",
"array",
"value",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qom_list {
	#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl crate::QmpCommand for qom_list { }
impl ::qapi_spec::Command for qom_list {
    const NAME: &'static str = "qom-list";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ObjectPropertyInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qom_get {
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "property")]
pub property: ::std::string::String,
}

impl crate::QmpCommand for qom_get { }
impl ::qapi_spec::Command for qom_get {
    const NAME: &'static str = "qom-get";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Any;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qom_set {
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "property")]
pub property: ::std::string::String,
	#[serde(rename = "value")]
pub value: ::qapi_spec::Any,
}

impl crate::QmpCommand for qom_set { }
impl ::qapi_spec::Command for qom_set {
    const NAME: &'static str = "qom-set";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qom_list_types {
	#[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
pub abstract_: Option<bool>,
	#[serde(rename = "implements", default, skip_serializing_if = "Option::is_none")]
pub implements: Option<::std::string::String>,
}

impl crate::QmpCommand for qom_list_types { }
impl ::qapi_spec::Command for qom_list_types {
    const NAME: &'static str = "qom-list-types";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ObjectTypeInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct qom_list_properties {
	#[serde(rename = "typename")]
pub typename: ::std::string::String,
}

impl crate::QmpCommand for qom_list_properties { }
impl ::qapi_spec::Command for qom_list_properties {
    const NAME: &'static str = "qom-list-properties";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ObjectPropertyInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetfilterInsert {
	#[serde(rename = "before")] before,
	#[serde(rename = "behind")] behind,
}

impl ::core::str::FromStr for NetfilterInsert {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NetfilterInsert {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

NetfilterInsert::before,
NetfilterInsert::behind,

    ];
    const NAMES: &'static [&'static str] = &[

"before",
"behind",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectType {
	#[serde(rename = "acpi-generic-initiator")] acpi_generic_initiator,
	#[serde(rename = "authz-list")] authz_list,
	#[serde(rename = "authz-listfile")] authz_listfile,
	#[serde(rename = "authz-pam")] authz_pam,
	#[serde(rename = "authz-simple")] authz_simple,
	#[serde(rename = "can-bus")] can_bus,
	#[serde(rename = "can-host-socketcan")] can_host_socketcan,
	#[serde(rename = "colo-compare")] colo_compare,
	#[serde(rename = "cryptodev-backend")] cryptodev_backend,
	#[serde(rename = "cryptodev-backend-builtin")] cryptodev_backend_builtin,
	#[serde(rename = "cryptodev-backend-lkcf")] cryptodev_backend_lkcf,
	#[serde(rename = "cryptodev-vhost-user")] cryptodev_vhost_user,
	#[serde(rename = "dbus-vmstate")] dbus_vmstate,
	#[serde(rename = "filter-buffer")] filter_buffer,
	#[serde(rename = "filter-dump")] filter_dump,
	#[serde(rename = "filter-mirror")] filter_mirror,
	#[serde(rename = "filter-redirector")] filter_redirector,
	#[serde(rename = "filter-replay")] filter_replay,
	#[serde(rename = "filter-rewriter")] filter_rewriter,
	#[serde(rename = "input-barrier")] input_barrier,
	#[serde(rename = "input-linux")] input_linux,
	#[serde(rename = "iommufd")] iommufd,
	#[serde(rename = "iothread")] iothread,
	#[serde(rename = "main-loop")] main_loop,
	#[serde(rename = "memory-backend-epc")] memory_backend_epc,
	#[serde(rename = "memory-backend-file")] memory_backend_file,
	#[serde(rename = "memory-backend-memfd")] memory_backend_memfd,
	#[serde(rename = "memory-backend-ram")] memory_backend_ram,
	#[serde(rename = "memory-backend-shm")] memory_backend_shm,
	#[serde(rename = "pef-guest")] pef_guest,
	#[serde(rename = "pr-manager-helper")] pr_manager_helper,
	#[serde(rename = "qtest")] qtest,
	#[serde(rename = "rng-builtin")] rng_builtin,
	#[serde(rename = "rng-egd")] rng_egd,
	#[serde(rename = "rng-random")] rng_random,
	#[serde(rename = "secret")] secret,
	#[serde(rename = "secret_keyring")] secret_keyring,
	#[serde(rename = "sev-guest")] sev_guest,
	#[serde(rename = "sev-snp-guest")] sev_snp_guest,
	#[serde(rename = "thread-context")] thread_context,
	#[serde(rename = "s390-pv-guest")] s390_pv_guest,
	#[serde(rename = "throttle-group")] throttle_group,
	#[serde(rename = "tls-creds-anon")] tls_creds_anon,
	#[serde(rename = "tls-creds-psk")] tls_creds_psk,
	#[serde(rename = "tls-creds-x509")] tls_creds_x509,
	#[serde(rename = "tls-cipher-suites")] tls_cipher_suites,
	#[serde(rename = "x-remote-object")] x_remote_object,
	#[serde(rename = "x-vfio-user-server")] x_vfio_user_server,
}

impl ::core::str::FromStr for ObjectType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ObjectType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 48;
    const VARIANTS: &'static [Self] = &[

ObjectType::acpi_generic_initiator,
ObjectType::authz_list,
ObjectType::authz_listfile,
ObjectType::authz_pam,
ObjectType::authz_simple,
ObjectType::can_bus,
ObjectType::can_host_socketcan,
ObjectType::colo_compare,
ObjectType::cryptodev_backend,
ObjectType::cryptodev_backend_builtin,
ObjectType::cryptodev_backend_lkcf,
ObjectType::cryptodev_vhost_user,
ObjectType::dbus_vmstate,
ObjectType::filter_buffer,
ObjectType::filter_dump,
ObjectType::filter_mirror,
ObjectType::filter_redirector,
ObjectType::filter_replay,
ObjectType::filter_rewriter,
ObjectType::input_barrier,
ObjectType::input_linux,
ObjectType::iommufd,
ObjectType::iothread,
ObjectType::main_loop,
ObjectType::memory_backend_epc,
ObjectType::memory_backend_file,
ObjectType::memory_backend_memfd,
ObjectType::memory_backend_ram,
ObjectType::memory_backend_shm,
ObjectType::pef_guest,
ObjectType::pr_manager_helper,
ObjectType::qtest,
ObjectType::rng_builtin,
ObjectType::rng_egd,
ObjectType::rng_random,
ObjectType::secret,
ObjectType::secret_keyring,
ObjectType::sev_guest,
ObjectType::sev_snp_guest,
ObjectType::thread_context,
ObjectType::s390_pv_guest,
ObjectType::throttle_group,
ObjectType::tls_creds_anon,
ObjectType::tls_creds_psk,
ObjectType::tls_creds_x509,
ObjectType::tls_cipher_suites,
ObjectType::x_remote_object,
ObjectType::x_vfio_user_server,

    ];
    const NAMES: &'static [&'static str] = &[

"acpi-generic-initiator",
"authz-list",
"authz-listfile",
"authz-pam",
"authz-simple",
"can-bus",
"can-host-socketcan",
"colo-compare",
"cryptodev-backend",
"cryptodev-backend-builtin",
"cryptodev-backend-lkcf",
"cryptodev-vhost-user",
"dbus-vmstate",
"filter-buffer",
"filter-dump",
"filter-mirror",
"filter-redirector",
"filter-replay",
"filter-rewriter",
"input-barrier",
"input-linux",
"iommufd",
"iothread",
"main-loop",
"memory-backend-epc",
"memory-backend-file",
"memory-backend-memfd",
"memory-backend-ram",
"memory-backend-shm",
"pef-guest",
"pr-manager-helper",
"qtest",
"rng-builtin",
"rng-egd",
"rng-random",
"secret",
"secret_keyring",
"sev-guest",
"sev-snp-guest",
"thread-context",
"s390-pv-guest",
"throttle-group",
"tls-creds-anon",
"tls-creds-psk",
"tls-creds-x509",
"tls-cipher-suites",
"x-remote-object",
"x-vfio-user-server",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct object_add(pub ObjectOptions);

impl From<ObjectOptions> for object_add {
    fn from(val: ObjectOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for object_add { }
impl ::qapi_spec::Command for object_add {
    const NAME: &'static str = "object-add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct object_del {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for object_del { }
impl ::qapi_spec::Command for object_del {
    const NAME: &'static str = "object-del";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct device_list_properties {
	#[serde(rename = "typename")]
pub typename: ::std::string::String,
}

impl crate::QmpCommand for device_list_properties { }
impl ::qapi_spec::Command for device_list_properties {
    const NAME: &'static str = "device-list-properties";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ObjectPropertyInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct device_add {
	#[serde(rename = "bus", default, skip_serializing_if = "Option::is_none")]
pub bus: Option<::std::string::String>,
	#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
	#[serde(rename = "driver")]
pub driver: ::std::string::String,

    #[serde(flatten)]
    pub arguments: ::qapi_spec::Dictionary,

}

impl crate::QmpCommand for device_add { }
impl ::qapi_spec::Command for device_add {
    const NAME: &'static str = "device_add";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct device_del {
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl crate::QmpCommand for device_del { }
impl ::qapi_spec::Command for device_del {
    const NAME: &'static str = "device_del";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEVICE_DELETED {
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl ::qapi_spec::Event for DEVICE_DELETED {
    const NAME: &'static str = "DEVICE_DELETED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEVICE_UNPLUG_GUEST_ERROR {
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl ::qapi_spec::Event for DEVICE_UNPLUG_GUEST_ERROR {
    const NAME: &'static str = "DEVICE_UNPLUG_GUEST_ERROR";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuS390Entitlement {
	#[serde(rename = "auto")] auto,
	#[serde(rename = "low")] low,
	#[serde(rename = "medium")] medium,
	#[serde(rename = "high")] high,
}

impl ::core::str::FromStr for CpuS390Entitlement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CpuS390Entitlement {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

CpuS390Entitlement::auto,
CpuS390Entitlement::low,
CpuS390Entitlement::medium,
CpuS390Entitlement::high,

    ];
    const NAMES: &'static [&'static str] = &[

"auto",
"low",
"medium",
"high",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysEmuTarget {
	#[serde(rename = "aarch64")] aarch64,
	#[serde(rename = "alpha")] alpha,
	#[serde(rename = "arm")] arm,
	#[serde(rename = "avr")] avr,
	#[serde(rename = "cris")] cris,
	#[serde(rename = "hppa")] hppa,
	#[serde(rename = "i386")] i386,
	#[serde(rename = "loongarch64")] loongarch64,
	#[serde(rename = "m68k")] m68k,
	#[serde(rename = "microblaze")] microblaze,
	#[serde(rename = "microblazeel")] microblazeel,
	#[serde(rename = "mips")] mips,
	#[serde(rename = "mips64")] mips64,
	#[serde(rename = "mips64el")] mips64el,
	#[serde(rename = "mipsel")] mipsel,
	#[serde(rename = "or1k")] or1k,
	#[serde(rename = "ppc")] ppc,
	#[serde(rename = "ppc64")] ppc64,
	#[serde(rename = "riscv32")] riscv32,
	#[serde(rename = "riscv64")] riscv64,
	#[serde(rename = "rx")] rx,
	#[serde(rename = "s390x")] s390x,
	#[serde(rename = "sh4")] sh4,
	#[serde(rename = "sh4eb")] sh4eb,
	#[serde(rename = "sparc")] sparc,
	#[serde(rename = "sparc64")] sparc64,
	#[serde(rename = "tricore")] tricore,
	#[serde(rename = "x86_64")] x86_64,
	#[serde(rename = "xtensa")] xtensa,
	#[serde(rename = "xtensaeb")] xtensaeb,
}

impl ::core::str::FromStr for SysEmuTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SysEmuTarget {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 30;
    const VARIANTS: &'static [Self] = &[

SysEmuTarget::aarch64,
SysEmuTarget::alpha,
SysEmuTarget::arm,
SysEmuTarget::avr,
SysEmuTarget::cris,
SysEmuTarget::hppa,
SysEmuTarget::i386,
SysEmuTarget::loongarch64,
SysEmuTarget::m68k,
SysEmuTarget::microblaze,
SysEmuTarget::microblazeel,
SysEmuTarget::mips,
SysEmuTarget::mips64,
SysEmuTarget::mips64el,
SysEmuTarget::mipsel,
SysEmuTarget::or1k,
SysEmuTarget::ppc,
SysEmuTarget::ppc64,
SysEmuTarget::riscv32,
SysEmuTarget::riscv64,
SysEmuTarget::rx,
SysEmuTarget::s390x,
SysEmuTarget::sh4,
SysEmuTarget::sh4eb,
SysEmuTarget::sparc,
SysEmuTarget::sparc64,
SysEmuTarget::tricore,
SysEmuTarget::x86_64,
SysEmuTarget::xtensa,
SysEmuTarget::xtensaeb,

    ];
    const NAMES: &'static [&'static str] = &[

"aarch64",
"alpha",
"arm",
"avr",
"cris",
"hppa",
"i386",
"loongarch64",
"m68k",
"microblaze",
"microblazeel",
"mips",
"mips64",
"mips64el",
"mipsel",
"or1k",
"ppc",
"ppc64",
"riscv32",
"riscv64",
"rx",
"s390x",
"sh4",
"sh4eb",
"sparc",
"sparc64",
"tricore",
"x86_64",
"xtensa",
"xtensaeb",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuS390State {
	#[serde(rename = "uninitialized")] uninitialized,
	#[serde(rename = "stopped")] stopped,
	#[serde(rename = "check-stop")] check_stop,
	#[serde(rename = "operating")] operating,
	#[serde(rename = "load")] load,
}

impl ::core::str::FromStr for CpuS390State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CpuS390State {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

CpuS390State::uninitialized,
CpuS390State::stopped,
CpuS390State::check_stop,
CpuS390State::operating,
CpuS390State::load,

    ];
    const NAMES: &'static [&'static str] = &[

"uninitialized",
"stopped",
"check-stop",
"operating",
"load",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cpus_fast {
}

impl crate::QmpCommand for query_cpus_fast { }
impl ::qapi_spec::Command for query_cpus_fast {
    const NAME: &'static str = "query-cpus-fast";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<CpuInfoFast>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_machines {
	#[serde(rename = "compat-props", default, skip_serializing_if = "Option::is_none")]
pub compat_props: Option<bool>,
}

impl crate::QmpCommand for query_machines { }
impl ::qapi_spec::Command for query_machines {
    const NAME: &'static str = "query-machines";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<MachineInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_current_machine {
}

impl crate::QmpCommand for query_current_machine { }
impl ::qapi_spec::Command for query_current_machine {
    const NAME: &'static str = "query-current-machine";
    const ALLOW_OOB: bool = false;

    type Ok = CurrentMachineParams;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_target {
}

impl crate::QmpCommand for query_target { }
impl ::qapi_spec::Command for query_target {
    const NAME: &'static str = "query-target";
    const ALLOW_OOB: bool = false;

    type Ok = TargetInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_uuid {
}

impl crate::QmpCommand for query_uuid { }
impl ::qapi_spec::Command for query_uuid {
    const NAME: &'static str = "query-uuid";
    const ALLOW_OOB: bool = false;

    type Ok = UuidInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_vm_generation_id {
}

impl crate::QmpCommand for query_vm_generation_id { }
impl ::qapi_spec::Command for query_vm_generation_id {
    const NAME: &'static str = "query-vm-generation-id";
    const ALLOW_OOB: bool = false;

    type Ok = GuidInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct system_reset {
}

impl crate::QmpCommand for system_reset { }
impl ::qapi_spec::Command for system_reset {
    const NAME: &'static str = "system_reset";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct system_powerdown {
}

impl crate::QmpCommand for system_powerdown { }
impl ::qapi_spec::Command for system_powerdown {
    const NAME: &'static str = "system_powerdown";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct system_wakeup {
}

impl crate::QmpCommand for system_wakeup { }
impl ::qapi_spec::Command for system_wakeup {
    const NAME: &'static str = "system_wakeup";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LostTickPolicy {
	#[serde(rename = "discard")] discard,
	#[serde(rename = "delay")] delay,
	#[serde(rename = "slew")] slew,
}

impl ::core::str::FromStr for LostTickPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for LostTickPolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

LostTickPolicy::discard,
LostTickPolicy::delay,
LostTickPolicy::slew,

    ];
    const NAMES: &'static [&'static str] = &[

"discard",
"delay",
"slew",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct inject_nmi {
}

impl crate::QmpCommand for inject_nmi { }
impl ::qapi_spec::Command for inject_nmi {
    const NAME: &'static str = "inject-nmi";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_kvm {
}

impl crate::QmpCommand for query_kvm { }
impl ::qapi_spec::Command for query_kvm {
    const NAME: &'static str = "query-kvm";
    const ALLOW_OOB: bool = false;

    type Ok = KvmInfo;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumaOptionsType {
	#[serde(rename = "node")] node,
	#[serde(rename = "dist")] dist,
	#[serde(rename = "cpu")] cpu,
	#[serde(rename = "hmat-lb")] hmat_lb,
	#[serde(rename = "hmat-cache")] hmat_cache,
}

impl ::core::str::FromStr for NumaOptionsType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for NumaOptionsType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

NumaOptionsType::node,
NumaOptionsType::dist,
NumaOptionsType::cpu,
NumaOptionsType::hmat_lb,
NumaOptionsType::hmat_cache,

    ];
    const NAMES: &'static [&'static str] = &[

"node",
"dist",
"cpu",
"hmat-lb",
"hmat-cache",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum X86CPURegister32 {
	#[serde(rename = "EAX")] EAX,
	#[serde(rename = "EBX")] EBX,
	#[serde(rename = "ECX")] ECX,
	#[serde(rename = "EDX")] EDX,
	#[serde(rename = "ESP")] ESP,
	#[serde(rename = "EBP")] EBP,
	#[serde(rename = "ESI")] ESI,
	#[serde(rename = "EDI")] EDI,
}

impl ::core::str::FromStr for X86CPURegister32 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for X86CPURegister32 {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 8;
    const VARIANTS: &'static [Self] = &[

X86CPURegister32::EAX,
X86CPURegister32::EBX,
X86CPURegister32::ECX,
X86CPURegister32::EDX,
X86CPURegister32::ESP,
X86CPURegister32::EBP,
X86CPURegister32::ESI,
X86CPURegister32::EDI,

    ];
    const NAMES: &'static [&'static str] = &[

"EAX",
"EBX",
"ECX",
"EDX",
"ESP",
"EBP",
"ESI",
"EDI",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HmatLBMemoryHierarchy {
	#[serde(rename = "memory")] memory,
	#[serde(rename = "first-level")] first_level,
	#[serde(rename = "second-level")] second_level,
	#[serde(rename = "third-level")] third_level,
}

impl ::core::str::FromStr for HmatLBMemoryHierarchy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HmatLBMemoryHierarchy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

HmatLBMemoryHierarchy::memory,
HmatLBMemoryHierarchy::first_level,
HmatLBMemoryHierarchy::second_level,
HmatLBMemoryHierarchy::third_level,

    ];
    const NAMES: &'static [&'static str] = &[

"memory",
"first-level",
"second-level",
"third-level",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HmatLBDataType {
	#[serde(rename = "access-latency")] access_latency,
	#[serde(rename = "read-latency")] read_latency,
	#[serde(rename = "write-latency")] write_latency,
	#[serde(rename = "access-bandwidth")] access_bandwidth,
	#[serde(rename = "read-bandwidth")] read_bandwidth,
	#[serde(rename = "write-bandwidth")] write_bandwidth,
}

impl ::core::str::FromStr for HmatLBDataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HmatLBDataType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

HmatLBDataType::access_latency,
HmatLBDataType::read_latency,
HmatLBDataType::write_latency,
HmatLBDataType::access_bandwidth,
HmatLBDataType::read_bandwidth,
HmatLBDataType::write_bandwidth,

    ];
    const NAMES: &'static [&'static str] = &[

"access-latency",
"read-latency",
"write-latency",
"access-bandwidth",
"read-bandwidth",
"write-bandwidth",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HmatCacheAssociativity {
	#[serde(rename = "none")] none,
	#[serde(rename = "direct")] direct,
	#[serde(rename = "complex")] complex,
}

impl ::core::str::FromStr for HmatCacheAssociativity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HmatCacheAssociativity {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

HmatCacheAssociativity::none,
HmatCacheAssociativity::direct,
HmatCacheAssociativity::complex,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"direct",
"complex",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HmatCacheWritePolicy {
	#[serde(rename = "none")] none,
	#[serde(rename = "write-back")] write_back,
	#[serde(rename = "write-through")] write_through,
}

impl ::core::str::FromStr for HmatCacheWritePolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for HmatCacheWritePolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

HmatCacheWritePolicy::none,
HmatCacheWritePolicy::write_back,
HmatCacheWritePolicy::write_through,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"write-back",
"write-through",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct memsave {
	#[serde(rename = "cpu-index", default, skip_serializing_if = "Option::is_none")]
pub cpu_index: Option<i64>,
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
	#[serde(rename = "size")]
pub size: u64,
	#[serde(rename = "val")]
pub val: u64,
}

impl crate::QmpCommand for memsave { }
impl ::qapi_spec::Command for memsave {
    const NAME: &'static str = "memsave";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct pmemsave {
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
	#[serde(rename = "size")]
pub size: u64,
	#[serde(rename = "val")]
pub val: u64,
}

impl crate::QmpCommand for pmemsave { }
impl ::qapi_spec::Command for pmemsave {
    const NAME: &'static str = "pmemsave";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_memdev {
}

impl crate::QmpCommand for query_memdev { }
impl ::qapi_spec::Command for query_memdev {
    const NAME: &'static str = "query-memdev";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<Memdev>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_hotpluggable_cpus {
}

impl crate::QmpCommand for query_hotpluggable_cpus { }
impl ::qapi_spec::Command for query_hotpluggable_cpus {
    const NAME: &'static str = "query-hotpluggable-cpus";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<HotpluggableCPU>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_numa_node(pub NumaOptions);

impl From<NumaOptions> for set_numa_node {
    fn from(val: NumaOptions) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for set_numa_node { }
impl ::qapi_spec::Command for set_numa_node {
    const NAME: &'static str = "set-numa-node";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct balloon {
	#[serde(rename = "value")]
pub value: i64,
}

impl crate::QmpCommand for balloon { }
impl ::qapi_spec::Command for balloon {
    const NAME: &'static str = "balloon";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_balloon {
}

impl crate::QmpCommand for query_balloon { }
impl ::qapi_spec::Command for query_balloon {
    const NAME: &'static str = "query-balloon";
    const ALLOW_OOB: bool = false;

    type Ok = BalloonInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BALLOON_CHANGE {
#[serde(rename = "actual")]
pub actual: i64,
}

impl ::qapi_spec::Event for BALLOON_CHANGE {
    const NAME: &'static str = "BALLOON_CHANGE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_hv_balloon_status_report {
}

impl crate::QmpCommand for query_hv_balloon_status_report { }
impl ::qapi_spec::Command for query_hv_balloon_status_report {
    const NAME: &'static str = "query-hv-balloon-status-report";
    const ALLOW_OOB: bool = false;

    type Ok = HvBalloonInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct HV_BALLOON_STATUS_REPORT {
#[serde(rename = "data")]
pub data: HvBalloonInfo,
}

impl ::qapi_spec::Event for HV_BALLOON_STATUS_REPORT {
    const NAME: &'static str = "HV_BALLOON_STATUS_REPORT";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_memory_size_summary {
}

impl crate::QmpCommand for query_memory_size_summary { }
impl ::qapi_spec::Command for query_memory_size_summary {
    const NAME: &'static str = "query-memory-size-summary";
    const ALLOW_OOB: bool = false;

    type Ok = MemoryInfo;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryDeviceInfoKind {
	#[serde(rename = "dimm")] dimm,
	#[serde(rename = "nvdimm")] nvdimm,
	#[serde(rename = "virtio-pmem")] virtio_pmem,
	#[serde(rename = "virtio-mem")] virtio_mem,
	#[serde(rename = "sgx-epc")] sgx_epc,
	#[serde(rename = "hv-balloon")] hv_balloon,
}

impl ::core::str::FromStr for MemoryDeviceInfoKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for MemoryDeviceInfoKind {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

MemoryDeviceInfoKind::dimm,
MemoryDeviceInfoKind::nvdimm,
MemoryDeviceInfoKind::virtio_pmem,
MemoryDeviceInfoKind::virtio_mem,
MemoryDeviceInfoKind::sgx_epc,
MemoryDeviceInfoKind::hv_balloon,

    ];
    const NAMES: &'static [&'static str] = &[

"dimm",
"nvdimm",
"virtio-pmem",
"virtio-mem",
"sgx-epc",
"hv-balloon",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_memory_devices {
}

impl crate::QmpCommand for query_memory_devices { }
impl ::qapi_spec::Command for query_memory_devices {
    const NAME: &'static str = "query-memory-devices";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<MemoryDeviceInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MEMORY_DEVICE_SIZE_CHANGE {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "qom-path")]
pub qom_path: ::std::string::String,
#[serde(rename = "size")]
pub size: u64,
}

impl ::qapi_spec::Event for MEMORY_DEVICE_SIZE_CHANGE {
    const NAME: &'static str = "MEMORY_DEVICE_SIZE_CHANGE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_irq {
}

impl crate::QmpCommand for x_query_irq { }
impl ::qapi_spec::Command for x_query_irq {
    const NAME: &'static str = "x-query-irq";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_jit {
}

impl crate::QmpCommand for x_query_jit { }
impl ::qapi_spec::Command for x_query_jit {
    const NAME: &'static str = "x-query-jit";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_numa {
}

impl crate::QmpCommand for x_query_numa { }
impl ::qapi_spec::Command for x_query_numa {
    const NAME: &'static str = "x-query-numa";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_opcount {
}

impl crate::QmpCommand for x_query_opcount { }
impl ::qapi_spec::Command for x_query_opcount {
    const NAME: &'static str = "x-query-opcount";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_ramblock {
}

impl crate::QmpCommand for x_query_ramblock { }
impl ::qapi_spec::Command for x_query_ramblock {
    const NAME: &'static str = "x-query-ramblock";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_roms {
}

impl crate::QmpCommand for x_query_roms { }
impl ::qapi_spec::Command for x_query_roms {
    const NAME: &'static str = "x-query-roms";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_usb {
}

impl crate::QmpCommand for x_query_usb { }
impl ::qapi_spec::Command for x_query_usb {
    const NAME: &'static str = "x-query-usb";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SmbiosEntryPointType {
	#[serde(rename = "32")] _32,
	#[serde(rename = "64")] _64,
	#[serde(rename = "auto")] auto,
}

impl ::core::str::FromStr for SmbiosEntryPointType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SmbiosEntryPointType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

SmbiosEntryPointType::_32,
SmbiosEntryPointType::_64,
SmbiosEntryPointType::auto,

    ];
    const NAMES: &'static [&'static str] = &[

"32",
"64",
"auto",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct dumpdtb {
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for dumpdtb { }
impl ::qapi_spec::Command for dumpdtb {
    const NAME: &'static str = "dumpdtb";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_interrupt_controllers {
}

impl crate::QmpCommand for x_query_interrupt_controllers { }
impl ::qapi_spec::Command for x_query_interrupt_controllers {
    const NAME: &'static str = "x-query-interrupt-controllers";
    const ALLOW_OOB: bool = false;

    type Ok = HumanReadableText;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuModelExpansionType {
	#[serde(rename = "static")] static_,
	#[serde(rename = "full")] full,
}

impl ::core::str::FromStr for CpuModelExpansionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CpuModelExpansionType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

CpuModelExpansionType::static_,
CpuModelExpansionType::full,

    ];
    const NAMES: &'static [&'static str] = &[

"static",
"full",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuModelCompareResult {
	#[serde(rename = "incompatible")] incompatible,
	#[serde(rename = "identical")] identical,
	#[serde(rename = "superset")] superset,
	#[serde(rename = "subset")] subset,
}

impl ::core::str::FromStr for CpuModelCompareResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CpuModelCompareResult {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

CpuModelCompareResult::incompatible,
CpuModelCompareResult::identical,
CpuModelCompareResult::superset,
CpuModelCompareResult::subset,

    ];
    const NAMES: &'static [&'static str] = &[

"incompatible",
"identical",
"superset",
"subset",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cpu_model_comparison {
	#[serde(rename = "modela")]
pub modela: CpuModelInfo,
	#[serde(rename = "modelb")]
pub modelb: CpuModelInfo,
}

impl crate::QmpCommand for query_cpu_model_comparison { }
impl ::qapi_spec::Command for query_cpu_model_comparison {
    const NAME: &'static str = "query-cpu-model-comparison";
    const ALLOW_OOB: bool = false;

    type Ok = CpuModelCompareInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cpu_model_baseline {
	#[serde(rename = "modela")]
pub modela: CpuModelInfo,
	#[serde(rename = "modelb")]
pub modelb: CpuModelInfo,
}

impl crate::QmpCommand for query_cpu_model_baseline { }
impl ::qapi_spec::Command for query_cpu_model_baseline {
    const NAME: &'static str = "query-cpu-model-baseline";
    const ALLOW_OOB: bool = false;

    type Ok = CpuModelBaselineInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cpu_model_expansion {
	#[serde(rename = "model")]
pub model: CpuModelInfo,
	#[serde(rename = "type")]
pub type_: CpuModelExpansionType,
}

impl crate::QmpCommand for query_cpu_model_expansion { }
impl ::qapi_spec::Command for query_cpu_model_expansion {
    const NAME: &'static str = "query-cpu-model-expansion";
    const ALLOW_OOB: bool = false;

    type Ok = CpuModelExpansionInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cpu_definitions {
}

impl crate::QmpCommand for query_cpu_definitions { }
impl ::qapi_spec::Command for query_cpu_definitions {
    const NAME: &'static str = "query-cpu-definitions";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<CpuDefinitionInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuS390Polarization {
	#[serde(rename = "horizontal")] horizontal,
	#[serde(rename = "vertical")] vertical,
}

impl ::core::str::FromStr for CpuS390Polarization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CpuS390Polarization {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

CpuS390Polarization::horizontal,
CpuS390Polarization::vertical,

    ];
    const NAMES: &'static [&'static str] = &[

"horizontal",
"vertical",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_cpu_topology {
	#[serde(rename = "book-id", default, skip_serializing_if = "Option::is_none")]
pub book_id: Option<u16>,
	#[serde(rename = "dedicated", default, skip_serializing_if = "Option::is_none")]
pub dedicated: Option<bool>,
	#[serde(rename = "drawer-id", default, skip_serializing_if = "Option::is_none")]
pub drawer_id: Option<u16>,
	#[serde(rename = "entitlement", default, skip_serializing_if = "Option::is_none")]
pub entitlement: Option<CpuS390Entitlement>,
	#[serde(rename = "socket-id", default, skip_serializing_if = "Option::is_none")]
pub socket_id: Option<u16>,
	#[serde(rename = "core-id")]
pub core_id: u16,
}

impl crate::QmpCommand for set_cpu_topology { }
impl ::qapi_spec::Command for set_cpu_topology {
    const NAME: &'static str = "set-cpu-topology";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPU_POLARIZATION_CHANGE {
#[serde(rename = "polarization")]
pub polarization: CpuS390Polarization,
}

impl ::qapi_spec::Event for CPU_POLARIZATION_CHANGE {
    const NAME: &'static str = "CPU_POLARIZATION_CHANGE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_s390x_cpu_polarization {
}

impl crate::QmpCommand for query_s390x_cpu_polarization { }
impl ::qapi_spec::Command for query_s390x_cpu_polarization {
    const NAME: &'static str = "query-s390x-cpu-polarization";
    const ALLOW_OOB: bool = false;

    type Ok = CpuPolarizationInfo;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReplayMode {
	#[serde(rename = "none")] none,
	#[serde(rename = "record")] record,
	#[serde(rename = "play")] play,
}

impl ::core::str::FromStr for ReplayMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ReplayMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

ReplayMode::none,
ReplayMode::record,
ReplayMode::play,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"record",
"play",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_replay {
}

impl crate::QmpCommand for query_replay { }
impl ::qapi_spec::Command for query_replay {
    const NAME: &'static str = "query-replay";
    const ALLOW_OOB: bool = false;

    type Ok = ReplayInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct replay_break {
	#[serde(rename = "icount")]
pub icount: i64,
}

impl crate::QmpCommand for replay_break { }
impl ::qapi_spec::Command for replay_break {
    const NAME: &'static str = "replay-break";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct replay_delete_break {
}

impl crate::QmpCommand for replay_delete_break { }
impl ::qapi_spec::Command for replay_delete_break {
    const NAME: &'static str = "replay-delete-break";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct replay_seek {
	#[serde(rename = "icount")]
pub icount: i64,
}

impl crate::QmpCommand for replay_seek { }
impl ::qapi_spec::Command for replay_seek {
    const NAME: &'static str = "replay-seek";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YankInstanceType {
	#[serde(rename = "block-node")] block_node,
	#[serde(rename = "chardev")] chardev,
	#[serde(rename = "migration")] migration,
}

impl ::core::str::FromStr for YankInstanceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for YankInstanceType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

YankInstanceType::block_node,
YankInstanceType::chardev,
YankInstanceType::migration,

    ];
    const NAMES: &'static [&'static str] = &[

"block-node",
"chardev",
"migration",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct yank {
	#[serde(rename = "instances")]
pub instances: Vec<YankInstance>,
}

impl crate::QmpCommand for yank { }
impl ::qapi_spec::Command for yank {
    const NAME: &'static str = "yank";
    const ALLOW_OOB: bool = true;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_yank {
}

impl crate::QmpCommand for query_yank { }
impl ::qapi_spec::Command for query_yank {
    const NAME: &'static str = "query-yank";
    const ALLOW_OOB: bool = true;

    type Ok = Vec<YankInstance>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct add_client {
	#[serde(rename = "skipauth", default, skip_serializing_if = "Option::is_none")]
pub skipauth: Option<bool>,
	#[serde(rename = "tls", default, skip_serializing_if = "Option::is_none")]
pub tls: Option<bool>,
	#[serde(rename = "fdname")]
pub fdname: ::std::string::String,
	#[serde(rename = "protocol")]
pub protocol: ::std::string::String,
}

impl crate::QmpCommand for add_client { }
impl ::qapi_spec::Command for add_client {
    const NAME: &'static str = "add_client";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_name {
}

impl crate::QmpCommand for query_name { }
impl ::qapi_spec::Command for query_name {
    const NAME: &'static str = "query-name";
    const ALLOW_OOB: bool = false;

    type Ok = NameInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_iothreads {
}

impl crate::QmpCommand for query_iothreads { }
impl ::qapi_spec::Command for query_iothreads {
    const NAME: &'static str = "query-iothreads";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<IOThreadInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct stop {
}

impl crate::QmpCommand for stop { }
impl ::qapi_spec::Command for stop {
    const NAME: &'static str = "stop";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cont {
}

impl crate::QmpCommand for cont { }
impl ::qapi_spec::Command for cont {
    const NAME: &'static str = "cont";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_exit_preconfig {
}

impl crate::QmpCommand for x_exit_preconfig { }
impl ::qapi_spec::Command for x_exit_preconfig {
    const NAME: &'static str = "x-exit-preconfig";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct human_monitor_command {
	#[serde(rename = "cpu-index", default, skip_serializing_if = "Option::is_none")]
pub cpu_index: Option<i64>,
	#[serde(rename = "command-line")]
pub command_line: ::std::string::String,
}

impl crate::QmpCommand for human_monitor_command { }
impl ::qapi_spec::Command for human_monitor_command {
    const NAME: &'static str = "human-monitor-command";
    const ALLOW_OOB: bool = false;

    type Ok = ::std::string::String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct getfd {
	#[serde(rename = "fdname")]
pub fdname: ::std::string::String,
}

impl crate::QmpCommand for getfd { }
impl ::qapi_spec::Command for getfd {
    const NAME: &'static str = "getfd";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct get_win32_socket {
	#[serde(rename = "fdname")]
pub fdname: ::std::string::String,
	#[serde(rename = "info", with = "::qapi_spec::base64")]
pub info: Vec<u8>,
}

impl crate::QmpCommand for get_win32_socket { }
impl ::qapi_spec::Command for get_win32_socket {
    const NAME: &'static str = "get-win32-socket";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct closefd {
	#[serde(rename = "fdname")]
pub fdname: ::std::string::String,
}

impl crate::QmpCommand for closefd { }
impl ::qapi_spec::Command for closefd {
    const NAME: &'static str = "closefd";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct add_fd {
	#[serde(rename = "fdset-id", default, skip_serializing_if = "Option::is_none")]
pub fdset_id: Option<i64>,
	#[serde(rename = "opaque", default, skip_serializing_if = "Option::is_none")]
pub opaque: Option<::std::string::String>,
}

impl crate::QmpCommand for add_fd { }
impl ::qapi_spec::Command for add_fd {
    const NAME: &'static str = "add-fd";
    const ALLOW_OOB: bool = false;

    type Ok = AddfdInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct remove_fd {
	#[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
pub fd: Option<i64>,
	#[serde(rename = "fdset-id")]
pub fdset_id: i64,
}

impl crate::QmpCommand for remove_fd { }
impl ::qapi_spec::Command for remove_fd {
    const NAME: &'static str = "remove-fd";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_fdsets {
}

impl crate::QmpCommand for query_fdsets { }
impl ::qapi_spec::Command for query_fdsets {
    const NAME: &'static str = "query-fdsets";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<FdsetInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommandLineParameterType {
	#[serde(rename = "string")] string,
	#[serde(rename = "boolean")] boolean,
	#[serde(rename = "number")] number,
	#[serde(rename = "size")] size,
}

impl ::core::str::FromStr for CommandLineParameterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CommandLineParameterType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

CommandLineParameterType::string,
CommandLineParameterType::boolean,
CommandLineParameterType::number,
CommandLineParameterType::size,

    ];
    const NAMES: &'static [&'static str] = &[

"string",
"boolean",
"number",
"size",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_command_line_options {
	#[serde(rename = "option", default, skip_serializing_if = "Option::is_none")]
pub option: Option<::std::string::String>,
}

impl crate::QmpCommand for query_command_line_options { }
impl ::qapi_spec::Command for query_command_line_options {
    const NAME: &'static str = "query-command-line-options";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<CommandLineOptionInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTC_CHANGE {
#[serde(rename = "offset")]
pub offset: i64,
#[serde(rename = "qom-path")]
pub qom_path: ::std::string::String,
}

impl ::qapi_spec::Event for RTC_CHANGE {
    const NAME: &'static str = "RTC_CHANGE";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VFU_CLIENT_HANGUP {
#[serde(rename = "dev-id")]
pub dev_id: ::std::string::String,
#[serde(rename = "dev-qom-path")]
pub dev_qom_path: ::std::string::String,
#[serde(rename = "vfu-id")]
pub vfu_id: ::std::string::String,
#[serde(rename = "vfu-qom-path")]
pub vfu_qom_path: ::std::string::String,
}

impl ::qapi_spec::Event for VFU_CLIENT_HANGUP {
    const NAME: &'static str = "VFU_CLIENT_HANGUP";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct rtc_reset_reinjection {
}

impl crate::QmpCommand for rtc_reset_reinjection { }
impl ::qapi_spec::Command for rtc_reset_reinjection {
    const NAME: &'static str = "rtc-reset-reinjection";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SevState {
	#[serde(rename = "uninit")] uninit,
	#[serde(rename = "launch-update")] launch_update,
	#[serde(rename = "launch-secret")] launch_secret,
	#[serde(rename = "running")] running,
	#[serde(rename = "send-update")] send_update,
	#[serde(rename = "receive-update")] receive_update,
}

impl ::core::str::FromStr for SevState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SevState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

SevState::uninit,
SevState::launch_update,
SevState::launch_secret,
SevState::running,
SevState::send_update,
SevState::receive_update,

    ];
    const NAMES: &'static [&'static str] = &[

"uninit",
"launch-update",
"launch-secret",
"running",
"send-update",
"receive-update",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SevGuestType {
	#[serde(rename = "sev")] sev,
	#[serde(rename = "sev-snp")] sev_snp,
}

impl ::core::str::FromStr for SevGuestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for SevGuestType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

SevGuestType::sev,
SevGuestType::sev_snp,

    ];
    const NAMES: &'static [&'static str] = &[

"sev",
"sev-snp",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sev {
}

impl crate::QmpCommand for query_sev { }
impl ::qapi_spec::Command for query_sev {
    const NAME: &'static str = "query-sev";
    const ALLOW_OOB: bool = false;

    type Ok = SevInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sev_launch_measure {
}

impl crate::QmpCommand for query_sev_launch_measure { }
impl ::qapi_spec::Command for query_sev_launch_measure {
    const NAME: &'static str = "query-sev-launch-measure";
    const ALLOW_OOB: bool = false;

    type Ok = SevLaunchMeasureInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sev_capabilities {
}

impl crate::QmpCommand for query_sev_capabilities { }
impl ::qapi_spec::Command for query_sev_capabilities {
    const NAME: &'static str = "query-sev-capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = SevCapability;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct sev_inject_launch_secret {
	#[serde(rename = "gpa", default, skip_serializing_if = "Option::is_none")]
pub gpa: Option<u64>,
	#[serde(rename = "packet-header")]
pub packet_header: ::std::string::String,
	#[serde(rename = "secret")]
pub secret: ::std::string::String,
}

impl crate::QmpCommand for sev_inject_launch_secret { }
impl ::qapi_spec::Command for sev_inject_launch_secret {
    const NAME: &'static str = "sev-inject-launch-secret";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sev_attestation_report {
	#[serde(rename = "mnonce")]
pub mnonce: ::std::string::String,
}

impl crate::QmpCommand for query_sev_attestation_report { }
impl ::qapi_spec::Command for query_sev_attestation_report {
    const NAME: &'static str = "query-sev-attestation-report";
    const ALLOW_OOB: bool = false;

    type Ok = SevAttestationReport;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct dump_skeys {
	#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl crate::QmpCommand for dump_skeys { }
impl ::qapi_spec::Command for dump_skeys {
    const NAME: &'static str = "dump-skeys";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_gic_capabilities {
}

impl crate::QmpCommand for query_gic_capabilities { }
impl ::qapi_spec::Command for query_gic_capabilities {
    const NAME: &'static str = "query-gic-capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<GICCapability>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sgx {
}

impl crate::QmpCommand for query_sgx { }
impl ::qapi_spec::Command for query_sgx {
    const NAME: &'static str = "query-sgx";
    const ALLOW_OOB: bool = false;

    type Ok = SGXInfo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_sgx_capabilities {
}

impl crate::QmpCommand for query_sgx_capabilities { }
impl ::qapi_spec::Command for query_sgx_capabilities {
    const NAME: &'static str = "query-sgx-capabilities";
    const ALLOW_OOB: bool = false;

    type Ok = SGXInfo;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvtchnPortType {
	#[serde(rename = "closed")] closed,
	#[serde(rename = "unbound")] unbound,
	#[serde(rename = "interdomain")] interdomain,
	#[serde(rename = "pirq")] pirq,
	#[serde(rename = "virq")] virq,
	#[serde(rename = "ipi")] ipi,
}

impl ::core::str::FromStr for EvtchnPortType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for EvtchnPortType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 6;
    const VARIANTS: &'static [Self] = &[

EvtchnPortType::closed,
EvtchnPortType::unbound,
EvtchnPortType::interdomain,
EvtchnPortType::pirq,
EvtchnPortType::virq,
EvtchnPortType::ipi,

    ];
    const NAMES: &'static [&'static str] = &[

"closed",
"unbound",
"interdomain",
"pirq",
"virq",
"ipi",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_event_list {
}

impl crate::QmpCommand for xen_event_list { }
impl ::qapi_spec::Command for xen_event_list {
    const NAME: &'static str = "xen-event-list";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<EvtchnInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct xen_event_inject {
	#[serde(rename = "port")]
pub port: u32,
}

impl crate::QmpCommand for xen_event_inject { }
impl ::qapi_spec::Command for xen_event_inject {
    const NAME: &'static str = "xen-event-inject";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioFormat {
	#[serde(rename = "u8")] u8,
	#[serde(rename = "s8")] s8,
	#[serde(rename = "u16")] u16,
	#[serde(rename = "s16")] s16,
	#[serde(rename = "u32")] u32,
	#[serde(rename = "s32")] s32,
	#[serde(rename = "f32")] f32,
}

impl ::core::str::FromStr for AudioFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for AudioFormat {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

AudioFormat::u8,
AudioFormat::s8,
AudioFormat::u16,
AudioFormat::s16,
AudioFormat::u32,
AudioFormat::s32,
AudioFormat::f32,

    ];
    const NAMES: &'static [&'static str] = &[

"u8",
"s8",
"u16",
"s16",
"u32",
"s32",
"f32",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudiodevDriver {
	#[serde(rename = "none")] none,
	#[serde(rename = "alsa")] alsa,
	#[serde(rename = "coreaudio")] coreaudio,
	#[serde(rename = "dbus")] dbus,
	#[serde(rename = "dsound")] dsound,
	#[serde(rename = "jack")] jack,
	#[serde(rename = "oss")] oss,
	#[serde(rename = "pa")] pa,
	#[serde(rename = "pipewire")] pipewire,
	#[serde(rename = "sdl")] sdl,
	#[serde(rename = "sndio")] sndio,
	#[serde(rename = "spice")] spice,
	#[serde(rename = "wav")] wav,
}

impl ::core::str::FromStr for AudiodevDriver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for AudiodevDriver {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 13;
    const VARIANTS: &'static [Self] = &[

AudiodevDriver::none,
AudiodevDriver::alsa,
AudiodevDriver::coreaudio,
AudiodevDriver::dbus,
AudiodevDriver::dsound,
AudiodevDriver::jack,
AudiodevDriver::oss,
AudiodevDriver::pa,
AudiodevDriver::pipewire,
AudiodevDriver::sdl,
AudiodevDriver::sndio,
AudiodevDriver::spice,
AudiodevDriver::wav,

    ];
    const NAMES: &'static [&'static str] = &[

"none",
"alsa",
"coreaudio",
"dbus",
"dsound",
"jack",
"oss",
"pa",
"pipewire",
"sdl",
"sndio",
"spice",
"wav",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_audiodevs {
}

impl crate::QmpCommand for query_audiodevs { }
impl ::qapi_spec::Command for query_audiodevs {
    const NAME: &'static str = "query-audiodevs";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<Audiodev>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ACPISlotType {
	#[serde(rename = "DIMM")] DIMM,
	#[serde(rename = "CPU")] CPU,
}

impl ::core::str::FromStr for ACPISlotType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for ACPISlotType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

ACPISlotType::DIMM,
ACPISlotType::CPU,

    ];
    const NAMES: &'static [&'static str] = &[

"DIMM",
"CPU",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_acpi_ospm_status {
}

impl crate::QmpCommand for query_acpi_ospm_status { }
impl ::qapi_spec::Command for query_acpi_ospm_status {
    const NAME: &'static str = "query-acpi-ospm-status";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<ACPIOSTInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACPI_DEVICE_OST {
#[serde(rename = "info")]
pub info: ACPIOSTInfo,
}

impl ::qapi_spec::Event for ACPI_DEVICE_OST {
    const NAME: &'static str = "ACPI_DEVICE_OST";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_pci {
}

impl crate::QmpCommand for query_pci { }
impl ::qapi_spec::Command for query_pci {
    const NAME: &'static str = "query-pci";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<PciInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatsType {
	#[serde(rename = "cumulative")] cumulative,
	#[serde(rename = "instant")] instant,
	#[serde(rename = "peak")] peak,
	#[serde(rename = "linear-histogram")] linear_histogram,
	#[serde(rename = "log2-histogram")] log2_histogram,
}

impl ::core::str::FromStr for StatsType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for StatsType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

StatsType::cumulative,
StatsType::instant,
StatsType::peak,
StatsType::linear_histogram,
StatsType::log2_histogram,

    ];
    const NAMES: &'static [&'static str] = &[

"cumulative",
"instant",
"peak",
"linear-histogram",
"log2-histogram",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatsUnit {
	#[serde(rename = "bytes")] bytes,
	#[serde(rename = "seconds")] seconds,
	#[serde(rename = "cycles")] cycles,
	#[serde(rename = "boolean")] boolean,
}

impl ::core::str::FromStr for StatsUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for StatsUnit {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

StatsUnit::bytes,
StatsUnit::seconds,
StatsUnit::cycles,
StatsUnit::boolean,

    ];
    const NAMES: &'static [&'static str] = &[

"bytes",
"seconds",
"cycles",
"boolean",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatsProvider {
	#[serde(rename = "kvm")] kvm,
	#[serde(rename = "cryptodev")] cryptodev,
}

impl ::core::str::FromStr for StatsProvider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for StatsProvider {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

StatsProvider::kvm,
StatsProvider::cryptodev,

    ];
    const NAMES: &'static [&'static str] = &[

"kvm",
"cryptodev",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatsTarget {
	#[serde(rename = "vm")] vm,
	#[serde(rename = "vcpu")] vcpu,
	#[serde(rename = "cryptodev")] cryptodev,
}

impl ::core::str::FromStr for StatsTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for StatsTarget {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

StatsTarget::vm,
StatsTarget::vcpu,
StatsTarget::cryptodev,

    ];
    const NAMES: &'static [&'static str] = &[

"vm",
"vcpu",
"cryptodev",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsValue {
	#[serde(rename = "boolean")] boolean(bool),
	#[serde(rename = "list")] list(Vec<u64>),
	#[serde(rename = "scalar")] scalar(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_stats(pub StatsFilter);

impl From<StatsFilter> for query_stats {
    fn from(val: StatsFilter) -> Self {
        Self(val)
    }
}


impl crate::QmpCommand for query_stats { }
impl ::qapi_spec::Command for query_stats {
    const NAME: &'static str = "query-stats";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<StatsResult>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_stats_schemas {
	#[serde(rename = "provider", default, skip_serializing_if = "Option::is_none")]
pub provider: Option<StatsProvider>,
}

impl crate::QmpCommand for query_stats_schemas { }
impl ::qapi_spec::Command for query_stats_schemas {
    const NAME: &'static str = "query-stats-schemas";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<StatsSchema>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_virtio {
}

impl crate::QmpCommand for x_query_virtio { }
impl ::qapi_spec::Command for x_query_virtio {
    const NAME: &'static str = "x-query-virtio";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<VirtioInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_virtio_status {
	#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl crate::QmpCommand for x_query_virtio_status { }
impl ::qapi_spec::Command for x_query_virtio_status {
    const NAME: &'static str = "x-query-virtio-status";
    const ALLOW_OOB: bool = false;

    type Ok = VirtioStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_virtio_queue_status {
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "queue")]
pub queue: u16,
}

impl crate::QmpCommand for x_query_virtio_queue_status { }
impl ::qapi_spec::Command for x_query_virtio_queue_status {
    const NAME: &'static str = "x-query-virtio-queue-status";
    const ALLOW_OOB: bool = false;

    type Ok = VirtQueueStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_virtio_vhost_queue_status {
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "queue")]
pub queue: u16,
}

impl crate::QmpCommand for x_query_virtio_vhost_queue_status { }
impl ::qapi_spec::Command for x_query_virtio_vhost_queue_status {
    const NAME: &'static str = "x-query-virtio-vhost-queue-status";
    const ALLOW_OOB: bool = false;

    type Ok = VirtVhostQueueStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct x_query_virtio_queue_element {
	#[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
pub index: Option<u16>,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "queue")]
pub queue: u16,
}

impl crate::QmpCommand for x_query_virtio_queue_element { }
impl ::qapi_spec::Command for x_query_virtio_queue_element {
    const NAME: &'static str = "x-query-virtio-queue-element";
    const ALLOW_OOB: bool = false;

    type Ok = VirtioQueueElement;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GranuleMode {
	#[serde(rename = "4k")] _4k,
	#[serde(rename = "8k")] _8k,
	#[serde(rename = "16k")] _16k,
	#[serde(rename = "64k")] _64k,
	#[serde(rename = "host")] host,
}

impl ::core::str::FromStr for GranuleMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for GranuleMode {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

GranuleMode::_4k,
GranuleMode::_8k,
GranuleMode::_16k,
GranuleMode::_64k,
GranuleMode::host,

    ];
    const NAMES: &'static [&'static str] = &[

"4k",
"8k",
"16k",
"64k",
"host",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VfioMigrationState {
	#[serde(rename = "stop")] stop,
	#[serde(rename = "running")] running,
	#[serde(rename = "stop-copy")] stop_copy,
	#[serde(rename = "resuming")] resuming,
	#[serde(rename = "running-p2p")] running_p2p,
	#[serde(rename = "pre-copy")] pre_copy,
	#[serde(rename = "pre-copy-p2p")] pre_copy_p2p,
}

impl ::core::str::FromStr for VfioMigrationState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for VfioMigrationState {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

VfioMigrationState::stop,
VfioMigrationState::running,
VfioMigrationState::stop_copy,
VfioMigrationState::resuming,
VfioMigrationState::running_p2p,
VfioMigrationState::pre_copy,
VfioMigrationState::pre_copy_p2p,

    ];
    const NAMES: &'static [&'static str] = &[

"stop",
"running",
"stop-copy",
"resuming",
"running-p2p",
"pre-copy",
"pre-copy-p2p",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VFIO_MIGRATION {
#[serde(rename = "device-id")]
pub device_id: ::std::string::String,
#[serde(rename = "device-state")]
pub device_state: VfioMigrationState,
#[serde(rename = "qom-path")]
pub qom_path: ::std::string::String,
}

impl ::qapi_spec::Event for VFIO_MIGRATION {
    const NAME: &'static str = "VFIO_MIGRATION";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptodevBackendAlgType {
	#[serde(rename = "sym")] sym,
	#[serde(rename = "asym")] asym,
}

impl ::core::str::FromStr for QCryptodevBackendAlgType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptodevBackendAlgType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

QCryptodevBackendAlgType::sym,
QCryptodevBackendAlgType::asym,

    ];
    const NAMES: &'static [&'static str] = &[

"sym",
"asym",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptodevBackendServiceType {
	#[serde(rename = "cipher")] cipher,
	#[serde(rename = "hash")] hash,
	#[serde(rename = "mac")] mac,
	#[serde(rename = "aead")] aead,
	#[serde(rename = "akcipher")] akcipher,
}

impl ::core::str::FromStr for QCryptodevBackendServiceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptodevBackendServiceType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 5;
    const VARIANTS: &'static [Self] = &[

QCryptodevBackendServiceType::cipher,
QCryptodevBackendServiceType::hash,
QCryptodevBackendServiceType::mac,
QCryptodevBackendServiceType::aead,
QCryptodevBackendServiceType::akcipher,

    ];
    const NAMES: &'static [&'static str] = &[

"cipher",
"hash",
"mac",
"aead",
"akcipher",

    ];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QCryptodevBackendType {
	#[serde(rename = "builtin")] builtin,
	#[serde(rename = "vhost-user")] vhost_user,
	#[serde(rename = "lkcf")] lkcf,
}

impl ::core::str::FromStr for QCryptodevBackendType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for QCryptodevBackendType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[

QCryptodevBackendType::builtin,
QCryptodevBackendType::vhost_user,
QCryptodevBackendType::lkcf,

    ];
    const NAMES: &'static [&'static str] = &[

"builtin",
"vhost-user",
"lkcf",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct query_cryptodev {
}

impl crate::QmpCommand for query_cryptodev { }
impl ::qapi_spec::Command for query_cryptodev {
    const NAME: &'static str = "query-cryptodev";
    const ALLOW_OOB: bool = false;

    type Ok = Vec<QCryptodevInfo>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CxlEventLog {
	#[serde(rename = "informational")] informational,
	#[serde(rename = "warning")] warning,
	#[serde(rename = "failure")] failure,
	#[serde(rename = "fatal")] fatal,
}

impl ::core::str::FromStr for CxlEventLog {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CxlEventLog {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

CxlEventLog::informational,
CxlEventLog::warning,
CxlEventLog::failure,
CxlEventLog::fatal,

    ];
    const NAMES: &'static [&'static str] = &[

"informational",
"warning",
"failure",
"fatal",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_general_media_event {
	#[serde(rename = "channel", default, skip_serializing_if = "Option::is_none")]
pub channel: Option<u8>,
	#[serde(rename = "component-id", default, skip_serializing_if = "Option::is_none")]
pub component_id: Option<::std::string::String>,
	#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<u32>,
	#[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
pub rank: Option<u8>,
	#[serde(rename = "descriptor")]
pub descriptor: u8,
	#[serde(rename = "dpa")]
pub dpa: u64,
	#[serde(rename = "flags")]
pub flags: u8,
	#[serde(rename = "log")]
pub log: CxlEventLog,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "transaction-type")]
pub transaction_type: u8,
	#[serde(rename = "type")]
pub type_: u8,
}

impl crate::QmpCommand for cxl_inject_general_media_event { }
impl ::qapi_spec::Command for cxl_inject_general_media_event {
    const NAME: &'static str = "cxl-inject-general-media-event";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_dram_event {
	#[serde(rename = "bank", default, skip_serializing_if = "Option::is_none")]
pub bank: Option<u8>,
	#[serde(rename = "bank-group", default, skip_serializing_if = "Option::is_none")]
pub bank_group: Option<u8>,
	#[serde(rename = "channel", default, skip_serializing_if = "Option::is_none")]
pub channel: Option<u8>,
	#[serde(rename = "column", default, skip_serializing_if = "Option::is_none")]
pub column: Option<u16>,
	#[serde(rename = "correction-mask", default, skip_serializing_if = "Option::is_none")]
pub correction_mask: Option<Vec<u64>>,
	#[serde(rename = "nibble-mask", default, skip_serializing_if = "Option::is_none")]
pub nibble_mask: Option<u32>,
	#[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
pub rank: Option<u8>,
	#[serde(rename = "row", default, skip_serializing_if = "Option::is_none")]
pub row: Option<u32>,
	#[serde(rename = "descriptor")]
pub descriptor: u8,
	#[serde(rename = "dpa")]
pub dpa: u64,
	#[serde(rename = "flags")]
pub flags: u8,
	#[serde(rename = "log")]
pub log: CxlEventLog,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "transaction-type")]
pub transaction_type: u8,
	#[serde(rename = "type")]
pub type_: u8,
}

impl crate::QmpCommand for cxl_inject_dram_event { }
impl ::qapi_spec::Command for cxl_inject_dram_event {
    const NAME: &'static str = "cxl-inject-dram-event";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_memory_module_event {
	#[serde(rename = "additional-status")]
pub additional_status: u8,
	#[serde(rename = "corrected-persistent-error-count")]
pub corrected_persistent_error_count: u32,
	#[serde(rename = "corrected-volatile-error-count")]
pub corrected_volatile_error_count: u32,
	#[serde(rename = "dirty-shutdown-count")]
pub dirty_shutdown_count: u32,
	#[serde(rename = "flags")]
pub flags: u8,
	#[serde(rename = "health-status")]
pub health_status: u8,
	#[serde(rename = "life-used")]
pub life_used: u8,
	#[serde(rename = "log")]
pub log: CxlEventLog,
	#[serde(rename = "media-status")]
pub media_status: u8,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "temperature")]
pub temperature: i16,
	#[serde(rename = "type")]
pub type_: u8,
}

impl crate::QmpCommand for cxl_inject_memory_module_event { }
impl ::qapi_spec::Command for cxl_inject_memory_module_event {
    const NAME: &'static str = "cxl-inject-memory-module-event";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_poison {
	#[serde(rename = "length")]
pub length: u64,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "start")]
pub start: u64,
}

impl crate::QmpCommand for cxl_inject_poison { }
impl ::qapi_spec::Command for cxl_inject_poison {
    const NAME: &'static str = "cxl-inject-poison";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CxlUncorErrorType {
	#[serde(rename = "cache-data-parity")] cache_data_parity,
	#[serde(rename = "cache-address-parity")] cache_address_parity,
	#[serde(rename = "cache-be-parity")] cache_be_parity,
	#[serde(rename = "cache-data-ecc")] cache_data_ecc,
	#[serde(rename = "mem-data-parity")] mem_data_parity,
	#[serde(rename = "mem-address-parity")] mem_address_parity,
	#[serde(rename = "mem-be-parity")] mem_be_parity,
	#[serde(rename = "mem-data-ecc")] mem_data_ecc,
	#[serde(rename = "reinit-threshold")] reinit_threshold,
	#[serde(rename = "rsvd-encoding")] rsvd_encoding,
	#[serde(rename = "poison-received")] poison_received,
	#[serde(rename = "receiver-overflow")] receiver_overflow,
	#[serde(rename = "internal")] internal,
	#[serde(rename = "cxl-ide-tx")] cxl_ide_tx,
	#[serde(rename = "cxl-ide-rx")] cxl_ide_rx,
}

impl ::core::str::FromStr for CxlUncorErrorType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CxlUncorErrorType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 15;
    const VARIANTS: &'static [Self] = &[

CxlUncorErrorType::cache_data_parity,
CxlUncorErrorType::cache_address_parity,
CxlUncorErrorType::cache_be_parity,
CxlUncorErrorType::cache_data_ecc,
CxlUncorErrorType::mem_data_parity,
CxlUncorErrorType::mem_address_parity,
CxlUncorErrorType::mem_be_parity,
CxlUncorErrorType::mem_data_ecc,
CxlUncorErrorType::reinit_threshold,
CxlUncorErrorType::rsvd_encoding,
CxlUncorErrorType::poison_received,
CxlUncorErrorType::receiver_overflow,
CxlUncorErrorType::internal,
CxlUncorErrorType::cxl_ide_tx,
CxlUncorErrorType::cxl_ide_rx,

    ];
    const NAMES: &'static [&'static str] = &[

"cache-data-parity",
"cache-address-parity",
"cache-be-parity",
"cache-data-ecc",
"mem-data-parity",
"mem-address-parity",
"mem-be-parity",
"mem-data-ecc",
"reinit-threshold",
"rsvd-encoding",
"poison-received",
"receiver-overflow",
"internal",
"cxl-ide-tx",
"cxl-ide-rx",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_uncorrectable_errors {
	#[serde(rename = "errors")]
pub errors: Vec<CXLUncorErrorRecord>,
	#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl crate::QmpCommand for cxl_inject_uncorrectable_errors { }
impl ::qapi_spec::Command for cxl_inject_uncorrectable_errors {
    const NAME: &'static str = "cxl-inject-uncorrectable-errors";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CxlCorErrorType {
	#[serde(rename = "cache-data-ecc")] cache_data_ecc,
	#[serde(rename = "mem-data-ecc")] mem_data_ecc,
	#[serde(rename = "crc-threshold")] crc_threshold,
	#[serde(rename = "retry-threshold")] retry_threshold,
	#[serde(rename = "cache-poison-received")] cache_poison_received,
	#[serde(rename = "mem-poison-received")] mem_poison_received,
	#[serde(rename = "physical")] physical,
}

impl ::core::str::FromStr for CxlCorErrorType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CxlCorErrorType {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 7;
    const VARIANTS: &'static [Self] = &[

CxlCorErrorType::cache_data_ecc,
CxlCorErrorType::mem_data_ecc,
CxlCorErrorType::crc_threshold,
CxlCorErrorType::retry_threshold,
CxlCorErrorType::cache_poison_received,
CxlCorErrorType::mem_poison_received,
CxlCorErrorType::physical,

    ];
    const NAMES: &'static [&'static str] = &[

"cache-data-ecc",
"mem-data-ecc",
"crc-threshold",
"retry-threshold",
"cache-poison-received",
"mem-poison-received",
"physical",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_inject_correctable_error {
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "type")]
pub type_: CxlCorErrorType,
}

impl crate::QmpCommand for cxl_inject_correctable_error { }
impl ::qapi_spec::Command for cxl_inject_correctable_error {
    const NAME: &'static str = "cxl-inject-correctable-error";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CxlExtentSelectionPolicy {
	#[serde(rename = "free")] free,
	#[serde(rename = "contiguous")] contiguous,
	#[serde(rename = "prescriptive")] prescriptive,
	#[serde(rename = "enable-shared-access")] enable_shared_access,
}

impl ::core::str::FromStr for CxlExtentSelectionPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CxlExtentSelectionPolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 4;
    const VARIANTS: &'static [Self] = &[

CxlExtentSelectionPolicy::free,
CxlExtentSelectionPolicy::contiguous,
CxlExtentSelectionPolicy::prescriptive,
CxlExtentSelectionPolicy::enable_shared_access,

    ];
    const NAMES: &'static [&'static str] = &[

"free",
"contiguous",
"prescriptive",
"enable-shared-access",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_add_dynamic_capacity {
	#[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
pub tag: Option<::std::string::String>,
	#[serde(rename = "extents")]
pub extents: Vec<CxlDynamicCapacityExtent>,
	#[serde(rename = "host-id")]
pub host_id: u16,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "region")]
pub region: u8,
	#[serde(rename = "selection-policy")]
pub selection_policy: CxlExtentSelectionPolicy,
}

impl crate::QmpCommand for cxl_add_dynamic_capacity { }
impl ::qapi_spec::Command for cxl_add_dynamic_capacity {
    const NAME: &'static str = "cxl-add-dynamic-capacity";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CxlExtentRemovalPolicy {
	#[serde(rename = "tag-based")] tag_based,
	#[serde(rename = "prescriptive")] prescriptive,
}

impl ::core::str::FromStr for CxlExtentRemovalPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::qapi_spec::Enum::from_name(s).ok_or(())
    }
}

unsafe impl ::qapi_spec::Enum for CxlExtentRemovalPolicy {
    fn discriminant(&self) -> usize { *self as usize }

    const COUNT: usize = 2;
    const VARIANTS: &'static [Self] = &[

CxlExtentRemovalPolicy::tag_based,
CxlExtentRemovalPolicy::prescriptive,

    ];
    const NAMES: &'static [&'static str] = &[

"tag-based",
"prescriptive",

    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cxl_release_dynamic_capacity {
	#[serde(rename = "forced-removal", default, skip_serializing_if = "Option::is_none")]
pub forced_removal: Option<bool>,
	#[serde(rename = "sanitize-on-release", default, skip_serializing_if = "Option::is_none")]
pub sanitize_on_release: Option<bool>,
	#[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
pub tag: Option<::std::string::String>,
	#[serde(rename = "extents")]
pub extents: Vec<CxlDynamicCapacityExtent>,
	#[serde(rename = "host-id")]
pub host_id: u16,
	#[serde(rename = "path")]
pub path: ::std::string::String,
	#[serde(rename = "region")]
pub region: u8,
	#[serde(rename = "removal-policy")]
pub removal_policy: CxlExtentRemovalPolicy,
}

impl crate::QmpCommand for cxl_release_dynamic_capacity { }
impl ::qapi_spec::Command for cxl_release_dynamic_capacity {
    const NAME: &'static str = "cxl-release-dynamic-capacity";
    const ALLOW_OOB: bool = false;

    type Ok = ::qapi_spec::Empty;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "driver")]
pub enum Audiodev {
	#[serde(rename = "alsa")]
	alsa {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "alsa")]
alsa: AudiodevAlsaOptions,
	},
	#[serde(rename = "coreaudio")]
	coreaudio {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "coreaudio")]
coreaudio: AudiodevCoreaudioOptions,
	},
	#[serde(rename = "dbus")]
	dbus {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "dbus")]
dbus: AudiodevGenericOptions,
	},
	#[serde(rename = "dsound")]
	dsound {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "dsound")]
dsound: AudiodevDsoundOptions,
	},
	#[serde(rename = "jack")]
	jack {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "jack")]
jack: AudiodevJackOptions,
	},
	#[serde(rename = "none")]
	none {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "none")]
none: AudiodevGenericOptions,
	},
	#[serde(rename = "oss")]
	oss {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "oss")]
oss: AudiodevOssOptions,
	},
	#[serde(rename = "pa")]
	pa {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "pa")]
pa: AudiodevPaOptions,
	},
	#[serde(rename = "pipewire")]
	pipewire {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "pipewire")]
pipewire: AudiodevPipewireOptions,
	},
	#[serde(rename = "sdl")]
	sdl {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "sdl")]
sdl: AudiodevSdlOptions,
	},
	#[serde(rename = "sndio")]
	sndio {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "sndio")]
sndio: AudiodevSndioOptions,
	},
	#[serde(rename = "spice")]
	spice {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "spice")]
spice: AudiodevGenericOptions,
	},
	#[serde(rename = "wav")]
	wav {
		#[serde(flatten)] #[serde(rename = "base")]
base: AudiodevBase,
		#[serde(flatten)] #[serde(rename = "wav")]
wav: AudiodevWavOptions,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevBase {
	#[serde(rename = "timer-period", default, skip_serializing_if = "Option::is_none")]
pub timer_period: Option<u32>,
	#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl Audiodev {
    pub fn driver(&self) -> AudiodevDriver {
        match *self {

            Audiodev::alsa { .. } => AudiodevDriver::alsa,

            Audiodev::coreaudio { .. } => AudiodevDriver::coreaudio,

            Audiodev::dbus { .. } => AudiodevDriver::dbus,

            Audiodev::dsound { .. } => AudiodevDriver::dsound,

            Audiodev::jack { .. } => AudiodevDriver::jack,

            Audiodev::none { .. } => AudiodevDriver::none,

            Audiodev::oss { .. } => AudiodevDriver::oss,

            Audiodev::pa { .. } => AudiodevDriver::pa,

            Audiodev::pipewire { .. } => AudiodevDriver::pipewire,

            Audiodev::sdl { .. } => AudiodevDriver::sdl,

            Audiodev::sndio { .. } => AudiodevDriver::sndio,

            Audiodev::spice { .. } => AudiodevDriver::spice,

            Audiodev::wav { .. } => AudiodevDriver::wav,

        }
    }
}

impl From<(AudiodevAlsaOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevAlsaOptions, AudiodevBase)) -> Self {
        Self::alsa {
            alsa: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevCoreaudioOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevCoreaudioOptions, AudiodevBase)) -> Self {
        Self::coreaudio {
            coreaudio: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevDsoundOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevDsoundOptions, AudiodevBase)) -> Self {
        Self::dsound {
            dsound: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevJackOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevJackOptions, AudiodevBase)) -> Self {
        Self::jack {
            jack: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevOssOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevOssOptions, AudiodevBase)) -> Self {
        Self::oss {
            oss: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevPaOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevPaOptions, AudiodevBase)) -> Self {
        Self::pa {
            pa: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevPipewireOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevPipewireOptions, AudiodevBase)) -> Self {
        Self::pipewire {
            pipewire: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevSdlOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevSdlOptions, AudiodevBase)) -> Self {
        Self::sdl {
            sdl: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevSndioOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevSndioOptions, AudiodevBase)) -> Self {
        Self::sndio {
            sndio: val.0,
            base: val.1,

        }
    }
}

impl From<(AudiodevWavOptions, AudiodevBase)> for Audiodev {
    fn from(val: (AudiodevWavOptions, AudiodevBase)) -> Self {
        Self::wav {
            wav: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BlockExportOptions {
	#[serde(rename = "fuse")]
	fuse {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockExportOptionsBase,
		#[serde(flatten)] #[serde(rename = "fuse")]
fuse: BlockExportOptionsFuse,
	},
	#[serde(rename = "nbd")]
	nbd {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockExportOptionsBase,
		#[serde(flatten)] #[serde(rename = "nbd")]
nbd: BlockExportOptionsNbd,
	},
	#[serde(rename = "vduse-blk")]
	vduse_blk {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockExportOptionsBase,
		#[serde(flatten)] #[serde(rename = "vduse-blk")]
vduse_blk: BlockExportOptionsVduseBlk,
	},
	#[serde(rename = "vhost-user-blk")]
	vhost_user_blk {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockExportOptionsBase,
		#[serde(flatten)] #[serde(rename = "vhost-user-blk")]
vhost_user_blk: BlockExportOptionsVhostUserBlk,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportOptionsBase {
	#[serde(rename = "fixed-iothread", default, skip_serializing_if = "Option::is_none")]
pub fixed_iothread: Option<bool>,
	#[serde(rename = "iothread", default, skip_serializing_if = "Option::is_none")]
pub iothread: Option<::std::string::String>,
	#[serde(rename = "writable", default, skip_serializing_if = "Option::is_none")]
pub writable: Option<bool>,
	#[serde(rename = "writethrough", default, skip_serializing_if = "Option::is_none")]
pub writethrough: Option<bool>,
	#[serde(rename = "id")]
pub id: ::std::string::String,
	#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

impl BlockExportOptions {
    pub fn type_(&self) -> BlockExportType {
        match *self {

            BlockExportOptions::fuse { .. } => BlockExportType::fuse,

            BlockExportOptions::nbd { .. } => BlockExportType::nbd,

            BlockExportOptions::vduse_blk { .. } => BlockExportType::vduse_blk,

            BlockExportOptions::vhost_user_blk { .. } => BlockExportType::vhost_user_blk,

        }
    }
}

impl From<(BlockExportOptionsFuse, BlockExportOptionsBase)> for BlockExportOptions {
    fn from(val: (BlockExportOptionsFuse, BlockExportOptionsBase)) -> Self {
        Self::fuse {
            fuse: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockExportOptionsNbd, BlockExportOptionsBase)> for BlockExportOptions {
    fn from(val: (BlockExportOptionsNbd, BlockExportOptionsBase)) -> Self {
        Self::nbd {
            nbd: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockExportOptionsVduseBlk, BlockExportOptionsBase)> for BlockExportOptions {
    fn from(val: (BlockExportOptionsVduseBlk, BlockExportOptionsBase)) -> Self {
        Self::vduse_blk {
            vduse_blk: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockExportOptionsVhostUserBlk, BlockExportOptionsBase)> for BlockExportOptions {
    fn from(val: (BlockExportOptionsVhostUserBlk, BlockExportOptionsBase)) -> Self {
        Self::vhost_user_blk {
            vhost_user_blk: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BlockJobChangeOptions {
	#[serde(rename = "mirror")]
	mirror {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "mirror")]
mirror: BlockJobChangeOptionsMirror,
	},
	#[serde(rename = "commit")]
	commit(::std::string::String),
	#[serde(rename = "stream")]
	stream(::std::string::String),
	#[serde(rename = "backup")]
	backup(::std::string::String),
	#[serde(rename = "create")]
	create(::std::string::String),
	#[serde(rename = "amend")]
	amend(::std::string::String),
	#[serde(rename = "snapshot-load")]
	snapshot_load(::std::string::String),
	#[serde(rename = "snapshot-save")]
	snapshot_save(::std::string::String),
	#[serde(rename = "snapshot-delete")]
	snapshot_delete(::std::string::String),
}

impl BlockJobChangeOptions {
    pub fn type_(&self) -> JobType {
        match *self {

            BlockJobChangeOptions::mirror { .. } => JobType::mirror,

            BlockJobChangeOptions::commit { .. } => JobType::commit,

            BlockJobChangeOptions::stream { .. } => JobType::stream,

            BlockJobChangeOptions::backup { .. } => JobType::backup,

            BlockJobChangeOptions::create { .. } => JobType::create,

            BlockJobChangeOptions::amend { .. } => JobType::amend,

            BlockJobChangeOptions::snapshot_load { .. } => JobType::snapshot_load,

            BlockJobChangeOptions::snapshot_save { .. } => JobType::snapshot_save,

            BlockJobChangeOptions::snapshot_delete { .. } => JobType::snapshot_delete,

        }
    }
}

impl From<(BlockJobChangeOptionsMirror, ::std::string::String)> for BlockJobChangeOptions {
    fn from(val: (BlockJobChangeOptionsMirror, ::std::string::String)) -> Self {
        Self::mirror {
            mirror: val.0,
            id: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BlockJobInfo {
	#[serde(rename = "mirror")]
	mirror {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockJobInfoBase,
		#[serde(flatten)] #[serde(rename = "mirror")]
mirror: BlockJobInfoMirror,
	},
	#[serde(rename = "commit")]
	commit(BlockJobInfoBase),
	#[serde(rename = "stream")]
	stream(BlockJobInfoBase),
	#[serde(rename = "backup")]
	backup(BlockJobInfoBase),
	#[serde(rename = "create")]
	create(BlockJobInfoBase),
	#[serde(rename = "amend")]
	amend(BlockJobInfoBase),
	#[serde(rename = "snapshot-load")]
	snapshot_load(BlockJobInfoBase),
	#[serde(rename = "snapshot-save")]
	snapshot_save(BlockJobInfoBase),
	#[serde(rename = "snapshot-delete")]
	snapshot_delete(BlockJobInfoBase),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockJobInfoBase {
	#[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
pub error: Option<::std::string::String>,
	#[serde(rename = "auto-dismiss")]
pub auto_dismiss: bool,
	#[serde(rename = "auto-finalize")]
pub auto_finalize: bool,
	#[serde(rename = "busy")]
pub busy: bool,
	#[serde(rename = "device")]
pub device: ::std::string::String,
	#[serde(rename = "io-status")]
pub io_status: BlockDeviceIoStatus,
	#[serde(rename = "len")]
pub len: i64,
	#[serde(rename = "offset")]
pub offset: i64,
	#[serde(rename = "paused")]
pub paused: bool,
	#[serde(rename = "ready")]
pub ready: bool,
	#[serde(rename = "speed")]
pub speed: i64,
	#[serde(rename = "status")]
pub status: JobStatus,
}

impl BlockJobInfo {
    pub fn type_(&self) -> JobType {
        match *self {

            BlockJobInfo::mirror { .. } => JobType::mirror,

            BlockJobInfo::commit { .. } => JobType::commit,

            BlockJobInfo::stream { .. } => JobType::stream,

            BlockJobInfo::backup { .. } => JobType::backup,

            BlockJobInfo::create { .. } => JobType::create,

            BlockJobInfo::amend { .. } => JobType::amend,

            BlockJobInfo::snapshot_load { .. } => JobType::snapshot_load,

            BlockJobInfo::snapshot_save { .. } => JobType::snapshot_save,

            BlockJobInfo::snapshot_delete { .. } => JobType::snapshot_delete,

        }
    }
}

impl From<(BlockJobInfoMirror, BlockJobInfoBase)> for BlockJobInfo {
    fn from(val: (BlockJobInfoMirror, BlockJobInfoBase)) -> Self {
        Self::mirror {
            mirror: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "driver")]
pub enum BlockStatsSpecific {
	#[serde(rename = "file")]
	file(BlockStatsSpecificFile),
	#[serde(rename = "host_device")]
	host_device(BlockStatsSpecificFile),
	#[serde(rename = "nvme")]
	nvme(BlockStatsSpecificNvme),
	#[serde(rename = "blkdebug")]
	blkdebug,
	#[serde(rename = "blklogwrites")]
	blklogwrites,
	#[serde(rename = "blkreplay")]
	blkreplay,
	#[serde(rename = "blkverify")]
	blkverify,
	#[serde(rename = "bochs")]
	bochs,
	#[serde(rename = "cloop")]
	cloop,
	#[serde(rename = "compress")]
	compress,
	#[serde(rename = "copy-before-write")]
	copy_before_write,
	#[serde(rename = "copy-on-read")]
	copy_on_read,
	#[serde(rename = "dmg")]
	dmg,
	#[serde(rename = "snapshot-access")]
	snapshot_access,
	#[serde(rename = "ftp")]
	ftp,
	#[serde(rename = "ftps")]
	ftps,
	#[serde(rename = "gluster")]
	gluster,
	#[serde(rename = "host_cdrom")]
	host_cdrom,
	#[serde(rename = "http")]
	http,
	#[serde(rename = "https")]
	https,
	#[serde(rename = "io_uring")]
	io_uring,
	#[serde(rename = "iscsi")]
	iscsi,
	#[serde(rename = "luks")]
	luks,
	#[serde(rename = "nbd")]
	nbd,
	#[serde(rename = "nfs")]
	nfs,
	#[serde(rename = "null-aio")]
	null_aio,
	#[serde(rename = "null-co")]
	null_co,
	#[serde(rename = "nvme-io_uring")]
	nvme_io_uring,
	#[serde(rename = "parallels")]
	parallels,
	#[serde(rename = "preallocate")]
	preallocate,
	#[serde(rename = "qcow")]
	qcow,
	#[serde(rename = "qcow2")]
	qcow2,
	#[serde(rename = "qed")]
	qed,
	#[serde(rename = "quorum")]
	quorum,
	#[serde(rename = "raw")]
	raw,
	#[serde(rename = "rbd")]
	rbd,
	#[serde(rename = "replication")]
	replication,
	#[serde(rename = "ssh")]
	ssh,
	#[serde(rename = "throttle")]
	throttle,
	#[serde(rename = "vdi")]
	vdi,
	#[serde(rename = "vhdx")]
	vhdx,
	#[serde(rename = "virtio-blk-vfio-pci")]
	virtio_blk_vfio_pci,
	#[serde(rename = "virtio-blk-vhost-user")]
	virtio_blk_vhost_user,
	#[serde(rename = "virtio-blk-vhost-vdpa")]
	virtio_blk_vhost_vdpa,
	#[serde(rename = "vmdk")]
	vmdk,
	#[serde(rename = "vpc")]
	vpc,
	#[serde(rename = "vvfat")]
	vvfat,
}

impl BlockStatsSpecific {
    pub fn driver(&self) -> BlockdevDriver {
        match *self {

            BlockStatsSpecific::file { .. } => BlockdevDriver::file,

            BlockStatsSpecific::host_device { .. } => BlockdevDriver::host_device,

            BlockStatsSpecific::nvme { .. } => BlockdevDriver::nvme,

            BlockStatsSpecific::blkdebug { .. } => BlockdevDriver::blkdebug,

            BlockStatsSpecific::blklogwrites { .. } => BlockdevDriver::blklogwrites,

            BlockStatsSpecific::blkreplay { .. } => BlockdevDriver::blkreplay,

            BlockStatsSpecific::blkverify { .. } => BlockdevDriver::blkverify,

            BlockStatsSpecific::bochs { .. } => BlockdevDriver::bochs,

            BlockStatsSpecific::cloop { .. } => BlockdevDriver::cloop,

            BlockStatsSpecific::compress { .. } => BlockdevDriver::compress,

            BlockStatsSpecific::copy_before_write { .. } => BlockdevDriver::copy_before_write,

            BlockStatsSpecific::copy_on_read { .. } => BlockdevDriver::copy_on_read,

            BlockStatsSpecific::dmg { .. } => BlockdevDriver::dmg,

            BlockStatsSpecific::snapshot_access { .. } => BlockdevDriver::snapshot_access,

            BlockStatsSpecific::ftp { .. } => BlockdevDriver::ftp,

            BlockStatsSpecific::ftps { .. } => BlockdevDriver::ftps,

            BlockStatsSpecific::gluster { .. } => BlockdevDriver::gluster,

            BlockStatsSpecific::host_cdrom { .. } => BlockdevDriver::host_cdrom,

            BlockStatsSpecific::http { .. } => BlockdevDriver::http,

            BlockStatsSpecific::https { .. } => BlockdevDriver::https,

            BlockStatsSpecific::io_uring { .. } => BlockdevDriver::io_uring,

            BlockStatsSpecific::iscsi { .. } => BlockdevDriver::iscsi,

            BlockStatsSpecific::luks { .. } => BlockdevDriver::luks,

            BlockStatsSpecific::nbd { .. } => BlockdevDriver::nbd,

            BlockStatsSpecific::nfs { .. } => BlockdevDriver::nfs,

            BlockStatsSpecific::null_aio { .. } => BlockdevDriver::null_aio,

            BlockStatsSpecific::null_co { .. } => BlockdevDriver::null_co,

            BlockStatsSpecific::nvme_io_uring { .. } => BlockdevDriver::nvme_io_uring,

            BlockStatsSpecific::parallels { .. } => BlockdevDriver::parallels,

            BlockStatsSpecific::preallocate { .. } => BlockdevDriver::preallocate,

            BlockStatsSpecific::qcow { .. } => BlockdevDriver::qcow,

            BlockStatsSpecific::qcow2 { .. } => BlockdevDriver::qcow2,

            BlockStatsSpecific::qed { .. } => BlockdevDriver::qed,

            BlockStatsSpecific::quorum { .. } => BlockdevDriver::quorum,

            BlockStatsSpecific::raw { .. } => BlockdevDriver::raw,

            BlockStatsSpecific::rbd { .. } => BlockdevDriver::rbd,

            BlockStatsSpecific::replication { .. } => BlockdevDriver::replication,

            BlockStatsSpecific::ssh { .. } => BlockdevDriver::ssh,

            BlockStatsSpecific::throttle { .. } => BlockdevDriver::throttle,

            BlockStatsSpecific::vdi { .. } => BlockdevDriver::vdi,

            BlockStatsSpecific::vhdx { .. } => BlockdevDriver::vhdx,

            BlockStatsSpecific::virtio_blk_vfio_pci { .. } => BlockdevDriver::virtio_blk_vfio_pci,

            BlockStatsSpecific::virtio_blk_vhost_user { .. } => BlockdevDriver::virtio_blk_vhost_user,

            BlockStatsSpecific::virtio_blk_vhost_vdpa { .. } => BlockdevDriver::virtio_blk_vhost_vdpa,

            BlockStatsSpecific::vmdk { .. } => BlockdevDriver::vmdk,

            BlockStatsSpecific::vpc { .. } => BlockdevDriver::vpc,

            BlockStatsSpecific::vvfat { .. } => BlockdevDriver::vvfat,

        }
    }
}

impl From<BlockStatsSpecificNvme> for BlockStatsSpecific {
    fn from(val: BlockStatsSpecificNvme) -> Self {
        Self::nvme(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "driver")]
pub enum BlockdevAmendOptions {
	#[serde(rename = "luks")]
	luks(BlockdevAmendOptionsLUKS),
	#[serde(rename = "qcow2")]
	qcow2(BlockdevAmendOptionsQcow2),
	#[serde(rename = "blkdebug")]
	blkdebug,
	#[serde(rename = "blklogwrites")]
	blklogwrites,
	#[serde(rename = "blkreplay")]
	blkreplay,
	#[serde(rename = "blkverify")]
	blkverify,
	#[serde(rename = "bochs")]
	bochs,
	#[serde(rename = "cloop")]
	cloop,
	#[serde(rename = "compress")]
	compress,
	#[serde(rename = "copy-before-write")]
	copy_before_write,
	#[serde(rename = "copy-on-read")]
	copy_on_read,
	#[serde(rename = "dmg")]
	dmg,
	#[serde(rename = "file")]
	file,
	#[serde(rename = "snapshot-access")]
	snapshot_access,
	#[serde(rename = "ftp")]
	ftp,
	#[serde(rename = "ftps")]
	ftps,
	#[serde(rename = "gluster")]
	gluster,
	#[serde(rename = "host_cdrom")]
	host_cdrom,
	#[serde(rename = "host_device")]
	host_device,
	#[serde(rename = "http")]
	http,
	#[serde(rename = "https")]
	https,
	#[serde(rename = "io_uring")]
	io_uring,
	#[serde(rename = "iscsi")]
	iscsi,
	#[serde(rename = "nbd")]
	nbd,
	#[serde(rename = "nfs")]
	nfs,
	#[serde(rename = "null-aio")]
	null_aio,
	#[serde(rename = "null-co")]
	null_co,
	#[serde(rename = "nvme")]
	nvme,
	#[serde(rename = "nvme-io_uring")]
	nvme_io_uring,
	#[serde(rename = "parallels")]
	parallels,
	#[serde(rename = "preallocate")]
	preallocate,
	#[serde(rename = "qcow")]
	qcow,
	#[serde(rename = "qed")]
	qed,
	#[serde(rename = "quorum")]
	quorum,
	#[serde(rename = "raw")]
	raw,
	#[serde(rename = "rbd")]
	rbd,
	#[serde(rename = "replication")]
	replication,
	#[serde(rename = "ssh")]
	ssh,
	#[serde(rename = "throttle")]
	throttle,
	#[serde(rename = "vdi")]
	vdi,
	#[serde(rename = "vhdx")]
	vhdx,
	#[serde(rename = "virtio-blk-vfio-pci")]
	virtio_blk_vfio_pci,
	#[serde(rename = "virtio-blk-vhost-user")]
	virtio_blk_vhost_user,
	#[serde(rename = "virtio-blk-vhost-vdpa")]
	virtio_blk_vhost_vdpa,
	#[serde(rename = "vmdk")]
	vmdk,
	#[serde(rename = "vpc")]
	vpc,
	#[serde(rename = "vvfat")]
	vvfat,
}

impl BlockdevAmendOptions {
    pub fn driver(&self) -> BlockdevDriver {
        match *self {

            BlockdevAmendOptions::luks { .. } => BlockdevDriver::luks,

            BlockdevAmendOptions::qcow2 { .. } => BlockdevDriver::qcow2,

            BlockdevAmendOptions::blkdebug { .. } => BlockdevDriver::blkdebug,

            BlockdevAmendOptions::blklogwrites { .. } => BlockdevDriver::blklogwrites,

            BlockdevAmendOptions::blkreplay { .. } => BlockdevDriver::blkreplay,

            BlockdevAmendOptions::blkverify { .. } => BlockdevDriver::blkverify,

            BlockdevAmendOptions::bochs { .. } => BlockdevDriver::bochs,

            BlockdevAmendOptions::cloop { .. } => BlockdevDriver::cloop,

            BlockdevAmendOptions::compress { .. } => BlockdevDriver::compress,

            BlockdevAmendOptions::copy_before_write { .. } => BlockdevDriver::copy_before_write,

            BlockdevAmendOptions::copy_on_read { .. } => BlockdevDriver::copy_on_read,

            BlockdevAmendOptions::dmg { .. } => BlockdevDriver::dmg,

            BlockdevAmendOptions::file { .. } => BlockdevDriver::file,

            BlockdevAmendOptions::snapshot_access { .. } => BlockdevDriver::snapshot_access,

            BlockdevAmendOptions::ftp { .. } => BlockdevDriver::ftp,

            BlockdevAmendOptions::ftps { .. } => BlockdevDriver::ftps,

            BlockdevAmendOptions::gluster { .. } => BlockdevDriver::gluster,

            BlockdevAmendOptions::host_cdrom { .. } => BlockdevDriver::host_cdrom,

            BlockdevAmendOptions::host_device { .. } => BlockdevDriver::host_device,

            BlockdevAmendOptions::http { .. } => BlockdevDriver::http,

            BlockdevAmendOptions::https { .. } => BlockdevDriver::https,

            BlockdevAmendOptions::io_uring { .. } => BlockdevDriver::io_uring,

            BlockdevAmendOptions::iscsi { .. } => BlockdevDriver::iscsi,

            BlockdevAmendOptions::nbd { .. } => BlockdevDriver::nbd,

            BlockdevAmendOptions::nfs { .. } => BlockdevDriver::nfs,

            BlockdevAmendOptions::null_aio { .. } => BlockdevDriver::null_aio,

            BlockdevAmendOptions::null_co { .. } => BlockdevDriver::null_co,

            BlockdevAmendOptions::nvme { .. } => BlockdevDriver::nvme,

            BlockdevAmendOptions::nvme_io_uring { .. } => BlockdevDriver::nvme_io_uring,

            BlockdevAmendOptions::parallels { .. } => BlockdevDriver::parallels,

            BlockdevAmendOptions::preallocate { .. } => BlockdevDriver::preallocate,

            BlockdevAmendOptions::qcow { .. } => BlockdevDriver::qcow,

            BlockdevAmendOptions::qed { .. } => BlockdevDriver::qed,

            BlockdevAmendOptions::quorum { .. } => BlockdevDriver::quorum,

            BlockdevAmendOptions::raw { .. } => BlockdevDriver::raw,

            BlockdevAmendOptions::rbd { .. } => BlockdevDriver::rbd,

            BlockdevAmendOptions::replication { .. } => BlockdevDriver::replication,

            BlockdevAmendOptions::ssh { .. } => BlockdevDriver::ssh,

            BlockdevAmendOptions::throttle { .. } => BlockdevDriver::throttle,

            BlockdevAmendOptions::vdi { .. } => BlockdevDriver::vdi,

            BlockdevAmendOptions::vhdx { .. } => BlockdevDriver::vhdx,

            BlockdevAmendOptions::virtio_blk_vfio_pci { .. } => BlockdevDriver::virtio_blk_vfio_pci,

            BlockdevAmendOptions::virtio_blk_vhost_user { .. } => BlockdevDriver::virtio_blk_vhost_user,

            BlockdevAmendOptions::virtio_blk_vhost_vdpa { .. } => BlockdevDriver::virtio_blk_vhost_vdpa,

            BlockdevAmendOptions::vmdk { .. } => BlockdevDriver::vmdk,

            BlockdevAmendOptions::vpc { .. } => BlockdevDriver::vpc,

            BlockdevAmendOptions::vvfat { .. } => BlockdevDriver::vvfat,

        }
    }
}

impl From<BlockdevAmendOptionsLUKS> for BlockdevAmendOptions {
    fn from(val: BlockdevAmendOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

impl From<BlockdevAmendOptionsQcow2> for BlockdevAmendOptions {
    fn from(val: BlockdevAmendOptionsQcow2) -> Self {
        Self::qcow2(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "driver")]
pub enum BlockdevCreateOptions {
	#[serde(rename = "file")]
	file(BlockdevCreateOptionsFile),
	#[serde(rename = "gluster")]
	gluster(BlockdevCreateOptionsGluster),
	#[serde(rename = "luks")]
	luks(BlockdevCreateOptionsLUKS),
	#[serde(rename = "nfs")]
	nfs(BlockdevCreateOptionsNfs),
	#[serde(rename = "parallels")]
	parallels(BlockdevCreateOptionsParallels),
	#[serde(rename = "qcow")]
	qcow(BlockdevCreateOptionsQcow),
	#[serde(rename = "qcow2")]
	qcow2(BlockdevCreateOptionsQcow2),
	#[serde(rename = "qed")]
	qed(BlockdevCreateOptionsQed),
	#[serde(rename = "rbd")]
	rbd(BlockdevCreateOptionsRbd),
	#[serde(rename = "ssh")]
	ssh(BlockdevCreateOptionsSsh),
	#[serde(rename = "vdi")]
	vdi(BlockdevCreateOptionsVdi),
	#[serde(rename = "vhdx")]
	vhdx(BlockdevCreateOptionsVhdx),
	#[serde(rename = "vmdk")]
	vmdk(BlockdevCreateOptionsVmdk),
	#[serde(rename = "vpc")]
	vpc(BlockdevCreateOptionsVpc),
	#[serde(rename = "blkdebug")]
	blkdebug,
	#[serde(rename = "blklogwrites")]
	blklogwrites,
	#[serde(rename = "blkreplay")]
	blkreplay,
	#[serde(rename = "blkverify")]
	blkverify,
	#[serde(rename = "bochs")]
	bochs,
	#[serde(rename = "cloop")]
	cloop,
	#[serde(rename = "compress")]
	compress,
	#[serde(rename = "copy-before-write")]
	copy_before_write,
	#[serde(rename = "copy-on-read")]
	copy_on_read,
	#[serde(rename = "dmg")]
	dmg,
	#[serde(rename = "snapshot-access")]
	snapshot_access,
	#[serde(rename = "ftp")]
	ftp,
	#[serde(rename = "ftps")]
	ftps,
	#[serde(rename = "host_cdrom")]
	host_cdrom,
	#[serde(rename = "host_device")]
	host_device,
	#[serde(rename = "http")]
	http,
	#[serde(rename = "https")]
	https,
	#[serde(rename = "io_uring")]
	io_uring,
	#[serde(rename = "iscsi")]
	iscsi,
	#[serde(rename = "nbd")]
	nbd,
	#[serde(rename = "null-aio")]
	null_aio,
	#[serde(rename = "null-co")]
	null_co,
	#[serde(rename = "nvme")]
	nvme,
	#[serde(rename = "nvme-io_uring")]
	nvme_io_uring,
	#[serde(rename = "preallocate")]
	preallocate,
	#[serde(rename = "quorum")]
	quorum,
	#[serde(rename = "raw")]
	raw,
	#[serde(rename = "replication")]
	replication,
	#[serde(rename = "throttle")]
	throttle,
	#[serde(rename = "virtio-blk-vfio-pci")]
	virtio_blk_vfio_pci,
	#[serde(rename = "virtio-blk-vhost-user")]
	virtio_blk_vhost_user,
	#[serde(rename = "virtio-blk-vhost-vdpa")]
	virtio_blk_vhost_vdpa,
	#[serde(rename = "vvfat")]
	vvfat,
}

impl BlockdevCreateOptions {
    pub fn driver(&self) -> BlockdevDriver {
        match *self {

            BlockdevCreateOptions::file { .. } => BlockdevDriver::file,

            BlockdevCreateOptions::gluster { .. } => BlockdevDriver::gluster,

            BlockdevCreateOptions::luks { .. } => BlockdevDriver::luks,

            BlockdevCreateOptions::nfs { .. } => BlockdevDriver::nfs,

            BlockdevCreateOptions::parallels { .. } => BlockdevDriver::parallels,

            BlockdevCreateOptions::qcow { .. } => BlockdevDriver::qcow,

            BlockdevCreateOptions::qcow2 { .. } => BlockdevDriver::qcow2,

            BlockdevCreateOptions::qed { .. } => BlockdevDriver::qed,

            BlockdevCreateOptions::rbd { .. } => BlockdevDriver::rbd,

            BlockdevCreateOptions::ssh { .. } => BlockdevDriver::ssh,

            BlockdevCreateOptions::vdi { .. } => BlockdevDriver::vdi,

            BlockdevCreateOptions::vhdx { .. } => BlockdevDriver::vhdx,

            BlockdevCreateOptions::vmdk { .. } => BlockdevDriver::vmdk,

            BlockdevCreateOptions::vpc { .. } => BlockdevDriver::vpc,

            BlockdevCreateOptions::blkdebug { .. } => BlockdevDriver::blkdebug,

            BlockdevCreateOptions::blklogwrites { .. } => BlockdevDriver::blklogwrites,

            BlockdevCreateOptions::blkreplay { .. } => BlockdevDriver::blkreplay,

            BlockdevCreateOptions::blkverify { .. } => BlockdevDriver::blkverify,

            BlockdevCreateOptions::bochs { .. } => BlockdevDriver::bochs,

            BlockdevCreateOptions::cloop { .. } => BlockdevDriver::cloop,

            BlockdevCreateOptions::compress { .. } => BlockdevDriver::compress,

            BlockdevCreateOptions::copy_before_write { .. } => BlockdevDriver::copy_before_write,

            BlockdevCreateOptions::copy_on_read { .. } => BlockdevDriver::copy_on_read,

            BlockdevCreateOptions::dmg { .. } => BlockdevDriver::dmg,

            BlockdevCreateOptions::snapshot_access { .. } => BlockdevDriver::snapshot_access,

            BlockdevCreateOptions::ftp { .. } => BlockdevDriver::ftp,

            BlockdevCreateOptions::ftps { .. } => BlockdevDriver::ftps,

            BlockdevCreateOptions::host_cdrom { .. } => BlockdevDriver::host_cdrom,

            BlockdevCreateOptions::host_device { .. } => BlockdevDriver::host_device,

            BlockdevCreateOptions::http { .. } => BlockdevDriver::http,

            BlockdevCreateOptions::https { .. } => BlockdevDriver::https,

            BlockdevCreateOptions::io_uring { .. } => BlockdevDriver::io_uring,

            BlockdevCreateOptions::iscsi { .. } => BlockdevDriver::iscsi,

            BlockdevCreateOptions::nbd { .. } => BlockdevDriver::nbd,

            BlockdevCreateOptions::null_aio { .. } => BlockdevDriver::null_aio,

            BlockdevCreateOptions::null_co { .. } => BlockdevDriver::null_co,

            BlockdevCreateOptions::nvme { .. } => BlockdevDriver::nvme,

            BlockdevCreateOptions::nvme_io_uring { .. } => BlockdevDriver::nvme_io_uring,

            BlockdevCreateOptions::preallocate { .. } => BlockdevDriver::preallocate,

            BlockdevCreateOptions::quorum { .. } => BlockdevDriver::quorum,

            BlockdevCreateOptions::raw { .. } => BlockdevDriver::raw,

            BlockdevCreateOptions::replication { .. } => BlockdevDriver::replication,

            BlockdevCreateOptions::throttle { .. } => BlockdevDriver::throttle,

            BlockdevCreateOptions::virtio_blk_vfio_pci { .. } => BlockdevDriver::virtio_blk_vfio_pci,

            BlockdevCreateOptions::virtio_blk_vhost_user { .. } => BlockdevDriver::virtio_blk_vhost_user,

            BlockdevCreateOptions::virtio_blk_vhost_vdpa { .. } => BlockdevDriver::virtio_blk_vhost_vdpa,

            BlockdevCreateOptions::vvfat { .. } => BlockdevDriver::vvfat,

        }
    }
}

impl From<BlockdevCreateOptionsFile> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsFile) -> Self {
        Self::file(val)
    }
}

impl From<BlockdevCreateOptionsGluster> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsGluster) -> Self {
        Self::gluster(val)
    }
}

impl From<BlockdevCreateOptionsLUKS> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

impl From<BlockdevCreateOptionsNfs> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsNfs) -> Self {
        Self::nfs(val)
    }
}

impl From<BlockdevCreateOptionsParallels> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsParallels) -> Self {
        Self::parallels(val)
    }
}

impl From<BlockdevCreateOptionsQcow> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsQcow) -> Self {
        Self::qcow(val)
    }
}

impl From<BlockdevCreateOptionsQcow2> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsQcow2) -> Self {
        Self::qcow2(val)
    }
}

impl From<BlockdevCreateOptionsQed> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsQed) -> Self {
        Self::qed(val)
    }
}

impl From<BlockdevCreateOptionsRbd> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsRbd) -> Self {
        Self::rbd(val)
    }
}

impl From<BlockdevCreateOptionsSsh> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsSsh) -> Self {
        Self::ssh(val)
    }
}

impl From<BlockdevCreateOptionsVdi> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsVdi) -> Self {
        Self::vdi(val)
    }
}

impl From<BlockdevCreateOptionsVhdx> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsVhdx) -> Self {
        Self::vhdx(val)
    }
}

impl From<BlockdevCreateOptionsVmdk> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsVmdk) -> Self {
        Self::vmdk(val)
    }
}

impl From<BlockdevCreateOptionsVpc> for BlockdevCreateOptions {
    fn from(val: BlockdevCreateOptionsVpc) -> Self {
        Self::vpc(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "driver")]
pub enum BlockdevOptions {
	#[serde(rename = "blkdebug")]
	blkdebug {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "blkdebug")]
blkdebug: BlockdevOptionsBlkdebug,
	},
	#[serde(rename = "blklogwrites")]
	blklogwrites {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "blklogwrites")]
blklogwrites: BlockdevOptionsBlklogwrites,
	},
	#[serde(rename = "blkreplay")]
	blkreplay {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "blkreplay")]
blkreplay: BlockdevOptionsBlkreplay,
	},
	#[serde(rename = "blkverify")]
	blkverify {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "blkverify")]
blkverify: BlockdevOptionsBlkverify,
	},
	#[serde(rename = "bochs")]
	bochs {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "bochs")]
bochs: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "cloop")]
	cloop {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "cloop")]
cloop: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "compress")]
	compress {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "compress")]
compress: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "copy-before-write")]
	copy_before_write {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "copy-before-write")]
copy_before_write: BlockdevOptionsCbw,
	},
	#[serde(rename = "copy-on-read")]
	copy_on_read {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "copy-on-read")]
copy_on_read: BlockdevOptionsCor,
	},
	#[serde(rename = "dmg")]
	dmg {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "dmg")]
dmg: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "file")]
	file {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "file")]
file: BlockdevOptionsFile,
	},
	#[serde(rename = "ftp")]
	ftp {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "ftp")]
ftp: BlockdevOptionsCurlFtp,
	},
	#[serde(rename = "ftps")]
	ftps {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "ftps")]
ftps: BlockdevOptionsCurlFtps,
	},
	#[serde(rename = "gluster")]
	gluster {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "gluster")]
gluster: BlockdevOptionsGluster,
	},
	#[serde(rename = "host_cdrom")]
	host_cdrom {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "host_cdrom")]
host_cdrom: BlockdevOptionsFile,
	},
	#[serde(rename = "host_device")]
	host_device {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "host_device")]
host_device: BlockdevOptionsFile,
	},
	#[serde(rename = "http")]
	http {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "http")]
http: BlockdevOptionsCurlHttp,
	},
	#[serde(rename = "https")]
	https {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "https")]
https: BlockdevOptionsCurlHttps,
	},
	#[serde(rename = "io_uring")]
	io_uring {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "io_uring")]
io_uring: BlockdevOptionsIoUring,
	},
	#[serde(rename = "iscsi")]
	iscsi {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "iscsi")]
iscsi: BlockdevOptionsIscsi,
	},
	#[serde(rename = "luks")]
	luks {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "luks")]
luks: BlockdevOptionsLUKS,
	},
	#[serde(rename = "nbd")]
	nbd {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "nbd")]
nbd: BlockdevOptionsNbd,
	},
	#[serde(rename = "nfs")]
	nfs {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "nfs")]
nfs: BlockdevOptionsNfs,
	},
	#[serde(rename = "null-aio")]
	null_aio {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "null-aio")]
null_aio: BlockdevOptionsNull,
	},
	#[serde(rename = "null-co")]
	null_co {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "null-co")]
null_co: BlockdevOptionsNull,
	},
	#[serde(rename = "nvme")]
	nvme {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "nvme")]
nvme: BlockdevOptionsNVMe,
	},
	#[serde(rename = "nvme-io_uring")]
	nvme_io_uring {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "nvme-io_uring")]
nvme_io_uring: BlockdevOptionsNvmeIoUring,
	},
	#[serde(rename = "parallels")]
	parallels {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "parallels")]
parallels: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "preallocate")]
	preallocate {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "preallocate")]
preallocate: BlockdevOptionsPreallocate,
	},
	#[serde(rename = "qcow")]
	qcow {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "qcow")]
qcow: BlockdevOptionsQcow,
	},
	#[serde(rename = "qcow2")]
	qcow2 {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "qcow2")]
qcow2: BlockdevOptionsQcow2,
	},
	#[serde(rename = "qed")]
	qed {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "qed")]
qed: BlockdevOptionsGenericCOWFormat,
	},
	#[serde(rename = "quorum")]
	quorum {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "quorum")]
quorum: BlockdevOptionsQuorum,
	},
	#[serde(rename = "raw")]
	raw {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "raw")]
raw: BlockdevOptionsRaw,
	},
	#[serde(rename = "rbd")]
	rbd {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "rbd")]
rbd: BlockdevOptionsRbd,
	},
	#[serde(rename = "replication")]
	replication {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "replication")]
replication: BlockdevOptionsReplication,
	},
	#[serde(rename = "snapshot-access")]
	snapshot_access {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "snapshot-access")]
snapshot_access: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "ssh")]
	ssh {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "ssh")]
ssh: BlockdevOptionsSsh,
	},
	#[serde(rename = "throttle")]
	throttle {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "throttle")]
throttle: BlockdevOptionsThrottle,
	},
	#[serde(rename = "vdi")]
	vdi {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "vdi")]
vdi: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "vhdx")]
	vhdx {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "vhdx")]
vhdx: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "virtio-blk-vfio-pci")]
	virtio_blk_vfio_pci {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "virtio-blk-vfio-pci")]
virtio_blk_vfio_pci: BlockdevOptionsVirtioBlkVfioPci,
	},
	#[serde(rename = "virtio-blk-vhost-user")]
	virtio_blk_vhost_user {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "virtio-blk-vhost-user")]
virtio_blk_vhost_user: BlockdevOptionsVirtioBlkVhostUser,
	},
	#[serde(rename = "virtio-blk-vhost-vdpa")]
	virtio_blk_vhost_vdpa {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "virtio-blk-vhost-vdpa")]
virtio_blk_vhost_vdpa: BlockdevOptionsVirtioBlkVhostVdpa,
	},
	#[serde(rename = "vmdk")]
	vmdk {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "vmdk")]
vmdk: BlockdevOptionsGenericCOWFormat,
	},
	#[serde(rename = "vpc")]
	vpc {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "vpc")]
vpc: BlockdevOptionsGenericFormat,
	},
	#[serde(rename = "vvfat")]
	vvfat {
		#[serde(flatten)] #[serde(rename = "base")]
base: BlockdevOptionsBase,
		#[serde(flatten)] #[serde(rename = "vvfat")]
vvfat: BlockdevOptionsVVFAT,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsBase {
	#[serde(rename = "auto-read-only", default, skip_serializing_if = "Option::is_none")]
pub auto_read_only: Option<bool>,
	#[serde(rename = "cache", default, skip_serializing_if = "Option::is_none")]
pub cache: Option<BlockdevCacheOptions>,
	#[serde(rename = "detect-zeroes", default, skip_serializing_if = "Option::is_none")]
pub detect_zeroes: Option<BlockdevDetectZeroesOptions>,
	#[serde(rename = "discard", default, skip_serializing_if = "Option::is_none")]
pub discard: Option<BlockdevDiscardOptions>,
	#[serde(rename = "force-share", default, skip_serializing_if = "Option::is_none")]
pub force_share: Option<bool>,
	#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
	#[serde(rename = "read-only", default, skip_serializing_if = "Option::is_none")]
pub read_only: Option<bool>,
}

impl BlockdevOptions {
    pub fn driver(&self) -> BlockdevDriver {
        match *self {

            BlockdevOptions::blkdebug { .. } => BlockdevDriver::blkdebug,

            BlockdevOptions::blklogwrites { .. } => BlockdevDriver::blklogwrites,

            BlockdevOptions::blkreplay { .. } => BlockdevDriver::blkreplay,

            BlockdevOptions::blkverify { .. } => BlockdevDriver::blkverify,

            BlockdevOptions::bochs { .. } => BlockdevDriver::bochs,

            BlockdevOptions::cloop { .. } => BlockdevDriver::cloop,

            BlockdevOptions::compress { .. } => BlockdevDriver::compress,

            BlockdevOptions::copy_before_write { .. } => BlockdevDriver::copy_before_write,

            BlockdevOptions::copy_on_read { .. } => BlockdevDriver::copy_on_read,

            BlockdevOptions::dmg { .. } => BlockdevDriver::dmg,

            BlockdevOptions::file { .. } => BlockdevDriver::file,

            BlockdevOptions::ftp { .. } => BlockdevDriver::ftp,

            BlockdevOptions::ftps { .. } => BlockdevDriver::ftps,

            BlockdevOptions::gluster { .. } => BlockdevDriver::gluster,

            BlockdevOptions::host_cdrom { .. } => BlockdevDriver::host_cdrom,

            BlockdevOptions::host_device { .. } => BlockdevDriver::host_device,

            BlockdevOptions::http { .. } => BlockdevDriver::http,

            BlockdevOptions::https { .. } => BlockdevDriver::https,

            BlockdevOptions::io_uring { .. } => BlockdevDriver::io_uring,

            BlockdevOptions::iscsi { .. } => BlockdevDriver::iscsi,

            BlockdevOptions::luks { .. } => BlockdevDriver::luks,

            BlockdevOptions::nbd { .. } => BlockdevDriver::nbd,

            BlockdevOptions::nfs { .. } => BlockdevDriver::nfs,

            BlockdevOptions::null_aio { .. } => BlockdevDriver::null_aio,

            BlockdevOptions::null_co { .. } => BlockdevDriver::null_co,

            BlockdevOptions::nvme { .. } => BlockdevDriver::nvme,

            BlockdevOptions::nvme_io_uring { .. } => BlockdevDriver::nvme_io_uring,

            BlockdevOptions::parallels { .. } => BlockdevDriver::parallels,

            BlockdevOptions::preallocate { .. } => BlockdevDriver::preallocate,

            BlockdevOptions::qcow { .. } => BlockdevDriver::qcow,

            BlockdevOptions::qcow2 { .. } => BlockdevDriver::qcow2,

            BlockdevOptions::qed { .. } => BlockdevDriver::qed,

            BlockdevOptions::quorum { .. } => BlockdevDriver::quorum,

            BlockdevOptions::raw { .. } => BlockdevDriver::raw,

            BlockdevOptions::rbd { .. } => BlockdevDriver::rbd,

            BlockdevOptions::replication { .. } => BlockdevDriver::replication,

            BlockdevOptions::snapshot_access { .. } => BlockdevDriver::snapshot_access,

            BlockdevOptions::ssh { .. } => BlockdevDriver::ssh,

            BlockdevOptions::throttle { .. } => BlockdevDriver::throttle,

            BlockdevOptions::vdi { .. } => BlockdevDriver::vdi,

            BlockdevOptions::vhdx { .. } => BlockdevDriver::vhdx,

            BlockdevOptions::virtio_blk_vfio_pci { .. } => BlockdevDriver::virtio_blk_vfio_pci,

            BlockdevOptions::virtio_blk_vhost_user { .. } => BlockdevDriver::virtio_blk_vhost_user,

            BlockdevOptions::virtio_blk_vhost_vdpa { .. } => BlockdevDriver::virtio_blk_vhost_vdpa,

            BlockdevOptions::vmdk { .. } => BlockdevDriver::vmdk,

            BlockdevOptions::vpc { .. } => BlockdevDriver::vpc,

            BlockdevOptions::vvfat { .. } => BlockdevDriver::vvfat,

        }
    }
}

impl From<(BlockdevOptionsBlkdebug, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsBlkdebug, BlockdevOptionsBase)) -> Self {
        Self::blkdebug {
            blkdebug: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsBlklogwrites, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsBlklogwrites, BlockdevOptionsBase)) -> Self {
        Self::blklogwrites {
            blklogwrites: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsBlkreplay, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsBlkreplay, BlockdevOptionsBase)) -> Self {
        Self::blkreplay {
            blkreplay: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsBlkverify, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsBlkverify, BlockdevOptionsBase)) -> Self {
        Self::blkverify {
            blkverify: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCbw, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCbw, BlockdevOptionsBase)) -> Self {
        Self::copy_before_write {
            copy_before_write: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCor, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCor, BlockdevOptionsBase)) -> Self {
        Self::copy_on_read {
            copy_on_read: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCurlFtp, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCurlFtp, BlockdevOptionsBase)) -> Self {
        Self::ftp {
            ftp: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCurlFtps, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCurlFtps, BlockdevOptionsBase)) -> Self {
        Self::ftps {
            ftps: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsGluster, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsGluster, BlockdevOptionsBase)) -> Self {
        Self::gluster {
            gluster: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCurlHttp, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCurlHttp, BlockdevOptionsBase)) -> Self {
        Self::http {
            http: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsCurlHttps, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsCurlHttps, BlockdevOptionsBase)) -> Self {
        Self::https {
            https: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsIoUring, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsIoUring, BlockdevOptionsBase)) -> Self {
        Self::io_uring {
            io_uring: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsIscsi, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsIscsi, BlockdevOptionsBase)) -> Self {
        Self::iscsi {
            iscsi: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsLUKS, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsLUKS, BlockdevOptionsBase)) -> Self {
        Self::luks {
            luks: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsNbd, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsNbd, BlockdevOptionsBase)) -> Self {
        Self::nbd {
            nbd: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsNfs, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsNfs, BlockdevOptionsBase)) -> Self {
        Self::nfs {
            nfs: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsNVMe, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsNVMe, BlockdevOptionsBase)) -> Self {
        Self::nvme {
            nvme: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsNvmeIoUring, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsNvmeIoUring, BlockdevOptionsBase)) -> Self {
        Self::nvme_io_uring {
            nvme_io_uring: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsPreallocate, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsPreallocate, BlockdevOptionsBase)) -> Self {
        Self::preallocate {
            preallocate: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsQcow, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsQcow, BlockdevOptionsBase)) -> Self {
        Self::qcow {
            qcow: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsQcow2, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsQcow2, BlockdevOptionsBase)) -> Self {
        Self::qcow2 {
            qcow2: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsQuorum, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsQuorum, BlockdevOptionsBase)) -> Self {
        Self::quorum {
            quorum: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsRaw, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsRaw, BlockdevOptionsBase)) -> Self {
        Self::raw {
            raw: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsRbd, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsRbd, BlockdevOptionsBase)) -> Self {
        Self::rbd {
            rbd: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsReplication, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsReplication, BlockdevOptionsBase)) -> Self {
        Self::replication {
            replication: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsSsh, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsSsh, BlockdevOptionsBase)) -> Self {
        Self::ssh {
            ssh: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsThrottle, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsThrottle, BlockdevOptionsBase)) -> Self {
        Self::throttle {
            throttle: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsVirtioBlkVfioPci, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsVirtioBlkVfioPci, BlockdevOptionsBase)) -> Self {
        Self::virtio_blk_vfio_pci {
            virtio_blk_vfio_pci: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsVirtioBlkVhostUser, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsVirtioBlkVhostUser, BlockdevOptionsBase)) -> Self {
        Self::virtio_blk_vhost_user {
            virtio_blk_vhost_user: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsVirtioBlkVhostVdpa, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsVirtioBlkVhostVdpa, BlockdevOptionsBase)) -> Self {
        Self::virtio_blk_vhost_vdpa {
            virtio_blk_vhost_vdpa: val.0,
            base: val.1,

        }
    }
}

impl From<(BlockdevOptionsVVFAT, BlockdevOptionsBase)> for BlockdevOptions {
    fn from(val: (BlockdevOptionsVVFAT, BlockdevOptionsBase)) -> Self {
        Self::vvfat {
            vvfat: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum BlockdevQcow2Encryption {
	#[serde(rename = "aes")]
	aes(QCryptoBlockOptionsQCow),
	#[serde(rename = "luks")]
	luks(QCryptoBlockOptionsLUKS),
}

impl BlockdevQcow2Encryption {
    pub fn format(&self) -> BlockdevQcow2EncryptionFormat {
        match *self {

            BlockdevQcow2Encryption::aes { .. } => BlockdevQcow2EncryptionFormat::aes,

            BlockdevQcow2Encryption::luks { .. } => BlockdevQcow2EncryptionFormat::luks,

        }
    }
}

impl From<QCryptoBlockOptionsQCow> for BlockdevQcow2Encryption {
    fn from(val: QCryptoBlockOptionsQCow) -> Self {
        Self::aes(val)
    }
}

impl From<QCryptoBlockOptionsLUKS> for BlockdevQcow2Encryption {
    fn from(val: QCryptoBlockOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum BlockdevQcowEncryption {
	#[serde(rename = "aes")]
	aes(QCryptoBlockOptionsQCow),
}

impl BlockdevQcowEncryption {
    pub fn format(&self) -> BlockdevQcowEncryptionFormat {
        match *self {

            BlockdevQcowEncryption::aes { .. } => BlockdevQcowEncryptionFormat::aes,

        }
    }
}

impl From<QCryptoBlockOptionsQCow> for BlockdevQcowEncryption {
    fn from(val: QCryptoBlockOptionsQCow) -> Self {
        Self::aes(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChardevBackend {
	#[serde(rename = "braille")]
	braille(ChardevCommonWrapper),
	#[serde(rename = "console")]
	console(ChardevCommonWrapper),
	#[serde(rename = "dbus")]
	dbus(ChardevDBusWrapper),
	#[serde(rename = "file")]
	file(ChardevFileWrapper),
	#[serde(rename = "memory")]
	memory(ChardevRingbufWrapper),
	#[serde(rename = "msmouse")]
	msmouse(ChardevCommonWrapper),
	#[serde(rename = "mux")]
	mux(ChardevMuxWrapper),
	#[serde(rename = "null")]
	null(ChardevCommonWrapper),
	#[serde(rename = "parallel")]
	parallel(ChardevHostdevWrapper),
	#[serde(rename = "pipe")]
	pipe(ChardevHostdevWrapper),
	#[serde(rename = "pty")]
	pty(ChardevCommonWrapper),
	#[serde(rename = "qemu-vdagent")]
	qemu_vdagent(ChardevQemuVDAgentWrapper),
	#[serde(rename = "ringbuf")]
	ringbuf(ChardevRingbufWrapper),
	#[serde(rename = "serial")]
	serial(ChardevHostdevWrapper),
	#[serde(rename = "socket")]
	socket(ChardevSocketWrapper),
	#[serde(rename = "spiceport")]
	spiceport(ChardevSpicePortWrapper),
	#[serde(rename = "spicevmc")]
	spicevmc(ChardevSpiceChannelWrapper),
	#[serde(rename = "stdio")]
	stdio(ChardevStdioWrapper),
	#[serde(rename = "testdev")]
	testdev(ChardevCommonWrapper),
	#[serde(rename = "udp")]
	udp(ChardevUdpWrapper),
	#[serde(rename = "vc")]
	vc(ChardevVCWrapper),
	#[serde(rename = "wctablet")]
	wctablet(ChardevCommonWrapper),
}

impl ChardevBackend {
    pub fn type_(&self) -> ChardevBackendKind {
        match *self {

            ChardevBackend::braille { .. } => ChardevBackendKind::braille,

            ChardevBackend::console { .. } => ChardevBackendKind::console,

            ChardevBackend::dbus { .. } => ChardevBackendKind::dbus,

            ChardevBackend::file { .. } => ChardevBackendKind::file,

            ChardevBackend::memory { .. } => ChardevBackendKind::memory,

            ChardevBackend::msmouse { .. } => ChardevBackendKind::msmouse,

            ChardevBackend::mux { .. } => ChardevBackendKind::mux,

            ChardevBackend::null { .. } => ChardevBackendKind::null,

            ChardevBackend::parallel { .. } => ChardevBackendKind::parallel,

            ChardevBackend::pipe { .. } => ChardevBackendKind::pipe,

            ChardevBackend::pty { .. } => ChardevBackendKind::pty,

            ChardevBackend::qemu_vdagent { .. } => ChardevBackendKind::qemu_vdagent,

            ChardevBackend::ringbuf { .. } => ChardevBackendKind::ringbuf,

            ChardevBackend::serial { .. } => ChardevBackendKind::serial,

            ChardevBackend::socket { .. } => ChardevBackendKind::socket,

            ChardevBackend::spiceport { .. } => ChardevBackendKind::spiceport,

            ChardevBackend::spicevmc { .. } => ChardevBackendKind::spicevmc,

            ChardevBackend::stdio { .. } => ChardevBackendKind::stdio,

            ChardevBackend::testdev { .. } => ChardevBackendKind::testdev,

            ChardevBackend::udp { .. } => ChardevBackendKind::udp,

            ChardevBackend::vc { .. } => ChardevBackendKind::vc,

            ChardevBackend::wctablet { .. } => ChardevBackendKind::wctablet,

        }
    }
}

impl From<ChardevDBusWrapper> for ChardevBackend {
    fn from(val: ChardevDBusWrapper) -> Self {
        Self::dbus(val)
    }
}

impl From<ChardevDBus> for ChardevBackend {
    fn from(val: ChardevDBus) -> Self {
        Self::dbus(ChardevDBusWrapper::from(val))
    }
}

impl From<ChardevFileWrapper> for ChardevBackend {
    fn from(val: ChardevFileWrapper) -> Self {
        Self::file(val)
    }
}

impl From<ChardevFile> for ChardevBackend {
    fn from(val: ChardevFile) -> Self {
        Self::file(ChardevFileWrapper::from(val))
    }
}

impl From<ChardevMuxWrapper> for ChardevBackend {
    fn from(val: ChardevMuxWrapper) -> Self {
        Self::mux(val)
    }
}

impl From<ChardevMux> for ChardevBackend {
    fn from(val: ChardevMux) -> Self {
        Self::mux(ChardevMuxWrapper::from(val))
    }
}

impl From<ChardevQemuVDAgentWrapper> for ChardevBackend {
    fn from(val: ChardevQemuVDAgentWrapper) -> Self {
        Self::qemu_vdagent(val)
    }
}

impl From<ChardevQemuVDAgent> for ChardevBackend {
    fn from(val: ChardevQemuVDAgent) -> Self {
        Self::qemu_vdagent(ChardevQemuVDAgentWrapper::from(val))
    }
}

impl From<ChardevSocketWrapper> for ChardevBackend {
    fn from(val: ChardevSocketWrapper) -> Self {
        Self::socket(val)
    }
}

impl From<ChardevSocket> for ChardevBackend {
    fn from(val: ChardevSocket) -> Self {
        Self::socket(ChardevSocketWrapper::from(val))
    }
}

impl From<ChardevSpicePortWrapper> for ChardevBackend {
    fn from(val: ChardevSpicePortWrapper) -> Self {
        Self::spiceport(val)
    }
}

impl From<ChardevSpicePort> for ChardevBackend {
    fn from(val: ChardevSpicePort) -> Self {
        Self::spiceport(ChardevSpicePortWrapper::from(val))
    }
}

impl From<ChardevSpiceChannelWrapper> for ChardevBackend {
    fn from(val: ChardevSpiceChannelWrapper) -> Self {
        Self::spicevmc(val)
    }
}

impl From<ChardevSpiceChannel> for ChardevBackend {
    fn from(val: ChardevSpiceChannel) -> Self {
        Self::spicevmc(ChardevSpiceChannelWrapper::from(val))
    }
}

impl From<ChardevStdioWrapper> for ChardevBackend {
    fn from(val: ChardevStdioWrapper) -> Self {
        Self::stdio(val)
    }
}

impl From<ChardevStdio> for ChardevBackend {
    fn from(val: ChardevStdio) -> Self {
        Self::stdio(ChardevStdioWrapper::from(val))
    }
}

impl From<ChardevUdpWrapper> for ChardevBackend {
    fn from(val: ChardevUdpWrapper) -> Self {
        Self::udp(val)
    }
}

impl From<ChardevUdp> for ChardevBackend {
    fn from(val: ChardevUdp) -> Self {
        Self::udp(ChardevUdpWrapper::from(val))
    }
}

impl From<ChardevVCWrapper> for ChardevBackend {
    fn from(val: ChardevVCWrapper) -> Self {
        Self::vc(val)
    }
}

impl From<ChardevVC> for ChardevBackend {
    fn from(val: ChardevVC) -> Self {
        Self::vc(ChardevVCWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "target")]
pub enum CpuInfoFast {
	#[serde(rename = "s390x")]
	s390x {
		#[serde(flatten)] #[serde(rename = "base")]
base: CpuInfoFastBase,
		#[serde(flatten)] #[serde(rename = "s390x")]
s390x: CpuInfoS390,
	},
	#[serde(rename = "aarch64")]
	aarch64(CpuInfoFastBase),
	#[serde(rename = "alpha")]
	alpha(CpuInfoFastBase),
	#[serde(rename = "arm")]
	arm(CpuInfoFastBase),
	#[serde(rename = "avr")]
	avr(CpuInfoFastBase),
	#[serde(rename = "cris")]
	cris(CpuInfoFastBase),
	#[serde(rename = "hppa")]
	hppa(CpuInfoFastBase),
	#[serde(rename = "i386")]
	i386(CpuInfoFastBase),
	#[serde(rename = "loongarch64")]
	loongarch64(CpuInfoFastBase),
	#[serde(rename = "m68k")]
	m68k(CpuInfoFastBase),
	#[serde(rename = "microblaze")]
	microblaze(CpuInfoFastBase),
	#[serde(rename = "microblazeel")]
	microblazeel(CpuInfoFastBase),
	#[serde(rename = "mips")]
	mips(CpuInfoFastBase),
	#[serde(rename = "mips64")]
	mips64(CpuInfoFastBase),
	#[serde(rename = "mips64el")]
	mips64el(CpuInfoFastBase),
	#[serde(rename = "mipsel")]
	mipsel(CpuInfoFastBase),
	#[serde(rename = "or1k")]
	or1k(CpuInfoFastBase),
	#[serde(rename = "ppc")]
	ppc(CpuInfoFastBase),
	#[serde(rename = "ppc64")]
	ppc64(CpuInfoFastBase),
	#[serde(rename = "riscv32")]
	riscv32(CpuInfoFastBase),
	#[serde(rename = "riscv64")]
	riscv64(CpuInfoFastBase),
	#[serde(rename = "rx")]
	rx(CpuInfoFastBase),
	#[serde(rename = "sh4")]
	sh4(CpuInfoFastBase),
	#[serde(rename = "sh4eb")]
	sh4eb(CpuInfoFastBase),
	#[serde(rename = "sparc")]
	sparc(CpuInfoFastBase),
	#[serde(rename = "sparc64")]
	sparc64(CpuInfoFastBase),
	#[serde(rename = "tricore")]
	tricore(CpuInfoFastBase),
	#[serde(rename = "x86_64")]
	x86_64(CpuInfoFastBase),
	#[serde(rename = "xtensa")]
	xtensa(CpuInfoFastBase),
	#[serde(rename = "xtensaeb")]
	xtensaeb(CpuInfoFastBase),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfoFastBase {
	#[serde(rename = "props", default, skip_serializing_if = "Option::is_none")]
pub props: Option<CpuInstanceProperties>,
	#[serde(rename = "cpu-index")]
pub cpu_index: i64,
	#[serde(rename = "qom-path")]
pub qom_path: ::std::string::String,
	#[serde(rename = "thread-id")]
pub thread_id: i64,
}

impl CpuInfoFast {
    pub fn target(&self) -> SysEmuTarget {
        match *self {

            CpuInfoFast::s390x { .. } => SysEmuTarget::s390x,

            CpuInfoFast::aarch64 { .. } => SysEmuTarget::aarch64,

            CpuInfoFast::alpha { .. } => SysEmuTarget::alpha,

            CpuInfoFast::arm { .. } => SysEmuTarget::arm,

            CpuInfoFast::avr { .. } => SysEmuTarget::avr,

            CpuInfoFast::cris { .. } => SysEmuTarget::cris,

            CpuInfoFast::hppa { .. } => SysEmuTarget::hppa,

            CpuInfoFast::i386 { .. } => SysEmuTarget::i386,

            CpuInfoFast::loongarch64 { .. } => SysEmuTarget::loongarch64,

            CpuInfoFast::m68k { .. } => SysEmuTarget::m68k,

            CpuInfoFast::microblaze { .. } => SysEmuTarget::microblaze,

            CpuInfoFast::microblazeel { .. } => SysEmuTarget::microblazeel,

            CpuInfoFast::mips { .. } => SysEmuTarget::mips,

            CpuInfoFast::mips64 { .. } => SysEmuTarget::mips64,

            CpuInfoFast::mips64el { .. } => SysEmuTarget::mips64el,

            CpuInfoFast::mipsel { .. } => SysEmuTarget::mipsel,

            CpuInfoFast::or1k { .. } => SysEmuTarget::or1k,

            CpuInfoFast::ppc { .. } => SysEmuTarget::ppc,

            CpuInfoFast::ppc64 { .. } => SysEmuTarget::ppc64,

            CpuInfoFast::riscv32 { .. } => SysEmuTarget::riscv32,

            CpuInfoFast::riscv64 { .. } => SysEmuTarget::riscv64,

            CpuInfoFast::rx { .. } => SysEmuTarget::rx,

            CpuInfoFast::sh4 { .. } => SysEmuTarget::sh4,

            CpuInfoFast::sh4eb { .. } => SysEmuTarget::sh4eb,

            CpuInfoFast::sparc { .. } => SysEmuTarget::sparc,

            CpuInfoFast::sparc64 { .. } => SysEmuTarget::sparc64,

            CpuInfoFast::tricore { .. } => SysEmuTarget::tricore,

            CpuInfoFast::x86_64 { .. } => SysEmuTarget::x86_64,

            CpuInfoFast::xtensa { .. } => SysEmuTarget::xtensa,

            CpuInfoFast::xtensaeb { .. } => SysEmuTarget::xtensaeb,

        }
    }
}

impl From<(CpuInfoS390, CpuInfoFastBase)> for CpuInfoFast {
    fn from(val: (CpuInfoS390, CpuInfoFastBase)) -> Self {
        Self::s390x {
            s390x: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DisplayOptions {
	#[serde(rename = "cocoa")]
	cocoa {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "cocoa")]
cocoa: DisplayCocoa,
	},
	#[serde(rename = "curses")]
	curses {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "curses")]
curses: DisplayCurses,
	},
	#[serde(rename = "dbus")]
	dbus {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "dbus")]
dbus: DisplayDBus,
	},
	#[serde(rename = "egl-headless")]
	egl_headless {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "egl-headless")]
egl_headless: DisplayEGLHeadless,
	},
	#[serde(rename = "gtk")]
	gtk {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "gtk")]
gtk: DisplayGTK,
	},
	#[serde(rename = "sdl")]
	sdl {
		#[serde(flatten)] #[serde(rename = "base")]
base: DisplayOptionsBase,
		#[serde(flatten)] #[serde(rename = "sdl")]
sdl: DisplaySDL,
	},
	#[serde(rename = "default")]
	default(DisplayOptionsBase),
	#[serde(rename = "none")]
	none(DisplayOptionsBase),
	#[serde(rename = "spice-app")]
	spice_app(DisplayOptionsBase),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayOptionsBase {
	#[serde(rename = "full-screen", default, skip_serializing_if = "Option::is_none")]
pub full_screen: Option<bool>,
	#[serde(rename = "gl", default, skip_serializing_if = "Option::is_none")]
pub gl: Option<DisplayGLMode>,
	#[serde(rename = "show-cursor", default, skip_serializing_if = "Option::is_none")]
pub show_cursor: Option<bool>,
	#[serde(rename = "window-close", default, skip_serializing_if = "Option::is_none")]
pub window_close: Option<bool>,
}

impl DisplayOptions {
    pub fn type_(&self) -> DisplayType {
        match *self {

            DisplayOptions::cocoa { .. } => DisplayType::cocoa,

            DisplayOptions::curses { .. } => DisplayType::curses,

            DisplayOptions::dbus { .. } => DisplayType::dbus,

            DisplayOptions::egl_headless { .. } => DisplayType::egl_headless,

            DisplayOptions::gtk { .. } => DisplayType::gtk,

            DisplayOptions::sdl { .. } => DisplayType::sdl,

            DisplayOptions::default { .. } => DisplayType::default,

            DisplayOptions::none { .. } => DisplayType::none,

            DisplayOptions::spice_app { .. } => DisplayType::spice_app,

        }
    }
}

impl From<(DisplayCocoa, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplayCocoa, DisplayOptionsBase)) -> Self {
        Self::cocoa {
            cocoa: val.0,
            base: val.1,

        }
    }
}

impl From<(DisplayCurses, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplayCurses, DisplayOptionsBase)) -> Self {
        Self::curses {
            curses: val.0,
            base: val.1,

        }
    }
}

impl From<(DisplayDBus, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplayDBus, DisplayOptionsBase)) -> Self {
        Self::dbus {
            dbus: val.0,
            base: val.1,

        }
    }
}

impl From<(DisplayEGLHeadless, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplayEGLHeadless, DisplayOptionsBase)) -> Self {
        Self::egl_headless {
            egl_headless: val.0,
            base: val.1,

        }
    }
}

impl From<(DisplayGTK, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplayGTK, DisplayOptionsBase)) -> Self {
        Self::gtk {
            gtk: val.0,
            base: val.1,

        }
    }
}

impl From<(DisplaySDL, DisplayOptionsBase)> for DisplayOptions {
    fn from(val: (DisplaySDL, DisplayOptionsBase)) -> Self {
        Self::sdl {
            sdl: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DisplayReloadOptions {
	#[serde(rename = "vnc")]
	vnc(DisplayReloadOptionsVNC),
}

impl DisplayReloadOptions {
    pub fn type_(&self) -> DisplayReloadType {
        match *self {

            DisplayReloadOptions::vnc { .. } => DisplayReloadType::vnc,

        }
    }
}

impl From<DisplayReloadOptionsVNC> for DisplayReloadOptions {
    fn from(val: DisplayReloadOptionsVNC) -> Self {
        Self::vnc(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DisplayUpdateOptions {
	#[serde(rename = "vnc")]
	vnc(DisplayUpdateOptionsVNC),
}

impl DisplayUpdateOptions {
    pub fn type_(&self) -> DisplayUpdateType {
        match *self {

            DisplayUpdateOptions::vnc { .. } => DisplayUpdateType::vnc,

        }
    }
}

impl From<DisplayUpdateOptionsVNC> for DisplayUpdateOptions {
    fn from(val: DisplayUpdateOptionsVNC) -> Self {
        Self::vnc(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "protocol")]
pub enum ExpirePasswordOptions {
	#[serde(rename = "vnc")]
	vnc {
		#[serde(rename = "time")]
time: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vnc")]
vnc: ExpirePasswordOptionsVnc,
	},
	#[serde(rename = "spice")]
	spice(::std::string::String),
}

impl ExpirePasswordOptions {
    pub fn protocol(&self) -> DisplayProtocol {
        match *self {

            ExpirePasswordOptions::vnc { .. } => DisplayProtocol::vnc,

            ExpirePasswordOptions::spice { .. } => DisplayProtocol::spice,

        }
    }
}

impl From<(ExpirePasswordOptionsVnc, ::std::string::String)> for ExpirePasswordOptions {
    fn from(val: (ExpirePasswordOptionsVnc, ::std::string::String)) -> Self {
        Self::vnc {
            vnc: val.0,
            time: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GuestPanicInformation {
	#[serde(rename = "hyper-v")]
	hyper_v(GuestPanicInformationHyperV),
	#[serde(rename = "s390")]
	s390(GuestPanicInformationS390),
}

impl GuestPanicInformation {
    pub fn type_(&self) -> GuestPanicInformationType {
        match *self {

            GuestPanicInformation::hyper_v { .. } => GuestPanicInformationType::hyper_v,

            GuestPanicInformation::s390 { .. } => GuestPanicInformationType::s390,

        }
    }
}

impl From<GuestPanicInformationHyperV> for GuestPanicInformation {
    fn from(val: GuestPanicInformationHyperV) -> Self {
        Self::hyper_v(val)
    }
}

impl From<GuestPanicInformationS390> for GuestPanicInformation {
    fn from(val: GuestPanicInformationS390) -> Self {
        Self::s390(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageInfoSpecific {
	#[serde(rename = "file")]
	file(ImageInfoSpecificFileWrapper),
	#[serde(rename = "luks")]
	luks(ImageInfoSpecificLUKSWrapper),
	#[serde(rename = "qcow2")]
	qcow2(ImageInfoSpecificQCow2Wrapper),
	#[serde(rename = "rbd")]
	rbd(ImageInfoSpecificRbdWrapper),
	#[serde(rename = "vmdk")]
	vmdk(ImageInfoSpecificVmdkWrapper),
}

impl ImageInfoSpecific {
    pub fn type_(&self) -> ImageInfoSpecificKind {
        match *self {

            ImageInfoSpecific::file { .. } => ImageInfoSpecificKind::file,

            ImageInfoSpecific::luks { .. } => ImageInfoSpecificKind::luks,

            ImageInfoSpecific::qcow2 { .. } => ImageInfoSpecificKind::qcow2,

            ImageInfoSpecific::rbd { .. } => ImageInfoSpecificKind::rbd,

            ImageInfoSpecific::vmdk { .. } => ImageInfoSpecificKind::vmdk,

        }
    }
}

impl From<ImageInfoSpecificFileWrapper> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificFileWrapper) -> Self {
        Self::file(val)
    }
}

impl From<ImageInfoSpecificFile> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificFile) -> Self {
        Self::file(ImageInfoSpecificFileWrapper::from(val))
    }
}

impl From<ImageInfoSpecificLUKSWrapper> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificLUKSWrapper) -> Self {
        Self::luks(val)
    }
}

impl From<QCryptoBlockInfoLUKS> for ImageInfoSpecific {
    fn from(val: QCryptoBlockInfoLUKS) -> Self {
        Self::luks(ImageInfoSpecificLUKSWrapper::from(val))
    }
}

impl From<ImageInfoSpecificQCow2Wrapper> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificQCow2Wrapper) -> Self {
        Self::qcow2(val)
    }
}

impl From<ImageInfoSpecificQCow2> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificQCow2) -> Self {
        Self::qcow2(ImageInfoSpecificQCow2Wrapper::from(val))
    }
}

impl From<ImageInfoSpecificRbdWrapper> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificRbdWrapper) -> Self {
        Self::rbd(val)
    }
}

impl From<ImageInfoSpecificRbd> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificRbd) -> Self {
        Self::rbd(ImageInfoSpecificRbdWrapper::from(val))
    }
}

impl From<ImageInfoSpecificVmdkWrapper> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificVmdkWrapper) -> Self {
        Self::vmdk(val)
    }
}

impl From<ImageInfoSpecificVmdk> for ImageInfoSpecific {
    fn from(val: ImageInfoSpecificVmdk) -> Self {
        Self::vmdk(ImageInfoSpecificVmdkWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum ImageInfoSpecificQCow2Encryption {
	#[serde(rename = "luks")]
	luks(QCryptoBlockInfoLUKS),
	#[serde(rename = "aes")]
	aes,
}

impl ImageInfoSpecificQCow2Encryption {
    pub fn format(&self) -> BlockdevQcow2EncryptionFormat {
        match *self {

            ImageInfoSpecificQCow2Encryption::luks { .. } => BlockdevQcow2EncryptionFormat::luks,

            ImageInfoSpecificQCow2Encryption::aes { .. } => BlockdevQcow2EncryptionFormat::aes,

        }
    }
}

impl From<QCryptoBlockInfoLUKS> for ImageInfoSpecificQCow2Encryption {
    fn from(val: QCryptoBlockInfoLUKS) -> Self {
        Self::luks(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputEvent {
	#[serde(rename = "abs")]
	abs(InputMoveEventWrapper),
	#[serde(rename = "btn")]
	btn(InputBtnEventWrapper),
	#[serde(rename = "key")]
	key(InputKeyEventWrapper),
	#[serde(rename = "mtt")]
	mtt(InputMultiTouchEventWrapper),
	#[serde(rename = "rel")]
	rel(InputMoveEventWrapper),
}

impl InputEvent {
    pub fn type_(&self) -> InputEventKind {
        match *self {

            InputEvent::abs { .. } => InputEventKind::abs,

            InputEvent::btn { .. } => InputEventKind::btn,

            InputEvent::key { .. } => InputEventKind::key,

            InputEvent::mtt { .. } => InputEventKind::mtt,

            InputEvent::rel { .. } => InputEventKind::rel,

        }
    }
}

impl From<InputBtnEventWrapper> for InputEvent {
    fn from(val: InputBtnEventWrapper) -> Self {
        Self::btn(val)
    }
}

impl From<InputBtnEvent> for InputEvent {
    fn from(val: InputBtnEvent) -> Self {
        Self::btn(InputBtnEventWrapper::from(val))
    }
}

impl From<InputKeyEventWrapper> for InputEvent {
    fn from(val: InputKeyEventWrapper) -> Self {
        Self::key(val)
    }
}

impl From<InputKeyEvent> for InputEvent {
    fn from(val: InputKeyEvent) -> Self {
        Self::key(InputKeyEventWrapper::from(val))
    }
}

impl From<InputMultiTouchEventWrapper> for InputEvent {
    fn from(val: InputMultiTouchEventWrapper) -> Self {
        Self::mtt(val)
    }
}

impl From<InputMultiTouchEvent> for InputEvent {
    fn from(val: InputMultiTouchEvent) -> Self {
        Self::mtt(InputMultiTouchEventWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KeyValue {
	#[serde(rename = "number")]
	number(IntWrapper),
	#[serde(rename = "qcode")]
	qcode(QKeyCodeWrapper),
}

impl KeyValue {
    pub fn type_(&self) -> KeyValueKind {
        match *self {

            KeyValue::number { .. } => KeyValueKind::number,

            KeyValue::qcode { .. } => KeyValueKind::qcode,

        }
    }
}

impl From<IntWrapper> for KeyValue {
    fn from(val: IntWrapper) -> Self {
        Self::number(val)
    }
}

impl From<i64> for KeyValue {
    fn from(val: i64) -> Self {
        Self::number(IntWrapper::from(val))
    }
}

impl From<QKeyCodeWrapper> for KeyValue {
    fn from(val: QKeyCodeWrapper) -> Self {
        Self::qcode(val)
    }
}

impl From<QKeyCode> for KeyValue {
    fn from(val: QKeyCode) -> Self {
        Self::qcode(QKeyCodeWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MemoryDeviceInfo {
	#[serde(rename = "dimm")]
	dimm(PCDIMMDeviceInfoWrapper),
	#[serde(rename = "hv-balloon")]
	hv_balloon(HvBalloonDeviceInfoWrapper),
	#[serde(rename = "nvdimm")]
	nvdimm(PCDIMMDeviceInfoWrapper),
	#[serde(rename = "sgx-epc")]
	sgx_epc(SgxEPCDeviceInfoWrapper),
	#[serde(rename = "virtio-mem")]
	virtio_mem(VirtioMEMDeviceInfoWrapper),
	#[serde(rename = "virtio-pmem")]
	virtio_pmem(VirtioPMEMDeviceInfoWrapper),
}

impl MemoryDeviceInfo {
    pub fn type_(&self) -> MemoryDeviceInfoKind {
        match *self {

            MemoryDeviceInfo::dimm { .. } => MemoryDeviceInfoKind::dimm,

            MemoryDeviceInfo::hv_balloon { .. } => MemoryDeviceInfoKind::hv_balloon,

            MemoryDeviceInfo::nvdimm { .. } => MemoryDeviceInfoKind::nvdimm,

            MemoryDeviceInfo::sgx_epc { .. } => MemoryDeviceInfoKind::sgx_epc,

            MemoryDeviceInfo::virtio_mem { .. } => MemoryDeviceInfoKind::virtio_mem,

            MemoryDeviceInfo::virtio_pmem { .. } => MemoryDeviceInfoKind::virtio_pmem,

        }
    }
}

impl From<HvBalloonDeviceInfoWrapper> for MemoryDeviceInfo {
    fn from(val: HvBalloonDeviceInfoWrapper) -> Self {
        Self::hv_balloon(val)
    }
}

impl From<HvBalloonDeviceInfo> for MemoryDeviceInfo {
    fn from(val: HvBalloonDeviceInfo) -> Self {
        Self::hv_balloon(HvBalloonDeviceInfoWrapper::from(val))
    }
}

impl From<SgxEPCDeviceInfoWrapper> for MemoryDeviceInfo {
    fn from(val: SgxEPCDeviceInfoWrapper) -> Self {
        Self::sgx_epc(val)
    }
}

impl From<SgxEPCDeviceInfo> for MemoryDeviceInfo {
    fn from(val: SgxEPCDeviceInfo) -> Self {
        Self::sgx_epc(SgxEPCDeviceInfoWrapper::from(val))
    }
}

impl From<VirtioMEMDeviceInfoWrapper> for MemoryDeviceInfo {
    fn from(val: VirtioMEMDeviceInfoWrapper) -> Self {
        Self::virtio_mem(val)
    }
}

impl From<VirtioMEMDeviceInfo> for MemoryDeviceInfo {
    fn from(val: VirtioMEMDeviceInfo) -> Self {
        Self::virtio_mem(VirtioMEMDeviceInfoWrapper::from(val))
    }
}

impl From<VirtioPMEMDeviceInfoWrapper> for MemoryDeviceInfo {
    fn from(val: VirtioPMEMDeviceInfoWrapper) -> Self {
        Self::virtio_pmem(val)
    }
}

impl From<VirtioPMEMDeviceInfo> for MemoryDeviceInfo {
    fn from(val: VirtioPMEMDeviceInfo) -> Self {
        Self::virtio_pmem(VirtioPMEMDeviceInfoWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "transport")]
pub enum MigrationAddress {
	#[serde(rename = "exec")]
	exec(MigrationExecCommand),
	#[serde(rename = "file")]
	file(FileMigrationArgs),
	#[serde(rename = "rdma")]
	rdma(InetSocketAddress),
	#[serde(rename = "socket")]
	socket(SocketAddress),
}

impl MigrationAddress {
    pub fn transport(&self) -> MigrationAddressType {
        match *self {

            MigrationAddress::exec { .. } => MigrationAddressType::exec,

            MigrationAddress::file { .. } => MigrationAddressType::file,

            MigrationAddress::rdma { .. } => MigrationAddressType::rdma,

            MigrationAddress::socket { .. } => MigrationAddressType::socket,

        }
    }
}

impl From<MigrationExecCommand> for MigrationAddress {
    fn from(val: MigrationExecCommand) -> Self {
        Self::exec(val)
    }
}

impl From<FileMigrationArgs> for MigrationAddress {
    fn from(val: FileMigrationArgs) -> Self {
        Self::file(val)
    }
}

impl From<InetSocketAddress> for MigrationAddress {
    fn from(val: InetSocketAddress) -> Self {
        Self::rdma(val)
    }
}

impl From<SocketAddress> for MigrationAddress {
    fn from(val: SocketAddress) -> Self {
        Self::socket(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Netdev {
	#[serde(rename = "af-xdp")]
	af_xdp {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "af-xdp")]
af_xdp: NetdevAFXDPOptions,
	},
	#[serde(rename = "bridge")]
	bridge {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "bridge")]
bridge: NetdevBridgeOptions,
	},
	#[serde(rename = "dgram")]
	dgram {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "dgram")]
dgram: NetdevDgramOptions,
	},
	#[serde(rename = "hubport")]
	hubport {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "hubport")]
hubport: NetdevHubPortOptions,
	},
	#[serde(rename = "l2tpv3")]
	l2tpv3 {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "l2tpv3")]
l2tpv3: NetdevL2TPv3Options,
	},
	#[serde(rename = "netmap")]
	netmap {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "netmap")]
netmap: NetdevNetmapOptions,
	},
	#[serde(rename = "nic")]
	nic {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "nic")]
nic: NetLegacyNicOptions,
	},
	#[serde(rename = "socket")]
	socket {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "socket")]
socket: NetdevSocketOptions,
	},
	#[serde(rename = "stream")]
	stream {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "stream")]
stream: NetdevStreamOptions,
	},
	#[serde(rename = "tap")]
	tap {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "tap")]
tap: NetdevTapOptions,
	},
	#[serde(rename = "user")]
	user {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "user")]
user: NetdevUserOptions,
	},
	#[serde(rename = "vde")]
	vde {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vde")]
vde: NetdevVdeOptions,
	},
	#[serde(rename = "vhost-user")]
	vhost_user {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vhost-user")]
vhost_user: NetdevVhostUserOptions,
	},
	#[serde(rename = "vhost-vdpa")]
	vhost_vdpa {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vhost-vdpa")]
vhost_vdpa: NetdevVhostVDPAOptions,
	},
	#[serde(rename = "vmnet-bridged")]
	vmnet_bridged {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vmnet-bridged")]
vmnet_bridged: NetdevVmnetBridgedOptions,
	},
	#[serde(rename = "vmnet-host")]
	vmnet_host {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vmnet-host")]
vmnet_host: NetdevVmnetHostOptions,
	},
	#[serde(rename = "vmnet-shared")]
	vmnet_shared {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "vmnet-shared")]
vmnet_shared: NetdevVmnetSharedOptions,
	},
	#[serde(rename = "none")]
	none(::std::string::String),
}

impl Netdev {
    pub fn type_(&self) -> NetClientDriver {
        match *self {

            Netdev::af_xdp { .. } => NetClientDriver::af_xdp,

            Netdev::bridge { .. } => NetClientDriver::bridge,

            Netdev::dgram { .. } => NetClientDriver::dgram,

            Netdev::hubport { .. } => NetClientDriver::hubport,

            Netdev::l2tpv3 { .. } => NetClientDriver::l2tpv3,

            Netdev::netmap { .. } => NetClientDriver::netmap,

            Netdev::nic { .. } => NetClientDriver::nic,

            Netdev::socket { .. } => NetClientDriver::socket,

            Netdev::stream { .. } => NetClientDriver::stream,

            Netdev::tap { .. } => NetClientDriver::tap,

            Netdev::user { .. } => NetClientDriver::user,

            Netdev::vde { .. } => NetClientDriver::vde,

            Netdev::vhost_user { .. } => NetClientDriver::vhost_user,

            Netdev::vhost_vdpa { .. } => NetClientDriver::vhost_vdpa,

            Netdev::vmnet_bridged { .. } => NetClientDriver::vmnet_bridged,

            Netdev::vmnet_host { .. } => NetClientDriver::vmnet_host,

            Netdev::vmnet_shared { .. } => NetClientDriver::vmnet_shared,

            Netdev::none { .. } => NetClientDriver::none,

        }
    }
}

impl From<(NetdevAFXDPOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevAFXDPOptions, ::std::string::String)) -> Self {
        Self::af_xdp {
            af_xdp: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevBridgeOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevBridgeOptions, ::std::string::String)) -> Self {
        Self::bridge {
            bridge: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevDgramOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevDgramOptions, ::std::string::String)) -> Self {
        Self::dgram {
            dgram: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevHubPortOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevHubPortOptions, ::std::string::String)) -> Self {
        Self::hubport {
            hubport: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevL2TPv3Options, ::std::string::String)> for Netdev {
    fn from(val: (NetdevL2TPv3Options, ::std::string::String)) -> Self {
        Self::l2tpv3 {
            l2tpv3: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevNetmapOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevNetmapOptions, ::std::string::String)) -> Self {
        Self::netmap {
            netmap: val.0,
            id: val.1,

        }
    }
}

impl From<(NetLegacyNicOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetLegacyNicOptions, ::std::string::String)) -> Self {
        Self::nic {
            nic: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevSocketOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevSocketOptions, ::std::string::String)) -> Self {
        Self::socket {
            socket: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevStreamOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevStreamOptions, ::std::string::String)) -> Self {
        Self::stream {
            stream: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevTapOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevTapOptions, ::std::string::String)) -> Self {
        Self::tap {
            tap: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevUserOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevUserOptions, ::std::string::String)) -> Self {
        Self::user {
            user: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVdeOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVdeOptions, ::std::string::String)) -> Self {
        Self::vde {
            vde: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVhostUserOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVhostUserOptions, ::std::string::String)) -> Self {
        Self::vhost_user {
            vhost_user: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVhostVDPAOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVhostVDPAOptions, ::std::string::String)) -> Self {
        Self::vhost_vdpa {
            vhost_vdpa: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVmnetBridgedOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVmnetBridgedOptions, ::std::string::String)) -> Self {
        Self::vmnet_bridged {
            vmnet_bridged: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVmnetHostOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVmnetHostOptions, ::std::string::String)) -> Self {
        Self::vmnet_host {
            vmnet_host: val.0,
            id: val.1,

        }
    }
}

impl From<(NetdevVmnetSharedOptions, ::std::string::String)> for Netdev {
    fn from(val: (NetdevVmnetSharedOptions, ::std::string::String)) -> Self {
        Self::vmnet_shared {
            vmnet_shared: val.0,
            id: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NumaOptions {
	#[serde(rename = "cpu")]
	cpu(NumaCpuOptions),
	#[serde(rename = "dist")]
	dist(NumaDistOptions),
	#[serde(rename = "hmat-cache")]
	hmat_cache(NumaHmatCacheOptions),
	#[serde(rename = "hmat-lb")]
	hmat_lb(NumaHmatLBOptions),
	#[serde(rename = "node")]
	node(NumaNodeOptions),
}

impl NumaOptions {
    pub fn type_(&self) -> NumaOptionsType {
        match *self {

            NumaOptions::cpu { .. } => NumaOptionsType::cpu,

            NumaOptions::dist { .. } => NumaOptionsType::dist,

            NumaOptions::hmat_cache { .. } => NumaOptionsType::hmat_cache,

            NumaOptions::hmat_lb { .. } => NumaOptionsType::hmat_lb,

            NumaOptions::node { .. } => NumaOptionsType::node,

        }
    }
}

impl From<NumaCpuOptions> for NumaOptions {
    fn from(val: NumaCpuOptions) -> Self {
        Self::cpu(val)
    }
}

impl From<NumaDistOptions> for NumaOptions {
    fn from(val: NumaDistOptions) -> Self {
        Self::dist(val)
    }
}

impl From<NumaHmatCacheOptions> for NumaOptions {
    fn from(val: NumaHmatCacheOptions) -> Self {
        Self::hmat_cache(val)
    }
}

impl From<NumaHmatLBOptions> for NumaOptions {
    fn from(val: NumaHmatLBOptions) -> Self {
        Self::hmat_lb(val)
    }
}

impl From<NumaNodeOptions> for NumaOptions {
    fn from(val: NumaNodeOptions) -> Self {
        Self::node(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "qom-type")]
pub enum ObjectOptions {
	#[serde(rename = "acpi-generic-initiator")]
	acpi_generic_initiator {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "acpi-generic-initiator")]
acpi_generic_initiator: AcpiGenericInitiatorProperties,
	},
	#[serde(rename = "authz-list")]
	authz_list {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "authz-list")]
authz_list: AuthZListProperties,
	},
	#[serde(rename = "authz-listfile")]
	authz_listfile {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "authz-listfile")]
authz_listfile: AuthZListFileProperties,
	},
	#[serde(rename = "authz-pam")]
	authz_pam {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "authz-pam")]
authz_pam: AuthZPAMProperties,
	},
	#[serde(rename = "authz-simple")]
	authz_simple {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "authz-simple")]
authz_simple: AuthZSimpleProperties,
	},
	#[serde(rename = "can-host-socketcan")]
	can_host_socketcan {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "can-host-socketcan")]
can_host_socketcan: CanHostSocketcanProperties,
	},
	#[serde(rename = "colo-compare")]
	colo_compare {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "colo-compare")]
colo_compare: ColoCompareProperties,
	},
	#[serde(rename = "cryptodev-backend")]
	cryptodev_backend {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "cryptodev-backend")]
cryptodev_backend: CryptodevBackendProperties,
	},
	#[serde(rename = "cryptodev-backend-builtin")]
	cryptodev_backend_builtin {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "cryptodev-backend-builtin")]
cryptodev_backend_builtin: CryptodevBackendProperties,
	},
	#[serde(rename = "cryptodev-backend-lkcf")]
	cryptodev_backend_lkcf {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "cryptodev-backend-lkcf")]
cryptodev_backend_lkcf: CryptodevBackendProperties,
	},
	#[serde(rename = "cryptodev-vhost-user")]
	cryptodev_vhost_user {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "cryptodev-vhost-user")]
cryptodev_vhost_user: CryptodevVhostUserProperties,
	},
	#[serde(rename = "dbus-vmstate")]
	dbus_vmstate {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "dbus-vmstate")]
dbus_vmstate: DBusVMStateProperties,
	},
	#[serde(rename = "filter-buffer")]
	filter_buffer {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-buffer")]
filter_buffer: FilterBufferProperties,
	},
	#[serde(rename = "filter-dump")]
	filter_dump {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-dump")]
filter_dump: FilterDumpProperties,
	},
	#[serde(rename = "filter-mirror")]
	filter_mirror {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-mirror")]
filter_mirror: FilterMirrorProperties,
	},
	#[serde(rename = "filter-redirector")]
	filter_redirector {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-redirector")]
filter_redirector: FilterRedirectorProperties,
	},
	#[serde(rename = "filter-replay")]
	filter_replay {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-replay")]
filter_replay: NetfilterProperties,
	},
	#[serde(rename = "filter-rewriter")]
	filter_rewriter {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "filter-rewriter")]
filter_rewriter: FilterRewriterProperties,
	},
	#[serde(rename = "input-barrier")]
	input_barrier {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "input-barrier")]
input_barrier: InputBarrierProperties,
	},
	#[serde(rename = "input-linux")]
	input_linux {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "input-linux")]
input_linux: InputLinuxProperties,
	},
	#[serde(rename = "iommufd")]
	iommufd {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "iommufd")]
iommufd: IOMMUFDProperties,
	},
	#[serde(rename = "iothread")]
	iothread {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "iothread")]
iothread: IothreadProperties,
	},
	#[serde(rename = "main-loop")]
	main_loop {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "main-loop")]
main_loop: MainLoopProperties,
	},
	#[serde(rename = "memory-backend-epc")]
	memory_backend_epc {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "memory-backend-epc")]
memory_backend_epc: MemoryBackendEpcProperties,
	},
	#[serde(rename = "memory-backend-file")]
	memory_backend_file {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "memory-backend-file")]
memory_backend_file: MemoryBackendFileProperties,
	},
	#[serde(rename = "memory-backend-memfd")]
	memory_backend_memfd {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "memory-backend-memfd")]
memory_backend_memfd: MemoryBackendMemfdProperties,
	},
	#[serde(rename = "memory-backend-ram")]
	memory_backend_ram {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "memory-backend-ram")]
memory_backend_ram: MemoryBackendProperties,
	},
	#[serde(rename = "memory-backend-shm")]
	memory_backend_shm {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "memory-backend-shm")]
memory_backend_shm: MemoryBackendShmProperties,
	},
	#[serde(rename = "pr-manager-helper")]
	pr_manager_helper {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "pr-manager-helper")]
pr_manager_helper: PrManagerHelperProperties,
	},
	#[serde(rename = "qtest")]
	qtest {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "qtest")]
qtest: QtestProperties,
	},
	#[serde(rename = "rng-builtin")]
	rng_builtin {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "rng-builtin")]
rng_builtin: RngProperties,
	},
	#[serde(rename = "rng-egd")]
	rng_egd {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "rng-egd")]
rng_egd: RngEgdProperties,
	},
	#[serde(rename = "rng-random")]
	rng_random {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "rng-random")]
rng_random: RngRandomProperties,
	},
	#[serde(rename = "secret")]
	secret {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "secret")]
secret: SecretProperties,
	},
	#[serde(rename = "secret_keyring")]
	secret_keyring {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "secret_keyring")]
secret_keyring: SecretKeyringProperties,
	},
	#[serde(rename = "sev-guest")]
	sev_guest {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "sev-guest")]
sev_guest: SevGuestProperties,
	},
	#[serde(rename = "sev-snp-guest")]
	sev_snp_guest {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "sev-snp-guest")]
sev_snp_guest: SevSnpGuestProperties,
	},
	#[serde(rename = "thread-context")]
	thread_context {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "thread-context")]
thread_context: ThreadContextProperties,
	},
	#[serde(rename = "throttle-group")]
	throttle_group {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "throttle-group")]
throttle_group: ThrottleGroupProperties,
	},
	#[serde(rename = "tls-cipher-suites")]
	tls_cipher_suites {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "tls-cipher-suites")]
tls_cipher_suites: TlsCredsProperties,
	},
	#[serde(rename = "tls-creds-anon")]
	tls_creds_anon {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "tls-creds-anon")]
tls_creds_anon: TlsCredsAnonProperties,
	},
	#[serde(rename = "tls-creds-psk")]
	tls_creds_psk {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "tls-creds-psk")]
tls_creds_psk: TlsCredsPskProperties,
	},
	#[serde(rename = "tls-creds-x509")]
	tls_creds_x509 {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "tls-creds-x509")]
tls_creds_x509: TlsCredsX509Properties,
	},
	#[serde(rename = "x-remote-object")]
	x_remote_object {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "x-remote-object")]
x_remote_object: RemoteObjectProperties,
	},
	#[serde(rename = "x-vfio-user-server")]
	x_vfio_user_server {
		#[serde(rename = "id")]
id: ::std::string::String,
		#[serde(flatten)] #[serde(rename = "x-vfio-user-server")]
x_vfio_user_server: VfioUserServerProperties,
	},
	#[serde(rename = "can-bus")]
	can_bus(::std::string::String),
	#[serde(rename = "pef-guest")]
	pef_guest(::std::string::String),
	#[serde(rename = "s390-pv-guest")]
	s390_pv_guest(::std::string::String),
}

impl ObjectOptions {
    pub fn qom_type(&self) -> ObjectType {
        match *self {

            ObjectOptions::acpi_generic_initiator { .. } => ObjectType::acpi_generic_initiator,

            ObjectOptions::authz_list { .. } => ObjectType::authz_list,

            ObjectOptions::authz_listfile { .. } => ObjectType::authz_listfile,

            ObjectOptions::authz_pam { .. } => ObjectType::authz_pam,

            ObjectOptions::authz_simple { .. } => ObjectType::authz_simple,

            ObjectOptions::can_host_socketcan { .. } => ObjectType::can_host_socketcan,

            ObjectOptions::colo_compare { .. } => ObjectType::colo_compare,

            ObjectOptions::cryptodev_backend { .. } => ObjectType::cryptodev_backend,

            ObjectOptions::cryptodev_backend_builtin { .. } => ObjectType::cryptodev_backend_builtin,

            ObjectOptions::cryptodev_backend_lkcf { .. } => ObjectType::cryptodev_backend_lkcf,

            ObjectOptions::cryptodev_vhost_user { .. } => ObjectType::cryptodev_vhost_user,

            ObjectOptions::dbus_vmstate { .. } => ObjectType::dbus_vmstate,

            ObjectOptions::filter_buffer { .. } => ObjectType::filter_buffer,

            ObjectOptions::filter_dump { .. } => ObjectType::filter_dump,

            ObjectOptions::filter_mirror { .. } => ObjectType::filter_mirror,

            ObjectOptions::filter_redirector { .. } => ObjectType::filter_redirector,

            ObjectOptions::filter_replay { .. } => ObjectType::filter_replay,

            ObjectOptions::filter_rewriter { .. } => ObjectType::filter_rewriter,

            ObjectOptions::input_barrier { .. } => ObjectType::input_barrier,

            ObjectOptions::input_linux { .. } => ObjectType::input_linux,

            ObjectOptions::iommufd { .. } => ObjectType::iommufd,

            ObjectOptions::iothread { .. } => ObjectType::iothread,

            ObjectOptions::main_loop { .. } => ObjectType::main_loop,

            ObjectOptions::memory_backend_epc { .. } => ObjectType::memory_backend_epc,

            ObjectOptions::memory_backend_file { .. } => ObjectType::memory_backend_file,

            ObjectOptions::memory_backend_memfd { .. } => ObjectType::memory_backend_memfd,

            ObjectOptions::memory_backend_ram { .. } => ObjectType::memory_backend_ram,

            ObjectOptions::memory_backend_shm { .. } => ObjectType::memory_backend_shm,

            ObjectOptions::pr_manager_helper { .. } => ObjectType::pr_manager_helper,

            ObjectOptions::qtest { .. } => ObjectType::qtest,

            ObjectOptions::rng_builtin { .. } => ObjectType::rng_builtin,

            ObjectOptions::rng_egd { .. } => ObjectType::rng_egd,

            ObjectOptions::rng_random { .. } => ObjectType::rng_random,

            ObjectOptions::secret { .. } => ObjectType::secret,

            ObjectOptions::secret_keyring { .. } => ObjectType::secret_keyring,

            ObjectOptions::sev_guest { .. } => ObjectType::sev_guest,

            ObjectOptions::sev_snp_guest { .. } => ObjectType::sev_snp_guest,

            ObjectOptions::thread_context { .. } => ObjectType::thread_context,

            ObjectOptions::throttle_group { .. } => ObjectType::throttle_group,

            ObjectOptions::tls_cipher_suites { .. } => ObjectType::tls_cipher_suites,

            ObjectOptions::tls_creds_anon { .. } => ObjectType::tls_creds_anon,

            ObjectOptions::tls_creds_psk { .. } => ObjectType::tls_creds_psk,

            ObjectOptions::tls_creds_x509 { .. } => ObjectType::tls_creds_x509,

            ObjectOptions::x_remote_object { .. } => ObjectType::x_remote_object,

            ObjectOptions::x_vfio_user_server { .. } => ObjectType::x_vfio_user_server,

            ObjectOptions::can_bus { .. } => ObjectType::can_bus,

            ObjectOptions::pef_guest { .. } => ObjectType::pef_guest,

            ObjectOptions::s390_pv_guest { .. } => ObjectType::s390_pv_guest,

        }
    }
}

impl From<(AcpiGenericInitiatorProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (AcpiGenericInitiatorProperties, ::std::string::String)) -> Self {
        Self::acpi_generic_initiator {
            acpi_generic_initiator: val.0,
            id: val.1,

        }
    }
}

impl From<(AuthZListProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (AuthZListProperties, ::std::string::String)) -> Self {
        Self::authz_list {
            authz_list: val.0,
            id: val.1,

        }
    }
}

impl From<(AuthZListFileProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (AuthZListFileProperties, ::std::string::String)) -> Self {
        Self::authz_listfile {
            authz_listfile: val.0,
            id: val.1,

        }
    }
}

impl From<(AuthZPAMProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (AuthZPAMProperties, ::std::string::String)) -> Self {
        Self::authz_pam {
            authz_pam: val.0,
            id: val.1,

        }
    }
}

impl From<(AuthZSimpleProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (AuthZSimpleProperties, ::std::string::String)) -> Self {
        Self::authz_simple {
            authz_simple: val.0,
            id: val.1,

        }
    }
}

impl From<(CanHostSocketcanProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (CanHostSocketcanProperties, ::std::string::String)) -> Self {
        Self::can_host_socketcan {
            can_host_socketcan: val.0,
            id: val.1,

        }
    }
}

impl From<(ColoCompareProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (ColoCompareProperties, ::std::string::String)) -> Self {
        Self::colo_compare {
            colo_compare: val.0,
            id: val.1,

        }
    }
}

impl From<(CryptodevVhostUserProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (CryptodevVhostUserProperties, ::std::string::String)) -> Self {
        Self::cryptodev_vhost_user {
            cryptodev_vhost_user: val.0,
            id: val.1,

        }
    }
}

impl From<(DBusVMStateProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (DBusVMStateProperties, ::std::string::String)) -> Self {
        Self::dbus_vmstate {
            dbus_vmstate: val.0,
            id: val.1,

        }
    }
}

impl From<(FilterBufferProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (FilterBufferProperties, ::std::string::String)) -> Self {
        Self::filter_buffer {
            filter_buffer: val.0,
            id: val.1,

        }
    }
}

impl From<(FilterDumpProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (FilterDumpProperties, ::std::string::String)) -> Self {
        Self::filter_dump {
            filter_dump: val.0,
            id: val.1,

        }
    }
}

impl From<(FilterMirrorProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (FilterMirrorProperties, ::std::string::String)) -> Self {
        Self::filter_mirror {
            filter_mirror: val.0,
            id: val.1,

        }
    }
}

impl From<(FilterRedirectorProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (FilterRedirectorProperties, ::std::string::String)) -> Self {
        Self::filter_redirector {
            filter_redirector: val.0,
            id: val.1,

        }
    }
}

impl From<(NetfilterProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (NetfilterProperties, ::std::string::String)) -> Self {
        Self::filter_replay {
            filter_replay: val.0,
            id: val.1,

        }
    }
}

impl From<(FilterRewriterProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (FilterRewriterProperties, ::std::string::String)) -> Self {
        Self::filter_rewriter {
            filter_rewriter: val.0,
            id: val.1,

        }
    }
}

impl From<(InputBarrierProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (InputBarrierProperties, ::std::string::String)) -> Self {
        Self::input_barrier {
            input_barrier: val.0,
            id: val.1,

        }
    }
}

impl From<(InputLinuxProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (InputLinuxProperties, ::std::string::String)) -> Self {
        Self::input_linux {
            input_linux: val.0,
            id: val.1,

        }
    }
}

impl From<(IOMMUFDProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (IOMMUFDProperties, ::std::string::String)) -> Self {
        Self::iommufd {
            iommufd: val.0,
            id: val.1,

        }
    }
}

impl From<(IothreadProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (IothreadProperties, ::std::string::String)) -> Self {
        Self::iothread {
            iothread: val.0,
            id: val.1,

        }
    }
}

impl From<(MainLoopProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MainLoopProperties, ::std::string::String)) -> Self {
        Self::main_loop {
            main_loop: val.0,
            id: val.1,

        }
    }
}

impl From<(MemoryBackendEpcProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MemoryBackendEpcProperties, ::std::string::String)) -> Self {
        Self::memory_backend_epc {
            memory_backend_epc: val.0,
            id: val.1,

        }
    }
}

impl From<(MemoryBackendFileProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MemoryBackendFileProperties, ::std::string::String)) -> Self {
        Self::memory_backend_file {
            memory_backend_file: val.0,
            id: val.1,

        }
    }
}

impl From<(MemoryBackendMemfdProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MemoryBackendMemfdProperties, ::std::string::String)) -> Self {
        Self::memory_backend_memfd {
            memory_backend_memfd: val.0,
            id: val.1,

        }
    }
}

impl From<(MemoryBackendProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MemoryBackendProperties, ::std::string::String)) -> Self {
        Self::memory_backend_ram {
            memory_backend_ram: val.0,
            id: val.1,

        }
    }
}

impl From<(MemoryBackendShmProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (MemoryBackendShmProperties, ::std::string::String)) -> Self {
        Self::memory_backend_shm {
            memory_backend_shm: val.0,
            id: val.1,

        }
    }
}

impl From<(PrManagerHelperProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (PrManagerHelperProperties, ::std::string::String)) -> Self {
        Self::pr_manager_helper {
            pr_manager_helper: val.0,
            id: val.1,

        }
    }
}

impl From<(QtestProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (QtestProperties, ::std::string::String)) -> Self {
        Self::qtest {
            qtest: val.0,
            id: val.1,

        }
    }
}

impl From<(RngProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (RngProperties, ::std::string::String)) -> Self {
        Self::rng_builtin {
            rng_builtin: val.0,
            id: val.1,

        }
    }
}

impl From<(RngEgdProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (RngEgdProperties, ::std::string::String)) -> Self {
        Self::rng_egd {
            rng_egd: val.0,
            id: val.1,

        }
    }
}

impl From<(RngRandomProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (RngRandomProperties, ::std::string::String)) -> Self {
        Self::rng_random {
            rng_random: val.0,
            id: val.1,

        }
    }
}

impl From<(SecretProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (SecretProperties, ::std::string::String)) -> Self {
        Self::secret {
            secret: val.0,
            id: val.1,

        }
    }
}

impl From<(SecretKeyringProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (SecretKeyringProperties, ::std::string::String)) -> Self {
        Self::secret_keyring {
            secret_keyring: val.0,
            id: val.1,

        }
    }
}

impl From<(SevGuestProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (SevGuestProperties, ::std::string::String)) -> Self {
        Self::sev_guest {
            sev_guest: val.0,
            id: val.1,

        }
    }
}

impl From<(SevSnpGuestProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (SevSnpGuestProperties, ::std::string::String)) -> Self {
        Self::sev_snp_guest {
            sev_snp_guest: val.0,
            id: val.1,

        }
    }
}

impl From<(ThreadContextProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (ThreadContextProperties, ::std::string::String)) -> Self {
        Self::thread_context {
            thread_context: val.0,
            id: val.1,

        }
    }
}

impl From<(ThrottleGroupProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (ThrottleGroupProperties, ::std::string::String)) -> Self {
        Self::throttle_group {
            throttle_group: val.0,
            id: val.1,

        }
    }
}

impl From<(TlsCredsProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (TlsCredsProperties, ::std::string::String)) -> Self {
        Self::tls_cipher_suites {
            tls_cipher_suites: val.0,
            id: val.1,

        }
    }
}

impl From<(TlsCredsAnonProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (TlsCredsAnonProperties, ::std::string::String)) -> Self {
        Self::tls_creds_anon {
            tls_creds_anon: val.0,
            id: val.1,

        }
    }
}

impl From<(TlsCredsPskProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (TlsCredsPskProperties, ::std::string::String)) -> Self {
        Self::tls_creds_psk {
            tls_creds_psk: val.0,
            id: val.1,

        }
    }
}

impl From<(TlsCredsX509Properties, ::std::string::String)> for ObjectOptions {
    fn from(val: (TlsCredsX509Properties, ::std::string::String)) -> Self {
        Self::tls_creds_x509 {
            tls_creds_x509: val.0,
            id: val.1,

        }
    }
}

impl From<(RemoteObjectProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (RemoteObjectProperties, ::std::string::String)) -> Self {
        Self::x_remote_object {
            x_remote_object: val.0,
            id: val.1,

        }
    }
}

impl From<(VfioUserServerProperties, ::std::string::String)> for ObjectOptions {
    fn from(val: (VfioUserServerProperties, ::std::string::String)) -> Self {
        Self::x_vfio_user_server {
            x_vfio_user_server: val.0,
            id: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "alg")]
pub enum QCryptoAkCipherOptions {
	#[serde(rename = "rsa")]
	rsa(QCryptoAkCipherOptionsRSA),
}

impl QCryptoAkCipherOptions {
    pub fn alg(&self) -> QCryptoAkCipherAlgorithm {
        match *self {

            QCryptoAkCipherOptions::rsa { .. } => QCryptoAkCipherAlgorithm::rsa,

        }
    }
}

impl From<QCryptoAkCipherOptionsRSA> for QCryptoAkCipherOptions {
    fn from(val: QCryptoAkCipherOptionsRSA) -> Self {
        Self::rsa(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum QCryptoBlockAmendOptions {
	#[serde(rename = "luks")]
	luks(QCryptoBlockAmendOptionsLUKS),
	#[serde(rename = "qcow")]
	qcow,
}

impl QCryptoBlockAmendOptions {
    pub fn format(&self) -> QCryptoBlockFormat {
        match *self {

            QCryptoBlockAmendOptions::luks { .. } => QCryptoBlockFormat::luks,

            QCryptoBlockAmendOptions::qcow { .. } => QCryptoBlockFormat::qcow,

        }
    }
}

impl From<QCryptoBlockAmendOptionsLUKS> for QCryptoBlockAmendOptions {
    fn from(val: QCryptoBlockAmendOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum QCryptoBlockCreateOptions {
	#[serde(rename = "luks")]
	luks(QCryptoBlockCreateOptionsLUKS),
	#[serde(rename = "qcow")]
	qcow(QCryptoBlockOptionsQCow),
}

impl QCryptoBlockCreateOptions {
    pub fn format(&self) -> QCryptoBlockFormat {
        match *self {

            QCryptoBlockCreateOptions::luks { .. } => QCryptoBlockFormat::luks,

            QCryptoBlockCreateOptions::qcow { .. } => QCryptoBlockFormat::qcow,

        }
    }
}

impl From<QCryptoBlockCreateOptionsLUKS> for QCryptoBlockCreateOptions {
    fn from(val: QCryptoBlockCreateOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

impl From<QCryptoBlockOptionsQCow> for QCryptoBlockCreateOptions {
    fn from(val: QCryptoBlockOptionsQCow) -> Self {
        Self::qcow(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum QCryptoBlockInfo {
	#[serde(rename = "luks")]
	luks(QCryptoBlockInfoLUKS),
	#[serde(rename = "qcow")]
	qcow,
}

impl QCryptoBlockInfo {
    pub fn format(&self) -> QCryptoBlockFormat {
        match *self {

            QCryptoBlockInfo::luks { .. } => QCryptoBlockFormat::luks,

            QCryptoBlockInfo::qcow { .. } => QCryptoBlockFormat::qcow,

        }
    }
}

impl From<QCryptoBlockInfoLUKS> for QCryptoBlockInfo {
    fn from(val: QCryptoBlockInfoLUKS) -> Self {
        Self::luks(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum QCryptoBlockOpenOptions {
	#[serde(rename = "luks")]
	luks(QCryptoBlockOptionsLUKS),
	#[serde(rename = "qcow")]
	qcow(QCryptoBlockOptionsQCow),
}

impl QCryptoBlockOpenOptions {
    pub fn format(&self) -> QCryptoBlockFormat {
        match *self {

            QCryptoBlockOpenOptions::luks { .. } => QCryptoBlockFormat::luks,

            QCryptoBlockOpenOptions::qcow { .. } => QCryptoBlockFormat::qcow,

        }
    }
}

impl From<QCryptoBlockOptionsLUKS> for QCryptoBlockOpenOptions {
    fn from(val: QCryptoBlockOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

impl From<QCryptoBlockOptionsQCow> for QCryptoBlockOpenOptions {
    fn from(val: QCryptoBlockOptionsQCow) -> Self {
        Self::qcow(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum RbdEncryptionCreateOptions {
	#[serde(rename = "luks")]
	luks(RbdEncryptionCreateOptionsLUKS),
	#[serde(rename = "luks2")]
	luks2(RbdEncryptionCreateOptionsLUKS2),
	#[serde(rename = "luks-any")]
	luks_any,
}

impl RbdEncryptionCreateOptions {
    pub fn format(&self) -> RbdImageEncryptionFormat {
        match *self {

            RbdEncryptionCreateOptions::luks { .. } => RbdImageEncryptionFormat::luks,

            RbdEncryptionCreateOptions::luks2 { .. } => RbdImageEncryptionFormat::luks2,

            RbdEncryptionCreateOptions::luks_any { .. } => RbdImageEncryptionFormat::luks_any,

        }
    }
}

impl From<RbdEncryptionCreateOptionsLUKS> for RbdEncryptionCreateOptions {
    fn from(val: RbdEncryptionCreateOptionsLUKS) -> Self {
        Self::luks(val)
    }
}

impl From<RbdEncryptionCreateOptionsLUKS2> for RbdEncryptionCreateOptions {
    fn from(val: RbdEncryptionCreateOptionsLUKS2) -> Self {
        Self::luks2(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum RbdEncryptionOptions {
	#[serde(rename = "luks")]
	luks {
		#[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
parent: Option<Box<RbdEncryptionOptions>>,
		#[serde(flatten)] #[serde(rename = "luks")]
luks: RbdEncryptionOptionsLUKS,
	},
	#[serde(rename = "luks-any")]
	luks_any {
		#[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
parent: Option<Box<RbdEncryptionOptions>>,
		#[serde(flatten)] #[serde(rename = "luks-any")]
luks_any: RbdEncryptionOptionsLUKSAny,
	},
	#[serde(rename = "luks2")]
	luks2 {
		#[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
parent: Option<Box<RbdEncryptionOptions>>,
		#[serde(flatten)] #[serde(rename = "luks2")]
luks2: RbdEncryptionOptionsLUKS2,
	},
}

impl RbdEncryptionOptions {
    pub fn format(&self) -> RbdImageEncryptionFormat {
        match *self {

            RbdEncryptionOptions::luks { .. } => RbdImageEncryptionFormat::luks,

            RbdEncryptionOptions::luks_any { .. } => RbdImageEncryptionFormat::luks_any,

            RbdEncryptionOptions::luks2 { .. } => RbdImageEncryptionFormat::luks2,

        }
    }
}

impl From<(RbdEncryptionOptionsLUKS, RbdEncryptionOptions)> for RbdEncryptionOptions {
    fn from(val: (RbdEncryptionOptionsLUKS, RbdEncryptionOptions)) -> Self {
        Self::luks {
            luks: val.0,
            parent: Some(Box::new(val.1)),

        }
    }
}

impl From<(RbdEncryptionOptionsLUKSAny, RbdEncryptionOptions)> for RbdEncryptionOptions {
    fn from(val: (RbdEncryptionOptionsLUKSAny, RbdEncryptionOptions)) -> Self {
        Self::luks_any {
            luks_any: val.0,
            parent: Some(Box::new(val.1)),

        }
    }
}

impl From<(RbdEncryptionOptionsLUKS2, RbdEncryptionOptions)> for RbdEncryptionOptions {
    fn from(val: (RbdEncryptionOptionsLUKS2, RbdEncryptionOptions)) -> Self {
        Self::luks2 {
            luks2: val.0,
            parent: Some(Box::new(val.1)),

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "meta-type")]
pub enum SchemaInfo {
	#[serde(rename = "alternate")]
	alternate {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "alternate")]
alternate: SchemaInfoAlternate,
	},
	#[serde(rename = "array")]
	array {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "array")]
array: SchemaInfoArray,
	},
	#[serde(rename = "builtin")]
	builtin {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "builtin")]
builtin: SchemaInfoBuiltin,
	},
	#[serde(rename = "command")]
	command {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "command")]
command: SchemaInfoCommand,
	},
	#[serde(rename = "enum")]
	enum_ {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "enum")]
enum_: SchemaInfoEnum,
	},
	#[serde(rename = "event")]
	event {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "event")]
event: SchemaInfoEvent,
	},
	#[serde(rename = "object")]
	object {
		#[serde(flatten)] #[serde(rename = "base")]
base: SchemaInfoBase,
		#[serde(flatten)] #[serde(rename = "object")]
object: SchemaInfoObject,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoBase {
	#[serde(rename = "features", default, skip_serializing_if = "Option::is_none")]
pub features: Option<Vec<::std::string::String>>,
	#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl SchemaInfo {
    pub fn meta_type(&self) -> SchemaMetaType {
        match *self {

            SchemaInfo::alternate { .. } => SchemaMetaType::alternate,

            SchemaInfo::array { .. } => SchemaMetaType::array,

            SchemaInfo::builtin { .. } => SchemaMetaType::builtin,

            SchemaInfo::command { .. } => SchemaMetaType::command,

            SchemaInfo::enum_ { .. } => SchemaMetaType::enum_,

            SchemaInfo::event { .. } => SchemaMetaType::event,

            SchemaInfo::object { .. } => SchemaMetaType::object,

        }
    }
}

impl From<(SchemaInfoAlternate, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoAlternate, SchemaInfoBase)) -> Self {
        Self::alternate {
            alternate: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoArray, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoArray, SchemaInfoBase)) -> Self {
        Self::array {
            array: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoBuiltin, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoBuiltin, SchemaInfoBase)) -> Self {
        Self::builtin {
            builtin: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoCommand, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoCommand, SchemaInfoBase)) -> Self {
        Self::command {
            command: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoEnum, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoEnum, SchemaInfoBase)) -> Self {
        Self::enum_ {
            enum_: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoEvent, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoEvent, SchemaInfoBase)) -> Self {
        Self::event {
            event: val.0,
            base: val.1,

        }
    }
}

impl From<(SchemaInfoObject, SchemaInfoBase)> for SchemaInfo {
    fn from(val: (SchemaInfoObject, SchemaInfoBase)) -> Self {
        Self::object {
            object: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "protocol")]
pub enum SetPasswordOptions {
	#[serde(rename = "vnc")]
	vnc {
		#[serde(flatten)] #[serde(rename = "base")]
base: SetPasswordOptionsBase,
		#[serde(flatten)] #[serde(rename = "vnc")]
vnc: SetPasswordOptionsVnc,
	},
	#[serde(rename = "spice")]
	spice(SetPasswordOptionsBase),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPasswordOptionsBase {
	#[serde(rename = "connected", default, skip_serializing_if = "Option::is_none")]
pub connected: Option<SetPasswordAction>,
	#[serde(rename = "password")]
pub password: ::std::string::String,
}

impl SetPasswordOptions {
    pub fn protocol(&self) -> DisplayProtocol {
        match *self {

            SetPasswordOptions::vnc { .. } => DisplayProtocol::vnc,

            SetPasswordOptions::spice { .. } => DisplayProtocol::spice,

        }
    }
}

impl From<(SetPasswordOptionsVnc, SetPasswordOptionsBase)> for SetPasswordOptions {
    fn from(val: (SetPasswordOptionsVnc, SetPasswordOptionsBase)) -> Self {
        Self::vnc {
            vnc: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "sev-type")]
pub enum SevInfo {
	#[serde(rename = "sev")]
	sev {
		#[serde(flatten)] #[serde(rename = "base")]
base: SevInfoBase,
		#[serde(flatten)] #[serde(rename = "sev")]
sev: SevGuestInfo,
	},
	#[serde(rename = "sev-snp")]
	sev_snp {
		#[serde(flatten)] #[serde(rename = "base")]
base: SevInfoBase,
		#[serde(flatten)] #[serde(rename = "sev-snp")]
sev_snp: SevSnpGuestInfo,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevInfoBase {
	#[serde(rename = "api-major")]
pub api_major: u8,
	#[serde(rename = "api-minor")]
pub api_minor: u8,
	#[serde(rename = "build-id")]
pub build_id: u8,
	#[serde(rename = "enabled")]
pub enabled: bool,
	#[serde(rename = "state")]
pub state: SevState,
}

impl SevInfo {
    pub fn sev_type(&self) -> SevGuestType {
        match *self {

            SevInfo::sev { .. } => SevGuestType::sev,

            SevInfo::sev_snp { .. } => SevGuestType::sev_snp,

        }
    }
}

impl From<(SevGuestInfo, SevInfoBase)> for SevInfo {
    fn from(val: (SevGuestInfo, SevInfoBase)) -> Self {
        Self::sev {
            sev: val.0,
            base: val.1,

        }
    }
}

impl From<(SevSnpGuestInfo, SevInfoBase)> for SevInfo {
    fn from(val: (SevSnpGuestInfo, SevInfoBase)) -> Self {
        Self::sev_snp {
            sev_snp: val.0,
            base: val.1,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SocketAddress {
	#[serde(rename = "fd")]
	fd(FdSocketAddress),
	#[serde(rename = "inet")]
	inet(InetSocketAddress),
	#[serde(rename = "unix")]
	unix(UnixSocketAddress),
	#[serde(rename = "vsock")]
	vsock(VsockSocketAddress),
}

impl SocketAddress {
    pub fn type_(&self) -> SocketAddressType {
        match *self {

            SocketAddress::fd { .. } => SocketAddressType::fd,

            SocketAddress::inet { .. } => SocketAddressType::inet,

            SocketAddress::unix { .. } => SocketAddressType::unix,

            SocketAddress::vsock { .. } => SocketAddressType::vsock,

        }
    }
}

impl From<FdSocketAddress> for SocketAddress {
    fn from(val: FdSocketAddress) -> Self {
        Self::fd(val)
    }
}

impl From<InetSocketAddress> for SocketAddress {
    fn from(val: InetSocketAddress) -> Self {
        Self::inet(val)
    }
}

impl From<UnixSocketAddress> for SocketAddress {
    fn from(val: UnixSocketAddress) -> Self {
        Self::unix(val)
    }
}

impl From<VsockSocketAddress> for SocketAddress {
    fn from(val: VsockSocketAddress) -> Self {
        Self::vsock(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SocketAddressLegacy {
	#[serde(rename = "fd")]
	fd(FdSocketAddressWrapper),
	#[serde(rename = "inet")]
	inet(InetSocketAddressWrapper),
	#[serde(rename = "unix")]
	unix(UnixSocketAddressWrapper),
	#[serde(rename = "vsock")]
	vsock(VsockSocketAddressWrapper),
}

impl SocketAddressLegacy {
    pub fn type_(&self) -> SocketAddressType {
        match *self {

            SocketAddressLegacy::fd { .. } => SocketAddressType::fd,

            SocketAddressLegacy::inet { .. } => SocketAddressType::inet,

            SocketAddressLegacy::unix { .. } => SocketAddressType::unix,

            SocketAddressLegacy::vsock { .. } => SocketAddressType::vsock,

        }
    }
}

impl From<FdSocketAddressWrapper> for SocketAddressLegacy {
    fn from(val: FdSocketAddressWrapper) -> Self {
        Self::fd(val)
    }
}

impl From<FdSocketAddress> for SocketAddressLegacy {
    fn from(val: FdSocketAddress) -> Self {
        Self::fd(FdSocketAddressWrapper::from(val))
    }
}

impl From<InetSocketAddressWrapper> for SocketAddressLegacy {
    fn from(val: InetSocketAddressWrapper) -> Self {
        Self::inet(val)
    }
}

impl From<InetSocketAddress> for SocketAddressLegacy {
    fn from(val: InetSocketAddress) -> Self {
        Self::inet(InetSocketAddressWrapper::from(val))
    }
}

impl From<UnixSocketAddressWrapper> for SocketAddressLegacy {
    fn from(val: UnixSocketAddressWrapper) -> Self {
        Self::unix(val)
    }
}

impl From<UnixSocketAddress> for SocketAddressLegacy {
    fn from(val: UnixSocketAddress) -> Self {
        Self::unix(UnixSocketAddressWrapper::from(val))
    }
}

impl From<VsockSocketAddressWrapper> for SocketAddressLegacy {
    fn from(val: VsockSocketAddressWrapper) -> Self {
        Self::vsock(val)
    }
}

impl From<VsockSocketAddress> for SocketAddressLegacy {
    fn from(val: VsockSocketAddress) -> Self {
        Self::vsock(VsockSocketAddressWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum SshHostKeyCheck {
	#[serde(rename = "hash")]
	hash(SshHostKeyHash),
	#[serde(rename = "none")]
	none,
	#[serde(rename = "known_hosts")]
	known_hosts,
}

impl SshHostKeyCheck {
    pub fn mode(&self) -> SshHostKeyCheckMode {
        match *self {

            SshHostKeyCheck::hash { .. } => SshHostKeyCheckMode::hash,

            SshHostKeyCheck::none { .. } => SshHostKeyCheckMode::none,

            SshHostKeyCheck::known_hosts { .. } => SshHostKeyCheckMode::known_hosts,

        }
    }
}

impl From<SshHostKeyHash> for SshHostKeyCheck {
    fn from(val: SshHostKeyHash) -> Self {
        Self::hash(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "target")]
pub enum StatsFilter {
	#[serde(rename = "vcpu")]
	vcpu {
		#[serde(rename = "providers", default, skip_serializing_if = "Option::is_none")]
providers: Option<Vec<StatsRequest>>,
		#[serde(flatten)] #[serde(rename = "vcpu")]
vcpu: StatsVCPUFilter,
	},
	#[serde(rename = "vm")]
	vm(Vec<StatsRequest>),
	#[serde(rename = "cryptodev")]
	cryptodev(Vec<StatsRequest>),
}

impl StatsFilter {
    pub fn target(&self) -> StatsTarget {
        match *self {

            StatsFilter::vcpu { .. } => StatsTarget::vcpu,

            StatsFilter::vm { .. } => StatsTarget::vm,

            StatsFilter::cryptodev { .. } => StatsTarget::cryptodev,

        }
    }
}

impl From<(StatsVCPUFilter, Vec<StatsRequest>)> for StatsFilter {
    fn from(val: (StatsVCPUFilter, Vec<StatsRequest>)) -> Self {
        Self::vcpu {
            vcpu: val.0,
            providers: val.1.into(),

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TpmTypeOptions {
	#[serde(rename = "emulator")]
	emulator(TPMEmulatorOptionsWrapper),
	#[serde(rename = "passthrough")]
	passthrough(TPMPassthroughOptionsWrapper),
}

impl TpmTypeOptions {
    pub fn type_(&self) -> TpmType {
        match *self {

            TpmTypeOptions::emulator { .. } => TpmType::emulator,

            TpmTypeOptions::passthrough { .. } => TpmType::passthrough,

        }
    }
}

impl From<TPMEmulatorOptionsWrapper> for TpmTypeOptions {
    fn from(val: TPMEmulatorOptionsWrapper) -> Self {
        Self::emulator(val)
    }
}

impl From<TPMEmulatorOptions> for TpmTypeOptions {
    fn from(val: TPMEmulatorOptions) -> Self {
        Self::emulator(TPMEmulatorOptionsWrapper::from(val))
    }
}

impl From<TPMPassthroughOptionsWrapper> for TpmTypeOptions {
    fn from(val: TPMPassthroughOptionsWrapper) -> Self {
        Self::passthrough(val)
    }
}

impl From<TPMPassthroughOptions> for TpmTypeOptions {
    fn from(val: TPMPassthroughOptions) -> Self {
        Self::passthrough(TPMPassthroughOptionsWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionAction {
	#[serde(rename = "abort")]
	abort(AbortWrapper),
	#[serde(rename = "block-dirty-bitmap-add")]
	block_dirty_bitmap_add(BlockDirtyBitmapAddWrapper),
	#[serde(rename = "block-dirty-bitmap-clear")]
	block_dirty_bitmap_clear(BlockDirtyBitmapWrapper),
	#[serde(rename = "block-dirty-bitmap-disable")]
	block_dirty_bitmap_disable(BlockDirtyBitmapWrapper),
	#[serde(rename = "block-dirty-bitmap-enable")]
	block_dirty_bitmap_enable(BlockDirtyBitmapWrapper),
	#[serde(rename = "block-dirty-bitmap-merge")]
	block_dirty_bitmap_merge(BlockDirtyBitmapMergeWrapper),
	#[serde(rename = "block-dirty-bitmap-remove")]
	block_dirty_bitmap_remove(BlockDirtyBitmapWrapper),
	#[serde(rename = "blockdev-backup")]
	blockdev_backup(BlockdevBackupWrapper),
	#[serde(rename = "blockdev-snapshot")]
	blockdev_snapshot(BlockdevSnapshotWrapper),
	#[serde(rename = "blockdev-snapshot-internal-sync")]
	blockdev_snapshot_internal_sync(BlockdevSnapshotInternalWrapper),
	#[serde(rename = "blockdev-snapshot-sync")]
	blockdev_snapshot_sync(BlockdevSnapshotSyncWrapper),
	#[serde(rename = "drive-backup")]
	drive_backup(DriveBackupWrapper),
}

impl TransactionAction {
    pub fn type_(&self) -> TransactionActionKind {
        match *self {

            TransactionAction::abort { .. } => TransactionActionKind::abort,

            TransactionAction::block_dirty_bitmap_add { .. } => TransactionActionKind::block_dirty_bitmap_add,

            TransactionAction::block_dirty_bitmap_clear { .. } => TransactionActionKind::block_dirty_bitmap_clear,

            TransactionAction::block_dirty_bitmap_disable { .. } => TransactionActionKind::block_dirty_bitmap_disable,

            TransactionAction::block_dirty_bitmap_enable { .. } => TransactionActionKind::block_dirty_bitmap_enable,

            TransactionAction::block_dirty_bitmap_merge { .. } => TransactionActionKind::block_dirty_bitmap_merge,

            TransactionAction::block_dirty_bitmap_remove { .. } => TransactionActionKind::block_dirty_bitmap_remove,

            TransactionAction::blockdev_backup { .. } => TransactionActionKind::blockdev_backup,

            TransactionAction::blockdev_snapshot { .. } => TransactionActionKind::blockdev_snapshot,

            TransactionAction::blockdev_snapshot_internal_sync { .. } => TransactionActionKind::blockdev_snapshot_internal_sync,

            TransactionAction::blockdev_snapshot_sync { .. } => TransactionActionKind::blockdev_snapshot_sync,

            TransactionAction::drive_backup { .. } => TransactionActionKind::drive_backup,

        }
    }
}

impl From<AbortWrapper> for TransactionAction {
    fn from(val: AbortWrapper) -> Self {
        Self::abort(val)
    }
}

impl From<Abort> for TransactionAction {
    fn from(val: Abort) -> Self {
        Self::abort(AbortWrapper::from(val))
    }
}

impl From<BlockDirtyBitmapAddWrapper> for TransactionAction {
    fn from(val: BlockDirtyBitmapAddWrapper) -> Self {
        Self::block_dirty_bitmap_add(val)
    }
}

impl From<BlockDirtyBitmapAdd> for TransactionAction {
    fn from(val: BlockDirtyBitmapAdd) -> Self {
        Self::block_dirty_bitmap_add(BlockDirtyBitmapAddWrapper::from(val))
    }
}

impl From<BlockDirtyBitmapMergeWrapper> for TransactionAction {
    fn from(val: BlockDirtyBitmapMergeWrapper) -> Self {
        Self::block_dirty_bitmap_merge(val)
    }
}

impl From<BlockDirtyBitmapMerge> for TransactionAction {
    fn from(val: BlockDirtyBitmapMerge) -> Self {
        Self::block_dirty_bitmap_merge(BlockDirtyBitmapMergeWrapper::from(val))
    }
}

impl From<BlockdevBackupWrapper> for TransactionAction {
    fn from(val: BlockdevBackupWrapper) -> Self {
        Self::blockdev_backup(val)
    }
}

impl From<BlockdevBackup> for TransactionAction {
    fn from(val: BlockdevBackup) -> Self {
        Self::blockdev_backup(BlockdevBackupWrapper::from(val))
    }
}

impl From<BlockdevSnapshotWrapper> for TransactionAction {
    fn from(val: BlockdevSnapshotWrapper) -> Self {
        Self::blockdev_snapshot(val)
    }
}

impl From<BlockdevSnapshot> for TransactionAction {
    fn from(val: BlockdevSnapshot) -> Self {
        Self::blockdev_snapshot(BlockdevSnapshotWrapper::from(val))
    }
}

impl From<BlockdevSnapshotInternalWrapper> for TransactionAction {
    fn from(val: BlockdevSnapshotInternalWrapper) -> Self {
        Self::blockdev_snapshot_internal_sync(val)
    }
}

impl From<BlockdevSnapshotInternal> for TransactionAction {
    fn from(val: BlockdevSnapshotInternal) -> Self {
        Self::blockdev_snapshot_internal_sync(BlockdevSnapshotInternalWrapper::from(val))
    }
}

impl From<BlockdevSnapshotSyncWrapper> for TransactionAction {
    fn from(val: BlockdevSnapshotSyncWrapper) -> Self {
        Self::blockdev_snapshot_sync(val)
    }
}

impl From<BlockdevSnapshotSync> for TransactionAction {
    fn from(val: BlockdevSnapshotSync) -> Self {
        Self::blockdev_snapshot_sync(BlockdevSnapshotSyncWrapper::from(val))
    }
}

impl From<DriveBackupWrapper> for TransactionAction {
    fn from(val: DriveBackupWrapper) -> Self {
        Self::drive_backup(val)
    }
}

impl From<DriveBackup> for TransactionAction {
    fn from(val: DriveBackup) -> Self {
        Self::drive_backup(DriveBackupWrapper::from(val))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum YankInstance {
	#[serde(rename = "block-node")]
	block_node(YankInstanceBlockNode),
	#[serde(rename = "chardev")]
	chardev(YankInstanceChardev),
	#[serde(rename = "migration")]
	migration,
}

impl YankInstance {
    pub fn type_(&self) -> YankInstanceType {
        match *self {

            YankInstance::block_node { .. } => YankInstanceType::block_node,

            YankInstance::chardev { .. } => YankInstanceType::chardev,

            YankInstance::migration { .. } => YankInstanceType::migration,

        }
    }
}

impl From<YankInstanceBlockNode> for YankInstance {
    fn from(val: YankInstanceBlockNode) -> Self {
        Self::block_node(val)
    }
}

impl From<YankInstanceChardev> for YankInstance {
    fn from(val: YankInstanceChardev) -> Self {
        Self::chardev(val)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACPIOSTInfo {
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
#[serde(rename = "slot")]
pub slot: ::std::string::String,
#[serde(rename = "slot-type")]
pub slot_type: ACPISlotType,
#[serde(rename = "source")]
pub source: i64,
#[serde(rename = "status")]
pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Abort {
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct AbortWrapper {
#[serde(rename = "data")]
pub data: Abort,
}

impl<T: Into<Abort>> From<T> for AbortWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<Abort> for AbortWrapper {
        fn as_ref(&self) -> &Abort {
            &self.data
        }
    }
impl ::std::ops::Deref for AbortWrapper {
    type Target = Abort;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl AbortWrapper {
    pub fn into_inner(self) -> Abort {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpiGenericInitiatorProperties {
#[serde(rename = "node")]
pub node: u32,
#[serde(rename = "pci-dev")]
pub pci_dev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcpiTableOptions {
#[serde(rename = "asl_compiler_id", default, skip_serializing_if = "Option::is_none")]
pub asl_compiler_id: Option<::std::string::String>,
#[serde(rename = "asl_compiler_rev", default, skip_serializing_if = "Option::is_none")]
pub asl_compiler_rev: Option<u32>,
#[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
pub data: Option<::std::string::String>,
#[serde(rename = "file", default, skip_serializing_if = "Option::is_none")]
pub file: Option<::std::string::String>,
#[serde(rename = "oem_id", default, skip_serializing_if = "Option::is_none")]
pub oem_id: Option<::std::string::String>,
#[serde(rename = "oem_rev", default, skip_serializing_if = "Option::is_none")]
pub oem_rev: Option<u32>,
#[serde(rename = "oem_table_id", default, skip_serializing_if = "Option::is_none")]
pub oem_table_id: Option<::std::string::String>,
#[serde(rename = "rev", default, skip_serializing_if = "Option::is_none")]
pub rev: Option<u8>,
#[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
pub sig: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddfdInfo {
#[serde(rename = "fd")]
pub fd: i64,
#[serde(rename = "fdset-id")]
pub fdset_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnounceParameters {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "interfaces", default, skip_serializing_if = "Option::is_none")]
pub interfaces: Option<Vec<::std::string::String>>,
#[serde(rename = "initial")]
pub initial: i64,
#[serde(rename = "max")]
pub max: i64,
#[serde(rename = "rounds")]
pub rounds: i64,
#[serde(rename = "step")]
pub step: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevAlsaOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevAlsaPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevAlsaPerDirectionOptions>,
#[serde(rename = "threshold", default, skip_serializing_if = "Option::is_none")]
pub threshold: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevAlsaPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "dev", default, skip_serializing_if = "Option::is_none")]
pub dev: Option<::std::string::String>,
#[serde(rename = "period-length", default, skip_serializing_if = "Option::is_none")]
pub period_length: Option<u32>,
#[serde(rename = "try-poll", default, skip_serializing_if = "Option::is_none")]
pub try_poll: Option<bool>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevAlsaPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
dev: Default::default(),
period_length: Default::default(),
try_poll: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevAlsaPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevCoreaudioOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevCoreaudioPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevCoreaudioPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevCoreaudioPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "buffer-count", default, skip_serializing_if = "Option::is_none")]
pub buffer_count: Option<u32>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevCoreaudioPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
buffer_count: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevCoreaudioPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevDsoundOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPerDirectionOptions>,
#[serde(rename = "latency", default, skip_serializing_if = "Option::is_none")]
pub latency: Option<u32>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevGenericOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevJackOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevJackPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevJackPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevJackPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "client-name", default, skip_serializing_if = "Option::is_none")]
pub client_name: Option<::std::string::String>,
#[serde(rename = "connect-ports", default, skip_serializing_if = "Option::is_none")]
pub connect_ports: Option<::std::string::String>,
#[serde(rename = "exact-name", default, skip_serializing_if = "Option::is_none")]
pub exact_name: Option<bool>,
#[serde(rename = "server-name", default, skip_serializing_if = "Option::is_none")]
pub server_name: Option<::std::string::String>,
#[serde(rename = "start-server", default, skip_serializing_if = "Option::is_none")]
pub start_server: Option<bool>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevJackPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
client_name: Default::default(),
connect_ports: Default::default(),
exact_name: Default::default(),
server_name: Default::default(),
start_server: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevJackPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevOssOptions {
#[serde(rename = "dsp-policy", default, skip_serializing_if = "Option::is_none")]
pub dsp_policy: Option<u32>,
#[serde(rename = "exclusive", default, skip_serializing_if = "Option::is_none")]
pub exclusive: Option<bool>,
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevOssPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevOssPerDirectionOptions>,
#[serde(rename = "try-mmap", default, skip_serializing_if = "Option::is_none")]
pub try_mmap: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevOssPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "buffer-count", default, skip_serializing_if = "Option::is_none")]
pub buffer_count: Option<u32>,
#[serde(rename = "dev", default, skip_serializing_if = "Option::is_none")]
pub dev: Option<::std::string::String>,
#[serde(rename = "try-poll", default, skip_serializing_if = "Option::is_none")]
pub try_poll: Option<bool>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevOssPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
buffer_count: Default::default(),
dev: Default::default(),
try_poll: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevOssPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevPaOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPaPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPaPerDirectionOptions>,
#[serde(rename = "server", default, skip_serializing_if = "Option::is_none")]
pub server: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevPaPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "latency", default, skip_serializing_if = "Option::is_none")]
pub latency: Option<u32>,
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
#[serde(rename = "stream-name", default, skip_serializing_if = "Option::is_none")]
pub stream_name: Option<::std::string::String>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevPaPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
latency: Default::default(),
name: Default::default(),
stream_name: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevPaPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevPerDirectionOptions {
#[serde(rename = "buffer-length", default, skip_serializing_if = "Option::is_none")]
pub buffer_length: Option<u32>,
#[serde(rename = "channels", default, skip_serializing_if = "Option::is_none")]
pub channels: Option<u32>,
#[serde(rename = "fixed-settings", default, skip_serializing_if = "Option::is_none")]
pub fixed_settings: Option<bool>,
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<AudioFormat>,
#[serde(rename = "frequency", default, skip_serializing_if = "Option::is_none")]
pub frequency: Option<u32>,
#[serde(rename = "mixing-engine", default, skip_serializing_if = "Option::is_none")]
pub mixing_engine: Option<bool>,
#[serde(rename = "voices", default, skip_serializing_if = "Option::is_none")]
pub voices: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevPipewireOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPipewirePerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPipewirePerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevPipewirePerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "latency", default, skip_serializing_if = "Option::is_none")]
pub latency: Option<u32>,
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
#[serde(rename = "stream-name", default, skip_serializing_if = "Option::is_none")]
pub stream_name: Option<::std::string::String>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevPipewirePerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
latency: Default::default(),
name: Default::default(),
stream_name: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevPipewirePerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevSdlOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevSdlPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevSdlPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiodevSdlPerDirectionOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: AudiodevPerDirectionOptions,
#[serde(rename = "buffer-count", default, skip_serializing_if = "Option::is_none")]
pub buffer_count: Option<u32>,
}

impl<T: Into<AudiodevPerDirectionOptions>> From<T> for AudiodevSdlPerDirectionOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
buffer_count: Default::default(),

        }
    }
}
    impl AsRef<AudiodevPerDirectionOptions> for AudiodevSdlPerDirectionOptions {
        fn as_ref(&self) -> &AudiodevPerDirectionOptions {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevSndioOptions {
#[serde(rename = "dev", default, skip_serializing_if = "Option::is_none")]
pub dev: Option<::std::string::String>,
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPerDirectionOptions>,
#[serde(rename = "latency", default, skip_serializing_if = "Option::is_none")]
pub latency: Option<u32>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPerDirectionOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudiodevWavOptions {
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<AudiodevPerDirectionOptions>,
#[serde(rename = "out", default, skip_serializing_if = "Option::is_none")]
pub out: Option<AudiodevPerDirectionOptions>,
#[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
pub path: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthZListFileProperties {
#[serde(rename = "refresh", default, skip_serializing_if = "Option::is_none")]
pub refresh: Option<bool>,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthZListProperties {
#[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
pub policy: Option<QAuthZListPolicy>,
#[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
pub rules: Option<Vec<QAuthZListRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthZPAMProperties {
#[serde(rename = "service")]
pub service: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for AuthZPAMProperties {
    fn from(val: T) -> Self {
        Self {
            service: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for AuthZPAMProperties {
        fn as_ref(&self) -> &::std::string::String {
            &self.service
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthZSimpleProperties {
#[serde(rename = "identity")]
pub identity: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for AuthZSimpleProperties {
    fn from(val: T) -> Self {
        Self {
            identity: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for AuthZSimpleProperties {
        fn as_ref(&self) -> &::std::string::String {
            &self.identity
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCommon {
#[serde(rename = "auto-dismiss", default, skip_serializing_if = "Option::is_none")]
pub auto_dismiss: Option<bool>,
#[serde(rename = "auto-finalize", default, skip_serializing_if = "Option::is_none")]
pub auto_finalize: Option<bool>,
#[serde(rename = "bitmap", default, skip_serializing_if = "Option::is_none")]
pub bitmap: Option<::std::string::String>,
#[serde(rename = "bitmap-mode", default, skip_serializing_if = "Option::is_none")]
pub bitmap_mode: Option<BitmapSyncMode>,
#[serde(rename = "compress", default, skip_serializing_if = "Option::is_none")]
pub compress: Option<bool>,
#[serde(rename = "discard-source", default, skip_serializing_if = "Option::is_none")]
pub discard_source: Option<bool>,
#[serde(rename = "filter-node-name", default, skip_serializing_if = "Option::is_none")]
pub filter_node_name: Option<::std::string::String>,
#[serde(rename = "job-id", default, skip_serializing_if = "Option::is_none")]
pub job_id: Option<::std::string::String>,
#[serde(rename = "on-source-error", default, skip_serializing_if = "Option::is_none")]
pub on_source_error: Option<BlockdevOnError>,
#[serde(rename = "on-target-error", default, skip_serializing_if = "Option::is_none")]
pub on_target_error: Option<BlockdevOnError>,
#[serde(rename = "speed", default, skip_serializing_if = "Option::is_none")]
pub speed: Option<i64>,
#[serde(rename = "x-perf", default, skip_serializing_if = "Option::is_none")]
pub x_perf: Option<BackupPerf>,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "sync")]
pub sync: MirrorSyncMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupPerf {
#[serde(rename = "max-chunk", default, skip_serializing_if = "Option::is_none")]
pub max_chunk: Option<i64>,
#[serde(rename = "max-workers", default, skip_serializing_if = "Option::is_none")]
pub max_workers: Option<i64>,
#[serde(rename = "use-copy-range", default, skip_serializing_if = "Option::is_none")]
pub use_copy_range: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalloonInfo {
#[serde(rename = "actual")]
pub actual: i64,
}

impl<T: Into<i64>> From<T> for BalloonInfo {
    fn from(val: T) -> Self {
        Self {
            actual: val.into(),

        }
    }
}
    impl AsRef<i64> for BalloonInfo {
        fn as_ref(&self) -> &i64 {
            &self.actual
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitmapMigrationBitmapAlias {
#[serde(rename = "transform", default, skip_serializing_if = "Option::is_none")]
pub transform: Option<BitmapMigrationBitmapAliasTransform>,
#[serde(rename = "alias")]
pub alias: ::std::string::String,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BitmapMigrationBitmapAliasTransform {
#[serde(rename = "persistent", default, skip_serializing_if = "Option::is_none")]
pub persistent: Option<bool>,
}

impl<T: Into<bool>> From<T> for BitmapMigrationBitmapAliasTransform {
    fn from(val: T) -> Self {
        Self {
            persistent: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitmapMigrationNodeAlias {
#[serde(rename = "alias")]
pub alias: ::std::string::String,
#[serde(rename = "bitmaps")]
pub bitmaps: Vec<BitmapMigrationBitmapAlias>,
#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlkdebugInjectErrorOptions {
#[serde(rename = "errno", default, skip_serializing_if = "Option::is_none")]
pub errno: Option<i64>,
#[serde(rename = "immediately", default, skip_serializing_if = "Option::is_none")]
pub immediately: Option<bool>,
#[serde(rename = "iotype", default, skip_serializing_if = "Option::is_none")]
pub iotype: Option<BlkdebugIOType>,
#[serde(rename = "once", default, skip_serializing_if = "Option::is_none")]
pub once: Option<bool>,
#[serde(rename = "sector", default, skip_serializing_if = "Option::is_none")]
pub sector: Option<i64>,
#[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
pub state: Option<i64>,
#[serde(rename = "event")]
pub event: BlkdebugEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlkdebugSetStateOptions {
#[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
pub state: Option<i64>,
#[serde(rename = "event")]
pub event: BlkdebugEvent,
#[serde(rename = "new_state")]
pub new_state: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockChildInfo {
#[serde(rename = "info")]
pub info: BlockGraphInfo,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeviceInfo {
#[serde(rename = "backing_file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
#[serde(rename = "bps_max", default, skip_serializing_if = "Option::is_none")]
pub bps_max: Option<i64>,
#[serde(rename = "bps_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_max_length: Option<i64>,
#[serde(rename = "bps_rd_max", default, skip_serializing_if = "Option::is_none")]
pub bps_rd_max: Option<i64>,
#[serde(rename = "bps_rd_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_rd_max_length: Option<i64>,
#[serde(rename = "bps_wr_max", default, skip_serializing_if = "Option::is_none")]
pub bps_wr_max: Option<i64>,
#[serde(rename = "bps_wr_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_wr_max_length: Option<i64>,
#[serde(rename = "dirty-bitmaps", default, skip_serializing_if = "Option::is_none")]
pub dirty_bitmaps: Option<Vec<BlockDirtyInfo>>,
#[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
pub group: Option<::std::string::String>,
#[serde(rename = "iops_max", default, skip_serializing_if = "Option::is_none")]
pub iops_max: Option<i64>,
#[serde(rename = "iops_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_max_length: Option<i64>,
#[serde(rename = "iops_rd_max", default, skip_serializing_if = "Option::is_none")]
pub iops_rd_max: Option<i64>,
#[serde(rename = "iops_rd_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_rd_max_length: Option<i64>,
#[serde(rename = "iops_size", default, skip_serializing_if = "Option::is_none")]
pub iops_size: Option<i64>,
#[serde(rename = "iops_wr_max", default, skip_serializing_if = "Option::is_none")]
pub iops_wr_max: Option<i64>,
#[serde(rename = "iops_wr_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_wr_max_length: Option<i64>,
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "backing_file_depth")]
pub backing_file_depth: i64,
#[serde(rename = "bps")]
pub bps: i64,
#[serde(rename = "bps_rd")]
pub bps_rd: i64,
#[serde(rename = "bps_wr")]
pub bps_wr: i64,
#[serde(rename = "cache")]
pub cache: BlockdevCacheInfo,
#[serde(rename = "detect_zeroes")]
pub detect_zeroes: BlockdevDetectZeroesOptions,
#[serde(rename = "drv")]
pub drv: ::std::string::String,
#[serde(rename = "encrypted")]
pub encrypted: bool,
#[serde(rename = "file")]
pub file: ::std::string::String,
#[serde(rename = "image")]
pub image: ImageInfo,
#[serde(rename = "iops")]
pub iops: i64,
#[serde(rename = "iops_rd")]
pub iops_rd: i64,
#[serde(rename = "iops_wr")]
pub iops_wr: i64,
#[serde(rename = "ro")]
pub ro: bool,
#[serde(rename = "write_threshold")]
pub write_threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeviceStats {
#[serde(rename = "flush_latency_histogram", default, skip_serializing_if = "Option::is_none")]
pub flush_latency_histogram: Option<BlockLatencyHistogramInfo>,
#[serde(rename = "idle_time_ns", default, skip_serializing_if = "Option::is_none")]
pub idle_time_ns: Option<i64>,
#[serde(rename = "rd_latency_histogram", default, skip_serializing_if = "Option::is_none")]
pub rd_latency_histogram: Option<BlockLatencyHistogramInfo>,
#[serde(rename = "wr_latency_histogram", default, skip_serializing_if = "Option::is_none")]
pub wr_latency_histogram: Option<BlockLatencyHistogramInfo>,
#[serde(rename = "zone_append_latency_histogram", default, skip_serializing_if = "Option::is_none")]
pub zone_append_latency_histogram: Option<BlockLatencyHistogramInfo>,
#[serde(rename = "account_failed")]
pub account_failed: bool,
#[serde(rename = "account_invalid")]
pub account_invalid: bool,
#[serde(rename = "failed_flush_operations")]
pub failed_flush_operations: i64,
#[serde(rename = "failed_rd_operations")]
pub failed_rd_operations: i64,
#[serde(rename = "failed_unmap_operations")]
pub failed_unmap_operations: i64,
#[serde(rename = "failed_wr_operations")]
pub failed_wr_operations: i64,
#[serde(rename = "failed_zone_append_operations")]
pub failed_zone_append_operations: i64,
#[serde(rename = "flush_operations")]
pub flush_operations: i64,
#[serde(rename = "flush_total_time_ns")]
pub flush_total_time_ns: i64,
#[serde(rename = "invalid_flush_operations")]
pub invalid_flush_operations: i64,
#[serde(rename = "invalid_rd_operations")]
pub invalid_rd_operations: i64,
#[serde(rename = "invalid_unmap_operations")]
pub invalid_unmap_operations: i64,
#[serde(rename = "invalid_wr_operations")]
pub invalid_wr_operations: i64,
#[serde(rename = "invalid_zone_append_operations")]
pub invalid_zone_append_operations: i64,
#[serde(rename = "rd_bytes")]
pub rd_bytes: i64,
#[serde(rename = "rd_merged")]
pub rd_merged: i64,
#[serde(rename = "rd_operations")]
pub rd_operations: i64,
#[serde(rename = "rd_total_time_ns")]
pub rd_total_time_ns: i64,
#[serde(rename = "timed_stats")]
pub timed_stats: Vec<BlockDeviceTimedStats>,
#[serde(rename = "unmap_bytes")]
pub unmap_bytes: i64,
#[serde(rename = "unmap_merged")]
pub unmap_merged: i64,
#[serde(rename = "unmap_operations")]
pub unmap_operations: i64,
#[serde(rename = "unmap_total_time_ns")]
pub unmap_total_time_ns: i64,
#[serde(rename = "wr_bytes")]
pub wr_bytes: i64,
#[serde(rename = "wr_highest_offset")]
pub wr_highest_offset: i64,
#[serde(rename = "wr_merged")]
pub wr_merged: i64,
#[serde(rename = "wr_operations")]
pub wr_operations: i64,
#[serde(rename = "wr_total_time_ns")]
pub wr_total_time_ns: i64,
#[serde(rename = "zone_append_bytes")]
pub zone_append_bytes: i64,
#[serde(rename = "zone_append_merged")]
pub zone_append_merged: i64,
#[serde(rename = "zone_append_operations")]
pub zone_append_operations: i64,
#[serde(rename = "zone_append_total_time_ns")]
pub zone_append_total_time_ns: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeviceTimedStats {
#[serde(rename = "avg_flush_latency_ns")]
pub avg_flush_latency_ns: i64,
#[serde(rename = "avg_rd_latency_ns")]
pub avg_rd_latency_ns: i64,
#[serde(rename = "avg_rd_queue_depth")]
pub avg_rd_queue_depth: f64,
#[serde(rename = "avg_wr_latency_ns")]
pub avg_wr_latency_ns: i64,
#[serde(rename = "avg_wr_queue_depth")]
pub avg_wr_queue_depth: f64,
#[serde(rename = "avg_zone_append_latency_ns")]
pub avg_zone_append_latency_ns: i64,
#[serde(rename = "avg_zone_append_queue_depth")]
pub avg_zone_append_queue_depth: f64,
#[serde(rename = "interval_length")]
pub interval_length: i64,
#[serde(rename = "max_flush_latency_ns")]
pub max_flush_latency_ns: i64,
#[serde(rename = "max_rd_latency_ns")]
pub max_rd_latency_ns: i64,
#[serde(rename = "max_wr_latency_ns")]
pub max_wr_latency_ns: i64,
#[serde(rename = "max_zone_append_latency_ns")]
pub max_zone_append_latency_ns: i64,
#[serde(rename = "min_flush_latency_ns")]
pub min_flush_latency_ns: i64,
#[serde(rename = "min_rd_latency_ns")]
pub min_rd_latency_ns: i64,
#[serde(rename = "min_wr_latency_ns")]
pub min_wr_latency_ns: i64,
#[serde(rename = "min_zone_append_latency_ns")]
pub min_zone_append_latency_ns: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDirtyBitmap {
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "node")]
pub node: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDirtyBitmapAdd {
#[serde(rename = "disabled", default, skip_serializing_if = "Option::is_none")]
pub disabled: Option<bool>,
#[serde(rename = "granularity", default, skip_serializing_if = "Option::is_none")]
pub granularity: Option<u32>,
#[serde(rename = "persistent", default, skip_serializing_if = "Option::is_none")]
pub persistent: Option<bool>,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "node")]
pub node: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockDirtyBitmapAddWrapper {
#[serde(rename = "data")]
pub data: BlockDirtyBitmapAdd,
}

impl<T: Into<BlockDirtyBitmapAdd>> From<T> for BlockDirtyBitmapAddWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockDirtyBitmapAdd> for BlockDirtyBitmapAddWrapper {
        fn as_ref(&self) -> &BlockDirtyBitmapAdd {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockDirtyBitmapAddWrapper {
    type Target = BlockDirtyBitmapAdd;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockDirtyBitmapAddWrapper {
    pub fn into_inner(self) -> BlockDirtyBitmapAdd {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDirtyBitmapMerge {
#[serde(rename = "bitmaps")]
pub bitmaps: Vec<BlockDirtyBitmapOrStr>,
#[serde(rename = "node")]
pub node: ::std::string::String,
#[serde(rename = "target")]
pub target: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockDirtyBitmapMergeWrapper {
#[serde(rename = "data")]
pub data: BlockDirtyBitmapMerge,
}

impl<T: Into<BlockDirtyBitmapMerge>> From<T> for BlockDirtyBitmapMergeWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockDirtyBitmapMerge> for BlockDirtyBitmapMergeWrapper {
        fn as_ref(&self) -> &BlockDirtyBitmapMerge {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockDirtyBitmapMergeWrapper {
    type Target = BlockDirtyBitmapMerge;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockDirtyBitmapMergeWrapper {
    pub fn into_inner(self) -> BlockDirtyBitmapMerge {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDirtyBitmapSha256 {
#[serde(rename = "sha256")]
pub sha256: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockDirtyBitmapSha256 {
    fn from(val: T) -> Self {
        Self {
            sha256: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockDirtyBitmapSha256 {
        fn as_ref(&self) -> &::std::string::String {
            &self.sha256
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockDirtyBitmapWrapper {
#[serde(rename = "data")]
pub data: BlockDirtyBitmap,
}

impl<T: Into<BlockDirtyBitmap>> From<T> for BlockDirtyBitmapWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockDirtyBitmap> for BlockDirtyBitmapWrapper {
        fn as_ref(&self) -> &BlockDirtyBitmap {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockDirtyBitmapWrapper {
    type Target = BlockDirtyBitmap;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockDirtyBitmapWrapper {
    pub fn into_inner(self) -> BlockDirtyBitmap {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDirtyInfo {
#[serde(rename = "inconsistent", default, skip_serializing_if = "Option::is_none")]
pub inconsistent: Option<bool>,
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
#[serde(rename = "busy")]
pub busy: bool,
#[serde(rename = "count")]
pub count: i64,
#[serde(rename = "granularity")]
pub granularity: u32,
#[serde(rename = "persistent")]
pub persistent: bool,
#[serde(rename = "recording")]
pub recording: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportInfo {
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
#[serde(rename = "shutting-down")]
pub shutting_down: bool,
#[serde(rename = "type")]
pub type_: BlockExportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportOptionsFuse {
#[serde(rename = "allow-other", default, skip_serializing_if = "Option::is_none")]
pub allow_other: Option<FuseExportAllowOther>,
#[serde(rename = "growable", default, skip_serializing_if = "Option::is_none")]
pub growable: Option<bool>,
#[serde(rename = "mountpoint")]
pub mountpoint: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportOptionsNbd {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockExportOptionsNbdBase,
#[serde(rename = "allocation-depth", default, skip_serializing_if = "Option::is_none")]
pub allocation_depth: Option<bool>,
#[serde(rename = "bitmaps", default, skip_serializing_if = "Option::is_none")]
pub bitmaps: Option<Vec<BlockDirtyBitmapOrStr>>,
}

impl<T: Into<BlockExportOptionsNbdBase>> From<T> for BlockExportOptionsNbd {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
allocation_depth: Default::default(),
bitmaps: Default::default(),

        }
    }
}
    impl AsRef<BlockExportOptionsNbdBase> for BlockExportOptionsNbd {
        fn as_ref(&self) -> &BlockExportOptionsNbdBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockExportOptionsNbdBase {
#[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
pub description: Option<::std::string::String>,
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportOptionsVduseBlk {
#[serde(rename = "logical-block-size", default, skip_serializing_if = "Option::is_none")]
pub logical_block_size: Option<u64>,
#[serde(rename = "num-queues", default, skip_serializing_if = "Option::is_none")]
pub num_queues: Option<u16>,
#[serde(rename = "queue-size", default, skip_serializing_if = "Option::is_none")]
pub queue_size: Option<u16>,
#[serde(rename = "serial", default, skip_serializing_if = "Option::is_none")]
pub serial: Option<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExportOptionsVhostUserBlk {
#[serde(rename = "logical-block-size", default, skip_serializing_if = "Option::is_none")]
pub logical_block_size: Option<u64>,
#[serde(rename = "num-queues", default, skip_serializing_if = "Option::is_none")]
pub num_queues: Option<u16>,
#[serde(rename = "addr")]
pub addr: SocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockGraphInfo {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockNodeInfo,
#[serde(rename = "children")]
pub children: Vec<BlockChildInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockIOThrottle {
#[serde(rename = "bps_max", default, skip_serializing_if = "Option::is_none")]
pub bps_max: Option<i64>,
#[serde(rename = "bps_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_max_length: Option<i64>,
#[serde(rename = "bps_rd_max", default, skip_serializing_if = "Option::is_none")]
pub bps_rd_max: Option<i64>,
#[serde(rename = "bps_rd_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_rd_max_length: Option<i64>,
#[serde(rename = "bps_wr_max", default, skip_serializing_if = "Option::is_none")]
pub bps_wr_max: Option<i64>,
#[serde(rename = "bps_wr_max_length", default, skip_serializing_if = "Option::is_none")]
pub bps_wr_max_length: Option<i64>,
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub device: Option<::std::string::String>,
#[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
pub group: Option<::std::string::String>,
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "iops_max", default, skip_serializing_if = "Option::is_none")]
pub iops_max: Option<i64>,
#[serde(rename = "iops_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_max_length: Option<i64>,
#[serde(rename = "iops_rd_max", default, skip_serializing_if = "Option::is_none")]
pub iops_rd_max: Option<i64>,
#[serde(rename = "iops_rd_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_rd_max_length: Option<i64>,
#[serde(rename = "iops_size", default, skip_serializing_if = "Option::is_none")]
pub iops_size: Option<i64>,
#[serde(rename = "iops_wr_max", default, skip_serializing_if = "Option::is_none")]
pub iops_wr_max: Option<i64>,
#[serde(rename = "iops_wr_max_length", default, skip_serializing_if = "Option::is_none")]
pub iops_wr_max_length: Option<i64>,
#[serde(rename = "bps")]
pub bps: i64,
#[serde(rename = "bps_rd")]
pub bps_rd: i64,
#[serde(rename = "bps_wr")]
pub bps_wr: i64,
#[serde(rename = "iops")]
pub iops: i64,
#[serde(rename = "iops_rd")]
pub iops_rd: i64,
#[serde(rename = "iops_wr")]
pub iops_wr: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
#[serde(rename = "inserted", default, skip_serializing_if = "Option::is_none")]
pub inserted: Option<BlockDeviceInfo>,
#[serde(rename = "io-status", default, skip_serializing_if = "Option::is_none")]
pub io_status: Option<BlockDeviceIoStatus>,
#[serde(rename = "qdev", default, skip_serializing_if = "Option::is_none")]
pub qdev: Option<::std::string::String>,
#[serde(rename = "tray_open", default, skip_serializing_if = "Option::is_none")]
pub tray_open: Option<bool>,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "locked")]
pub locked: bool,
#[serde(rename = "removable")]
pub removable: bool,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockJobChangeOptionsMirror {
#[serde(rename = "copy-mode")]
pub copy_mode: MirrorCopyMode,
}

impl<T: Into<MirrorCopyMode>> From<T> for BlockJobChangeOptionsMirror {
    fn from(val: T) -> Self {
        Self {
            copy_mode: val.into(),

        }
    }
}
    impl AsRef<MirrorCopyMode> for BlockJobChangeOptionsMirror {
        fn as_ref(&self) -> &MirrorCopyMode {
            &self.copy_mode
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockJobInfoMirror {
#[serde(rename = "actively-synced")]
pub actively_synced: bool,
}

impl<T: Into<bool>> From<T> for BlockJobInfoMirror {
    fn from(val: T) -> Self {
        Self {
            actively_synced: val.into(),

        }
    }
}
    impl AsRef<bool> for BlockJobInfoMirror {
        fn as_ref(&self) -> &bool {
            &self.actively_synced
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLatencyHistogramInfo {
#[serde(rename = "bins")]
pub bins: Vec<u64>,
#[serde(rename = "boundaries")]
pub boundaries: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMeasureInfo {
#[serde(rename = "bitmaps", default, skip_serializing_if = "Option::is_none")]
pub bitmaps: Option<i64>,
#[serde(rename = "fully-allocated")]
pub fully_allocated: i64,
#[serde(rename = "required")]
pub required: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNodeInfo {
#[serde(rename = "actual-size", default, skip_serializing_if = "Option::is_none")]
pub actual_size: Option<i64>,
#[serde(rename = "backing-filename", default, skip_serializing_if = "Option::is_none")]
pub backing_filename: Option<::std::string::String>,
#[serde(rename = "backing-filename-format", default, skip_serializing_if = "Option::is_none")]
pub backing_filename_format: Option<::std::string::String>,
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<i64>,
#[serde(rename = "compressed", default, skip_serializing_if = "Option::is_none")]
pub compressed: Option<bool>,
#[serde(rename = "dirty-flag", default, skip_serializing_if = "Option::is_none")]
pub dirty_flag: Option<bool>,
#[serde(rename = "encrypted", default, skip_serializing_if = "Option::is_none")]
pub encrypted: Option<bool>,
#[serde(rename = "format-specific", default, skip_serializing_if = "Option::is_none")]
pub format_specific: Option<ImageInfoSpecific>,
#[serde(rename = "full-backing-filename", default, skip_serializing_if = "Option::is_none")]
pub full_backing_filename: Option<::std::string::String>,
#[serde(rename = "snapshots", default, skip_serializing_if = "Option::is_none")]
pub snapshots: Option<Vec<SnapshotInfo>>,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "format")]
pub format: ::std::string::String,
#[serde(rename = "virtual-size")]
pub virtual_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStats {
#[serde(rename = "backing", default, skip_serializing_if = "Option::is_none")]
pub backing: Option<Box<BlockStats>>,
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
#[serde(rename = "driver-specific", default, skip_serializing_if = "Option::is_none")]
pub driver_specific: Option<BlockStatsSpecific>,
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
pub parent: Option<Box<BlockStats>>,
#[serde(rename = "qdev", default, skip_serializing_if = "Option::is_none")]
pub qdev: Option<::std::string::String>,
#[serde(rename = "stats")]
pub stats: BlockDeviceStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatsSpecificFile {
#[serde(rename = "discard-bytes-ok")]
pub discard_bytes_ok: u64,
#[serde(rename = "discard-nb-failed")]
pub discard_nb_failed: u64,
#[serde(rename = "discard-nb-ok")]
pub discard_nb_ok: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatsSpecificNvme {
#[serde(rename = "aligned-accesses")]
pub aligned_accesses: u64,
#[serde(rename = "completion-errors")]
pub completion_errors: u64,
#[serde(rename = "unaligned-accesses")]
pub unaligned_accesses: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevAmendOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: QCryptoBlockAmendOptionsLUKS,
}

impl<T: Into<QCryptoBlockAmendOptionsLUKS>> From<T> for BlockdevAmendOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<QCryptoBlockAmendOptionsLUKS> for BlockdevAmendOptionsLUKS {
        fn as_ref(&self) -> &QCryptoBlockAmendOptionsLUKS {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockdevAmendOptionsQcow2 {
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<QCryptoBlockAmendOptions>,
}

impl<T: Into<QCryptoBlockAmendOptions>> From<T> for BlockdevAmendOptionsQcow2 {
    fn from(val: T) -> Self {
        Self {
            encrypt: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevBackup {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BackupCommon,
#[serde(rename = "target")]
pub target: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockdevBackupWrapper {
#[serde(rename = "data")]
pub data: BlockdevBackup,
}

impl<T: Into<BlockdevBackup>> From<T> for BlockdevBackupWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockdevBackup> for BlockdevBackupWrapper {
        fn as_ref(&self) -> &BlockdevBackup {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockdevBackupWrapper {
    type Target = BlockdevBackup;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockdevBackupWrapper {
    pub fn into_inner(self) -> BlockdevBackup {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCacheInfo {
#[serde(rename = "direct")]
pub direct: bool,
#[serde(rename = "no-flush")]
pub no_flush: bool,
#[serde(rename = "writeback")]
pub writeback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockdevCacheOptions {
#[serde(rename = "direct", default, skip_serializing_if = "Option::is_none")]
pub direct: Option<bool>,
#[serde(rename = "no-flush", default, skip_serializing_if = "Option::is_none")]
pub no_flush: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsFile {
#[serde(rename = "extent-size-hint", default, skip_serializing_if = "Option::is_none")]
pub extent_size_hint: Option<u64>,
#[serde(rename = "nocow", default, skip_serializing_if = "Option::is_none")]
pub nocow: Option<bool>,
#[serde(rename = "preallocation", default, skip_serializing_if = "Option::is_none")]
pub preallocation: Option<PreallocMode>,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsGluster {
#[serde(rename = "preallocation", default, skip_serializing_if = "Option::is_none")]
pub preallocation: Option<PreallocMode>,
#[serde(rename = "location")]
pub location: BlockdevOptionsGluster,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: QCryptoBlockCreateOptionsLUKS,
#[serde(rename = "file", default, skip_serializing_if = "Option::is_none")]
pub file: Option<BlockdevRef>,
#[serde(rename = "header", default, skip_serializing_if = "Option::is_none")]
pub header: Option<BlockdevRef>,
#[serde(rename = "preallocation", default, skip_serializing_if = "Option::is_none")]
pub preallocation: Option<PreallocMode>,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsNfs {
#[serde(rename = "location")]
pub location: BlockdevOptionsNfs,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsParallels {
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<u64>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsQcow {
#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<QCryptoBlockCreateOptions>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsQcow2 {
#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
#[serde(rename = "backing-fmt", default, skip_serializing_if = "Option::is_none")]
pub backing_fmt: Option<BlockdevDriver>,
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<u64>,
#[serde(rename = "compression-type", default, skip_serializing_if = "Option::is_none")]
pub compression_type: Option<Qcow2CompressionType>,
#[serde(rename = "data-file", default, skip_serializing_if = "Option::is_none")]
pub data_file: Option<BlockdevRef>,
#[serde(rename = "data-file-raw", default, skip_serializing_if = "Option::is_none")]
pub data_file_raw: Option<bool>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<QCryptoBlockCreateOptions>,
#[serde(rename = "extended-l2", default, skip_serializing_if = "Option::is_none")]
pub extended_l2: Option<bool>,
#[serde(rename = "lazy-refcounts", default, skip_serializing_if = "Option::is_none")]
pub lazy_refcounts: Option<bool>,
#[serde(rename = "preallocation", default, skip_serializing_if = "Option::is_none")]
pub preallocation: Option<PreallocMode>,
#[serde(rename = "refcount-bits", default, skip_serializing_if = "Option::is_none")]
pub refcount_bits: Option<i64>,
#[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
pub version: Option<BlockdevQcow2Version>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsQed {
#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
#[serde(rename = "backing-fmt", default, skip_serializing_if = "Option::is_none")]
pub backing_fmt: Option<BlockdevDriver>,
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<u64>,
#[serde(rename = "table-size", default, skip_serializing_if = "Option::is_none")]
pub table_size: Option<i64>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsRbd {
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<u64>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<RbdEncryptionCreateOptions>,
#[serde(rename = "location")]
pub location: BlockdevOptionsRbd,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsSsh {
#[serde(rename = "location")]
pub location: BlockdevOptionsSsh,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsVdi {
#[serde(rename = "preallocation", default, skip_serializing_if = "Option::is_none")]
pub preallocation: Option<PreallocMode>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsVhdx {
#[serde(rename = "block-size", default, skip_serializing_if = "Option::is_none")]
pub block_size: Option<u64>,
#[serde(rename = "block-state-zero", default, skip_serializing_if = "Option::is_none")]
pub block_state_zero: Option<bool>,
#[serde(rename = "log-size", default, skip_serializing_if = "Option::is_none")]
pub log_size: Option<u64>,
#[serde(rename = "subformat", default, skip_serializing_if = "Option::is_none")]
pub subformat: Option<BlockdevVhdxSubformat>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsVmdk {
#[serde(rename = "adapter-type", default, skip_serializing_if = "Option::is_none")]
pub adapter_type: Option<BlockdevVmdkAdapterType>,
#[serde(rename = "backing-file", default, skip_serializing_if = "Option::is_none")]
pub backing_file: Option<::std::string::String>,
#[serde(rename = "extents", default, skip_serializing_if = "Option::is_none")]
pub extents: Option<Vec<BlockdevRef>>,
#[serde(rename = "hwversion", default, skip_serializing_if = "Option::is_none")]
pub hwversion: Option<::std::string::String>,
#[serde(rename = "subformat", default, skip_serializing_if = "Option::is_none")]
pub subformat: Option<BlockdevVmdkSubformat>,
#[serde(rename = "toolsversion", default, skip_serializing_if = "Option::is_none")]
pub toolsversion: Option<::std::string::String>,
#[serde(rename = "zeroed-grain", default, skip_serializing_if = "Option::is_none")]
pub zeroed_grain: Option<bool>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevCreateOptionsVpc {
#[serde(rename = "force-size", default, skip_serializing_if = "Option::is_none")]
pub force_size: Option<bool>,
#[serde(rename = "subformat", default, skip_serializing_if = "Option::is_none")]
pub subformat: Option<BlockdevVpcSubformat>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsBlkdebug {
#[serde(rename = "align", default, skip_serializing_if = "Option::is_none")]
pub align: Option<i64>,
#[serde(rename = "config", default, skip_serializing_if = "Option::is_none")]
pub config: Option<::std::string::String>,
#[serde(rename = "inject-error", default, skip_serializing_if = "Option::is_none")]
pub inject_error: Option<Vec<BlkdebugInjectErrorOptions>>,
#[serde(rename = "max-discard", default, skip_serializing_if = "Option::is_none")]
pub max_discard: Option<i32>,
#[serde(rename = "max-transfer", default, skip_serializing_if = "Option::is_none")]
pub max_transfer: Option<i32>,
#[serde(rename = "max-write-zero", default, skip_serializing_if = "Option::is_none")]
pub max_write_zero: Option<i32>,
#[serde(rename = "opt-discard", default, skip_serializing_if = "Option::is_none")]
pub opt_discard: Option<i32>,
#[serde(rename = "opt-write-zero", default, skip_serializing_if = "Option::is_none")]
pub opt_write_zero: Option<i32>,
#[serde(rename = "set-state", default, skip_serializing_if = "Option::is_none")]
pub set_state: Option<Vec<BlkdebugSetStateOptions>>,
#[serde(rename = "take-child-perms", default, skip_serializing_if = "Option::is_none")]
pub take_child_perms: Option<Vec<BlockPermission>>,
#[serde(rename = "unshare-child-perms", default, skip_serializing_if = "Option::is_none")]
pub unshare_child_perms: Option<Vec<BlockPermission>>,
#[serde(rename = "image")]
pub image: BlockdevRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsBlklogwrites {
#[serde(rename = "log-append", default, skip_serializing_if = "Option::is_none")]
pub log_append: Option<bool>,
#[serde(rename = "log-sector-size", default, skip_serializing_if = "Option::is_none")]
pub log_sector_size: Option<u32>,
#[serde(rename = "log-super-update-interval", default, skip_serializing_if = "Option::is_none")]
pub log_super_update_interval: Option<u64>,
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "log")]
pub log: BlockdevRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsBlkreplay {
#[serde(rename = "image")]
pub image: BlockdevRef,
}

impl<T: Into<BlockdevRef>> From<T> for BlockdevOptionsBlkreplay {
    fn from(val: T) -> Self {
        Self {
            image: val.into(),

        }
    }
}
    impl AsRef<BlockdevRef> for BlockdevOptionsBlkreplay {
        fn as_ref(&self) -> &BlockdevRef {
            &self.image
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsBlkverify {
#[serde(rename = "raw")]
pub raw: BlockdevRef,
#[serde(rename = "test")]
pub test: BlockdevRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCbw {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "bitmap", default, skip_serializing_if = "Option::is_none")]
pub bitmap: Option<BlockDirtyBitmap>,
#[serde(rename = "cbw-timeout", default, skip_serializing_if = "Option::is_none")]
pub cbw_timeout: Option<u32>,
#[serde(rename = "on-cbw-error", default, skip_serializing_if = "Option::is_none")]
pub on_cbw_error: Option<OnCbwError>,
#[serde(rename = "target")]
pub target: BlockdevRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCor {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "bottom", default, skip_serializing_if = "Option::is_none")]
pub bottom: Option<::std::string::String>,
}

impl<T: Into<BlockdevOptionsGenericFormat>> From<T> for BlockdevOptionsCor {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
bottom: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericFormat> for BlockdevOptionsCor {
        fn as_ref(&self) -> &BlockdevOptionsGenericFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCurlBase {
#[serde(rename = "password-secret", default, skip_serializing_if = "Option::is_none")]
pub password_secret: Option<::std::string::String>,
#[serde(rename = "proxy-password-secret", default, skip_serializing_if = "Option::is_none")]
pub proxy_password_secret: Option<::std::string::String>,
#[serde(rename = "proxy-username", default, skip_serializing_if = "Option::is_none")]
pub proxy_username: Option<::std::string::String>,
#[serde(rename = "readahead", default, skip_serializing_if = "Option::is_none")]
pub readahead: Option<i64>,
#[serde(rename = "timeout", default, skip_serializing_if = "Option::is_none")]
pub timeout: Option<i64>,
#[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
pub username: Option<::std::string::String>,
#[serde(rename = "url")]
pub url: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCurlFtp {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsCurlBase,
}

impl<T: Into<BlockdevOptionsCurlBase>> From<T> for BlockdevOptionsCurlFtp {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<BlockdevOptionsCurlBase> for BlockdevOptionsCurlFtp {
        fn as_ref(&self) -> &BlockdevOptionsCurlBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCurlFtps {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsCurlBase,
#[serde(rename = "sslverify", default, skip_serializing_if = "Option::is_none")]
pub sslverify: Option<bool>,
}

impl<T: Into<BlockdevOptionsCurlBase>> From<T> for BlockdevOptionsCurlFtps {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
sslverify: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsCurlBase> for BlockdevOptionsCurlFtps {
        fn as_ref(&self) -> &BlockdevOptionsCurlBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCurlHttp {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsCurlBase,
#[serde(rename = "cookie", default, skip_serializing_if = "Option::is_none")]
pub cookie: Option<::std::string::String>,
#[serde(rename = "cookie-secret", default, skip_serializing_if = "Option::is_none")]
pub cookie_secret: Option<::std::string::String>,
}

impl<T: Into<BlockdevOptionsCurlBase>> From<T> for BlockdevOptionsCurlHttp {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cookie: Default::default(),
cookie_secret: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsCurlBase> for BlockdevOptionsCurlHttp {
        fn as_ref(&self) -> &BlockdevOptionsCurlBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsCurlHttps {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsCurlBase,
#[serde(rename = "cookie", default, skip_serializing_if = "Option::is_none")]
pub cookie: Option<::std::string::String>,
#[serde(rename = "cookie-secret", default, skip_serializing_if = "Option::is_none")]
pub cookie_secret: Option<::std::string::String>,
#[serde(rename = "sslverify", default, skip_serializing_if = "Option::is_none")]
pub sslverify: Option<bool>,
}

impl<T: Into<BlockdevOptionsCurlBase>> From<T> for BlockdevOptionsCurlHttps {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cookie: Default::default(),
cookie_secret: Default::default(),
sslverify: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsCurlBase> for BlockdevOptionsCurlHttps {
        fn as_ref(&self) -> &BlockdevOptionsCurlBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsFile {
#[serde(rename = "aio", default, skip_serializing_if = "Option::is_none")]
pub aio: Option<BlockdevAioOptions>,
#[serde(rename = "aio-max-batch", default, skip_serializing_if = "Option::is_none")]
pub aio_max_batch: Option<i64>,
#[serde(rename = "drop-cache", default, skip_serializing_if = "Option::is_none")]
pub drop_cache: Option<bool>,
#[serde(rename = "locking", default, skip_serializing_if = "Option::is_none")]
pub locking: Option<OnOffAuto>,
#[serde(rename = "pr-manager", default, skip_serializing_if = "Option::is_none")]
pub pr_manager: Option<::std::string::String>,
#[serde(rename = "x-check-cache-dropped", default, skip_serializing_if = "Option::is_none")]
pub x_check_cache_dropped: Option<bool>,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsGenericCOWFormat {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "backing", default, skip_serializing_if = "Option::is_none")]
pub backing: Option<BlockdevRefOrNull>,
}

impl<T: Into<BlockdevOptionsGenericFormat>> From<T> for BlockdevOptionsGenericCOWFormat {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
backing: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericFormat> for BlockdevOptionsGenericCOWFormat {
        fn as_ref(&self) -> &BlockdevOptionsGenericFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsGenericFormat {
#[serde(rename = "file")]
pub file: BlockdevRef,
}

impl<T: Into<BlockdevRef>> From<T> for BlockdevOptionsGenericFormat {
    fn from(val: T) -> Self {
        Self {
            file: val.into(),

        }
    }
}
    impl AsRef<BlockdevRef> for BlockdevOptionsGenericFormat {
        fn as_ref(&self) -> &BlockdevRef {
            &self.file
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsGluster {
#[serde(rename = "debug", default, skip_serializing_if = "Option::is_none")]
pub debug: Option<i64>,
#[serde(rename = "logfile", default, skip_serializing_if = "Option::is_none")]
pub logfile: Option<::std::string::String>,
#[serde(rename = "path")]
pub path: ::std::string::String,
#[serde(rename = "server")]
pub server: Vec<SocketAddress>,
#[serde(rename = "volume")]
pub volume: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsIoUring {
#[serde(rename = "filename")]
pub filename: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockdevOptionsIoUring {
    fn from(val: T) -> Self {
        Self {
            filename: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockdevOptionsIoUring {
        fn as_ref(&self) -> &::std::string::String {
            &self.filename
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsIscsi {
#[serde(rename = "header-digest", default, skip_serializing_if = "Option::is_none")]
pub header_digest: Option<IscsiHeaderDigest>,
#[serde(rename = "initiator-name", default, skip_serializing_if = "Option::is_none")]
pub initiator_name: Option<::std::string::String>,
#[serde(rename = "lun", default, skip_serializing_if = "Option::is_none")]
pub lun: Option<i64>,
#[serde(rename = "password-secret", default, skip_serializing_if = "Option::is_none")]
pub password_secret: Option<::std::string::String>,
#[serde(rename = "timeout", default, skip_serializing_if = "Option::is_none")]
pub timeout: Option<i64>,
#[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
pub user: Option<::std::string::String>,
#[serde(rename = "portal")]
pub portal: ::std::string::String,
#[serde(rename = "target")]
pub target: ::std::string::String,
#[serde(rename = "transport")]
pub transport: IscsiTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "header", default, skip_serializing_if = "Option::is_none")]
pub header: Option<BlockdevRef>,
#[serde(rename = "key-secret", default, skip_serializing_if = "Option::is_none")]
pub key_secret: Option<::std::string::String>,
}

impl<T: Into<BlockdevOptionsGenericFormat>> From<T> for BlockdevOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
header: Default::default(),
key_secret: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericFormat> for BlockdevOptionsLUKS {
        fn as_ref(&self) -> &BlockdevOptionsGenericFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsNVMe {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "namespace")]
pub namespace: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsNbd {
#[serde(rename = "export", default, skip_serializing_if = "Option::is_none")]
pub export: Option<::std::string::String>,
#[serde(rename = "open-timeout", default, skip_serializing_if = "Option::is_none")]
pub open_timeout: Option<u32>,
#[serde(rename = "reconnect-delay", default, skip_serializing_if = "Option::is_none")]
pub reconnect_delay: Option<u32>,
#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<::std::string::String>,
#[serde(rename = "tls-hostname", default, skip_serializing_if = "Option::is_none")]
pub tls_hostname: Option<::std::string::String>,
#[serde(rename = "x-dirty-bitmap", default, skip_serializing_if = "Option::is_none")]
pub x_dirty_bitmap: Option<::std::string::String>,
#[serde(rename = "server")]
pub server: SocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsNfs {
#[serde(rename = "debug", default, skip_serializing_if = "Option::is_none")]
pub debug: Option<i64>,
#[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
pub group: Option<i64>,
#[serde(rename = "page-cache-size", default, skip_serializing_if = "Option::is_none")]
pub page_cache_size: Option<i64>,
#[serde(rename = "readahead-size", default, skip_serializing_if = "Option::is_none")]
pub readahead_size: Option<i64>,
#[serde(rename = "tcp-syn-count", default, skip_serializing_if = "Option::is_none")]
pub tcp_syn_count: Option<i64>,
#[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
pub user: Option<i64>,
#[serde(rename = "path")]
pub path: ::std::string::String,
#[serde(rename = "server")]
pub server: NFSServer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockdevOptionsNull {
#[serde(rename = "latency-ns", default, skip_serializing_if = "Option::is_none")]
pub latency_ns: Option<u64>,
#[serde(rename = "read-zeroes", default, skip_serializing_if = "Option::is_none")]
pub read_zeroes: Option<bool>,
#[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsNvmeIoUring {
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockdevOptionsNvmeIoUring {
    fn from(val: T) -> Self {
        Self {
            path: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockdevOptionsNvmeIoUring {
        fn as_ref(&self) -> &::std::string::String {
            &self.path
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsPreallocate {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "prealloc-align", default, skip_serializing_if = "Option::is_none")]
pub prealloc_align: Option<i64>,
#[serde(rename = "prealloc-size", default, skip_serializing_if = "Option::is_none")]
pub prealloc_size: Option<i64>,
}

impl<T: Into<BlockdevOptionsGenericFormat>> From<T> for BlockdevOptionsPreallocate {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
prealloc_align: Default::default(),
prealloc_size: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericFormat> for BlockdevOptionsPreallocate {
        fn as_ref(&self) -> &BlockdevOptionsGenericFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsQcow {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericCOWFormat,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<BlockdevQcowEncryption>,
}

impl<T: Into<BlockdevOptionsGenericCOWFormat>> From<T> for BlockdevOptionsQcow {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
encrypt: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericCOWFormat> for BlockdevOptionsQcow {
        fn as_ref(&self) -> &BlockdevOptionsGenericCOWFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsQcow2 {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericCOWFormat,
#[serde(rename = "cache-clean-interval", default, skip_serializing_if = "Option::is_none")]
pub cache_clean_interval: Option<i64>,
#[serde(rename = "cache-size", default, skip_serializing_if = "Option::is_none")]
pub cache_size: Option<i64>,
#[serde(rename = "data-file", default, skip_serializing_if = "Option::is_none")]
pub data_file: Option<BlockdevRef>,
#[serde(rename = "discard-no-unref", default, skip_serializing_if = "Option::is_none")]
pub discard_no_unref: Option<bool>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<BlockdevQcow2Encryption>,
#[serde(rename = "l2-cache-entry-size", default, skip_serializing_if = "Option::is_none")]
pub l2_cache_entry_size: Option<i64>,
#[serde(rename = "l2-cache-size", default, skip_serializing_if = "Option::is_none")]
pub l2_cache_size: Option<i64>,
#[serde(rename = "lazy-refcounts", default, skip_serializing_if = "Option::is_none")]
pub lazy_refcounts: Option<bool>,
#[serde(rename = "overlap-check", default, skip_serializing_if = "Option::is_none")]
pub overlap_check: Option<Qcow2OverlapChecks>,
#[serde(rename = "pass-discard-other", default, skip_serializing_if = "Option::is_none")]
pub pass_discard_other: Option<bool>,
#[serde(rename = "pass-discard-request", default, skip_serializing_if = "Option::is_none")]
pub pass_discard_request: Option<bool>,
#[serde(rename = "pass-discard-snapshot", default, skip_serializing_if = "Option::is_none")]
pub pass_discard_snapshot: Option<bool>,
#[serde(rename = "refcount-cache-size", default, skip_serializing_if = "Option::is_none")]
pub refcount_cache_size: Option<i64>,
}

impl<T: Into<BlockdevOptionsGenericCOWFormat>> From<T> for BlockdevOptionsQcow2 {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cache_clean_interval: Default::default(),
cache_size: Default::default(),
data_file: Default::default(),
discard_no_unref: Default::default(),
encrypt: Default::default(),
l2_cache_entry_size: Default::default(),
l2_cache_size: Default::default(),
lazy_refcounts: Default::default(),
overlap_check: Default::default(),
pass_discard_other: Default::default(),
pass_discard_request: Default::default(),
pass_discard_snapshot: Default::default(),
refcount_cache_size: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericCOWFormat> for BlockdevOptionsQcow2 {
        fn as_ref(&self) -> &BlockdevOptionsGenericCOWFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsQuorum {
#[serde(rename = "blkverify", default, skip_serializing_if = "Option::is_none")]
pub blkverify: Option<bool>,
#[serde(rename = "read-pattern", default, skip_serializing_if = "Option::is_none")]
pub read_pattern: Option<QuorumReadPattern>,
#[serde(rename = "rewrite-corrupted", default, skip_serializing_if = "Option::is_none")]
pub rewrite_corrupted: Option<bool>,
#[serde(rename = "children")]
pub children: Vec<BlockdevRef>,
#[serde(rename = "vote-threshold")]
pub vote_threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsRaw {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
pub offset: Option<i64>,
#[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
pub size: Option<i64>,
}

impl<T: Into<BlockdevOptionsGenericFormat>> From<T> for BlockdevOptionsRaw {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
offset: Default::default(),
size: Default::default(),

        }
    }
}
    impl AsRef<BlockdevOptionsGenericFormat> for BlockdevOptionsRaw {
        fn as_ref(&self) -> &BlockdevOptionsGenericFormat {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsRbd {
#[serde(rename = "auth-client-required", default, skip_serializing_if = "Option::is_none")]
pub auth_client_required: Option<Vec<RbdAuthMode>>,
#[serde(rename = "conf", default, skip_serializing_if = "Option::is_none")]
pub conf: Option<::std::string::String>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<RbdEncryptionOptions>,
#[serde(rename = "key-secret", default, skip_serializing_if = "Option::is_none")]
pub key_secret: Option<::std::string::String>,
#[serde(rename = "namespace", default, skip_serializing_if = "Option::is_none")]
pub namespace: Option<::std::string::String>,
#[serde(rename = "server", default, skip_serializing_if = "Option::is_none")]
pub server: Option<Vec<InetSocketAddressBase>>,
#[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
pub snapshot: Option<::std::string::String>,
#[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
pub user: Option<::std::string::String>,
#[serde(rename = "image")]
pub image: ::std::string::String,
#[serde(rename = "pool")]
pub pool: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsReplication {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockdevOptionsGenericFormat,
#[serde(rename = "top-id", default, skip_serializing_if = "Option::is_none")]
pub top_id: Option<::std::string::String>,
#[serde(rename = "mode")]
pub mode: ReplicationMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsSsh {
#[serde(rename = "host-key-check", default, skip_serializing_if = "Option::is_none")]
pub host_key_check: Option<SshHostKeyCheck>,
#[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
pub user: Option<::std::string::String>,
#[serde(rename = "path")]
pub path: ::std::string::String,
#[serde(rename = "server")]
pub server: InetSocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsThrottle {
#[serde(rename = "file")]
pub file: BlockdevRef,
#[serde(rename = "throttle-group")]
pub throttle_group: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsVVFAT {
#[serde(rename = "fat-type", default, skip_serializing_if = "Option::is_none")]
pub fat_type: Option<i64>,
#[serde(rename = "floppy", default, skip_serializing_if = "Option::is_none")]
pub floppy: Option<bool>,
#[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
pub label: Option<::std::string::String>,
#[serde(rename = "rw", default, skip_serializing_if = "Option::is_none")]
pub rw: Option<bool>,
#[serde(rename = "dir")]
pub dir: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsVirtioBlkVfioPci {
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockdevOptionsVirtioBlkVfioPci {
    fn from(val: T) -> Self {
        Self {
            path: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockdevOptionsVirtioBlkVfioPci {
        fn as_ref(&self) -> &::std::string::String {
            &self.path
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsVirtioBlkVhostUser {
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockdevOptionsVirtioBlkVhostUser {
    fn from(val: T) -> Self {
        Self {
            path: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockdevOptionsVirtioBlkVhostUser {
        fn as_ref(&self) -> &::std::string::String {
            &self.path
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevOptionsVirtioBlkVhostVdpa {
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for BlockdevOptionsVirtioBlkVhostVdpa {
    fn from(val: T) -> Self {
        Self {
            path: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for BlockdevOptionsVirtioBlkVhostVdpa {
        fn as_ref(&self) -> &::std::string::String {
            &self.path
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevSnapshot {
#[serde(rename = "node")]
pub node: ::std::string::String,
#[serde(rename = "overlay")]
pub overlay: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevSnapshotInternal {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockdevSnapshotInternalWrapper {
#[serde(rename = "data")]
pub data: BlockdevSnapshotInternal,
}

impl<T: Into<BlockdevSnapshotInternal>> From<T> for BlockdevSnapshotInternalWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockdevSnapshotInternal> for BlockdevSnapshotInternalWrapper {
        fn as_ref(&self) -> &BlockdevSnapshotInternal {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockdevSnapshotInternalWrapper {
    type Target = BlockdevSnapshotInternal;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockdevSnapshotInternalWrapper {
    pub fn into_inner(self) -> BlockdevSnapshotInternal {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockdevSnapshotSync {
#[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
pub device: Option<::std::string::String>,
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<::std::string::String>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<NewImageMode>,
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "snapshot-node-name", default, skip_serializing_if = "Option::is_none")]
pub snapshot_node_name: Option<::std::string::String>,
#[serde(rename = "snapshot-file")]
pub snapshot_file: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockdevSnapshotSyncWrapper {
#[serde(rename = "data")]
pub data: BlockdevSnapshotSync,
}

impl<T: Into<BlockdevSnapshotSync>> From<T> for BlockdevSnapshotSyncWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockdevSnapshotSync> for BlockdevSnapshotSyncWrapper {
        fn as_ref(&self) -> &BlockdevSnapshotSync {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockdevSnapshotSyncWrapper {
    type Target = BlockdevSnapshotSync;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockdevSnapshotSyncWrapper {
    pub fn into_inner(self) -> BlockdevSnapshotSync {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct BlockdevSnapshotWrapper {
#[serde(rename = "data")]
pub data: BlockdevSnapshot,
}

impl<T: Into<BlockdevSnapshot>> From<T> for BlockdevSnapshotWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<BlockdevSnapshot> for BlockdevSnapshotWrapper {
        fn as_ref(&self) -> &BlockdevSnapshot {
            &self.data
        }
    }
impl ::std::ops::Deref for BlockdevSnapshotWrapper {
    type Target = BlockdevSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl BlockdevSnapshotWrapper {
    pub fn into_inner(self) -> BlockdevSnapshot {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BootConfiguration {
#[serde(rename = "menu", default, skip_serializing_if = "Option::is_none")]
pub menu: Option<bool>,
#[serde(rename = "once", default, skip_serializing_if = "Option::is_none")]
pub once: Option<::std::string::String>,
#[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
pub order: Option<::std::string::String>,
#[serde(rename = "reboot-timeout", default, skip_serializing_if = "Option::is_none")]
pub reboot_timeout: Option<i64>,
#[serde(rename = "splash", default, skip_serializing_if = "Option::is_none")]
pub splash: Option<::std::string::String>,
#[serde(rename = "splash-time", default, skip_serializing_if = "Option::is_none")]
pub splash_time: Option<i64>,
#[serde(rename = "strict", default, skip_serializing_if = "Option::is_none")]
pub strict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct COLOStatus {
#[serde(rename = "last-mode")]
pub last_mode: COLOMode,
#[serde(rename = "mode")]
pub mode: COLOMode,
#[serde(rename = "reason")]
pub reason: COLOExitReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CXLFMWProperties {
#[serde(rename = "cxl-fmw")]
pub cxl_fmw: Vec<CXLFixedMemoryWindowOptions>,
}

impl<T: Into<Vec<CXLFixedMemoryWindowOptions>>> From<T> for CXLFMWProperties {
    fn from(val: T) -> Self {
        Self {
            cxl_fmw: val.into(),

        }
    }
}
    impl AsRef<Vec<CXLFixedMemoryWindowOptions>> for CXLFMWProperties {
        fn as_ref(&self) -> &Vec<CXLFixedMemoryWindowOptions> {
            &self.cxl_fmw
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CXLFixedMemoryWindowOptions {
#[serde(rename = "interleave-granularity", default, skip_serializing_if = "Option::is_none")]
pub interleave_granularity: Option<u64>,
#[serde(rename = "size")]
pub size: u64,
#[serde(rename = "targets")]
pub targets: Vec<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CXLUncorErrorRecord {
#[serde(rename = "header")]
pub header: Vec<u32>,
#[serde(rename = "type")]
pub type_: CxlUncorErrorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanHostSocketcanProperties {
#[serde(rename = "canbus")]
pub canbus: ::std::string::String,
#[serde(rename = "if")]
pub if_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevBackendInfo {
#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for ChardevBackendInfo {
    fn from(val: T) -> Self {
        Self {
            name: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for ChardevBackendInfo {
        fn as_ref(&self) -> &::std::string::String {
            &self.name
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChardevCommon {
#[serde(rename = "logappend", default, skip_serializing_if = "Option::is_none")]
pub logappend: Option<bool>,
#[serde(rename = "logfile", default, skip_serializing_if = "Option::is_none")]
pub logfile: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevCommonWrapper {
#[serde(rename = "data")]
pub data: ChardevCommon,
}

impl<T: Into<ChardevCommon>> From<T> for ChardevCommonWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevCommon> for ChardevCommonWrapper {
        fn as_ref(&self) -> &ChardevCommon {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevCommonWrapper {
    type Target = ChardevCommon;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevCommonWrapper {
    pub fn into_inner(self) -> ChardevCommon {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevDBus {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevDBusWrapper {
#[serde(rename = "data")]
pub data: ChardevDBus,
}

impl<T: Into<ChardevDBus>> From<T> for ChardevDBusWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevDBus> for ChardevDBusWrapper {
        fn as_ref(&self) -> &ChardevDBus {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevDBusWrapper {
    type Target = ChardevDBus;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevDBusWrapper {
    pub fn into_inner(self) -> ChardevDBus {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevFile {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "append", default, skip_serializing_if = "Option::is_none")]
pub append: Option<bool>,
#[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
pub in_: Option<::std::string::String>,
#[serde(rename = "out")]
pub out: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevFileWrapper {
#[serde(rename = "data")]
pub data: ChardevFile,
}

impl<T: Into<ChardevFile>> From<T> for ChardevFileWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevFile> for ChardevFileWrapper {
        fn as_ref(&self) -> &ChardevFile {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevFileWrapper {
    type Target = ChardevFile;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevFileWrapper {
    pub fn into_inner(self) -> ChardevFile {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevHostdev {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "device")]
pub device: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevHostdevWrapper {
#[serde(rename = "data")]
pub data: ChardevHostdev,
}

impl<T: Into<ChardevHostdev>> From<T> for ChardevHostdevWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevHostdev> for ChardevHostdevWrapper {
        fn as_ref(&self) -> &ChardevHostdev {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevHostdevWrapper {
    type Target = ChardevHostdev;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevHostdevWrapper {
    pub fn into_inner(self) -> ChardevHostdev {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevInfo {
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "frontend-open")]
pub frontend_open: bool,
#[serde(rename = "label")]
pub label: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevMux {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevMuxWrapper {
#[serde(rename = "data")]
pub data: ChardevMux,
}

impl<T: Into<ChardevMux>> From<T> for ChardevMuxWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevMux> for ChardevMuxWrapper {
        fn as_ref(&self) -> &ChardevMux {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevMuxWrapper {
    type Target = ChardevMux;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevMuxWrapper {
    pub fn into_inner(self) -> ChardevMux {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevQemuVDAgent {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "clipboard", default, skip_serializing_if = "Option::is_none")]
pub clipboard: Option<bool>,
#[serde(rename = "mouse", default, skip_serializing_if = "Option::is_none")]
pub mouse: Option<bool>,
}

impl<T: Into<ChardevCommon>> From<T> for ChardevQemuVDAgent {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
clipboard: Default::default(),
mouse: Default::default(),

        }
    }
}
    impl AsRef<ChardevCommon> for ChardevQemuVDAgent {
        fn as_ref(&self) -> &ChardevCommon {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevQemuVDAgentWrapper {
#[serde(rename = "data")]
pub data: ChardevQemuVDAgent,
}

impl<T: Into<ChardevQemuVDAgent>> From<T> for ChardevQemuVDAgentWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevQemuVDAgent> for ChardevQemuVDAgentWrapper {
        fn as_ref(&self) -> &ChardevQemuVDAgent {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevQemuVDAgentWrapper {
    type Target = ChardevQemuVDAgent;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevQemuVDAgentWrapper {
    pub fn into_inner(self) -> ChardevQemuVDAgent {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChardevReturn {
#[serde(rename = "pty", default, skip_serializing_if = "Option::is_none")]
pub pty: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for ChardevReturn {
    fn from(val: T) -> Self {
        Self {
            pty: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevRingbuf {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
pub size: Option<i64>,
}

impl<T: Into<ChardevCommon>> From<T> for ChardevRingbuf {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
size: Default::default(),

        }
    }
}
    impl AsRef<ChardevCommon> for ChardevRingbuf {
        fn as_ref(&self) -> &ChardevCommon {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevRingbufWrapper {
#[serde(rename = "data")]
pub data: ChardevRingbuf,
}

impl<T: Into<ChardevRingbuf>> From<T> for ChardevRingbufWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevRingbuf> for ChardevRingbufWrapper {
        fn as_ref(&self) -> &ChardevRingbuf {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevRingbufWrapper {
    type Target = ChardevRingbuf;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevRingbufWrapper {
    pub fn into_inner(self) -> ChardevRingbuf {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevSocket {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "nodelay", default, skip_serializing_if = "Option::is_none")]
pub nodelay: Option<bool>,
#[serde(rename = "reconnect", default, skip_serializing_if = "Option::is_none")]
pub reconnect: Option<i64>,
#[serde(rename = "server", default, skip_serializing_if = "Option::is_none")]
pub server: Option<bool>,
#[serde(rename = "telnet", default, skip_serializing_if = "Option::is_none")]
pub telnet: Option<bool>,
#[serde(rename = "tls-authz", default, skip_serializing_if = "Option::is_none")]
pub tls_authz: Option<::std::string::String>,
#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<::std::string::String>,
#[serde(rename = "tn3270", default, skip_serializing_if = "Option::is_none")]
pub tn3270: Option<bool>,
#[serde(rename = "wait", default, skip_serializing_if = "Option::is_none")]
pub wait: Option<bool>,
#[serde(rename = "websocket", default, skip_serializing_if = "Option::is_none")]
pub websocket: Option<bool>,
#[serde(rename = "addr")]
pub addr: SocketAddressLegacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevSocketWrapper {
#[serde(rename = "data")]
pub data: ChardevSocket,
}

impl<T: Into<ChardevSocket>> From<T> for ChardevSocketWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevSocket> for ChardevSocketWrapper {
        fn as_ref(&self) -> &ChardevSocket {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevSocketWrapper {
    type Target = ChardevSocket;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevSocketWrapper {
    pub fn into_inner(self) -> ChardevSocket {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevSpiceChannel {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevSpiceChannelWrapper {
#[serde(rename = "data")]
pub data: ChardevSpiceChannel,
}

impl<T: Into<ChardevSpiceChannel>> From<T> for ChardevSpiceChannelWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevSpiceChannel> for ChardevSpiceChannelWrapper {
        fn as_ref(&self) -> &ChardevSpiceChannel {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevSpiceChannelWrapper {
    type Target = ChardevSpiceChannel;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevSpiceChannelWrapper {
    pub fn into_inner(self) -> ChardevSpiceChannel {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevSpicePort {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "fqdn")]
pub fqdn: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevSpicePortWrapper {
#[serde(rename = "data")]
pub data: ChardevSpicePort,
}

impl<T: Into<ChardevSpicePort>> From<T> for ChardevSpicePortWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevSpicePort> for ChardevSpicePortWrapper {
        fn as_ref(&self) -> &ChardevSpicePort {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevSpicePortWrapper {
    type Target = ChardevSpicePort;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevSpicePortWrapper {
    pub fn into_inner(self) -> ChardevSpicePort {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevStdio {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "signal", default, skip_serializing_if = "Option::is_none")]
pub signal: Option<bool>,
}

impl<T: Into<ChardevCommon>> From<T> for ChardevStdio {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
signal: Default::default(),

        }
    }
}
    impl AsRef<ChardevCommon> for ChardevStdio {
        fn as_ref(&self) -> &ChardevCommon {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevStdioWrapper {
#[serde(rename = "data")]
pub data: ChardevStdio,
}

impl<T: Into<ChardevStdio>> From<T> for ChardevStdioWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevStdio> for ChardevStdioWrapper {
        fn as_ref(&self) -> &ChardevStdio {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevStdioWrapper {
    type Target = ChardevStdio;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevStdioWrapper {
    pub fn into_inner(self) -> ChardevStdio {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevUdp {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "local", default, skip_serializing_if = "Option::is_none")]
pub local: Option<SocketAddressLegacy>,
#[serde(rename = "remote")]
pub remote: SocketAddressLegacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevUdpWrapper {
#[serde(rename = "data")]
pub data: ChardevUdp,
}

impl<T: Into<ChardevUdp>> From<T> for ChardevUdpWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevUdp> for ChardevUdpWrapper {
        fn as_ref(&self) -> &ChardevUdp {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevUdpWrapper {
    type Target = ChardevUdp;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevUdpWrapper {
    pub fn into_inner(self) -> ChardevUdp {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChardevVC {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: ChardevCommon,
#[serde(rename = "cols", default, skip_serializing_if = "Option::is_none")]
pub cols: Option<i64>,
#[serde(rename = "height", default, skip_serializing_if = "Option::is_none")]
pub height: Option<i64>,
#[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
pub rows: Option<i64>,
#[serde(rename = "width", default, skip_serializing_if = "Option::is_none")]
pub width: Option<i64>,
}

impl<T: Into<ChardevCommon>> From<T> for ChardevVC {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cols: Default::default(),
height: Default::default(),
rows: Default::default(),
width: Default::default(),

        }
    }
}
    impl AsRef<ChardevCommon> for ChardevVC {
        fn as_ref(&self) -> &ChardevCommon {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ChardevVCWrapper {
#[serde(rename = "data")]
pub data: ChardevVC,
}

impl<T: Into<ChardevVC>> From<T> for ChardevVCWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ChardevVC> for ChardevVCWrapper {
        fn as_ref(&self) -> &ChardevVC {
            &self.data
        }
    }
impl ::std::ops::Deref for ChardevVCWrapper {
    type Target = ChardevVC;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ChardevVCWrapper {
    pub fn into_inner(self) -> ChardevVC {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColoCompareProperties {
#[serde(rename = "compare_timeout", default, skip_serializing_if = "Option::is_none")]
pub compare_timeout: Option<u64>,
#[serde(rename = "expired_scan_cycle", default, skip_serializing_if = "Option::is_none")]
pub expired_scan_cycle: Option<u32>,
#[serde(rename = "max_queue_size", default, skip_serializing_if = "Option::is_none")]
pub max_queue_size: Option<u32>,
#[serde(rename = "notify_dev", default, skip_serializing_if = "Option::is_none")]
pub notify_dev: Option<::std::string::String>,
#[serde(rename = "vnet_hdr_support", default, skip_serializing_if = "Option::is_none")]
pub vnet_hdr_support: Option<bool>,
#[serde(rename = "iothread")]
pub iothread: ::std::string::String,
#[serde(rename = "outdev")]
pub outdev: ::std::string::String,
#[serde(rename = "primary_in")]
pub primary_in: ::std::string::String,
#[serde(rename = "secondary_in")]
pub secondary_in: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
#[serde(rename = "name")]
pub name: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for CommandInfo {
    fn from(val: T) -> Self {
        Self {
            name: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for CommandInfo {
        fn as_ref(&self) -> &::std::string::String {
            &self.name
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLineOptionInfo {
#[serde(rename = "option")]
pub option: ::std::string::String,
#[serde(rename = "parameters")]
pub parameters: Vec<CommandLineParameterInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLineParameterInfo {
#[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
pub default: Option<::std::string::String>,
#[serde(rename = "help", default, skip_serializing_if = "Option::is_none")]
pub help: Option<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "type")]
pub type_: CommandLineParameterType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompatPolicy {
#[serde(rename = "deprecated-input", default, skip_serializing_if = "Option::is_none")]
pub deprecated_input: Option<CompatPolicyInput>,
#[serde(rename = "deprecated-output", default, skip_serializing_if = "Option::is_none")]
pub deprecated_output: Option<CompatPolicyOutput>,
#[serde(rename = "unstable-input", default, skip_serializing_if = "Option::is_none")]
pub unstable_input: Option<CompatPolicyInput>,
#[serde(rename = "unstable-output", default, skip_serializing_if = "Option::is_none")]
pub unstable_output: Option<CompatPolicyOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatProperty {
#[serde(rename = "property")]
pub property: ::std::string::String,
#[serde(rename = "qom-type")]
pub qom_type: ::std::string::String,
#[serde(rename = "value")]
pub value: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
#[serde(rename = "busy")]
pub busy: i64,
#[serde(rename = "busy-rate")]
pub busy_rate: f64,
#[serde(rename = "compressed-size")]
pub compressed_size: i64,
#[serde(rename = "compression-rate")]
pub compression_rate: f64,
#[serde(rename = "pages")]
pub pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuDefinitionInfo {
#[serde(rename = "alias-of", default, skip_serializing_if = "Option::is_none")]
pub alias_of: Option<::std::string::String>,
#[serde(rename = "migration-safe", default, skip_serializing_if = "Option::is_none")]
pub migration_safe: Option<bool>,
#[serde(rename = "unavailable-features", default, skip_serializing_if = "Option::is_none")]
pub unavailable_features: Option<Vec<::std::string::String>>,
#[serde(rename = "deprecated")]
pub deprecated: bool,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "static")]
pub static_: bool,
#[serde(rename = "typename")]
pub typename: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfoS390 {
#[serde(rename = "dedicated", default, skip_serializing_if = "Option::is_none")]
pub dedicated: Option<bool>,
#[serde(rename = "entitlement", default, skip_serializing_if = "Option::is_none")]
pub entitlement: Option<CpuS390Entitlement>,
#[serde(rename = "cpu-state")]
pub cpu_state: CpuS390State,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuInstanceProperties {
#[serde(rename = "book-id", default, skip_serializing_if = "Option::is_none")]
pub book_id: Option<i64>,
#[serde(rename = "cluster-id", default, skip_serializing_if = "Option::is_none")]
pub cluster_id: Option<i64>,
#[serde(rename = "core-id", default, skip_serializing_if = "Option::is_none")]
pub core_id: Option<i64>,
#[serde(rename = "die-id", default, skip_serializing_if = "Option::is_none")]
pub die_id: Option<i64>,
#[serde(rename = "drawer-id", default, skip_serializing_if = "Option::is_none")]
pub drawer_id: Option<i64>,
#[serde(rename = "module-id", default, skip_serializing_if = "Option::is_none")]
pub module_id: Option<i64>,
#[serde(rename = "node-id", default, skip_serializing_if = "Option::is_none")]
pub node_id: Option<i64>,
#[serde(rename = "socket-id", default, skip_serializing_if = "Option::is_none")]
pub socket_id: Option<i64>,
#[serde(rename = "thread-id", default, skip_serializing_if = "Option::is_none")]
pub thread_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuModelBaselineInfo {
#[serde(rename = "model")]
pub model: CpuModelInfo,
}

impl<T: Into<CpuModelInfo>> From<T> for CpuModelBaselineInfo {
    fn from(val: T) -> Self {
        Self {
            model: val.into(),

        }
    }
}
    impl AsRef<CpuModelInfo> for CpuModelBaselineInfo {
        fn as_ref(&self) -> &CpuModelInfo {
            &self.model
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuModelCompareInfo {
#[serde(rename = "responsible-properties")]
pub responsible_properties: Vec<::std::string::String>,
#[serde(rename = "result")]
pub result: CpuModelCompareResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuModelExpansionInfo {
#[serde(rename = "deprecated-props")]
pub deprecated_props: Vec<::std::string::String>,
#[serde(rename = "model")]
pub model: CpuModelInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuModelInfo {
#[serde(rename = "props", default, skip_serializing_if = "Option::is_none")]
pub props: Option<::qapi_spec::Dictionary>,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuPolarizationInfo {
#[serde(rename = "polarization")]
pub polarization: CpuS390Polarization,
}

impl<T: Into<CpuS390Polarization>> From<T> for CpuPolarizationInfo {
    fn from(val: T) -> Self {
        Self {
            polarization: val.into(),

        }
    }
}
    impl AsRef<CpuS390Polarization> for CpuPolarizationInfo {
        fn as_ref(&self) -> &CpuS390Polarization {
            &self.polarization
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CryptodevBackendProperties {
#[serde(rename = "queues", default, skip_serializing_if = "Option::is_none")]
pub queues: Option<u32>,
#[serde(rename = "throttle-bps", default, skip_serializing_if = "Option::is_none")]
pub throttle_bps: Option<u64>,
#[serde(rename = "throttle-ops", default, skip_serializing_if = "Option::is_none")]
pub throttle_ops: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptodevVhostUserProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: CryptodevBackendProperties,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentMachineParams {
#[serde(rename = "wakeup-suspend-support")]
pub wakeup_suspend_support: bool,
}

impl<T: Into<bool>> From<T> for CurrentMachineParams {
    fn from(val: T) -> Self {
        Self {
            wakeup_suspend_support: val.into(),

        }
    }
}
    impl AsRef<bool> for CurrentMachineParams {
        fn as_ref(&self) -> &bool {
            &self.wakeup_suspend_support
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CxlDynamicCapacityExtent {
#[serde(rename = "len")]
pub len: u64,
#[serde(rename = "offset")]
pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBusVMStateProperties {
#[serde(rename = "id-list", default, skip_serializing_if = "Option::is_none")]
pub id_list: Option<::std::string::String>,
#[serde(rename = "addr")]
pub addr: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirtyLimitInfo {
#[serde(rename = "cpu-index")]
pub cpu_index: i64,
#[serde(rename = "current-rate")]
pub current_rate: u64,
#[serde(rename = "limit-rate")]
pub limit_rate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirtyRateInfo {
#[serde(rename = "dirty-rate", default, skip_serializing_if = "Option::is_none")]
pub dirty_rate: Option<i64>,
#[serde(rename = "vcpu-dirty-rate", default, skip_serializing_if = "Option::is_none")]
pub vcpu_dirty_rate: Option<Vec<DirtyRateVcpu>>,
#[serde(rename = "calc-time")]
pub calc_time: i64,
#[serde(rename = "calc-time-unit")]
pub calc_time_unit: TimeUnit,
#[serde(rename = "mode")]
pub mode: DirtyRateMeasureMode,
#[serde(rename = "sample-pages")]
pub sample_pages: u64,
#[serde(rename = "start-time")]
pub start_time: i64,
#[serde(rename = "status")]
pub status: DirtyRateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirtyRateVcpu {
#[serde(rename = "dirty-rate")]
pub dirty_rate: i64,
#[serde(rename = "id")]
pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayCocoa {
#[serde(rename = "full-grab", default, skip_serializing_if = "Option::is_none")]
pub full_grab: Option<bool>,
#[serde(rename = "left-command-key", default, skip_serializing_if = "Option::is_none")]
pub left_command_key: Option<bool>,
#[serde(rename = "swap-opt-cmd", default, skip_serializing_if = "Option::is_none")]
pub swap_opt_cmd: Option<bool>,
#[serde(rename = "zoom-interpolation", default, skip_serializing_if = "Option::is_none")]
pub zoom_interpolation: Option<bool>,
#[serde(rename = "zoom-to-fit", default, skip_serializing_if = "Option::is_none")]
pub zoom_to_fit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayCurses {
#[serde(rename = "charset", default, skip_serializing_if = "Option::is_none")]
pub charset: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for DisplayCurses {
    fn from(val: T) -> Self {
        Self {
            charset: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayDBus {
#[serde(rename = "addr", default, skip_serializing_if = "Option::is_none")]
pub addr: Option<::std::string::String>,
#[serde(rename = "audiodev", default, skip_serializing_if = "Option::is_none")]
pub audiodev: Option<::std::string::String>,
#[serde(rename = "p2p", default, skip_serializing_if = "Option::is_none")]
pub p2p: Option<bool>,
#[serde(rename = "rendernode", default, skip_serializing_if = "Option::is_none")]
pub rendernode: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayEGLHeadless {
#[serde(rename = "rendernode", default, skip_serializing_if = "Option::is_none")]
pub rendernode: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for DisplayEGLHeadless {
    fn from(val: T) -> Self {
        Self {
            rendernode: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayGTK {
#[serde(rename = "grab-on-hover", default, skip_serializing_if = "Option::is_none")]
pub grab_on_hover: Option<bool>,
#[serde(rename = "show-menubar", default, skip_serializing_if = "Option::is_none")]
pub show_menubar: Option<bool>,
#[serde(rename = "show-tabs", default, skip_serializing_if = "Option::is_none")]
pub show_tabs: Option<bool>,
#[serde(rename = "zoom-to-fit", default, skip_serializing_if = "Option::is_none")]
pub zoom_to_fit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayReloadOptionsVNC {
#[serde(rename = "tls-certs", default, skip_serializing_if = "Option::is_none")]
pub tls_certs: Option<bool>,
}

impl<T: Into<bool>> From<T> for DisplayReloadOptionsVNC {
    fn from(val: T) -> Self {
        Self {
            tls_certs: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplaySDL {
#[serde(rename = "grab-mod", default, skip_serializing_if = "Option::is_none")]
pub grab_mod: Option<HotKeyMod>,
}

impl<T: Into<HotKeyMod>> From<T> for DisplaySDL {
    fn from(val: T) -> Self {
        Self {
            grab_mod: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayUpdateOptionsVNC {
#[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
pub addresses: Option<Vec<SocketAddress>>,
}

impl<T: Into<Vec<SocketAddress>>> From<T> for DisplayUpdateOptionsVNC {
    fn from(val: T) -> Self {
        Self {
            addresses: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveBackup {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BackupCommon,
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<::std::string::String>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<NewImageMode>,
#[serde(rename = "target")]
pub target: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct DriveBackupWrapper {
#[serde(rename = "data")]
pub data: DriveBackup,
}

impl<T: Into<DriveBackup>> From<T> for DriveBackupWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<DriveBackup> for DriveBackupWrapper {
        fn as_ref(&self) -> &DriveBackup {
            &self.data
        }
    }
impl ::std::ops::Deref for DriveBackupWrapper {
    type Target = DriveBackup;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DriveBackupWrapper {
    pub fn into_inner(self) -> DriveBackup {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveMirror {
#[serde(rename = "auto-dismiss", default, skip_serializing_if = "Option::is_none")]
pub auto_dismiss: Option<bool>,
#[serde(rename = "auto-finalize", default, skip_serializing_if = "Option::is_none")]
pub auto_finalize: Option<bool>,
#[serde(rename = "buf-size", default, skip_serializing_if = "Option::is_none")]
pub buf_size: Option<i64>,
#[serde(rename = "copy-mode", default, skip_serializing_if = "Option::is_none")]
pub copy_mode: Option<MirrorCopyMode>,
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<::std::string::String>,
#[serde(rename = "granularity", default, skip_serializing_if = "Option::is_none")]
pub granularity: Option<u32>,
#[serde(rename = "job-id", default, skip_serializing_if = "Option::is_none")]
pub job_id: Option<::std::string::String>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<NewImageMode>,
#[serde(rename = "node-name", default, skip_serializing_if = "Option::is_none")]
pub node_name: Option<::std::string::String>,
#[serde(rename = "on-source-error", default, skip_serializing_if = "Option::is_none")]
pub on_source_error: Option<BlockdevOnError>,
#[serde(rename = "on-target-error", default, skip_serializing_if = "Option::is_none")]
pub on_target_error: Option<BlockdevOnError>,
#[serde(rename = "replaces", default, skip_serializing_if = "Option::is_none")]
pub replaces: Option<::std::string::String>,
#[serde(rename = "speed", default, skip_serializing_if = "Option::is_none")]
pub speed: Option<i64>,
#[serde(rename = "unmap", default, skip_serializing_if = "Option::is_none")]
pub unmap: Option<bool>,
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "sync")]
pub sync: MirrorSyncMode,
#[serde(rename = "target")]
pub target: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DummyBlockCoreForceArrays {
#[serde(rename = "unused-block-graph-info")]
pub unused_block_graph_info: Vec<BlockGraphInfo>,
}

impl<T: Into<Vec<BlockGraphInfo>>> From<T> for DummyBlockCoreForceArrays {
    fn from(val: T) -> Self {
        Self {
            unused_block_graph_info: val.into(),

        }
    }
}
    impl AsRef<Vec<BlockGraphInfo>> for DummyBlockCoreForceArrays {
        fn as_ref(&self) -> &Vec<BlockGraphInfo> {
            &self.unused_block_graph_info
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DummyForceArrays {
#[serde(rename = "unused")]
pub unused: Vec<X86CPUFeatureWordInfo>,
}

impl<T: Into<Vec<X86CPUFeatureWordInfo>>> From<T> for DummyForceArrays {
    fn from(val: T) -> Self {
        Self {
            unused: val.into(),

        }
    }
}
    impl AsRef<Vec<X86CPUFeatureWordInfo>> for DummyForceArrays {
        fn as_ref(&self) -> &Vec<X86CPUFeatureWordInfo> {
            &self.unused
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DummyVirtioForceArrays {
#[serde(rename = "unused-iothread-vq-mapping")]
pub unused_iothread_vq_mapping: Vec<IOThreadVirtQueueMapping>,
}

impl<T: Into<Vec<IOThreadVirtQueueMapping>>> From<T> for DummyVirtioForceArrays {
    fn from(val: T) -> Self {
        Self {
            unused_iothread_vq_mapping: val.into(),

        }
    }
}
    impl AsRef<Vec<IOThreadVirtQueueMapping>> for DummyVirtioForceArrays {
        fn as_ref(&self) -> &Vec<IOThreadVirtQueueMapping> {
            &self.unused_iothread_vq_mapping
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumpGuestMemoryCapability {
#[serde(rename = "formats")]
pub formats: Vec<DumpGuestMemoryFormat>,
}

impl<T: Into<Vec<DumpGuestMemoryFormat>>> From<T> for DumpGuestMemoryCapability {
    fn from(val: T) -> Self {
        Self {
            formats: val.into(),

        }
    }
}
    impl AsRef<Vec<DumpGuestMemoryFormat>> for DumpGuestMemoryCapability {
        fn as_ref(&self) -> &Vec<DumpGuestMemoryFormat> {
            &self.formats
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumpQueryResult {
#[serde(rename = "completed")]
pub completed: i64,
#[serde(rename = "status")]
pub status: DumpStatus,
#[serde(rename = "total")]
pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbpfObject {
#[serde(rename = "object")]
pub object: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for EbpfObject {
    fn from(val: T) -> Self {
        Self {
            object: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for EbpfObject {
        fn as_ref(&self) -> &::std::string::String {
            &self.object
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLoopBaseProperties {
#[serde(rename = "aio-max-batch", default, skip_serializing_if = "Option::is_none")]
pub aio_max_batch: Option<i64>,
#[serde(rename = "thread-pool-max", default, skip_serializing_if = "Option::is_none")]
pub thread_pool_max: Option<i64>,
#[serde(rename = "thread-pool-min", default, skip_serializing_if = "Option::is_none")]
pub thread_pool_min: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvtchnInfo {
#[serde(rename = "masked")]
pub masked: bool,
#[serde(rename = "pending")]
pub pending: bool,
#[serde(rename = "port")]
pub port: u16,
#[serde(rename = "remote-domain")]
pub remote_domain: ::std::string::String,
#[serde(rename = "target")]
pub target: u16,
#[serde(rename = "type")]
pub type_: EvtchnPortType,
#[serde(rename = "vcpu")]
pub vcpu: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpirePasswordOptionsVnc {
#[serde(rename = "display", default, skip_serializing_if = "Option::is_none")]
pub display: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for ExpirePasswordOptionsVnc {
    fn from(val: T) -> Self {
        Self {
            display: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdSocketAddress {
#[serde(rename = "str")]
pub str: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for FdSocketAddress {
    fn from(val: T) -> Self {
        Self {
            str: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for FdSocketAddress {
        fn as_ref(&self) -> &::std::string::String {
            &self.str
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct FdSocketAddressWrapper {
#[serde(rename = "data")]
pub data: FdSocketAddress,
}

impl<T: Into<FdSocketAddress>> From<T> for FdSocketAddressWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<FdSocketAddress> for FdSocketAddressWrapper {
        fn as_ref(&self) -> &FdSocketAddress {
            &self.data
        }
    }
impl ::std::ops::Deref for FdSocketAddressWrapper {
    type Target = FdSocketAddress;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl FdSocketAddressWrapper {
    pub fn into_inner(self) -> FdSocketAddress {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdsetFdInfo {
#[serde(rename = "opaque", default, skip_serializing_if = "Option::is_none")]
pub opaque: Option<::std::string::String>,
#[serde(rename = "fd")]
pub fd: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdsetInfo {
#[serde(rename = "fds")]
pub fds: Vec<FdsetFdInfo>,
#[serde(rename = "fdset-id")]
pub fdset_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMigrationArgs {
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "offset")]
pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBufferProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: NetfilterProperties,
#[serde(rename = "interval")]
pub interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDumpProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: NetfilterProperties,
#[serde(rename = "maxlen", default, skip_serializing_if = "Option::is_none")]
pub maxlen: Option<u32>,
#[serde(rename = "file")]
pub file: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterMirrorProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: NetfilterProperties,
#[serde(rename = "vnet_hdr_support", default, skip_serializing_if = "Option::is_none")]
pub vnet_hdr_support: Option<bool>,
#[serde(rename = "outdev")]
pub outdev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRedirectorProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: NetfilterProperties,
#[serde(rename = "indev", default, skip_serializing_if = "Option::is_none")]
pub indev: Option<::std::string::String>,
#[serde(rename = "outdev", default, skip_serializing_if = "Option::is_none")]
pub outdev: Option<::std::string::String>,
#[serde(rename = "vnet_hdr_support", default, skip_serializing_if = "Option::is_none")]
pub vnet_hdr_support: Option<bool>,
}

impl<T: Into<NetfilterProperties>> From<T> for FilterRedirectorProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
indev: Default::default(),
outdev: Default::default(),
vnet_hdr_support: Default::default(),

        }
    }
}
    impl AsRef<NetfilterProperties> for FilterRedirectorProperties {
        fn as_ref(&self) -> &NetfilterProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRewriterProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: NetfilterProperties,
#[serde(rename = "vnet_hdr_support", default, skip_serializing_if = "Option::is_none")]
pub vnet_hdr_support: Option<bool>,
}

impl<T: Into<NetfilterProperties>> From<T> for FilterRewriterProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
vnet_hdr_support: Default::default(),

        }
    }
}
    impl AsRef<NetfilterProperties> for FilterRewriterProperties {
        fn as_ref(&self) -> &NetfilterProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GICCapability {
#[serde(rename = "emulated")]
pub emulated: bool,
#[serde(rename = "kernel")]
pub kernel: bool,
#[serde(rename = "version")]
pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestPanicInformationHyperV {
#[serde(rename = "arg1")]
pub arg1: u64,
#[serde(rename = "arg2")]
pub arg2: u64,
#[serde(rename = "arg3")]
pub arg3: u64,
#[serde(rename = "arg4")]
pub arg4: u64,
#[serde(rename = "arg5")]
pub arg5: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestPanicInformationS390 {
#[serde(rename = "core")]
pub core: u32,
#[serde(rename = "psw-addr")]
pub psw_addr: u64,
#[serde(rename = "psw-mask")]
pub psw_mask: u64,
#[serde(rename = "reason")]
pub reason: S390CrashReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidInfo {
#[serde(rename = "guid")]
pub guid: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for GuidInfo {
    fn from(val: T) -> Self {
        Self {
            guid: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for GuidInfo {
        fn as_ref(&self) -> &::std::string::String {
            &self.guid
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotpluggableCPU {
#[serde(rename = "qom-path", default, skip_serializing_if = "Option::is_none")]
pub qom_path: Option<::std::string::String>,
#[serde(rename = "props")]
pub props: CpuInstanceProperties,
#[serde(rename = "type")]
pub type_: ::std::string::String,
#[serde(rename = "vcpus-count")]
pub vcpus_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanReadableText {
#[serde(rename = "human-readable-text")]
pub human_readable_text: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for HumanReadableText {
    fn from(val: T) -> Self {
        Self {
            human_readable_text: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for HumanReadableText {
        fn as_ref(&self) -> &::std::string::String {
            &self.human_readable_text
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HvBalloonDeviceInfo {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "memaddr", default, skip_serializing_if = "Option::is_none")]
pub memaddr: Option<u64>,
#[serde(rename = "memdev", default, skip_serializing_if = "Option::is_none")]
pub memdev: Option<::std::string::String>,
#[serde(rename = "max-size")]
pub max_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct HvBalloonDeviceInfoWrapper {
#[serde(rename = "data")]
pub data: HvBalloonDeviceInfo,
}

impl<T: Into<HvBalloonDeviceInfo>> From<T> for HvBalloonDeviceInfoWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<HvBalloonDeviceInfo> for HvBalloonDeviceInfoWrapper {
        fn as_ref(&self) -> &HvBalloonDeviceInfo {
            &self.data
        }
    }
impl ::std::ops::Deref for HvBalloonDeviceInfoWrapper {
    type Target = HvBalloonDeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl HvBalloonDeviceInfoWrapper {
    pub fn into_inner(self) -> HvBalloonDeviceInfo {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HvBalloonInfo {
#[serde(rename = "available")]
pub available: u64,
#[serde(rename = "committed")]
pub committed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IOMMUFDProperties {
#[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
pub fd: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for IOMMUFDProperties {
    fn from(val: T) -> Self {
        Self {
            fd: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThreadInfo {
#[serde(rename = "aio-max-batch")]
pub aio_max_batch: i64,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "poll-grow")]
pub poll_grow: i64,
#[serde(rename = "poll-max-ns")]
pub poll_max_ns: i64,
#[serde(rename = "poll-shrink")]
pub poll_shrink: i64,
#[serde(rename = "thread-id")]
pub thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThreadVirtQueueMapping {
#[serde(rename = "vqs", default, skip_serializing_if = "Option::is_none")]
pub vqs: Option<Vec<u16>>,
#[serde(rename = "iothread")]
pub iothread: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCheck {
#[serde(rename = "allocated-clusters", default, skip_serializing_if = "Option::is_none")]
pub allocated_clusters: Option<i64>,
#[serde(rename = "compressed-clusters", default, skip_serializing_if = "Option::is_none")]
pub compressed_clusters: Option<i64>,
#[serde(rename = "corruptions", default, skip_serializing_if = "Option::is_none")]
pub corruptions: Option<i64>,
#[serde(rename = "corruptions-fixed", default, skip_serializing_if = "Option::is_none")]
pub corruptions_fixed: Option<i64>,
#[serde(rename = "fragmented-clusters", default, skip_serializing_if = "Option::is_none")]
pub fragmented_clusters: Option<i64>,
#[serde(rename = "image-end-offset", default, skip_serializing_if = "Option::is_none")]
pub image_end_offset: Option<i64>,
#[serde(rename = "leaks", default, skip_serializing_if = "Option::is_none")]
pub leaks: Option<i64>,
#[serde(rename = "leaks-fixed", default, skip_serializing_if = "Option::is_none")]
pub leaks_fixed: Option<i64>,
#[serde(rename = "total-clusters", default, skip_serializing_if = "Option::is_none")]
pub total_clusters: Option<i64>,
#[serde(rename = "check-errors")]
pub check_errors: i64,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "format")]
pub format: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockNodeInfo,
#[serde(rename = "backing-image", default, skip_serializing_if = "Option::is_none")]
pub backing_image: Option<Box<ImageInfo>>,
}

impl<T: Into<BlockNodeInfo>> From<T> for ImageInfo {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
backing_image: Default::default(),

        }
    }
}
    impl AsRef<BlockNodeInfo> for ImageInfo {
        fn as_ref(&self) -> &BlockNodeInfo {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageInfoSpecificFile {
#[serde(rename = "extent-size-hint", default, skip_serializing_if = "Option::is_none")]
pub extent_size_hint: Option<u64>,
}

impl<T: Into<u64>> From<T> for ImageInfoSpecificFile {
    fn from(val: T) -> Self {
        Self {
            extent_size_hint: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ImageInfoSpecificFileWrapper {
#[serde(rename = "data")]
pub data: ImageInfoSpecificFile,
}

impl<T: Into<ImageInfoSpecificFile>> From<T> for ImageInfoSpecificFileWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ImageInfoSpecificFile> for ImageInfoSpecificFileWrapper {
        fn as_ref(&self) -> &ImageInfoSpecificFile {
            &self.data
        }
    }
impl ::std::ops::Deref for ImageInfoSpecificFileWrapper {
    type Target = ImageInfoSpecificFile;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ImageInfoSpecificFileWrapper {
    pub fn into_inner(self) -> ImageInfoSpecificFile {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ImageInfoSpecificLUKSWrapper {
#[serde(rename = "data")]
pub data: QCryptoBlockInfoLUKS,
}

impl<T: Into<QCryptoBlockInfoLUKS>> From<T> for ImageInfoSpecificLUKSWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<QCryptoBlockInfoLUKS> for ImageInfoSpecificLUKSWrapper {
        fn as_ref(&self) -> &QCryptoBlockInfoLUKS {
            &self.data
        }
    }
impl ::std::ops::Deref for ImageInfoSpecificLUKSWrapper {
    type Target = QCryptoBlockInfoLUKS;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ImageInfoSpecificLUKSWrapper {
    pub fn into_inner(self) -> QCryptoBlockInfoLUKS {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfoSpecificQCow2 {
#[serde(rename = "bitmaps", default, skip_serializing_if = "Option::is_none")]
pub bitmaps: Option<Vec<Qcow2BitmapInfo>>,
#[serde(rename = "corrupt", default, skip_serializing_if = "Option::is_none")]
pub corrupt: Option<bool>,
#[serde(rename = "data-file", default, skip_serializing_if = "Option::is_none")]
pub data_file: Option<::std::string::String>,
#[serde(rename = "data-file-raw", default, skip_serializing_if = "Option::is_none")]
pub data_file_raw: Option<bool>,
#[serde(rename = "encrypt", default, skip_serializing_if = "Option::is_none")]
pub encrypt: Option<ImageInfoSpecificQCow2Encryption>,
#[serde(rename = "extended-l2", default, skip_serializing_if = "Option::is_none")]
pub extended_l2: Option<bool>,
#[serde(rename = "lazy-refcounts", default, skip_serializing_if = "Option::is_none")]
pub lazy_refcounts: Option<bool>,
#[serde(rename = "compat")]
pub compat: ::std::string::String,
#[serde(rename = "compression-type")]
pub compression_type: Qcow2CompressionType,
#[serde(rename = "refcount-bits")]
pub refcount_bits: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageInfoSpecificQCow2EncryptionBase {
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ImageInfoSpecificQCow2Wrapper {
#[serde(rename = "data")]
pub data: ImageInfoSpecificQCow2,
}

impl<T: Into<ImageInfoSpecificQCow2>> From<T> for ImageInfoSpecificQCow2Wrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ImageInfoSpecificQCow2> for ImageInfoSpecificQCow2Wrapper {
        fn as_ref(&self) -> &ImageInfoSpecificQCow2 {
            &self.data
        }
    }
impl ::std::ops::Deref for ImageInfoSpecificQCow2Wrapper {
    type Target = ImageInfoSpecificQCow2;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ImageInfoSpecificQCow2Wrapper {
    pub fn into_inner(self) -> ImageInfoSpecificQCow2 {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageInfoSpecificRbd {
#[serde(rename = "encryption-format", default, skip_serializing_if = "Option::is_none")]
pub encryption_format: Option<RbdImageEncryptionFormat>,
}

impl<T: Into<RbdImageEncryptionFormat>> From<T> for ImageInfoSpecificRbd {
    fn from(val: T) -> Self {
        Self {
            encryption_format: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ImageInfoSpecificRbdWrapper {
#[serde(rename = "data")]
pub data: ImageInfoSpecificRbd,
}

impl<T: Into<ImageInfoSpecificRbd>> From<T> for ImageInfoSpecificRbdWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ImageInfoSpecificRbd> for ImageInfoSpecificRbdWrapper {
        fn as_ref(&self) -> &ImageInfoSpecificRbd {
            &self.data
        }
    }
impl ::std::ops::Deref for ImageInfoSpecificRbdWrapper {
    type Target = ImageInfoSpecificRbd;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ImageInfoSpecificRbdWrapper {
    pub fn into_inner(self) -> ImageInfoSpecificRbd {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfoSpecificVmdk {
#[serde(rename = "cid")]
pub cid: i64,
#[serde(rename = "create-type")]
pub create_type: ::std::string::String,
#[serde(rename = "extents")]
pub extents: Vec<VmdkExtentInfo>,
#[serde(rename = "parent-cid")]
pub parent_cid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct ImageInfoSpecificVmdkWrapper {
#[serde(rename = "data")]
pub data: ImageInfoSpecificVmdk,
}

impl<T: Into<ImageInfoSpecificVmdk>> From<T> for ImageInfoSpecificVmdkWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<ImageInfoSpecificVmdk> for ImageInfoSpecificVmdkWrapper {
        fn as_ref(&self) -> &ImageInfoSpecificVmdk {
            &self.data
        }
    }
impl ::std::ops::Deref for ImageInfoSpecificVmdkWrapper {
    type Target = ImageInfoSpecificVmdk;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl ImageInfoSpecificVmdkWrapper {
    pub fn into_inner(self) -> ImageInfoSpecificVmdk {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InetSocketAddress {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: InetSocketAddressBase,
#[serde(rename = "ipv4", default, skip_serializing_if = "Option::is_none")]
pub ipv4: Option<bool>,
#[serde(rename = "ipv6", default, skip_serializing_if = "Option::is_none")]
pub ipv6: Option<bool>,
#[serde(rename = "keep-alive", default, skip_serializing_if = "Option::is_none")]
pub keep_alive: Option<bool>,
#[serde(rename = "mptcp", default, skip_serializing_if = "Option::is_none")]
pub mptcp: Option<bool>,
#[serde(rename = "numeric", default, skip_serializing_if = "Option::is_none")]
pub numeric: Option<bool>,
#[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
pub to: Option<u16>,
}

impl<T: Into<InetSocketAddressBase>> From<T> for InetSocketAddress {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
ipv4: Default::default(),
ipv6: Default::default(),
keep_alive: Default::default(),
mptcp: Default::default(),
numeric: Default::default(),
to: Default::default(),

        }
    }
}
    impl AsRef<InetSocketAddressBase> for InetSocketAddress {
        fn as_ref(&self) -> &InetSocketAddressBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InetSocketAddressBase {
#[serde(rename = "host")]
pub host: ::std::string::String,
#[serde(rename = "port")]
pub port: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct InetSocketAddressWrapper {
#[serde(rename = "data")]
pub data: InetSocketAddress,
}

impl<T: Into<InetSocketAddress>> From<T> for InetSocketAddressWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<InetSocketAddress> for InetSocketAddressWrapper {
        fn as_ref(&self) -> &InetSocketAddress {
            &self.data
        }
    }
impl ::std::ops::Deref for InetSocketAddressWrapper {
    type Target = InetSocketAddress;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl InetSocketAddressWrapper {
    pub fn into_inner(self) -> InetSocketAddress {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBarrierProperties {
#[serde(rename = "height", default, skip_serializing_if = "Option::is_none")]
pub height: Option<::std::string::String>,
#[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
pub port: Option<::std::string::String>,
#[serde(rename = "server", default, skip_serializing_if = "Option::is_none")]
pub server: Option<::std::string::String>,
#[serde(rename = "width", default, skip_serializing_if = "Option::is_none")]
pub width: Option<::std::string::String>,
#[serde(rename = "x-origin", default, skip_serializing_if = "Option::is_none")]
pub x_origin: Option<::std::string::String>,
#[serde(rename = "y-origin", default, skip_serializing_if = "Option::is_none")]
pub y_origin: Option<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBtnEvent {
#[serde(rename = "button")]
pub button: InputButton,
#[serde(rename = "down")]
pub down: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct InputBtnEventWrapper {
#[serde(rename = "data")]
pub data: InputBtnEvent,
}

impl<T: Into<InputBtnEvent>> From<T> for InputBtnEventWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<InputBtnEvent> for InputBtnEventWrapper {
        fn as_ref(&self) -> &InputBtnEvent {
            &self.data
        }
    }
impl ::std::ops::Deref for InputBtnEventWrapper {
    type Target = InputBtnEvent;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl InputBtnEventWrapper {
    pub fn into_inner(self) -> InputBtnEvent {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputKeyEvent {
#[serde(rename = "down")]
pub down: bool,
#[serde(rename = "key")]
pub key: KeyValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct InputKeyEventWrapper {
#[serde(rename = "data")]
pub data: InputKeyEvent,
}

impl<T: Into<InputKeyEvent>> From<T> for InputKeyEventWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<InputKeyEvent> for InputKeyEventWrapper {
        fn as_ref(&self) -> &InputKeyEvent {
            &self.data
        }
    }
impl ::std::ops::Deref for InputKeyEventWrapper {
    type Target = InputKeyEvent;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl InputKeyEventWrapper {
    pub fn into_inner(self) -> InputKeyEvent {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLinuxProperties {
#[serde(rename = "grab-toggle", default, skip_serializing_if = "Option::is_none")]
pub grab_toggle: Option<GrabToggleKeys>,
#[serde(rename = "grab_all", default, skip_serializing_if = "Option::is_none")]
pub grab_all: Option<bool>,
#[serde(rename = "repeat", default, skip_serializing_if = "Option::is_none")]
pub repeat: Option<bool>,
#[serde(rename = "evdev")]
pub evdev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMoveEvent {
#[serde(rename = "axis")]
pub axis: InputAxis,
#[serde(rename = "value")]
pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct InputMoveEventWrapper {
#[serde(rename = "data")]
pub data: InputMoveEvent,
}

impl<T: Into<InputMoveEvent>> From<T> for InputMoveEventWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<InputMoveEvent> for InputMoveEventWrapper {
        fn as_ref(&self) -> &InputMoveEvent {
            &self.data
        }
    }
impl ::std::ops::Deref for InputMoveEventWrapper {
    type Target = InputMoveEvent;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl InputMoveEventWrapper {
    pub fn into_inner(self) -> InputMoveEvent {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMultiTouchEvent {
#[serde(rename = "axis")]
pub axis: InputAxis,
#[serde(rename = "slot")]
pub slot: i64,
#[serde(rename = "tracking-id")]
pub tracking_id: i64,
#[serde(rename = "type")]
pub type_: InputMultiTouchType,
#[serde(rename = "value")]
pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct InputMultiTouchEventWrapper {
#[serde(rename = "data")]
pub data: InputMultiTouchEvent,
}

impl<T: Into<InputMultiTouchEvent>> From<T> for InputMultiTouchEventWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<InputMultiTouchEvent> for InputMultiTouchEventWrapper {
        fn as_ref(&self) -> &InputMultiTouchEvent {
            &self.data
        }
    }
impl ::std::ops::Deref for InputMultiTouchEventWrapper {
    type Target = InputMultiTouchEvent;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl InputMultiTouchEventWrapper {
    pub fn into_inner(self) -> InputMultiTouchEvent {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct IntWrapper {
#[serde(rename = "data")]
pub data: i64,
}

impl<T: Into<i64>> From<T> for IntWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<i64> for IntWrapper {
        fn as_ref(&self) -> &i64 {
            &self.data
        }
    }
impl ::std::ops::Deref for IntWrapper {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl IntWrapper {
    pub fn into_inner(self) -> i64 {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IothreadProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: EventLoopBaseProperties,
#[serde(rename = "poll-grow", default, skip_serializing_if = "Option::is_none")]
pub poll_grow: Option<i64>,
#[serde(rename = "poll-max-ns", default, skip_serializing_if = "Option::is_none")]
pub poll_max_ns: Option<i64>,
#[serde(rename = "poll-shrink", default, skip_serializing_if = "Option::is_none")]
pub poll_shrink: Option<i64>,
}

impl<T: Into<EventLoopBaseProperties>> From<T> for IothreadProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
poll_grow: Default::default(),
poll_max_ns: Default::default(),
poll_shrink: Default::default(),

        }
    }
}
    impl AsRef<EventLoopBaseProperties> for IothreadProperties {
        fn as_ref(&self) -> &EventLoopBaseProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
#[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
pub error: Option<::std::string::String>,
#[serde(rename = "current-progress")]
pub current_progress: i64,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "status")]
pub status: JobStatus,
#[serde(rename = "total-progress")]
pub total_progress: i64,
#[serde(rename = "type")]
pub type_: JobType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvmInfo {
#[serde(rename = "enabled")]
pub enabled: bool,
#[serde(rename = "present")]
pub present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
#[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
pub alias: Option<::std::string::String>,
#[serde(rename = "compat-props", default, skip_serializing_if = "Option::is_none")]
pub compat_props: Option<Vec<CompatProperty>>,
#[serde(rename = "default-cpu-type", default, skip_serializing_if = "Option::is_none")]
pub default_cpu_type: Option<::std::string::String>,
#[serde(rename = "default-ram-id", default, skip_serializing_if = "Option::is_none")]
pub default_ram_id: Option<::std::string::String>,
#[serde(rename = "is-default", default, skip_serializing_if = "Option::is_none")]
pub is_default: Option<bool>,
#[serde(rename = "acpi")]
pub acpi: bool,
#[serde(rename = "cpu-max")]
pub cpu_max: i64,
#[serde(rename = "deprecated")]
pub deprecated: bool,
#[serde(rename = "hotpluggable-cpus")]
pub hotpluggable_cpus: bool,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "numa-mem-supported")]
pub numa_mem_supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainLoopProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: EventLoopBaseProperties,
}

impl<T: Into<EventLoopBaseProperties>> From<T> for MainLoopProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<EventLoopBaseProperties> for MainLoopProperties {
        fn as_ref(&self) -> &EventLoopBaseProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEntry {
#[serde(rename = "filename", default, skip_serializing_if = "Option::is_none")]
pub filename: Option<::std::string::String>,
#[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
pub offset: Option<i64>,
#[serde(rename = "compressed")]
pub compressed: bool,
#[serde(rename = "data")]
pub data: bool,
#[serde(rename = "depth")]
pub depth: i64,
#[serde(rename = "length")]
pub length: i64,
#[serde(rename = "present")]
pub present: bool,
#[serde(rename = "start")]
pub start: i64,
#[serde(rename = "zero")]
pub zero: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memdev {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "reserve", default, skip_serializing_if = "Option::is_none")]
pub reserve: Option<bool>,
#[serde(rename = "dump")]
pub dump: bool,
#[serde(rename = "host-nodes")]
pub host_nodes: Vec<u16>,
#[serde(rename = "merge")]
pub merge: bool,
#[serde(rename = "policy")]
pub policy: HostMemPolicy,
#[serde(rename = "prealloc")]
pub prealloc: bool,
#[serde(rename = "share")]
pub share: bool,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendEpcProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: MemoryBackendProperties,
}

impl<T: Into<MemoryBackendProperties>> From<T> for MemoryBackendEpcProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<MemoryBackendProperties> for MemoryBackendEpcProperties {
        fn as_ref(&self) -> &MemoryBackendProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendFileProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: MemoryBackendProperties,
#[serde(rename = "align", default, skip_serializing_if = "Option::is_none")]
pub align: Option<u64>,
#[serde(rename = "discard-data", default, skip_serializing_if = "Option::is_none")]
pub discard_data: Option<bool>,
#[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
pub offset: Option<u64>,
#[serde(rename = "pmem", default, skip_serializing_if = "Option::is_none")]
pub pmem: Option<bool>,
#[serde(rename = "readonly", default, skip_serializing_if = "Option::is_none")]
pub readonly: Option<bool>,
#[serde(rename = "rom", default, skip_serializing_if = "Option::is_none")]
pub rom: Option<OnOffAuto>,
#[serde(rename = "mem-path")]
pub mem_path: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendMemfdProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: MemoryBackendProperties,
#[serde(rename = "hugetlb", default, skip_serializing_if = "Option::is_none")]
pub hugetlb: Option<bool>,
#[serde(rename = "hugetlbsize", default, skip_serializing_if = "Option::is_none")]
pub hugetlbsize: Option<u64>,
#[serde(rename = "seal", default, skip_serializing_if = "Option::is_none")]
pub seal: Option<bool>,
}

impl<T: Into<MemoryBackendProperties>> From<T> for MemoryBackendMemfdProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
hugetlb: Default::default(),
hugetlbsize: Default::default(),
seal: Default::default(),

        }
    }
}
    impl AsRef<MemoryBackendProperties> for MemoryBackendMemfdProperties {
        fn as_ref(&self) -> &MemoryBackendProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendProperties {
#[serde(rename = "dump", default, skip_serializing_if = "Option::is_none")]
pub dump: Option<bool>,
#[serde(rename = "host-nodes", default, skip_serializing_if = "Option::is_none")]
pub host_nodes: Option<Vec<u16>>,
#[serde(rename = "merge", default, skip_serializing_if = "Option::is_none")]
pub merge: Option<bool>,
#[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
pub policy: Option<HostMemPolicy>,
#[serde(rename = "prealloc", default, skip_serializing_if = "Option::is_none")]
pub prealloc: Option<bool>,
#[serde(rename = "prealloc-context", default, skip_serializing_if = "Option::is_none")]
pub prealloc_context: Option<::std::string::String>,
#[serde(rename = "prealloc-threads", default, skip_serializing_if = "Option::is_none")]
pub prealloc_threads: Option<u32>,
#[serde(rename = "reserve", default, skip_serializing_if = "Option::is_none")]
pub reserve: Option<bool>,
#[serde(rename = "share", default, skip_serializing_if = "Option::is_none")]
pub share: Option<bool>,
#[serde(rename = "x-use-canonical-path-for-ramblock-id", default, skip_serializing_if = "Option::is_none")]
pub x_use_canonical_path_for_ramblock_id: Option<bool>,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendShmProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: MemoryBackendProperties,
}

impl<T: Into<MemoryBackendProperties>> From<T> for MemoryBackendShmProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<MemoryBackendProperties> for MemoryBackendShmProperties {
        fn as_ref(&self) -> &MemoryBackendProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFailureFlags {
#[serde(rename = "action-required")]
pub action_required: bool,
#[serde(rename = "recursive")]
pub recursive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
#[serde(rename = "plugged-memory", default, skip_serializing_if = "Option::is_none")]
pub plugged_memory: Option<u64>,
#[serde(rename = "base-memory")]
pub base_memory: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemorySizeConfiguration {
#[serde(rename = "max-size", default, skip_serializing_if = "Option::is_none")]
pub max_size: Option<u64>,
#[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
pub size: Option<u64>,
#[serde(rename = "slots", default, skip_serializing_if = "Option::is_none")]
pub slots: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrateSetParameters {
#[serde(rename = "announce-initial", default, skip_serializing_if = "Option::is_none")]
pub announce_initial: Option<u64>,
#[serde(rename = "announce-max", default, skip_serializing_if = "Option::is_none")]
pub announce_max: Option<u64>,
#[serde(rename = "announce-rounds", default, skip_serializing_if = "Option::is_none")]
pub announce_rounds: Option<u64>,
#[serde(rename = "announce-step", default, skip_serializing_if = "Option::is_none")]
pub announce_step: Option<u64>,
#[serde(rename = "avail-switchover-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub avail_switchover_bandwidth: Option<u64>,
#[serde(rename = "block-bitmap-mapping", default, skip_serializing_if = "Option::is_none")]
pub block_bitmap_mapping: Option<Vec<BitmapMigrationNodeAlias>>,
#[serde(rename = "cpu-throttle-increment", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_increment: Option<u8>,
#[serde(rename = "cpu-throttle-initial", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_initial: Option<u8>,
#[serde(rename = "cpu-throttle-tailslow", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_tailslow: Option<bool>,
#[serde(rename = "direct-io", default, skip_serializing_if = "Option::is_none")]
pub direct_io: Option<bool>,
#[serde(rename = "downtime-limit", default, skip_serializing_if = "Option::is_none")]
pub downtime_limit: Option<u64>,
#[serde(rename = "max-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub max_bandwidth: Option<u64>,
#[serde(rename = "max-cpu-throttle", default, skip_serializing_if = "Option::is_none")]
pub max_cpu_throttle: Option<u8>,
#[serde(rename = "max-postcopy-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub max_postcopy_bandwidth: Option<u64>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<MigMode>,
#[serde(rename = "multifd-channels", default, skip_serializing_if = "Option::is_none")]
pub multifd_channels: Option<u8>,
#[serde(rename = "multifd-compression", default, skip_serializing_if = "Option::is_none")]
pub multifd_compression: Option<MultiFDCompression>,
#[serde(rename = "multifd-zlib-level", default, skip_serializing_if = "Option::is_none")]
pub multifd_zlib_level: Option<u8>,
#[serde(rename = "multifd-zstd-level", default, skip_serializing_if = "Option::is_none")]
pub multifd_zstd_level: Option<u8>,
#[serde(rename = "throttle-trigger-threshold", default, skip_serializing_if = "Option::is_none")]
pub throttle_trigger_threshold: Option<u8>,
#[serde(rename = "tls-authz", default, skip_serializing_if = "Option::is_none")]
pub tls_authz: Option<StrOrNull>,
#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<StrOrNull>,
#[serde(rename = "tls-hostname", default, skip_serializing_if = "Option::is_none")]
pub tls_hostname: Option<StrOrNull>,
#[serde(rename = "vcpu-dirty-limit", default, skip_serializing_if = "Option::is_none")]
pub vcpu_dirty_limit: Option<u64>,
#[serde(rename = "x-checkpoint-delay", default, skip_serializing_if = "Option::is_none")]
pub x_checkpoint_delay: Option<u32>,
#[serde(rename = "x-vcpu-dirty-limit-period", default, skip_serializing_if = "Option::is_none")]
pub x_vcpu_dirty_limit_period: Option<u64>,
#[serde(rename = "xbzrle-cache-size", default, skip_serializing_if = "Option::is_none")]
pub xbzrle_cache_size: Option<u64>,
#[serde(rename = "zero-page-detection", default, skip_serializing_if = "Option::is_none")]
pub zero_page_detection: Option<ZeroPageDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationCapabilityStatus {
#[serde(rename = "capability")]
pub capability: MigrationCapability,
#[serde(rename = "state")]
pub state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationChannel {
#[serde(rename = "addr")]
pub addr: MigrationAddress,
#[serde(rename = "channel-type")]
pub channel_type: MigrationChannelType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationExecCommand {
#[serde(rename = "args")]
pub args: Vec<::std::string::String>,
}

impl<T: Into<Vec<::std::string::String>>> From<T> for MigrationExecCommand {
    fn from(val: T) -> Self {
        Self {
            args: val.into(),

        }
    }
}
    impl AsRef<Vec<::std::string::String>> for MigrationExecCommand {
        fn as_ref(&self) -> &Vec<::std::string::String> {
            &self.args
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationInfo {
#[serde(rename = "blocked-reasons", default, skip_serializing_if = "Option::is_none")]
pub blocked_reasons: Option<Vec<::std::string::String>>,
#[serde(rename = "cpu-throttle-percentage", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_percentage: Option<i64>,
#[serde(rename = "dirty-limit-ring-full-time", default, skip_serializing_if = "Option::is_none")]
pub dirty_limit_ring_full_time: Option<u64>,
#[serde(rename = "dirty-limit-throttle-time-per-round", default, skip_serializing_if = "Option::is_none")]
pub dirty_limit_throttle_time_per_round: Option<u64>,
#[serde(rename = "downtime", default, skip_serializing_if = "Option::is_none")]
pub downtime: Option<i64>,
#[serde(rename = "error-desc", default, skip_serializing_if = "Option::is_none")]
pub error_desc: Option<::std::string::String>,
#[serde(rename = "expected-downtime", default, skip_serializing_if = "Option::is_none")]
pub expected_downtime: Option<i64>,
#[serde(rename = "postcopy-blocktime", default, skip_serializing_if = "Option::is_none")]
pub postcopy_blocktime: Option<u32>,
#[serde(rename = "postcopy-vcpu-blocktime", default, skip_serializing_if = "Option::is_none")]
pub postcopy_vcpu_blocktime: Option<Vec<u32>>,
#[serde(rename = "ram", default, skip_serializing_if = "Option::is_none")]
pub ram: Option<MigrationStats>,
#[serde(rename = "setup-time", default, skip_serializing_if = "Option::is_none")]
pub setup_time: Option<i64>,
#[serde(rename = "socket-address", default, skip_serializing_if = "Option::is_none")]
pub socket_address: Option<Vec<SocketAddress>>,
#[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
pub status: Option<MigrationStatus>,
#[serde(rename = "total-time", default, skip_serializing_if = "Option::is_none")]
pub total_time: Option<i64>,
#[serde(rename = "vfio", default, skip_serializing_if = "Option::is_none")]
pub vfio: Option<VfioStats>,
#[serde(rename = "xbzrle-cache", default, skip_serializing_if = "Option::is_none")]
pub xbzrle_cache: Option<XBZRLECacheStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationParameters {
#[serde(rename = "announce-initial", default, skip_serializing_if = "Option::is_none")]
pub announce_initial: Option<u64>,
#[serde(rename = "announce-max", default, skip_serializing_if = "Option::is_none")]
pub announce_max: Option<u64>,
#[serde(rename = "announce-rounds", default, skip_serializing_if = "Option::is_none")]
pub announce_rounds: Option<u64>,
#[serde(rename = "announce-step", default, skip_serializing_if = "Option::is_none")]
pub announce_step: Option<u64>,
#[serde(rename = "avail-switchover-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub avail_switchover_bandwidth: Option<u64>,
#[serde(rename = "block-bitmap-mapping", default, skip_serializing_if = "Option::is_none")]
pub block_bitmap_mapping: Option<Vec<BitmapMigrationNodeAlias>>,
#[serde(rename = "cpu-throttle-increment", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_increment: Option<u8>,
#[serde(rename = "cpu-throttle-initial", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_initial: Option<u8>,
#[serde(rename = "cpu-throttle-tailslow", default, skip_serializing_if = "Option::is_none")]
pub cpu_throttle_tailslow: Option<bool>,
#[serde(rename = "direct-io", default, skip_serializing_if = "Option::is_none")]
pub direct_io: Option<bool>,
#[serde(rename = "downtime-limit", default, skip_serializing_if = "Option::is_none")]
pub downtime_limit: Option<u64>,
#[serde(rename = "max-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub max_bandwidth: Option<u64>,
#[serde(rename = "max-cpu-throttle", default, skip_serializing_if = "Option::is_none")]
pub max_cpu_throttle: Option<u8>,
#[serde(rename = "max-postcopy-bandwidth", default, skip_serializing_if = "Option::is_none")]
pub max_postcopy_bandwidth: Option<u64>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<MigMode>,
#[serde(rename = "multifd-channels", default, skip_serializing_if = "Option::is_none")]
pub multifd_channels: Option<u8>,
#[serde(rename = "multifd-compression", default, skip_serializing_if = "Option::is_none")]
pub multifd_compression: Option<MultiFDCompression>,
#[serde(rename = "multifd-zlib-level", default, skip_serializing_if = "Option::is_none")]
pub multifd_zlib_level: Option<u8>,
#[serde(rename = "multifd-zstd-level", default, skip_serializing_if = "Option::is_none")]
pub multifd_zstd_level: Option<u8>,
#[serde(rename = "throttle-trigger-threshold", default, skip_serializing_if = "Option::is_none")]
pub throttle_trigger_threshold: Option<u8>,
#[serde(rename = "tls-authz", default, skip_serializing_if = "Option::is_none")]
pub tls_authz: Option<::std::string::String>,
#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<::std::string::String>,
#[serde(rename = "tls-hostname", default, skip_serializing_if = "Option::is_none")]
pub tls_hostname: Option<::std::string::String>,
#[serde(rename = "vcpu-dirty-limit", default, skip_serializing_if = "Option::is_none")]
pub vcpu_dirty_limit: Option<u64>,
#[serde(rename = "x-checkpoint-delay", default, skip_serializing_if = "Option::is_none")]
pub x_checkpoint_delay: Option<u32>,
#[serde(rename = "x-vcpu-dirty-limit-period", default, skip_serializing_if = "Option::is_none")]
pub x_vcpu_dirty_limit_period: Option<u64>,
#[serde(rename = "xbzrle-cache-size", default, skip_serializing_if = "Option::is_none")]
pub xbzrle_cache_size: Option<u64>,
#[serde(rename = "zero-page-detection", default, skip_serializing_if = "Option::is_none")]
pub zero_page_detection: Option<ZeroPageDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStats {
#[serde(rename = "dirty-pages-rate")]
pub dirty_pages_rate: i64,
#[serde(rename = "dirty-sync-count")]
pub dirty_sync_count: i64,
#[serde(rename = "dirty-sync-missed-zero-copy")]
pub dirty_sync_missed_zero_copy: u64,
#[serde(rename = "downtime-bytes")]
pub downtime_bytes: u64,
#[serde(rename = "duplicate")]
pub duplicate: i64,
#[serde(rename = "mbps")]
pub mbps: f64,
#[serde(rename = "multifd-bytes")]
pub multifd_bytes: u64,
#[serde(rename = "normal")]
pub normal: i64,
#[serde(rename = "normal-bytes")]
pub normal_bytes: i64,
#[serde(rename = "page-size")]
pub page_size: i64,
#[serde(rename = "pages-per-second")]
pub pages_per_second: u64,
#[serde(rename = "postcopy-bytes")]
pub postcopy_bytes: u64,
#[serde(rename = "postcopy-requests")]
pub postcopy_requests: i64,
#[serde(rename = "precopy-bytes")]
pub precopy_bytes: u64,
#[serde(rename = "remaining")]
pub remaining: i64,
#[serde(rename = "total")]
pub total: i64,
#[serde(rename = "transferred")]
pub transferred: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationThreadInfo {
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "thread-id")]
pub thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorOptions {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<MonitorMode>,
#[serde(rename = "pretty", default, skip_serializing_if = "Option::is_none")]
pub pretty: Option<bool>,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseInfo {
#[serde(rename = "absolute")]
pub absolute: bool,
#[serde(rename = "current")]
pub current: bool,
#[serde(rename = "index")]
pub index: i64,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFSServer {
#[serde(rename = "host")]
pub host: ::std::string::String,
#[serde(rename = "type")]
pub type_: NFSTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NameInfo {
#[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
pub name: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for NameInfo {
    fn from(val: T) -> Self {
        Self {
            name: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NbdServerAddOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: BlockExportOptionsNbdBase,
#[serde(rename = "bitmap", default, skip_serializing_if = "Option::is_none")]
pub bitmap: Option<::std::string::String>,
#[serde(rename = "writable", default, skip_serializing_if = "Option::is_none")]
pub writable: Option<bool>,
#[serde(rename = "device")]
pub device: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NbdServerOptions {
#[serde(rename = "max-connections", default, skip_serializing_if = "Option::is_none")]
pub max_connections: Option<u32>,
#[serde(rename = "tls-authz", default, skip_serializing_if = "Option::is_none")]
pub tls_authz: Option<::std::string::String>,
#[serde(rename = "tls-creds", default, skip_serializing_if = "Option::is_none")]
pub tls_creds: Option<::std::string::String>,
#[serde(rename = "addr")]
pub addr: SocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetLegacyNicOptions {
#[serde(rename = "addr", default, skip_serializing_if = "Option::is_none")]
pub addr: Option<::std::string::String>,
#[serde(rename = "macaddr", default, skip_serializing_if = "Option::is_none")]
pub macaddr: Option<::std::string::String>,
#[serde(rename = "model", default, skip_serializing_if = "Option::is_none")]
pub model: Option<::std::string::String>,
#[serde(rename = "netdev", default, skip_serializing_if = "Option::is_none")]
pub netdev: Option<::std::string::String>,
#[serde(rename = "vectors", default, skip_serializing_if = "Option::is_none")]
pub vectors: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevAFXDPOptions {
#[serde(rename = "force-copy", default, skip_serializing_if = "Option::is_none")]
pub force_copy: Option<bool>,
#[serde(rename = "inhibit", default, skip_serializing_if = "Option::is_none")]
pub inhibit: Option<bool>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<AFXDPMode>,
#[serde(rename = "queues", default, skip_serializing_if = "Option::is_none")]
pub queues: Option<i64>,
#[serde(rename = "sock-fds", default, skip_serializing_if = "Option::is_none")]
pub sock_fds: Option<::std::string::String>,
#[serde(rename = "start-queue", default, skip_serializing_if = "Option::is_none")]
pub start_queue: Option<i64>,
#[serde(rename = "ifname")]
pub ifname: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevBridgeOptions {
#[serde(rename = "br", default, skip_serializing_if = "Option::is_none")]
pub br: Option<::std::string::String>,
#[serde(rename = "helper", default, skip_serializing_if = "Option::is_none")]
pub helper: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevDgramOptions {
#[serde(rename = "local", default, skip_serializing_if = "Option::is_none")]
pub local: Option<SocketAddress>,
#[serde(rename = "remote", default, skip_serializing_if = "Option::is_none")]
pub remote: Option<SocketAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevHubPortOptions {
#[serde(rename = "netdev", default, skip_serializing_if = "Option::is_none")]
pub netdev: Option<::std::string::String>,
#[serde(rename = "hubid")]
pub hubid: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevL2TPv3Options {
#[serde(rename = "cookie64", default, skip_serializing_if = "Option::is_none")]
pub cookie64: Option<bool>,
#[serde(rename = "counter", default, skip_serializing_if = "Option::is_none")]
pub counter: Option<bool>,
#[serde(rename = "dstport", default, skip_serializing_if = "Option::is_none")]
pub dstport: Option<::std::string::String>,
#[serde(rename = "ipv6", default, skip_serializing_if = "Option::is_none")]
pub ipv6: Option<bool>,
#[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
pub offset: Option<u32>,
#[serde(rename = "pincounter", default, skip_serializing_if = "Option::is_none")]
pub pincounter: Option<bool>,
#[serde(rename = "rxcookie", default, skip_serializing_if = "Option::is_none")]
pub rxcookie: Option<u64>,
#[serde(rename = "rxsession", default, skip_serializing_if = "Option::is_none")]
pub rxsession: Option<u32>,
#[serde(rename = "srcport", default, skip_serializing_if = "Option::is_none")]
pub srcport: Option<::std::string::String>,
#[serde(rename = "txcookie", default, skip_serializing_if = "Option::is_none")]
pub txcookie: Option<u64>,
#[serde(rename = "udp", default, skip_serializing_if = "Option::is_none")]
pub udp: Option<bool>,
#[serde(rename = "dst")]
pub dst: ::std::string::String,
#[serde(rename = "src")]
pub src: ::std::string::String,
#[serde(rename = "txsession")]
pub txsession: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevNetmapOptions {
#[serde(rename = "devname", default, skip_serializing_if = "Option::is_none")]
pub devname: Option<::std::string::String>,
#[serde(rename = "ifname")]
pub ifname: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevSocketOptions {
#[serde(rename = "connect", default, skip_serializing_if = "Option::is_none")]
pub connect: Option<::std::string::String>,
#[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
pub fd: Option<::std::string::String>,
#[serde(rename = "listen", default, skip_serializing_if = "Option::is_none")]
pub listen: Option<::std::string::String>,
#[serde(rename = "localaddr", default, skip_serializing_if = "Option::is_none")]
pub localaddr: Option<::std::string::String>,
#[serde(rename = "mcast", default, skip_serializing_if = "Option::is_none")]
pub mcast: Option<::std::string::String>,
#[serde(rename = "udp", default, skip_serializing_if = "Option::is_none")]
pub udp: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevStreamOptions {
#[serde(rename = "reconnect", default, skip_serializing_if = "Option::is_none")]
pub reconnect: Option<u32>,
#[serde(rename = "server", default, skip_serializing_if = "Option::is_none")]
pub server: Option<bool>,
#[serde(rename = "addr")]
pub addr: SocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevTapOptions {
#[serde(rename = "br", default, skip_serializing_if = "Option::is_none")]
pub br: Option<::std::string::String>,
#[serde(rename = "downscript", default, skip_serializing_if = "Option::is_none")]
pub downscript: Option<::std::string::String>,
#[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
pub fd: Option<::std::string::String>,
#[serde(rename = "fds", default, skip_serializing_if = "Option::is_none")]
pub fds: Option<::std::string::String>,
#[serde(rename = "helper", default, skip_serializing_if = "Option::is_none")]
pub helper: Option<::std::string::String>,
#[serde(rename = "ifname", default, skip_serializing_if = "Option::is_none")]
pub ifname: Option<::std::string::String>,
#[serde(rename = "poll-us", default, skip_serializing_if = "Option::is_none")]
pub poll_us: Option<u32>,
#[serde(rename = "queues", default, skip_serializing_if = "Option::is_none")]
pub queues: Option<u32>,
#[serde(rename = "script", default, skip_serializing_if = "Option::is_none")]
pub script: Option<::std::string::String>,
#[serde(rename = "sndbuf", default, skip_serializing_if = "Option::is_none")]
pub sndbuf: Option<u64>,
#[serde(rename = "vhost", default, skip_serializing_if = "Option::is_none")]
pub vhost: Option<bool>,
#[serde(rename = "vhostfd", default, skip_serializing_if = "Option::is_none")]
pub vhostfd: Option<::std::string::String>,
#[serde(rename = "vhostfds", default, skip_serializing_if = "Option::is_none")]
pub vhostfds: Option<::std::string::String>,
#[serde(rename = "vhostforce", default, skip_serializing_if = "Option::is_none")]
pub vhostforce: Option<bool>,
#[serde(rename = "vnet_hdr", default, skip_serializing_if = "Option::is_none")]
pub vnet_hdr: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevUserOptions {
#[serde(rename = "bootfile", default, skip_serializing_if = "Option::is_none")]
pub bootfile: Option<::std::string::String>,
#[serde(rename = "dhcpstart", default, skip_serializing_if = "Option::is_none")]
pub dhcpstart: Option<::std::string::String>,
#[serde(rename = "dns", default, skip_serializing_if = "Option::is_none")]
pub dns: Option<::std::string::String>,
#[serde(rename = "dnssearch", default, skip_serializing_if = "Option::is_none")]
pub dnssearch: Option<Vec<String>>,
#[serde(rename = "domainname", default, skip_serializing_if = "Option::is_none")]
pub domainname: Option<::std::string::String>,
#[serde(rename = "guestfwd", default, skip_serializing_if = "Option::is_none")]
pub guestfwd: Option<Vec<String>>,
#[serde(rename = "host", default, skip_serializing_if = "Option::is_none")]
pub host: Option<::std::string::String>,
#[serde(rename = "hostfwd", default, skip_serializing_if = "Option::is_none")]
pub hostfwd: Option<Vec<String>>,
#[serde(rename = "hostname", default, skip_serializing_if = "Option::is_none")]
pub hostname: Option<::std::string::String>,
#[serde(rename = "ip", default, skip_serializing_if = "Option::is_none")]
pub ip: Option<::std::string::String>,
#[serde(rename = "ipv4", default, skip_serializing_if = "Option::is_none")]
pub ipv4: Option<bool>,
#[serde(rename = "ipv6", default, skip_serializing_if = "Option::is_none")]
pub ipv6: Option<bool>,
#[serde(rename = "ipv6-dns", default, skip_serializing_if = "Option::is_none")]
pub ipv6_dns: Option<::std::string::String>,
#[serde(rename = "ipv6-host", default, skip_serializing_if = "Option::is_none")]
pub ipv6_host: Option<::std::string::String>,
#[serde(rename = "ipv6-prefix", default, skip_serializing_if = "Option::is_none")]
pub ipv6_prefix: Option<::std::string::String>,
#[serde(rename = "ipv6-prefixlen", default, skip_serializing_if = "Option::is_none")]
pub ipv6_prefixlen: Option<i64>,
#[serde(rename = "net", default, skip_serializing_if = "Option::is_none")]
pub net: Option<::std::string::String>,
#[serde(rename = "restrict", default, skip_serializing_if = "Option::is_none")]
pub restrict: Option<bool>,
#[serde(rename = "smb", default, skip_serializing_if = "Option::is_none")]
pub smb: Option<::std::string::String>,
#[serde(rename = "smbserver", default, skip_serializing_if = "Option::is_none")]
pub smbserver: Option<::std::string::String>,
#[serde(rename = "tftp", default, skip_serializing_if = "Option::is_none")]
pub tftp: Option<::std::string::String>,
#[serde(rename = "tftp-server-name", default, skip_serializing_if = "Option::is_none")]
pub tftp_server_name: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevVdeOptions {
#[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
pub group: Option<::std::string::String>,
#[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
pub mode: Option<u16>,
#[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
pub port: Option<u16>,
#[serde(rename = "sock", default, skip_serializing_if = "Option::is_none")]
pub sock: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevVhostUserOptions {
#[serde(rename = "queues", default, skip_serializing_if = "Option::is_none")]
pub queues: Option<i64>,
#[serde(rename = "vhostforce", default, skip_serializing_if = "Option::is_none")]
pub vhostforce: Option<bool>,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevVhostVDPAOptions {
#[serde(rename = "queues", default, skip_serializing_if = "Option::is_none")]
pub queues: Option<i64>,
#[serde(rename = "vhostdev", default, skip_serializing_if = "Option::is_none")]
pub vhostdev: Option<::std::string::String>,
#[serde(rename = "vhostfd", default, skip_serializing_if = "Option::is_none")]
pub vhostfd: Option<::std::string::String>,
#[serde(rename = "x-svq", default, skip_serializing_if = "Option::is_none")]
pub x_svq: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetdevVmnetBridgedOptions {
#[serde(rename = "isolated", default, skip_serializing_if = "Option::is_none")]
pub isolated: Option<bool>,
#[serde(rename = "ifname")]
pub ifname: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevVmnetHostOptions {
#[serde(rename = "end-address", default, skip_serializing_if = "Option::is_none")]
pub end_address: Option<::std::string::String>,
#[serde(rename = "isolated", default, skip_serializing_if = "Option::is_none")]
pub isolated: Option<bool>,
#[serde(rename = "net-uuid", default, skip_serializing_if = "Option::is_none")]
pub net_uuid: Option<::std::string::String>,
#[serde(rename = "start-address", default, skip_serializing_if = "Option::is_none")]
pub start_address: Option<::std::string::String>,
#[serde(rename = "subnet-mask", default, skip_serializing_if = "Option::is_none")]
pub subnet_mask: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetdevVmnetSharedOptions {
#[serde(rename = "end-address", default, skip_serializing_if = "Option::is_none")]
pub end_address: Option<::std::string::String>,
#[serde(rename = "isolated", default, skip_serializing_if = "Option::is_none")]
pub isolated: Option<bool>,
#[serde(rename = "nat66-prefix", default, skip_serializing_if = "Option::is_none")]
pub nat66_prefix: Option<::std::string::String>,
#[serde(rename = "start-address", default, skip_serializing_if = "Option::is_none")]
pub start_address: Option<::std::string::String>,
#[serde(rename = "subnet-mask", default, skip_serializing_if = "Option::is_none")]
pub subnet_mask: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetfilterProperties {
#[serde(rename = "insert", default, skip_serializing_if = "Option::is_none")]
pub insert: Option<NetfilterInsert>,
#[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
pub position: Option<::std::string::String>,
#[serde(rename = "queue", default, skip_serializing_if = "Option::is_none")]
pub queue: Option<NetFilterDirection>,
#[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
pub status: Option<::std::string::String>,
#[serde(rename = "netdev")]
pub netdev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaCpuOptions {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: CpuInstanceProperties,
}

impl<T: Into<CpuInstanceProperties>> From<T> for NumaCpuOptions {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<CpuInstanceProperties> for NumaCpuOptions {
        fn as_ref(&self) -> &CpuInstanceProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaDistOptions {
#[serde(rename = "dst")]
pub dst: u16,
#[serde(rename = "src")]
pub src: u16,
#[serde(rename = "val")]
pub val: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaHmatCacheOptions {
#[serde(rename = "associativity")]
pub associativity: HmatCacheAssociativity,
#[serde(rename = "level")]
pub level: u8,
#[serde(rename = "line")]
pub line: u16,
#[serde(rename = "node-id")]
pub node_id: u32,
#[serde(rename = "policy")]
pub policy: HmatCacheWritePolicy,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaHmatLBOptions {
#[serde(rename = "bandwidth", default, skip_serializing_if = "Option::is_none")]
pub bandwidth: Option<u64>,
#[serde(rename = "latency", default, skip_serializing_if = "Option::is_none")]
pub latency: Option<u64>,
#[serde(rename = "data-type")]
pub data_type: HmatLBDataType,
#[serde(rename = "hierarchy")]
pub hierarchy: HmatLBMemoryHierarchy,
#[serde(rename = "initiator")]
pub initiator: u16,
#[serde(rename = "target")]
pub target: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumaNodeOptions {
#[serde(rename = "cpus", default, skip_serializing_if = "Option::is_none")]
pub cpus: Option<Vec<u16>>,
#[serde(rename = "initiator", default, skip_serializing_if = "Option::is_none")]
pub initiator: Option<u16>,
#[serde(rename = "mem", default, skip_serializing_if = "Option::is_none")]
pub mem: Option<u64>,
#[serde(rename = "memdev", default, skip_serializing_if = "Option::is_none")]
pub memdev: Option<::std::string::String>,
#[serde(rename = "nodeid", default, skip_serializing_if = "Option::is_none")]
pub nodeid: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPropertyInfo {
#[serde(rename = "default-value", default, skip_serializing_if = "Option::is_none")]
pub default_value: Option<::qapi_spec::Any>,
#[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
pub description: Option<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTypeInfo {
#[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
pub abstract_: Option<bool>,
#[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
pub parent: Option<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCDIMMDeviceInfo {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "addr")]
pub addr: i64,
#[serde(rename = "hotpluggable")]
pub hotpluggable: bool,
#[serde(rename = "hotplugged")]
pub hotplugged: bool,
#[serde(rename = "memdev")]
pub memdev: ::std::string::String,
#[serde(rename = "node")]
pub node: i64,
#[serde(rename = "size")]
pub size: i64,
#[serde(rename = "slot")]
pub slot: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct PCDIMMDeviceInfoWrapper {
#[serde(rename = "data")]
pub data: PCDIMMDeviceInfo,
}

impl<T: Into<PCDIMMDeviceInfo>> From<T> for PCDIMMDeviceInfoWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<PCDIMMDeviceInfo> for PCDIMMDeviceInfoWrapper {
        fn as_ref(&self) -> &PCDIMMDeviceInfo {
            &self.data
        }
    }
impl ::std::ops::Deref for PCDIMMDeviceInfoWrapper {
    type Target = PCDIMMDeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl PCDIMMDeviceInfoWrapper {
    pub fn into_inner(self) -> PCDIMMDeviceInfo {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRManagerInfo {
#[serde(rename = "connected")]
pub connected: bool,
#[serde(rename = "id")]
pub id: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciBridgeInfo {
#[serde(rename = "devices", default, skip_serializing_if = "Option::is_none")]
pub devices: Option<Vec<PciDeviceInfo>>,
#[serde(rename = "bus")]
pub bus: PciBusInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciBusInfo {
#[serde(rename = "io_range")]
pub io_range: PciMemoryRange,
#[serde(rename = "memory_range")]
pub memory_range: PciMemoryRange,
#[serde(rename = "number")]
pub number: i64,
#[serde(rename = "prefetchable_range")]
pub prefetchable_range: PciMemoryRange,
#[serde(rename = "secondary")]
pub secondary: i64,
#[serde(rename = "subordinate")]
pub subordinate: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDeviceClass {
#[serde(rename = "desc", default, skip_serializing_if = "Option::is_none")]
pub desc: Option<::std::string::String>,
#[serde(rename = "class")]
pub class: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDeviceId {
#[serde(rename = "subsystem", default, skip_serializing_if = "Option::is_none")]
pub subsystem: Option<i64>,
#[serde(rename = "subsystem-vendor", default, skip_serializing_if = "Option::is_none")]
pub subsystem_vendor: Option<i64>,
#[serde(rename = "device")]
pub device: i64,
#[serde(rename = "vendor")]
pub vendor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDeviceInfo {
#[serde(rename = "irq", default, skip_serializing_if = "Option::is_none")]
pub irq: Option<i64>,
#[serde(rename = "pci_bridge", default, skip_serializing_if = "Option::is_none")]
pub pci_bridge: Option<PciBridgeInfo>,
#[serde(rename = "bus")]
pub bus: i64,
#[serde(rename = "class_info")]
pub class_info: PciDeviceClass,
#[serde(rename = "function")]
pub function: i64,
#[serde(rename = "id")]
pub id: PciDeviceId,
#[serde(rename = "irq_pin")]
pub irq_pin: i64,
#[serde(rename = "qdev_id")]
pub qdev_id: ::std::string::String,
#[serde(rename = "regions")]
pub regions: Vec<PciMemoryRegion>,
#[serde(rename = "slot")]
pub slot: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciInfo {
#[serde(rename = "bus")]
pub bus: i64,
#[serde(rename = "devices")]
pub devices: Vec<PciDeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciMemoryRange {
#[serde(rename = "base")]
pub base: i64,
#[serde(rename = "limit")]
pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciMemoryRegion {
#[serde(rename = "mem_type_64", default, skip_serializing_if = "Option::is_none")]
pub mem_type_64: Option<bool>,
#[serde(rename = "prefetch", default, skip_serializing_if = "Option::is_none")]
pub prefetch: Option<bool>,
#[serde(rename = "address")]
pub address: i64,
#[serde(rename = "bar")]
pub bar: i64,
#[serde(rename = "size")]
pub size: i64,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrManagerHelperProperties {
#[serde(rename = "path")]
pub path: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for PrManagerHelperProperties {
    fn from(val: T) -> Self {
        Self {
            path: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for PrManagerHelperProperties {
        fn as_ref(&self) -> &::std::string::String {
            &self.path
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAuthZListRule {
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<QAuthZListFormat>,
#[serde(rename = "match")]
pub match_: ::std::string::String,
#[serde(rename = "policy")]
pub policy: QAuthZListPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptoAkCipherOptionsRSA {
#[serde(rename = "hash-alg")]
pub hash_alg: QCryptoHashAlgorithm,
#[serde(rename = "padding-alg")]
pub padding_alg: QCryptoRSAPaddingAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptoBlockAmendOptionsLUKS {
#[serde(rename = "iter-time", default, skip_serializing_if = "Option::is_none")]
pub iter_time: Option<i64>,
#[serde(rename = "keyslot", default, skip_serializing_if = "Option::is_none")]
pub keyslot: Option<i64>,
#[serde(rename = "new-secret", default, skip_serializing_if = "Option::is_none")]
pub new_secret: Option<::std::string::String>,
#[serde(rename = "old-secret", default, skip_serializing_if = "Option::is_none")]
pub old_secret: Option<::std::string::String>,
#[serde(rename = "secret", default, skip_serializing_if = "Option::is_none")]
pub secret: Option<::std::string::String>,
#[serde(rename = "state")]
pub state: QCryptoBlockLUKSKeyslotState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptoBlockCreateOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: QCryptoBlockOptionsLUKS,
#[serde(rename = "cipher-alg", default, skip_serializing_if = "Option::is_none")]
pub cipher_alg: Option<QCryptoCipherAlgorithm>,
#[serde(rename = "cipher-mode", default, skip_serializing_if = "Option::is_none")]
pub cipher_mode: Option<QCryptoCipherMode>,
#[serde(rename = "hash-alg", default, skip_serializing_if = "Option::is_none")]
pub hash_alg: Option<QCryptoHashAlgorithm>,
#[serde(rename = "iter-time", default, skip_serializing_if = "Option::is_none")]
pub iter_time: Option<i64>,
#[serde(rename = "ivgen-alg", default, skip_serializing_if = "Option::is_none")]
pub ivgen_alg: Option<QCryptoIVGenAlgorithm>,
#[serde(rename = "ivgen-hash-alg", default, skip_serializing_if = "Option::is_none")]
pub ivgen_hash_alg: Option<QCryptoHashAlgorithm>,
}

impl<T: Into<QCryptoBlockOptionsLUKS>> From<T> for QCryptoBlockCreateOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cipher_alg: Default::default(),
cipher_mode: Default::default(),
hash_alg: Default::default(),
iter_time: Default::default(),
ivgen_alg: Default::default(),
ivgen_hash_alg: Default::default(),

        }
    }
}
    impl AsRef<QCryptoBlockOptionsLUKS> for QCryptoBlockCreateOptionsLUKS {
        fn as_ref(&self) -> &QCryptoBlockOptionsLUKS {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QCryptoBlockInfoBase {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptoBlockInfoLUKS {
#[serde(rename = "ivgen-hash-alg", default, skip_serializing_if = "Option::is_none")]
pub ivgen_hash_alg: Option<QCryptoHashAlgorithm>,
#[serde(rename = "cipher-alg")]
pub cipher_alg: QCryptoCipherAlgorithm,
#[serde(rename = "cipher-mode")]
pub cipher_mode: QCryptoCipherMode,
#[serde(rename = "detached-header")]
pub detached_header: bool,
#[serde(rename = "hash-alg")]
pub hash_alg: QCryptoHashAlgorithm,
#[serde(rename = "ivgen-alg")]
pub ivgen_alg: QCryptoIVGenAlgorithm,
#[serde(rename = "master-key-iters")]
pub master_key_iters: i64,
#[serde(rename = "payload-offset")]
pub payload_offset: i64,
#[serde(rename = "slots")]
pub slots: Vec<QCryptoBlockInfoLUKSSlot>,
#[serde(rename = "uuid")]
pub uuid: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptoBlockInfoLUKSSlot {
#[serde(rename = "iters", default, skip_serializing_if = "Option::is_none")]
pub iters: Option<i64>,
#[serde(rename = "stripes", default, skip_serializing_if = "Option::is_none")]
pub stripes: Option<i64>,
#[serde(rename = "active")]
pub active: bool,
#[serde(rename = "key-offset")]
pub key_offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QCryptoBlockOptionsBase {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QCryptoBlockOptionsLUKS {
#[serde(rename = "key-secret", default, skip_serializing_if = "Option::is_none")]
pub key_secret: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for QCryptoBlockOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            key_secret: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QCryptoBlockOptionsQCow {
#[serde(rename = "key-secret", default, skip_serializing_if = "Option::is_none")]
pub key_secret: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for QCryptoBlockOptionsQCow {
    fn from(val: T) -> Self {
        Self {
            key_secret: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptodevBackendClient {
#[serde(rename = "queue")]
pub queue: u32,
#[serde(rename = "type")]
pub type_: QCryptodevBackendType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCryptodevInfo {
#[serde(rename = "client")]
pub client: Vec<QCryptodevBackendClient>,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "service")]
pub service: Vec<QCryptodevBackendServiceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct QKeyCodeWrapper {
#[serde(rename = "data")]
pub data: QKeyCode,
}

impl<T: Into<QKeyCode>> From<T> for QKeyCodeWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<QKeyCode> for QKeyCodeWrapper {
        fn as_ref(&self) -> &QKeyCode {
            &self.data
        }
    }
impl ::std::ops::Deref for QKeyCodeWrapper {
    type Target = QKeyCode;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl QKeyCodeWrapper {
    pub fn into_inner(self) -> QKeyCode {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qcow2BitmapInfo {
#[serde(rename = "flags")]
pub flags: Vec<Qcow2BitmapInfoFlags>,
#[serde(rename = "granularity")]
pub granularity: u32,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Qcow2OverlapCheckFlags {
#[serde(rename = "active-l1", default, skip_serializing_if = "Option::is_none")]
pub active_l1: Option<bool>,
#[serde(rename = "active-l2", default, skip_serializing_if = "Option::is_none")]
pub active_l2: Option<bool>,
#[serde(rename = "bitmap-directory", default, skip_serializing_if = "Option::is_none")]
pub bitmap_directory: Option<bool>,
#[serde(rename = "inactive-l1", default, skip_serializing_if = "Option::is_none")]
pub inactive_l1: Option<bool>,
#[serde(rename = "inactive-l2", default, skip_serializing_if = "Option::is_none")]
pub inactive_l2: Option<bool>,
#[serde(rename = "main-header", default, skip_serializing_if = "Option::is_none")]
pub main_header: Option<bool>,
#[serde(rename = "refcount-block", default, skip_serializing_if = "Option::is_none")]
pub refcount_block: Option<bool>,
#[serde(rename = "refcount-table", default, skip_serializing_if = "Option::is_none")]
pub refcount_table: Option<bool>,
#[serde(rename = "snapshot-table", default, skip_serializing_if = "Option::is_none")]
pub snapshot_table: Option<bool>,
#[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
pub template: Option<Qcow2OverlapCheckMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QtestProperties {
#[serde(rename = "log", default, skip_serializing_if = "Option::is_none")]
pub log: Option<::std::string::String>,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionCreateOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionCreateOptionsLUKSBase,
}

impl<T: Into<RbdEncryptionCreateOptionsLUKSBase>> From<T> for RbdEncryptionCreateOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<RbdEncryptionCreateOptionsLUKSBase> for RbdEncryptionCreateOptionsLUKS {
        fn as_ref(&self) -> &RbdEncryptionCreateOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionCreateOptionsLUKS2 {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionCreateOptionsLUKSBase,
}

impl<T: Into<RbdEncryptionCreateOptionsLUKSBase>> From<T> for RbdEncryptionCreateOptionsLUKS2 {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<RbdEncryptionCreateOptionsLUKSBase> for RbdEncryptionCreateOptionsLUKS2 {
        fn as_ref(&self) -> &RbdEncryptionCreateOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionCreateOptionsLUKSBase {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionOptionsLUKSBase,
#[serde(rename = "cipher-alg", default, skip_serializing_if = "Option::is_none")]
pub cipher_alg: Option<QCryptoCipherAlgorithm>,
}

impl<T: Into<RbdEncryptionOptionsLUKSBase>> From<T> for RbdEncryptionCreateOptionsLUKSBase {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
cipher_alg: Default::default(),

        }
    }
}
    impl AsRef<RbdEncryptionOptionsLUKSBase> for RbdEncryptionCreateOptionsLUKSBase {
        fn as_ref(&self) -> &RbdEncryptionOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionOptionsLUKS {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionOptionsLUKSBase,
}

impl<T: Into<RbdEncryptionOptionsLUKSBase>> From<T> for RbdEncryptionOptionsLUKS {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<RbdEncryptionOptionsLUKSBase> for RbdEncryptionOptionsLUKS {
        fn as_ref(&self) -> &RbdEncryptionOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionOptionsLUKS2 {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionOptionsLUKSBase,
}

impl<T: Into<RbdEncryptionOptionsLUKSBase>> From<T> for RbdEncryptionOptionsLUKS2 {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<RbdEncryptionOptionsLUKSBase> for RbdEncryptionOptionsLUKS2 {
        fn as_ref(&self) -> &RbdEncryptionOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionOptionsLUKSAny {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RbdEncryptionOptionsLUKSBase,
}

impl<T: Into<RbdEncryptionOptionsLUKSBase>> From<T> for RbdEncryptionOptionsLUKSAny {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),

        }
    }
}
    impl AsRef<RbdEncryptionOptionsLUKSBase> for RbdEncryptionOptionsLUKSAny {
        fn as_ref(&self) -> &RbdEncryptionOptionsLUKSBase {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbdEncryptionOptionsLUKSBase {
#[serde(rename = "key-secret")]
pub key_secret: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for RbdEncryptionOptionsLUKSBase {
    fn from(val: T) -> Self {
        Self {
            key_secret: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for RbdEncryptionOptionsLUKSBase {
        fn as_ref(&self) -> &::std::string::String {
            &self.key_secret
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteObjectProperties {
#[serde(rename = "devid")]
pub devid: ::std::string::String,
#[serde(rename = "fd")]
pub fd: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayInfo {
#[serde(rename = "filename", default, skip_serializing_if = "Option::is_none")]
pub filename: Option<::std::string::String>,
#[serde(rename = "icount")]
pub icount: i64,
#[serde(rename = "mode")]
pub mode: ReplayMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
#[serde(rename = "desc", default, skip_serializing_if = "Option::is_none")]
pub desc: Option<::std::string::String>,
#[serde(rename = "error")]
pub error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngEgdProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RngProperties,
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RngProperties {
#[serde(rename = "opened", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub opened: Option<bool>,
}

impl<T: Into<bool>> From<T> for RngProperties {
    fn from(val: T) -> Self {
        Self {
            opened: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngRandomProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: RngProperties,
#[serde(rename = "filename", default, skip_serializing_if = "Option::is_none")]
pub filename: Option<::std::string::String>,
}

impl<T: Into<RngProperties>> From<T> for RngRandomProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
filename: Default::default(),

        }
    }
}
    impl AsRef<RngProperties> for RngRandomProperties {
        fn as_ref(&self) -> &RngProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockerOfDpaFlow {
#[serde(rename = "action")]
pub action: RockerOfDpaFlowAction,
#[serde(rename = "cookie")]
pub cookie: u64,
#[serde(rename = "hits")]
pub hits: u64,
#[serde(rename = "key")]
pub key: RockerOfDpaFlowKey,
#[serde(rename = "mask")]
pub mask: RockerOfDpaFlowMask,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RockerOfDpaFlowAction {
#[serde(rename = "goto-tbl", default, skip_serializing_if = "Option::is_none")]
pub goto_tbl: Option<u32>,
#[serde(rename = "group-id", default, skip_serializing_if = "Option::is_none")]
pub group_id: Option<u32>,
#[serde(rename = "new-vlan-id", default, skip_serializing_if = "Option::is_none")]
pub new_vlan_id: Option<u16>,
#[serde(rename = "out-pport", default, skip_serializing_if = "Option::is_none")]
pub out_pport: Option<u32>,
#[serde(rename = "tunnel-lport", default, skip_serializing_if = "Option::is_none")]
pub tunnel_lport: Option<u32>,
#[serde(rename = "vlan-id", default, skip_serializing_if = "Option::is_none")]
pub vlan_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockerOfDpaFlowKey {
#[serde(rename = "eth-dst", default, skip_serializing_if = "Option::is_none")]
pub eth_dst: Option<::std::string::String>,
#[serde(rename = "eth-src", default, skip_serializing_if = "Option::is_none")]
pub eth_src: Option<::std::string::String>,
#[serde(rename = "eth-type", default, skip_serializing_if = "Option::is_none")]
pub eth_type: Option<u16>,
#[serde(rename = "in-pport", default, skip_serializing_if = "Option::is_none")]
pub in_pport: Option<u32>,
#[serde(rename = "ip-dst", default, skip_serializing_if = "Option::is_none")]
pub ip_dst: Option<::std::string::String>,
#[serde(rename = "ip-proto", default, skip_serializing_if = "Option::is_none")]
pub ip_proto: Option<u8>,
#[serde(rename = "ip-tos", default, skip_serializing_if = "Option::is_none")]
pub ip_tos: Option<u8>,
#[serde(rename = "tunnel-id", default, skip_serializing_if = "Option::is_none")]
pub tunnel_id: Option<u32>,
#[serde(rename = "vlan-id", default, skip_serializing_if = "Option::is_none")]
pub vlan_id: Option<u16>,
#[serde(rename = "priority")]
pub priority: u32,
#[serde(rename = "tbl-id")]
pub tbl_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RockerOfDpaFlowMask {
#[serde(rename = "eth-dst", default, skip_serializing_if = "Option::is_none")]
pub eth_dst: Option<::std::string::String>,
#[serde(rename = "eth-src", default, skip_serializing_if = "Option::is_none")]
pub eth_src: Option<::std::string::String>,
#[serde(rename = "in-pport", default, skip_serializing_if = "Option::is_none")]
pub in_pport: Option<u32>,
#[serde(rename = "ip-proto", default, skip_serializing_if = "Option::is_none")]
pub ip_proto: Option<u8>,
#[serde(rename = "ip-tos", default, skip_serializing_if = "Option::is_none")]
pub ip_tos: Option<u8>,
#[serde(rename = "tunnel-id", default, skip_serializing_if = "Option::is_none")]
pub tunnel_id: Option<u32>,
#[serde(rename = "vlan-id", default, skip_serializing_if = "Option::is_none")]
pub vlan_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockerOfDpaGroup {
#[serde(rename = "group-id", default, skip_serializing_if = "Option::is_none")]
pub group_id: Option<u32>,
#[serde(rename = "group-ids", default, skip_serializing_if = "Option::is_none")]
pub group_ids: Option<Vec<u32>>,
#[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
pub index: Option<u32>,
#[serde(rename = "out-pport", default, skip_serializing_if = "Option::is_none")]
pub out_pport: Option<u32>,
#[serde(rename = "pop-vlan", default, skip_serializing_if = "Option::is_none")]
pub pop_vlan: Option<u8>,
#[serde(rename = "pport", default, skip_serializing_if = "Option::is_none")]
pub pport: Option<u32>,
#[serde(rename = "set-eth-dst", default, skip_serializing_if = "Option::is_none")]
pub set_eth_dst: Option<::std::string::String>,
#[serde(rename = "set-eth-src", default, skip_serializing_if = "Option::is_none")]
pub set_eth_src: Option<::std::string::String>,
#[serde(rename = "set-vlan-id", default, skip_serializing_if = "Option::is_none")]
pub set_vlan_id: Option<u16>,
#[serde(rename = "ttl-check", default, skip_serializing_if = "Option::is_none")]
pub ttl_check: Option<u8>,
#[serde(rename = "vlan-id", default, skip_serializing_if = "Option::is_none")]
pub vlan_id: Option<u16>,
#[serde(rename = "id")]
pub id: u32,
#[serde(rename = "type")]
pub type_: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockerPort {
#[serde(rename = "autoneg")]
pub autoneg: RockerPortAutoneg,
#[serde(rename = "duplex")]
pub duplex: RockerPortDuplex,
#[serde(rename = "enabled")]
pub enabled: bool,
#[serde(rename = "link-up")]
pub link_up: bool,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "speed")]
pub speed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockerSwitch {
#[serde(rename = "id")]
pub id: u64,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "ports")]
pub ports: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RxFilterInfo {
#[serde(rename = "broadcast-allowed")]
pub broadcast_allowed: bool,
#[serde(rename = "main-mac")]
pub main_mac: ::std::string::String,
#[serde(rename = "multicast")]
pub multicast: RxState,
#[serde(rename = "multicast-overflow")]
pub multicast_overflow: bool,
#[serde(rename = "multicast-table")]
pub multicast_table: Vec<::std::string::String>,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "promiscuous")]
pub promiscuous: bool,
#[serde(rename = "unicast")]
pub unicast: RxState,
#[serde(rename = "unicast-overflow")]
pub unicast_overflow: bool,
#[serde(rename = "unicast-table")]
pub unicast_table: Vec<::std::string::String>,
#[serde(rename = "vlan")]
pub vlan: RxState,
#[serde(rename = "vlan-table")]
pub vlan_table: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGXEPCSection {
#[serde(rename = "node")]
pub node: i64,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGXInfo {
#[serde(rename = "flc")]
pub flc: bool,
#[serde(rename = "sections")]
pub sections: Vec<SGXEPCSection>,
#[serde(rename = "sgx")]
pub sgx: bool,
#[serde(rename = "sgx1")]
pub sgx1: bool,
#[serde(rename = "sgx2")]
pub sgx2: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMPConfiguration {
#[serde(rename = "books", default, skip_serializing_if = "Option::is_none")]
pub books: Option<i64>,
#[serde(rename = "clusters", default, skip_serializing_if = "Option::is_none")]
pub clusters: Option<i64>,
#[serde(rename = "cores", default, skip_serializing_if = "Option::is_none")]
pub cores: Option<i64>,
#[serde(rename = "cpus", default, skip_serializing_if = "Option::is_none")]
pub cpus: Option<i64>,
#[serde(rename = "dies", default, skip_serializing_if = "Option::is_none")]
pub dies: Option<i64>,
#[serde(rename = "drawers", default, skip_serializing_if = "Option::is_none")]
pub drawers: Option<i64>,
#[serde(rename = "maxcpus", default, skip_serializing_if = "Option::is_none")]
pub maxcpus: Option<i64>,
#[serde(rename = "modules", default, skip_serializing_if = "Option::is_none")]
pub modules: Option<i64>,
#[serde(rename = "sockets", default, skip_serializing_if = "Option::is_none")]
pub sockets: Option<i64>,
#[serde(rename = "threads", default, skip_serializing_if = "Option::is_none")]
pub threads: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoAlternate {
#[serde(rename = "members")]
pub members: Vec<SchemaInfoAlternateMember>,
}

impl<T: Into<Vec<SchemaInfoAlternateMember>>> From<T> for SchemaInfoAlternate {
    fn from(val: T) -> Self {
        Self {
            members: val.into(),

        }
    }
}
    impl AsRef<Vec<SchemaInfoAlternateMember>> for SchemaInfoAlternate {
        fn as_ref(&self) -> &Vec<SchemaInfoAlternateMember> {
            &self.members
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoAlternateMember {
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for SchemaInfoAlternateMember {
    fn from(val: T) -> Self {
        Self {
            type_: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for SchemaInfoAlternateMember {
        fn as_ref(&self) -> &::std::string::String {
            &self.type_
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoArray {
#[serde(rename = "element-type")]
pub element_type: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for SchemaInfoArray {
    fn from(val: T) -> Self {
        Self {
            element_type: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for SchemaInfoArray {
        fn as_ref(&self) -> &::std::string::String {
            &self.element_type
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoBuiltin {
#[serde(rename = "json-type")]
pub json_type: JSONType,
}

impl<T: Into<JSONType>> From<T> for SchemaInfoBuiltin {
    fn from(val: T) -> Self {
        Self {
            json_type: val.into(),

        }
    }
}
    impl AsRef<JSONType> for SchemaInfoBuiltin {
        fn as_ref(&self) -> &JSONType {
            &self.json_type
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoCommand {
#[serde(rename = "allow-oob", default, skip_serializing_if = "Option::is_none")]
pub allow_oob: Option<bool>,
#[serde(rename = "arg-type")]
pub arg_type: ::std::string::String,
#[serde(rename = "ret-type")]
pub ret_type: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoEnum {
#[serde(rename = "members")]
pub members: Vec<SchemaInfoEnumMember>,
#[serde(rename = "values")] #[deprecated]
pub values: Vec<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoEnumMember {
#[serde(rename = "features", default, skip_serializing_if = "Option::is_none")]
pub features: Option<Vec<::std::string::String>>,
#[serde(rename = "name")]
pub name: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoEvent {
#[serde(rename = "arg-type")]
pub arg_type: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for SchemaInfoEvent {
    fn from(val: T) -> Self {
        Self {
            arg_type: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for SchemaInfoEvent {
        fn as_ref(&self) -> &::std::string::String {
            &self.arg_type
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoObject {
#[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
pub tag: Option<::std::string::String>,
#[serde(rename = "variants", default, skip_serializing_if = "Option::is_none")]
pub variants: Option<Vec<SchemaInfoObjectVariant>>,
#[serde(rename = "members")]
pub members: Vec<SchemaInfoObjectMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoObjectMember {
#[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
pub default: Option<::qapi_spec::Any>,
#[serde(rename = "features", default, skip_serializing_if = "Option::is_none")]
pub features: Option<Vec<::std::string::String>>,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfoObjectVariant {
#[serde(rename = "case")]
pub case: ::std::string::String,
#[serde(rename = "type")]
pub type_: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretCommonProperties {
#[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
pub format: Option<QCryptoSecretFormat>,
#[serde(rename = "iv", with = "::qapi_spec::base64_opt", default, skip_serializing_if = "Option::is_none")]
pub iv: Option<Vec<u8>>,
#[serde(rename = "keyid", default, skip_serializing_if = "Option::is_none")]
pub keyid: Option<::std::string::String>,
#[serde(rename = "loaded", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub loaded: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeyringProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SecretCommonProperties,
#[serde(rename = "serial")]
pub serial: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SecretCommonProperties,
#[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
pub data: Option<::std::string::String>,
#[serde(rename = "file", default, skip_serializing_if = "Option::is_none")]
pub file: Option<::std::string::String>,
}

impl<T: Into<SecretCommonProperties>> From<T> for SecretProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
data: Default::default(),
file: Default::default(),

        }
    }
}
    impl AsRef<SecretCommonProperties> for SecretProperties {
        fn as_ref(&self) -> &SecretCommonProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetPasswordOptionsVnc {
#[serde(rename = "display", default, skip_serializing_if = "Option::is_none")]
pub display: Option<::std::string::String>,
}

impl<T: Into<::std::string::String>> From<T> for SetPasswordOptionsVnc {
    fn from(val: T) -> Self {
        Self {
            display: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevAttestationReport {
#[serde(rename = "data")]
pub data: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for SevAttestationReport {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for SevAttestationReport {
        fn as_ref(&self) -> &::std::string::String {
            &self.data
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevCapability {
#[serde(rename = "cbitpos")]
pub cbitpos: i64,
#[serde(rename = "cert-chain")]
pub cert_chain: ::std::string::String,
#[serde(rename = "cpu0-id")]
pub cpu0_id: ::std::string::String,
#[serde(rename = "pdh")]
pub pdh: ::std::string::String,
#[serde(rename = "reduced-phys-bits")]
pub reduced_phys_bits: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevCommonProperties {
#[serde(rename = "cbitpos", default, skip_serializing_if = "Option::is_none")]
pub cbitpos: Option<u32>,
#[serde(rename = "kernel-hashes", default, skip_serializing_if = "Option::is_none")]
pub kernel_hashes: Option<bool>,
#[serde(rename = "sev-device", default, skip_serializing_if = "Option::is_none")]
pub sev_device: Option<::std::string::String>,
#[serde(rename = "reduced-phys-bits")]
pub reduced_phys_bits: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevGuestInfo {
#[serde(rename = "handle")]
pub handle: u32,
#[serde(rename = "policy")]
pub policy: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevGuestProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SevCommonProperties,
#[serde(rename = "dh-cert-file", with = "::qapi_spec::base64_opt", default, skip_serializing_if = "Option::is_none")]
pub dh_cert_file: Option<Vec<u8>>,
#[serde(rename = "handle", default, skip_serializing_if = "Option::is_none")]
pub handle: Option<u32>,
#[serde(rename = "legacy-vm-type", default, skip_serializing_if = "Option::is_none")]
pub legacy_vm_type: Option<OnOffAuto>,
#[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
pub policy: Option<u32>,
#[serde(rename = "session-file", with = "::qapi_spec::base64_opt", default, skip_serializing_if = "Option::is_none")]
pub session_file: Option<Vec<u8>>,
}

impl<T: Into<SevCommonProperties>> From<T> for SevGuestProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
dh_cert_file: Default::default(),
handle: Default::default(),
legacy_vm_type: Default::default(),
policy: Default::default(),
session_file: Default::default(),

        }
    }
}
    impl AsRef<SevCommonProperties> for SevGuestProperties {
        fn as_ref(&self) -> &SevCommonProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevLaunchMeasureInfo {
#[serde(rename = "data")]
pub data: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for SevLaunchMeasureInfo {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for SevLaunchMeasureInfo {
        fn as_ref(&self) -> &::std::string::String {
            &self.data
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevSnpGuestInfo {
#[serde(rename = "snp-policy")]
pub snp_policy: u64,
}

impl<T: Into<u64>> From<T> for SevSnpGuestInfo {
    fn from(val: T) -> Self {
        Self {
            snp_policy: val.into(),

        }
    }
}
    impl AsRef<u64> for SevSnpGuestInfo {
        fn as_ref(&self) -> &u64 {
            &self.snp_policy
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevSnpGuestProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SevCommonProperties,
#[serde(rename = "author-key-enabled", default, skip_serializing_if = "Option::is_none")]
pub author_key_enabled: Option<bool>,
#[serde(rename = "guest-visible-workarounds", default, skip_serializing_if = "Option::is_none")]
pub guest_visible_workarounds: Option<::std::string::String>,
#[serde(rename = "host-data", default, skip_serializing_if = "Option::is_none")]
pub host_data: Option<::std::string::String>,
#[serde(rename = "id-auth", default, skip_serializing_if = "Option::is_none")]
pub id_auth: Option<::std::string::String>,
#[serde(rename = "id-block", default, skip_serializing_if = "Option::is_none")]
pub id_block: Option<::std::string::String>,
#[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
pub policy: Option<u64>,
#[serde(rename = "vcek-disabled", default, skip_serializing_if = "Option::is_none")]
pub vcek_disabled: Option<bool>,
}

impl<T: Into<SevCommonProperties>> From<T> for SevSnpGuestProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
author_key_enabled: Default::default(),
guest_visible_workarounds: Default::default(),
host_data: Default::default(),
id_auth: Default::default(),
id_block: Default::default(),
policy: Default::default(),
vcek_disabled: Default::default(),

        }
    }
}
    impl AsRef<SevCommonProperties> for SevSnpGuestProperties {
        fn as_ref(&self) -> &SevCommonProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgxEPC {
#[serde(rename = "memdev")]
pub memdev: ::std::string::String,
#[serde(rename = "node")]
pub node: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgxEPCDeviceInfo {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "memaddr")]
pub memaddr: u64,
#[serde(rename = "memdev")]
pub memdev: ::std::string::String,
#[serde(rename = "node")]
pub node: i64,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct SgxEPCDeviceInfoWrapper {
#[serde(rename = "data")]
pub data: SgxEPCDeviceInfo,
}

impl<T: Into<SgxEPCDeviceInfo>> From<T> for SgxEPCDeviceInfoWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<SgxEPCDeviceInfo> for SgxEPCDeviceInfoWrapper {
        fn as_ref(&self) -> &SgxEPCDeviceInfo {
            &self.data
        }
    }
impl ::std::ops::Deref for SgxEPCDeviceInfoWrapper {
    type Target = SgxEPCDeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl SgxEPCDeviceInfoWrapper {
    pub fn into_inner(self) -> SgxEPCDeviceInfo {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgxEPCProperties {
#[serde(rename = "sgx-epc")]
pub sgx_epc: Vec<SgxEPC>,
}

impl<T: Into<Vec<SgxEPC>>> From<T> for SgxEPCProperties {
    fn from(val: T) -> Self {
        Self {
            sgx_epc: val.into(),

        }
    }
}
    impl AsRef<Vec<SgxEPC>> for SgxEPCProperties {
        fn as_ref(&self) -> &Vec<SgxEPC> {
            &self.sgx_epc
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
#[serde(rename = "icount", default, skip_serializing_if = "Option::is_none")]
pub icount: Option<i64>,
#[serde(rename = "date-nsec")]
pub date_nsec: i64,
#[serde(rename = "date-sec")]
pub date_sec: i64,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "vm-clock-nsec")]
pub vm_clock_nsec: i64,
#[serde(rename = "vm-clock-sec")]
pub vm_clock_sec: i64,
#[serde(rename = "vm-state-size")]
pub vm_state_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceBasicInfo {
#[serde(rename = "family")]
pub family: NetworkAddressFamily,
#[serde(rename = "host")]
pub host: ::std::string::String,
#[serde(rename = "port")]
pub port: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceChannel {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SpiceBasicInfo,
#[serde(rename = "channel-id")]
pub channel_id: i64,
#[serde(rename = "channel-type")]
pub channel_type: i64,
#[serde(rename = "connection-id")]
pub connection_id: i64,
#[serde(rename = "tls")]
pub tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceInfo {
#[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
pub auth: Option<::std::string::String>,
#[serde(rename = "channels", default, skip_serializing_if = "Option::is_none")]
pub channels: Option<Vec<SpiceChannel>>,
#[serde(rename = "compiled-version", default, skip_serializing_if = "Option::is_none")]
pub compiled_version: Option<::std::string::String>,
#[serde(rename = "host", default, skip_serializing_if = "Option::is_none")]
pub host: Option<::std::string::String>,
#[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
pub port: Option<i64>,
#[serde(rename = "tls-port", default, skip_serializing_if = "Option::is_none")]
pub tls_port: Option<i64>,
#[serde(rename = "enabled")]
pub enabled: bool,
#[serde(rename = "migrated")]
pub migrated: bool,
#[serde(rename = "mouse-mode")]
pub mouse_mode: SpiceQueryMouseMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceServerInfo {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: SpiceBasicInfo,
#[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
pub auth: Option<::std::string::String>,
}

impl<T: Into<SpiceBasicInfo>> From<T> for SpiceServerInfo {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
auth: Default::default(),

        }
    }
}
    impl AsRef<SpiceBasicInfo> for SpiceServerInfo {
        fn as_ref(&self) -> &SpiceBasicInfo {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshHostKeyHash {
#[serde(rename = "hash")]
pub hash: ::std::string::String,
#[serde(rename = "type")]
pub type_: SshHostKeyCheckHashType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "value")]
pub value: StatsValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsRequest {
#[serde(rename = "names", default, skip_serializing_if = "Option::is_none")]
pub names: Option<Vec<::std::string::String>>,
#[serde(rename = "provider")]
pub provider: StatsProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResult {
#[serde(rename = "qom-path", default, skip_serializing_if = "Option::is_none")]
pub qom_path: Option<::std::string::String>,
#[serde(rename = "provider")]
pub provider: StatsProvider,
#[serde(rename = "stats")]
pub stats: Vec<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSchema {
#[serde(rename = "provider")]
pub provider: StatsProvider,
#[serde(rename = "stats")]
pub stats: Vec<StatsSchemaValue>,
#[serde(rename = "target")]
pub target: StatsTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSchemaValue {
#[serde(rename = "base", default, skip_serializing_if = "Option::is_none")]
pub base: Option<i8>,
#[serde(rename = "bucket-size", default, skip_serializing_if = "Option::is_none")]
pub bucket_size: Option<u32>,
#[serde(rename = "unit", default, skip_serializing_if = "Option::is_none")]
pub unit: Option<StatsUnit>,
#[serde(rename = "exponent")]
pub exponent: i16,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "type")]
pub type_: StatsType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsVCPUFilter {
#[serde(rename = "vcpus", default, skip_serializing_if = "Option::is_none")]
pub vcpus: Option<Vec<::std::string::String>>,
}

impl<T: Into<Vec<::std::string::String>>> From<T> for StatsVCPUFilter {
    fn from(val: T) -> Self {
        Self {
            vcpus: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
#[serde(rename = "running")]
pub running: bool,
#[serde(rename = "status")]
pub status: RunState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct String {
#[serde(rename = "str")]
pub str: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for String {
    fn from(val: T) -> Self {
        Self {
            str: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for String {
        fn as_ref(&self) -> &::std::string::String {
            &self.str
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMEmulatorOptions {
#[serde(rename = "chardev")]
pub chardev: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for TPMEmulatorOptions {
    fn from(val: T) -> Self {
        Self {
            chardev: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for TPMEmulatorOptions {
        fn as_ref(&self) -> &::std::string::String {
            &self.chardev
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct TPMEmulatorOptionsWrapper {
#[serde(rename = "data")]
pub data: TPMEmulatorOptions,
}

impl<T: Into<TPMEmulatorOptions>> From<T> for TPMEmulatorOptionsWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<TPMEmulatorOptions> for TPMEmulatorOptionsWrapper {
        fn as_ref(&self) -> &TPMEmulatorOptions {
            &self.data
        }
    }
impl ::std::ops::Deref for TPMEmulatorOptionsWrapper {
    type Target = TPMEmulatorOptions;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl TPMEmulatorOptionsWrapper {
    pub fn into_inner(self) -> TPMEmulatorOptions {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMInfo {
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "model")]
pub model: TpmModel,
#[serde(rename = "options")]
pub options: TpmTypeOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TPMPassthroughOptions {
#[serde(rename = "cancel-path", default, skip_serializing_if = "Option::is_none")]
pub cancel_path: Option<::std::string::String>,
#[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
pub path: Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct TPMPassthroughOptionsWrapper {
#[serde(rename = "data")]
pub data: TPMPassthroughOptions,
}

impl<T: Into<TPMPassthroughOptions>> From<T> for TPMPassthroughOptionsWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<TPMPassthroughOptions> for TPMPassthroughOptionsWrapper {
        fn as_ref(&self) -> &TPMPassthroughOptions {
            &self.data
        }
    }
impl ::std::ops::Deref for TPMPassthroughOptionsWrapper {
    type Target = TPMPassthroughOptions;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl TPMPassthroughOptionsWrapper {
    pub fn into_inner(self) -> TPMPassthroughOptions {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
#[serde(rename = "arch")]
pub arch: SysEmuTarget,
}

impl<T: Into<SysEmuTarget>> From<T> for TargetInfo {
    fn from(val: T) -> Self {
        Self {
            arch: val.into(),

        }
    }
}
    impl AsRef<SysEmuTarget> for TargetInfo {
        fn as_ref(&self) -> &SysEmuTarget {
            &self.arch
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadContextProperties {
#[serde(rename = "cpu-affinity", default, skip_serializing_if = "Option::is_none")]
pub cpu_affinity: Option<Vec<u16>>,
#[serde(rename = "node-affinity", default, skip_serializing_if = "Option::is_none")]
pub node_affinity: Option<Vec<u16>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThrottleGroupProperties {
#[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
pub limits: Option<ThrottleLimits>,
#[serde(rename = "x-bps-read", default, skip_serializing_if = "Option::is_none")]
pub x_bps_read: Option<i64>,
#[serde(rename = "x-bps-read-max", default, skip_serializing_if = "Option::is_none")]
pub x_bps_read_max: Option<i64>,
#[serde(rename = "x-bps-read-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_bps_read_max_length: Option<i64>,
#[serde(rename = "x-bps-total", default, skip_serializing_if = "Option::is_none")]
pub x_bps_total: Option<i64>,
#[serde(rename = "x-bps-total-max", default, skip_serializing_if = "Option::is_none")]
pub x_bps_total_max: Option<i64>,
#[serde(rename = "x-bps-total-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_bps_total_max_length: Option<i64>,
#[serde(rename = "x-bps-write", default, skip_serializing_if = "Option::is_none")]
pub x_bps_write: Option<i64>,
#[serde(rename = "x-bps-write-max", default, skip_serializing_if = "Option::is_none")]
pub x_bps_write_max: Option<i64>,
#[serde(rename = "x-bps-write-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_bps_write_max_length: Option<i64>,
#[serde(rename = "x-iops-read", default, skip_serializing_if = "Option::is_none")]
pub x_iops_read: Option<i64>,
#[serde(rename = "x-iops-read-max", default, skip_serializing_if = "Option::is_none")]
pub x_iops_read_max: Option<i64>,
#[serde(rename = "x-iops-read-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_iops_read_max_length: Option<i64>,
#[serde(rename = "x-iops-size", default, skip_serializing_if = "Option::is_none")]
pub x_iops_size: Option<i64>,
#[serde(rename = "x-iops-total", default, skip_serializing_if = "Option::is_none")]
pub x_iops_total: Option<i64>,
#[serde(rename = "x-iops-total-max", default, skip_serializing_if = "Option::is_none")]
pub x_iops_total_max: Option<i64>,
#[serde(rename = "x-iops-total-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_iops_total_max_length: Option<i64>,
#[serde(rename = "x-iops-write", default, skip_serializing_if = "Option::is_none")]
pub x_iops_write: Option<i64>,
#[serde(rename = "x-iops-write-max", default, skip_serializing_if = "Option::is_none")]
pub x_iops_write_max: Option<i64>,
#[serde(rename = "x-iops-write-max-length", default, skip_serializing_if = "Option::is_none")]
pub x_iops_write_max_length: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThrottleLimits {
#[serde(rename = "bps-read", default, skip_serializing_if = "Option::is_none")]
pub bps_read: Option<i64>,
#[serde(rename = "bps-read-max", default, skip_serializing_if = "Option::is_none")]
pub bps_read_max: Option<i64>,
#[serde(rename = "bps-read-max-length", default, skip_serializing_if = "Option::is_none")]
pub bps_read_max_length: Option<i64>,
#[serde(rename = "bps-total", default, skip_serializing_if = "Option::is_none")]
pub bps_total: Option<i64>,
#[serde(rename = "bps-total-max", default, skip_serializing_if = "Option::is_none")]
pub bps_total_max: Option<i64>,
#[serde(rename = "bps-total-max-length", default, skip_serializing_if = "Option::is_none")]
pub bps_total_max_length: Option<i64>,
#[serde(rename = "bps-write", default, skip_serializing_if = "Option::is_none")]
pub bps_write: Option<i64>,
#[serde(rename = "bps-write-max", default, skip_serializing_if = "Option::is_none")]
pub bps_write_max: Option<i64>,
#[serde(rename = "bps-write-max-length", default, skip_serializing_if = "Option::is_none")]
pub bps_write_max_length: Option<i64>,
#[serde(rename = "iops-read", default, skip_serializing_if = "Option::is_none")]
pub iops_read: Option<i64>,
#[serde(rename = "iops-read-max", default, skip_serializing_if = "Option::is_none")]
pub iops_read_max: Option<i64>,
#[serde(rename = "iops-read-max-length", default, skip_serializing_if = "Option::is_none")]
pub iops_read_max_length: Option<i64>,
#[serde(rename = "iops-size", default, skip_serializing_if = "Option::is_none")]
pub iops_size: Option<i64>,
#[serde(rename = "iops-total", default, skip_serializing_if = "Option::is_none")]
pub iops_total: Option<i64>,
#[serde(rename = "iops-total-max", default, skip_serializing_if = "Option::is_none")]
pub iops_total_max: Option<i64>,
#[serde(rename = "iops-total-max-length", default, skip_serializing_if = "Option::is_none")]
pub iops_total_max_length: Option<i64>,
#[serde(rename = "iops-write", default, skip_serializing_if = "Option::is_none")]
pub iops_write: Option<i64>,
#[serde(rename = "iops-write-max", default, skip_serializing_if = "Option::is_none")]
pub iops_write_max: Option<i64>,
#[serde(rename = "iops-write-max-length", default, skip_serializing_if = "Option::is_none")]
pub iops_write_max_length: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsCredsAnonProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: TlsCredsProperties,
#[serde(rename = "loaded", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub loaded: Option<bool>,
}

impl<T: Into<TlsCredsProperties>> From<T> for TlsCredsAnonProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
loaded: Default::default(),

        }
    }
}
    impl AsRef<TlsCredsProperties> for TlsCredsAnonProperties {
        fn as_ref(&self) -> &TlsCredsProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsCredsProperties {
#[serde(rename = "dir", default, skip_serializing_if = "Option::is_none")]
pub dir: Option<::std::string::String>,
#[serde(rename = "endpoint", default, skip_serializing_if = "Option::is_none")]
pub endpoint: Option<QCryptoTLSCredsEndpoint>,
#[serde(rename = "priority", default, skip_serializing_if = "Option::is_none")]
pub priority: Option<::std::string::String>,
#[serde(rename = "verify-peer", default, skip_serializing_if = "Option::is_none")]
pub verify_peer: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsCredsPskProperties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: TlsCredsProperties,
#[serde(rename = "loaded", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub loaded: Option<bool>,
#[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
pub username: Option<::std::string::String>,
}

impl<T: Into<TlsCredsProperties>> From<T> for TlsCredsPskProperties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
loaded: Default::default(),
username: Default::default(),

        }
    }
}
    impl AsRef<TlsCredsProperties> for TlsCredsPskProperties {
        fn as_ref(&self) -> &TlsCredsProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsCredsX509Properties {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: TlsCredsProperties,
#[serde(rename = "loaded", default, skip_serializing_if = "Option::is_none")] #[deprecated]
pub loaded: Option<bool>,
#[serde(rename = "passwordid", default, skip_serializing_if = "Option::is_none")]
pub passwordid: Option<::std::string::String>,
#[serde(rename = "sanity-check", default, skip_serializing_if = "Option::is_none")]
pub sanity_check: Option<bool>,
}

impl<T: Into<TlsCredsProperties>> From<T> for TlsCredsX509Properties {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
loaded: Default::default(),
passwordid: Default::default(),
sanity_check: Default::default(),

        }
    }
}
    impl AsRef<TlsCredsProperties> for TlsCredsX509Properties {
        fn as_ref(&self) -> &TlsCredsProperties {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEventInfo {
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "state")]
pub state: TraceEventState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionProperties {
#[serde(rename = "completion-mode", default, skip_serializing_if = "Option::is_none")]
pub completion_mode: Option<ActionCompletionMode>,
}

impl<T: Into<ActionCompletionMode>> From<T> for TransactionProperties {
    fn from(val: T) -> Self {
        Self {
            completion_mode: val.into().into(),

        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnixSocketAddress {
#[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
pub abstract_: Option<bool>,
#[serde(rename = "tight", default, skip_serializing_if = "Option::is_none")]
pub tight: Option<bool>,
#[serde(rename = "path")]
pub path: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct UnixSocketAddressWrapper {
#[serde(rename = "data")]
pub data: UnixSocketAddress,
}

impl<T: Into<UnixSocketAddress>> From<T> for UnixSocketAddressWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<UnixSocketAddress> for UnixSocketAddressWrapper {
        fn as_ref(&self) -> &UnixSocketAddress {
            &self.data
        }
    }
impl ::std::ops::Deref for UnixSocketAddressWrapper {
    type Target = UnixSocketAddress;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl UnixSocketAddressWrapper {
    pub fn into_inner(self) -> UnixSocketAddress {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidInfo {
#[serde(rename = "UUID")]
pub UUID: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for UuidInfo {
    fn from(val: T) -> Self {
        Self {
            UUID: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for UuidInfo {
        fn as_ref(&self) -> &::std::string::String {
            &self.UUID
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
#[serde(rename = "package")]
pub package: ::std::string::String,
#[serde(rename = "qemu")]
pub qemu: VersionTriple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionTriple {
#[serde(rename = "major")]
pub major: i64,
#[serde(rename = "micro")]
pub micro: i64,
#[serde(rename = "minor")]
pub minor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfioStats {
#[serde(rename = "transferred")]
pub transferred: i64,
}

impl<T: Into<i64>> From<T> for VfioStats {
    fn from(val: T) -> Self {
        Self {
            transferred: val.into(),

        }
    }
}
    impl AsRef<i64> for VfioStats {
        fn as_ref(&self) -> &i64 {
            &self.transferred
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfioUserServerProperties {
#[serde(rename = "device")]
pub device: ::std::string::String,
#[serde(rename = "socket")]
pub socket: SocketAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VhostDeviceProtocols {
#[serde(rename = "unknown-protocols", default, skip_serializing_if = "Option::is_none")]
pub unknown_protocols: Option<u64>,
#[serde(rename = "protocols")]
pub protocols: Vec<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VhostStatus {
#[serde(rename = "acked-features")]
pub acked_features: VirtioDeviceFeatures,
#[serde(rename = "backend-cap")]
pub backend_cap: u64,
#[serde(rename = "backend-features")]
pub backend_features: VirtioDeviceFeatures,
#[serde(rename = "features")]
pub features: VirtioDeviceFeatures,
#[serde(rename = "log-enabled")]
pub log_enabled: bool,
#[serde(rename = "log-size")]
pub log_size: u64,
#[serde(rename = "max-queues")]
pub max_queues: u64,
#[serde(rename = "n-mem-sections")]
pub n_mem_sections: i64,
#[serde(rename = "n-tmp-sections")]
pub n_tmp_sections: i64,
#[serde(rename = "nvqs")]
pub nvqs: u32,
#[serde(rename = "protocol-features")]
pub protocol_features: VhostDeviceProtocols,
#[serde(rename = "vq-index")]
pub vq_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtQueueStatus {
#[serde(rename = "last-avail-idx", default, skip_serializing_if = "Option::is_none")]
pub last_avail_idx: Option<u16>,
#[serde(rename = "shadow-avail-idx", default, skip_serializing_if = "Option::is_none")]
pub shadow_avail_idx: Option<u16>,
#[serde(rename = "inuse")]
pub inuse: u32,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "queue-index")]
pub queue_index: u16,
#[serde(rename = "signalled-used")]
pub signalled_used: u16,
#[serde(rename = "signalled-used-valid")]
pub signalled_used_valid: bool,
#[serde(rename = "used-idx")]
pub used_idx: u16,
#[serde(rename = "vring-align")]
pub vring_align: u32,
#[serde(rename = "vring-avail")]
pub vring_avail: u64,
#[serde(rename = "vring-desc")]
pub vring_desc: u64,
#[serde(rename = "vring-num")]
pub vring_num: u32,
#[serde(rename = "vring-num-default")]
pub vring_num_default: u32,
#[serde(rename = "vring-used")]
pub vring_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtVhostQueueStatus {
#[serde(rename = "avail")]
pub avail: u64,
#[serde(rename = "avail-phys")]
pub avail_phys: u64,
#[serde(rename = "avail-size")]
pub avail_size: u32,
#[serde(rename = "call")]
pub call: i64,
#[serde(rename = "desc")]
pub desc: u64,
#[serde(rename = "desc-phys")]
pub desc_phys: u64,
#[serde(rename = "desc-size")]
pub desc_size: u32,
#[serde(rename = "kick")]
pub kick: i64,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "num")]
pub num: i64,
#[serde(rename = "used")]
pub used: u64,
#[serde(rename = "used-phys")]
pub used_phys: u64,
#[serde(rename = "used-size")]
pub used_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioDeviceFeatures {
#[serde(rename = "dev-features", default, skip_serializing_if = "Option::is_none")]
pub dev_features: Option<Vec<::std::string::String>>,
#[serde(rename = "unknown-dev-features", default, skip_serializing_if = "Option::is_none")]
pub unknown_dev_features: Option<u64>,
#[serde(rename = "transports")]
pub transports: Vec<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioDeviceStatus {
#[serde(rename = "unknown-statuses", default, skip_serializing_if = "Option::is_none")]
pub unknown_statuses: Option<u8>,
#[serde(rename = "statuses")]
pub statuses: Vec<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioInfo {
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "path")]
pub path: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioMEMDeviceInfo {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "block-size")]
pub block_size: u64,
#[serde(rename = "max-size")]
pub max_size: u64,
#[serde(rename = "memaddr")]
pub memaddr: u64,
#[serde(rename = "memdev")]
pub memdev: ::std::string::String,
#[serde(rename = "node")]
pub node: i64,
#[serde(rename = "requested-size")]
pub requested_size: u64,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct VirtioMEMDeviceInfoWrapper {
#[serde(rename = "data")]
pub data: VirtioMEMDeviceInfo,
}

impl<T: Into<VirtioMEMDeviceInfo>> From<T> for VirtioMEMDeviceInfoWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<VirtioMEMDeviceInfo> for VirtioMEMDeviceInfoWrapper {
        fn as_ref(&self) -> &VirtioMEMDeviceInfo {
            &self.data
        }
    }
impl ::std::ops::Deref for VirtioMEMDeviceInfoWrapper {
    type Target = VirtioMEMDeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl VirtioMEMDeviceInfoWrapper {
    pub fn into_inner(self) -> VirtioMEMDeviceInfo {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioPMEMDeviceInfo {
#[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
pub id: Option<::std::string::String>,
#[serde(rename = "memaddr")]
pub memaddr: u64,
#[serde(rename = "memdev")]
pub memdev: ::std::string::String,
#[serde(rename = "size")]
pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct VirtioPMEMDeviceInfoWrapper {
#[serde(rename = "data")]
pub data: VirtioPMEMDeviceInfo,
}

impl<T: Into<VirtioPMEMDeviceInfo>> From<T> for VirtioPMEMDeviceInfoWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<VirtioPMEMDeviceInfo> for VirtioPMEMDeviceInfoWrapper {
        fn as_ref(&self) -> &VirtioPMEMDeviceInfo {
            &self.data
        }
    }
impl ::std::ops::Deref for VirtioPMEMDeviceInfoWrapper {
    type Target = VirtioPMEMDeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl VirtioPMEMDeviceInfoWrapper {
    pub fn into_inner(self) -> VirtioPMEMDeviceInfo {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioQueueElement {
#[serde(rename = "avail")]
pub avail: VirtioRingAvail,
#[serde(rename = "descs")]
pub descs: Vec<VirtioRingDesc>,
#[serde(rename = "index")]
pub index: u32,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "used")]
pub used: VirtioRingUsed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioRingAvail {
#[serde(rename = "flags")]
pub flags: u16,
#[serde(rename = "idx")]
pub idx: u16,
#[serde(rename = "ring")]
pub ring: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioRingDesc {
#[serde(rename = "addr")]
pub addr: u64,
#[serde(rename = "flags")]
pub flags: Vec<::std::string::String>,
#[serde(rename = "len")]
pub len: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioRingUsed {
#[serde(rename = "flags")]
pub flags: u16,
#[serde(rename = "idx")]
pub idx: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtioStatus {
#[serde(rename = "vhost-dev", default, skip_serializing_if = "Option::is_none")]
pub vhost_dev: Option<VhostStatus>,
#[serde(rename = "backend-features")]
pub backend_features: VirtioDeviceFeatures,
#[serde(rename = "broken")]
pub broken: bool,
#[serde(rename = "bus-name")]
pub bus_name: ::std::string::String,
#[serde(rename = "device-endian")]
pub device_endian: ::std::string::String,
#[serde(rename = "device-id")]
pub device_id: u16,
#[serde(rename = "disable-legacy-check")]
pub disable_legacy_check: bool,
#[serde(rename = "disabled")]
pub disabled: bool,
#[serde(rename = "guest-features")]
pub guest_features: VirtioDeviceFeatures,
#[serde(rename = "host-features")]
pub host_features: VirtioDeviceFeatures,
#[serde(rename = "isr")]
pub isr: u8,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "num-vqs")]
pub num_vqs: i64,
#[serde(rename = "queue-sel")]
pub queue_sel: u16,
#[serde(rename = "start-on-kick")]
pub start_on_kick: bool,
#[serde(rename = "started")]
pub started: bool,
#[serde(rename = "status")]
pub status: VirtioDeviceStatus,
#[serde(rename = "use-guest-notifier-mask")]
pub use_guest_notifier_mask: bool,
#[serde(rename = "use-started")]
pub use_started: bool,
#[serde(rename = "vhost-started")]
pub vhost_started: bool,
#[serde(rename = "vm-running")]
pub vm_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmdkExtentInfo {
#[serde(rename = "cluster-size", default, skip_serializing_if = "Option::is_none")]
pub cluster_size: Option<i64>,
#[serde(rename = "compressed", default, skip_serializing_if = "Option::is_none")]
pub compressed: Option<bool>,
#[serde(rename = "filename")]
pub filename: ::std::string::String,
#[serde(rename = "format")]
pub format: ::std::string::String,
#[serde(rename = "virtual-size")]
pub virtual_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncBasicInfo {
#[serde(rename = "family")]
pub family: NetworkAddressFamily,
#[serde(rename = "host")]
pub host: ::std::string::String,
#[serde(rename = "service")]
pub service: ::std::string::String,
#[serde(rename = "websocket")]
pub websocket: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncClientInfo {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: VncBasicInfo,
#[serde(rename = "sasl_username", default, skip_serializing_if = "Option::is_none")]
pub sasl_username: Option<::std::string::String>,
#[serde(rename = "x509_dname", default, skip_serializing_if = "Option::is_none")]
pub x509_dname: Option<::std::string::String>,
}

impl<T: Into<VncBasicInfo>> From<T> for VncClientInfo {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
sasl_username: Default::default(),
x509_dname: Default::default(),

        }
    }
}
    impl AsRef<VncBasicInfo> for VncClientInfo {
        fn as_ref(&self) -> &VncBasicInfo {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncInfo {
#[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
pub auth: Option<::std::string::String>,
#[serde(rename = "clients", default, skip_serializing_if = "Option::is_none")]
pub clients: Option<Vec<VncClientInfo>>,
#[serde(rename = "family", default, skip_serializing_if = "Option::is_none")]
pub family: Option<NetworkAddressFamily>,
#[serde(rename = "host", default, skip_serializing_if = "Option::is_none")]
pub host: Option<::std::string::String>,
#[serde(rename = "service", default, skip_serializing_if = "Option::is_none")]
pub service: Option<::std::string::String>,
#[serde(rename = "enabled")]
pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncInfo2 {
#[serde(rename = "display", default, skip_serializing_if = "Option::is_none")]
pub display: Option<::std::string::String>,
#[serde(rename = "vencrypt", default, skip_serializing_if = "Option::is_none")]
pub vencrypt: Option<VncVencryptSubAuth>,
#[serde(rename = "auth")]
pub auth: VncPrimaryAuth,
#[serde(rename = "clients")]
pub clients: Vec<VncClientInfo>,
#[serde(rename = "id")]
pub id: ::std::string::String,
#[serde(rename = "server")]
pub server: Vec<VncServerInfo2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncServerInfo {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: VncBasicInfo,
#[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
pub auth: Option<::std::string::String>,
}

impl<T: Into<VncBasicInfo>> From<T> for VncServerInfo {
    fn from(val: T) -> Self {
        Self {
            base: val.into(),
auth: Default::default(),

        }
    }
}
    impl AsRef<VncBasicInfo> for VncServerInfo {
        fn as_ref(&self) -> &VncBasicInfo {
            &self.base
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncServerInfo2 {
#[serde(flatten)]
#[serde(rename = "base")]
pub base: VncBasicInfo,
#[serde(rename = "vencrypt", default, skip_serializing_if = "Option::is_none")]
pub vencrypt: Option<VncVencryptSubAuth>,
#[serde(rename = "auth")]
pub auth: VncPrimaryAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsockSocketAddress {
#[serde(rename = "cid")]
pub cid: ::std::string::String,
#[serde(rename = "port")]
pub port: ::std::string::String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]#[repr(transparent)]
pub struct VsockSocketAddressWrapper {
#[serde(rename = "data")]
pub data: VsockSocketAddress,
}

impl<T: Into<VsockSocketAddress>> From<T> for VsockSocketAddressWrapper {
    fn from(val: T) -> Self {
        Self {
            data: val.into(),

        }
    }
}
    impl AsRef<VsockSocketAddress> for VsockSocketAddressWrapper {
        fn as_ref(&self) -> &VsockSocketAddress {
            &self.data
        }
    }
impl ::std::ops::Deref for VsockSocketAddressWrapper {
    type Target = VsockSocketAddress;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl VsockSocketAddressWrapper {
    pub fn into_inner(self) -> VsockSocketAddress {
        self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X86CPUFeatureWordInfo {
#[serde(rename = "cpuid-input-ecx", default, skip_serializing_if = "Option::is_none")]
pub cpuid_input_ecx: Option<i64>,
#[serde(rename = "cpuid-input-eax")]
pub cpuid_input_eax: i64,
#[serde(rename = "cpuid-register")]
pub cpuid_register: X86CPURegister32,
#[serde(rename = "features")]
pub features: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XBZRLECacheStats {
#[serde(rename = "bytes")]
pub bytes: i64,
#[serde(rename = "cache-miss")]
pub cache_miss: i64,
#[serde(rename = "cache-miss-rate")]
pub cache_miss_rate: f64,
#[serde(rename = "cache-size")]
pub cache_size: u64,
#[serde(rename = "encoding-rate")]
pub encoding_rate: f64,
#[serde(rename = "overflow")]
pub overflow: i64,
#[serde(rename = "pages")]
pub pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XDbgBlockGraph {
#[serde(rename = "edges")]
pub edges: Vec<XDbgBlockGraphEdge>,
#[serde(rename = "nodes")]
pub nodes: Vec<XDbgBlockGraphNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XDbgBlockGraphEdge {
#[serde(rename = "child")]
pub child: u64,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "parent")]
pub parent: u64,
#[serde(rename = "perm")]
pub perm: Vec<BlockPermission>,
#[serde(rename = "shared-perm")]
pub shared_perm: Vec<BlockPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XDbgBlockGraphNode {
#[serde(rename = "id")]
pub id: u64,
#[serde(rename = "name")]
pub name: ::std::string::String,
#[serde(rename = "type")]
pub type_: XDbgBlockGraphNodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YankInstanceBlockNode {
#[serde(rename = "node-name")]
pub node_name: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for YankInstanceBlockNode {
    fn from(val: T) -> Self {
        Self {
            node_name: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for YankInstanceBlockNode {
        fn as_ref(&self) -> &::std::string::String {
            &self.node_name
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YankInstanceChardev {
#[serde(rename = "id")]
pub id: ::std::string::String,
}

impl<T: Into<::std::string::String>> From<T> for YankInstanceChardev {
    fn from(val: T) -> Self {
        Self {
            id: val.into(),

        }
    }
}
    impl AsRef<::std::string::String> for YankInstanceChardev {
        fn as_ref(&self) -> &::std::string::String {
            &self.id
        }
    }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
	#[serde(rename = "SHUTDOWN")] SHUTDOWN {
         data: SHUTDOWN,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "POWERDOWN")] POWERDOWN {
        #[serde(default)]  data: POWERDOWN,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "RESET")] RESET {
         data: RESET,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "STOP")] STOP {
        #[serde(default)]  data: STOP,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "RESUME")] RESUME {
        #[serde(default)]  data: RESUME,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SUSPEND")] SUSPEND {
        #[serde(default)]  data: SUSPEND,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SUSPEND_DISK")] SUSPEND_DISK {
        #[serde(default)]  data: SUSPEND_DISK,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "WAKEUP")] WAKEUP {
        #[serde(default)]  data: WAKEUP,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "WATCHDOG")] WATCHDOG {
         data: WATCHDOG,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "GUEST_PANICKED")] GUEST_PANICKED {
         data: GUEST_PANICKED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "GUEST_CRASHLOADED")] GUEST_CRASHLOADED {
         data: GUEST_CRASHLOADED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "GUEST_PVSHUTDOWN")] GUEST_PVSHUTDOWN {
        #[serde(default)]  data: GUEST_PVSHUTDOWN,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "MEMORY_FAILURE")] MEMORY_FAILURE {
         data: MEMORY_FAILURE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "JOB_STATUS_CHANGE")] JOB_STATUS_CHANGE {
         data: JOB_STATUS_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "DEVICE_TRAY_MOVED")] DEVICE_TRAY_MOVED {
         data: DEVICE_TRAY_MOVED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "PR_MANAGER_STATUS_CHANGED")] PR_MANAGER_STATUS_CHANGED {
         data: PR_MANAGER_STATUS_CHANGED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_IMAGE_CORRUPTED")] BLOCK_IMAGE_CORRUPTED {
         data: BLOCK_IMAGE_CORRUPTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_IO_ERROR")] BLOCK_IO_ERROR {
         data: BLOCK_IO_ERROR,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_JOB_COMPLETED")] BLOCK_JOB_COMPLETED {
         data: BLOCK_JOB_COMPLETED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_JOB_CANCELLED")] BLOCK_JOB_CANCELLED {
         data: BLOCK_JOB_CANCELLED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_JOB_ERROR")] BLOCK_JOB_ERROR {
         data: BLOCK_JOB_ERROR,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_JOB_READY")] BLOCK_JOB_READY {
         data: BLOCK_JOB_READY,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_JOB_PENDING")] BLOCK_JOB_PENDING {
         data: BLOCK_JOB_PENDING,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_WRITE_THRESHOLD")] BLOCK_WRITE_THRESHOLD {
         data: BLOCK_WRITE_THRESHOLD,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "QUORUM_FAILURE")] QUORUM_FAILURE {
         data: QUORUM_FAILURE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "QUORUM_REPORT_BAD")] QUORUM_REPORT_BAD {
         data: QUORUM_REPORT_BAD,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BLOCK_EXPORT_DELETED")] BLOCK_EXPORT_DELETED {
         data: BLOCK_EXPORT_DELETED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VSERPORT_CHANGE")] VSERPORT_CHANGE {
         data: VSERPORT_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "DUMP_COMPLETED")] DUMP_COMPLETED {
         data: DUMP_COMPLETED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "NIC_RX_FILTER_CHANGED")] NIC_RX_FILTER_CHANGED {
         data: NIC_RX_FILTER_CHANGED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "FAILOVER_NEGOTIATED")] FAILOVER_NEGOTIATED {
         data: FAILOVER_NEGOTIATED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "NETDEV_STREAM_CONNECTED")] NETDEV_STREAM_CONNECTED {
         data: NETDEV_STREAM_CONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "NETDEV_STREAM_DISCONNECTED")] NETDEV_STREAM_DISCONNECTED {
         data: NETDEV_STREAM_DISCONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SPICE_CONNECTED")] SPICE_CONNECTED {
         data: SPICE_CONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SPICE_INITIALIZED")] SPICE_INITIALIZED {
         data: SPICE_INITIALIZED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SPICE_DISCONNECTED")] SPICE_DISCONNECTED {
         data: SPICE_DISCONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "SPICE_MIGRATE_COMPLETED")] SPICE_MIGRATE_COMPLETED {
        #[serde(default)]  data: SPICE_MIGRATE_COMPLETED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VNC_CONNECTED")] VNC_CONNECTED {
         data: VNC_CONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VNC_INITIALIZED")] VNC_INITIALIZED {
         data: VNC_INITIALIZED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VNC_DISCONNECTED")] VNC_DISCONNECTED {
         data: VNC_DISCONNECTED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "MIGRATION")] MIGRATION {
         data: MIGRATION,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "MIGRATION_PASS")] MIGRATION_PASS {
         data: MIGRATION_PASS,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "COLO_EXIT")] COLO_EXIT {
         data: COLO_EXIT,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "UNPLUG_PRIMARY")] UNPLUG_PRIMARY {
         data: UNPLUG_PRIMARY,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "DEVICE_DELETED")] DEVICE_DELETED {
         data: DEVICE_DELETED,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "DEVICE_UNPLUG_GUEST_ERROR")] DEVICE_UNPLUG_GUEST_ERROR {
         data: DEVICE_UNPLUG_GUEST_ERROR,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "BALLOON_CHANGE")] BALLOON_CHANGE {
         data: BALLOON_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "HV_BALLOON_STATUS_REPORT")] HV_BALLOON_STATUS_REPORT {
         data: HV_BALLOON_STATUS_REPORT,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "MEMORY_DEVICE_SIZE_CHANGE")] MEMORY_DEVICE_SIZE_CHANGE {
         data: MEMORY_DEVICE_SIZE_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "CPU_POLARIZATION_CHANGE")] CPU_POLARIZATION_CHANGE {
         data: CPU_POLARIZATION_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "RTC_CHANGE")] RTC_CHANGE {
         data: RTC_CHANGE,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VFU_CLIENT_HANGUP")] VFU_CLIENT_HANGUP {
         data: VFU_CLIENT_HANGUP,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "ACPI_DEVICE_OST")] ACPI_DEVICE_OST {
         data: ACPI_DEVICE_OST,
        timestamp: ::qapi_spec::Timestamp,
    },
	#[serde(rename = "VFIO_MIGRATION")] VFIO_MIGRATION {
         data: VFIO_MIGRATION,
        timestamp: ::qapi_spec::Timestamp,
    },
}

impl Event {
    pub fn timestamp(&self) -> ::qapi_spec::Timestamp {
        match *self {
Event::SHUTDOWN { timestamp, .. } => timestamp,
Event::POWERDOWN { timestamp, .. } => timestamp,
Event::RESET { timestamp, .. } => timestamp,
Event::STOP { timestamp, .. } => timestamp,
Event::RESUME { timestamp, .. } => timestamp,
Event::SUSPEND { timestamp, .. } => timestamp,
Event::SUSPEND_DISK { timestamp, .. } => timestamp,
Event::WAKEUP { timestamp, .. } => timestamp,
Event::WATCHDOG { timestamp, .. } => timestamp,
Event::GUEST_PANICKED { timestamp, .. } => timestamp,
Event::GUEST_CRASHLOADED { timestamp, .. } => timestamp,
Event::GUEST_PVSHUTDOWN { timestamp, .. } => timestamp,
Event::MEMORY_FAILURE { timestamp, .. } => timestamp,
Event::JOB_STATUS_CHANGE { timestamp, .. } => timestamp,
Event::DEVICE_TRAY_MOVED { timestamp, .. } => timestamp,
Event::PR_MANAGER_STATUS_CHANGED { timestamp, .. } => timestamp,
Event::BLOCK_IMAGE_CORRUPTED { timestamp, .. } => timestamp,
Event::BLOCK_IO_ERROR { timestamp, .. } => timestamp,
Event::BLOCK_JOB_COMPLETED { timestamp, .. } => timestamp,
Event::BLOCK_JOB_CANCELLED { timestamp, .. } => timestamp,
Event::BLOCK_JOB_ERROR { timestamp, .. } => timestamp,
Event::BLOCK_JOB_READY { timestamp, .. } => timestamp,
Event::BLOCK_JOB_PENDING { timestamp, .. } => timestamp,
Event::BLOCK_WRITE_THRESHOLD { timestamp, .. } => timestamp,
Event::QUORUM_FAILURE { timestamp, .. } => timestamp,
Event::QUORUM_REPORT_BAD { timestamp, .. } => timestamp,
Event::BLOCK_EXPORT_DELETED { timestamp, .. } => timestamp,
Event::VSERPORT_CHANGE { timestamp, .. } => timestamp,
Event::DUMP_COMPLETED { timestamp, .. } => timestamp,
Event::NIC_RX_FILTER_CHANGED { timestamp, .. } => timestamp,
Event::FAILOVER_NEGOTIATED { timestamp, .. } => timestamp,
Event::NETDEV_STREAM_CONNECTED { timestamp, .. } => timestamp,
Event::NETDEV_STREAM_DISCONNECTED { timestamp, .. } => timestamp,
Event::SPICE_CONNECTED { timestamp, .. } => timestamp,
Event::SPICE_INITIALIZED { timestamp, .. } => timestamp,
Event::SPICE_DISCONNECTED { timestamp, .. } => timestamp,
Event::SPICE_MIGRATE_COMPLETED { timestamp, .. } => timestamp,
Event::VNC_CONNECTED { timestamp, .. } => timestamp,
Event::VNC_INITIALIZED { timestamp, .. } => timestamp,
Event::VNC_DISCONNECTED { timestamp, .. } => timestamp,
Event::MIGRATION { timestamp, .. } => timestamp,
Event::MIGRATION_PASS { timestamp, .. } => timestamp,
Event::COLO_EXIT { timestamp, .. } => timestamp,
Event::UNPLUG_PRIMARY { timestamp, .. } => timestamp,
Event::DEVICE_DELETED { timestamp, .. } => timestamp,
Event::DEVICE_UNPLUG_GUEST_ERROR { timestamp, .. } => timestamp,
Event::BALLOON_CHANGE { timestamp, .. } => timestamp,
Event::HV_BALLOON_STATUS_REPORT { timestamp, .. } => timestamp,
Event::MEMORY_DEVICE_SIZE_CHANGE { timestamp, .. } => timestamp,
Event::CPU_POLARIZATION_CHANGE { timestamp, .. } => timestamp,
Event::RTC_CHANGE { timestamp, .. } => timestamp,
Event::VFU_CLIENT_HANGUP { timestamp, .. } => timestamp,
Event::ACPI_DEVICE_OST { timestamp, .. } => timestamp,
Event::VFIO_MIGRATION { timestamp, .. } => timestamp,

        }
    }
}
