#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn is_has_admin_access() -> bool {
    windows::is_app_elevated().unwrap_or(false)
}

#[cfg(any(target_os = "linux"))]
pub fn is_has_admin_access() -> bool {
    !nix::unistd::geteuid().is_root()
}
