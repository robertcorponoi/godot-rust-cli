use crate::config_utils::{
    add_platform_to_config, get_config_as_object, remove_platform_from_config_if_exists,
};
use crate::cross_utils::add_image_override_if_necessary;
use crate::log_utils::log_error_to_console;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process::exit;

/// Adds a new platform to the platforms that godot-rust-cli will build the
/// library for.
///
/// Note that at this time godot-rust-cli only supports platforms with
/// 64-bit architectures. If demand for 32-bit is desired it can be added but
/// as of now only 64-bit is supported for any platform.
///
/// Platforms only need to be added if you are buliding for a different
/// platform than your native platform. For example, if you are developing
/// on Windows then you don't need to add Windows as a platfrom because the
/// library will automatically be built for your native platform.
///
/// Any platform other than your native platform will be built using the
/// `cross` crate from https://github.com/rust-embedded/cross. This means that
/// in order to cross-compile, you will need to follow the instructions for
/// setting it up which is essentially just installing the crate and making
/// sure that you have docker or podman.
///
/// The list of platforms that can be provided are:
/// Android
/// Linux
/// Windows
/// MacOS
///
/// If you would like another platform to be added then please open an issue in
/// the GitHub or let me know in the Discord.
///
/// # Arguments
///
/// `platform` - The platform to compile for.
pub fn add_platform(platform: &str) {
    let platform_normalized = platform.to_lowercase();

    if VALID_PLATFORMS.contains_key(&platform_normalized.as_str()) {
        let mut config = get_config_as_object();
        // Add the platform to the `platforms` array in the config.
        add_platform_to_config(&platform_normalized, &mut config);

        add_image_override_if_necessary(&platform_normalized);
    } else {
        log_error_to_console(&format!("The target {} isn't a valid target. Please file an issue in the GitHub or Discord if this is incorrect.", &platform));
        exit(1);
    }
}

/// Removes a platform from the configuration.
///
/// # Arguments
///
/// `platform` - The platform to remove.
pub fn remove_platform(platform: &str) {
    let mut config = get_config_as_object();
    remove_platform_from_config_if_exists(platform, &mut config);
}

lazy_static! {
    static ref VALID_PLATFORMS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("android.arm", "aarch64-linux-android");
        m.insert("android", "x86_64-linux-android");
        m.insert("windows", "x86_64-pc-windows-gnu");
        m.insert("linux", "x86_64-unknown-linux-gnu");
        // m.insert("macos", "x86_64-apple-darwin");
        m
    };
}
