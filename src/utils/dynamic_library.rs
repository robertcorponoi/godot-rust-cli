/// Returns the build file extension for windows.
#[cfg(target_os = "windows")]
pub fn get_dynamic_library_ext() -> &'static str {
    return "dll";
}

/// Returns the build file extension for linux.
#[cfg(target_os = "linux")]
pub fn get_dynamic_library_ext() -> &'static str {
    return "so";
}

/// Returns the build file extension for macos.
#[cfg(target_os = "macos")]
pub fn get_dynamic_library_ext() -> &'static str {
    return "dylib";
}
