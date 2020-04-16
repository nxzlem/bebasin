#[cfg(target_os = "windows")]
pub const HOSTS_PATH: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts";
#[cfg(target_os = "windows")]
pub const HOSTS_BACKUP_PATH: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub fn is_has_admin_access() -> bool {
    windows::is_app_elevated().unwrap_or(false)
}

///home/andraantariksa/
#[cfg(target_os = "linux")]
pub const HOSTS_PATH: &'static str = "/home/andraantariksa/etc/hosts";
#[cfg(target_os = "linux")]
pub const HOSTS_BACKUP_PATH: &'static str = "/home/andraantariksa/etc/hosts.backup";

#[cfg(target_os = "linux")]
pub fn updated_application_path(filename: &str) -> String {
    format!("{}-updated", filename)
}

#[cfg(target_os = "linux")]
pub fn is_has_admin_access() -> bool {
    !nix::unistd::geteuid().is_root()
}
