use crate::config_utils::{get_config_as_object, save_config_to_file};
use crate::log_utils::{log_error_to_console, log_info_to_console, log_success_to_console};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process::exit;

/// Adds a new target and the platform that the target corresponds to in Godot.
///
/// A target only needs to be added if you are trying to target something other
/// than your native target. For example, if you are developing on 64-bit
/// windows and you just want to support 64-bit windows, then you don't need to
/// provide a target as godot-rust-cli will use the default target.
///
/// The target should be a possible architecture that the library can be built
/// for. Since cross-compliation is a tricky matter, godot-rust-cli uses
/// [cross](https://github.com/rust-embedded/cross) to do so. This means that
/// the target you provide must be supported by cross. In addition, to limit
/// the things that can go wrong, godot-rust-cli limits the targets within that
/// list. As targets are requested they can be added, tested, and supported.
/// The list of valid targets is provided below.
///
/// Note: If you have multiple team members working on a project and not all of
/// them have the ability to cross compile then godot-rust-cli will just compile
/// for the native target and skip any cross compilation.
///
/// Targets - Platforms
/// armv7-linux-androideabi - Android.armeabi-v7a
/// aarch64-linux-android - Android.arm64-v8a
/// i686-linux-android - Android.x86
/// x86_64-linux-android - Android.x86_64
/// x86_64-pc-windows-gnu - Windows.64
/// i686-pc-windows-gnu - Windows.32
/// x86_64-unknown-linux-gnu - Linux.64
/// i686-unknown-linux-gnu - Linux.32
///
/// # Arguments
///
/// `target` The target tuple. This must be one of the targets supported by the cross binary.
/// `overwrite_existing_target` - Indicates whether an existing platform with the provided target should be overwritten or not.
pub fn add_target(target: &str, overwrite_existing_target: bool) {
    if VALID_TARGETS.contains_key(&target) {
        let target_platform = VALID_TARGETS
            .get(&target)
            .expect("Unable to get platform while adding target.");

        let mut config = get_config_as_object();

        // We need to check whether the config already contains a target for
        // the specified platform. If so, then the user will need to run the
        // command with the --overwrite flag to overwrite the existing target for
        // the platform.
        if config.targets.contains_key(target_platform.to_owned()) && !overwrite_existing_target {
            log_error_to_console("The config already contains a target for the platform that is associated with the target provided. If you would like to overwrite the platform in the config, pass the --overwrite flag to the command.");
            exit(1);
        } else {
            config
                .targets
                .insert(target_platform.to_string(), target.to_owned());

            log_success_to_console(&format!(
                "Target {} for platform {} added.",
                target, target_platform
            ));
            save_config_to_file(&mut config);
        }
    } else {
        log_error_to_console("The provided target isn't supported. Please file an issue to GitHub or the discord to ask for this target to be supported.");
        exit(1);
    }
}

/// Removes a target that was added to the list of targets that the library can
/// be built for.
///
/// # Arguments
///
/// `name` - The name of the target to remove.
pub fn remove_target(name: &str) {
    log_info_to_console(&format!("Adding target {} to the configuration.", &name));

    let mut config = get_config_as_object();

    if config.targets.contains_key(&name.to_owned()) {
        config.targets.remove(&name.to_owned());
        save_config_to_file(&mut config);

        log_success_to_console(&format!("Target {} added to the configuration.", &name));
    } else {
        // If the target to remove doesn't exist in the config, then let the
        // user know and return early.
        log_error_to_console(&format!(
            "The target {} doesn't exist in the configuration.",
            &name
        ));
        exit(1);
    }
}

/// Returns the user's native platform.
pub fn get_native_platform() -> String {
    let native_platform_os = match std::env::consts::OS {
        "windows" => "Windows",
        "linux" => "X11",
        "macos" => "OSX",
        _ => "X11",
    };
    let native_platform_arch = match std::env::consts::ARCH {
        "x86" => "x32",
        "x86_64" => "x64",
        _ => "x64",
    };
    return format!("{}.{}", native_platform_os, native_platform_arch);
}

lazy_static! {
    static ref VALID_TARGETS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("armv7-linux-androideabi", "Android.armeabi-v7a");
        m.insert("aarch64-linux-android", "Android.arm64-v8a");
        m.insert("i686-linux-android", "Android.x86");
        m.insert("x86_64-linux-android", "Android.x86_64");
        m.insert("x86_64-pc-windows-gnu", "Windows.64");
        m.insert("i686-pc-windows-gnu", "Windows.32");
        m.insert("x86_64-unknown-linux-gnu", "Linux.64");
        m.insert("i686-unknown-linux-gnu", "Linux.32");
        m
    };
}
