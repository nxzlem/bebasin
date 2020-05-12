// Windows
#[cfg(target_os = "windows")]
pub const HOSTS_PATH: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts";
#[cfg(target_os = "windows")]
pub const HOSTS_BACKUP_PATH: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts-backup";

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub fn is_has_admin_access() -> bool {
    windows::is_app_elevated().unwrap_or(false)
}

// Linux
#[cfg(target_os = "linux")]
pub const HOSTS_PATH: &'static str = "/etc/hosts";
#[cfg(target_os = "linux")]
pub const HOSTS_BACKUP_PATH: &'static str = "/etc/hosts-backup";

// macos
#[cfg(target_os = "macos")]
pub const HOSTS_PATH: &'static str = "/private/etc/hosts";
#[cfg(target_os = "macos")]
pub const HOSTS_BACKUP_PATH: &'static str = "/private/etc/hosts-backup";

// *nix
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn is_has_admin_access() -> bool {
    !nix::unistd::geteuid().is_root()
}
