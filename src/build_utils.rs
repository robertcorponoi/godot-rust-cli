use std::collections::HashMap;
use std::env::{consts, current_dir};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;

use chrono::offset::Local;
use lazy_static::lazy_static;
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};

use crate::log_utils::{log_error_to_console, log_info_to_console, log_success_to_console};

lazy_static! {
    pub static ref PLATFORM_TOOLCHAINS: HashMap<&'static str, &'static str> = {
        let mut toolchains = HashMap::new();
        toolchains.insert("android.arm", "aarch64-linux-android");
        toolchains.insert("android", "x86_64-linux-android");
        toolchains.insert("windows", "x86_64-pc-windows-gnu");
        toolchains.insert("linux", "x86_64-unknown-linux-gnu");
        toolchains.insert("macos", "x86_64-apple-darwin");
        toolchains
    };
}

/// Builds the library for the specified platform. If the platform is not the
/// user's native platform, then the `cross` command will be used.
///
/// # Arguments
///
/// `rust_library_name`             - The name of the Rust library.
/// `godot_project_absolute_path`   - The absolute path to the Godot project.
/// `platform`                      - The platform to target the build for.
/// `is_release`                    - Indicates whether the build is a release build or not. This is passed in by the user as an argument to the `build` command.
/// `is_plugin`                     - Indicates whether the Godot project is a plugin or not.
pub fn build_for_platform(
    rust_library_name: &str,
    godot_project_absolute_path: &str,
    platform: &str,
    is_release: bool,
    is_plugin: bool,
) {
    log_info_to_console(&format!("Building library for {}", platform));

    // Get the user's native platform to determine whether we need to use the
    // `cross` command or not.
    let native_platform = consts::OS.to_lowercase();

    // We need the path to the dynamic library that is built so that we can
    // copy it over to the Godot project. We do this in several steps.
    // First, we get the current directory of the Rust library.
    let current_dir = current_dir().expect("Unable to run build, please try again");

    // Next, we have to determine whether the dynamic library is going to be
    // output to the `debug` or `release` directory, which we can know by
    // using the `is_release` argument that was passed to this function.
    let debug_or_release = if is_release { "release" } else { "debug" };

    // Get the toolchain that we expect the dynamic library to be under for
    // the platform that we are targeting.
    let toolchain = PLATFORM_TOOLCHAINS
        .get(platform)
        .expect("Unable to run build, please try again");

    // The dynamic library will have a prefix of "lib" for the Linux
    // platform and "lib" for anything else.
    let dynamic_library_prefix = if platform == "windows" { "" } else { "lib" };

    // The dynamic library will have an extension of "dll" for windows,
    // "dylib" for macOS, and "so" for everything else.
    let dynamic_library_ext = if platform == "windows" {
        "dll"
    } else if platform == "macos" {
        "dylib"
    } else {
        "so"
    };

    // Combine the prefix, the name of the library, and the extension to get
    // the expected file name of the dynamic library generated.
    let dynamic_library_filename = format!(
        "{}{}.{}",
        &dynamic_library_prefix, &rust_library_name, &dynamic_library_ext
    );

    // Finally we can combine everything from above to get the path to the
    // dynamic library for this build.
    let dynamic_library_file_path = Path::new(&current_dir)
        .join("target")
        .join(toolchain)
        .join(debug_or_release)
        .join(&dynamic_library_filename);

    if platform == native_platform {
        // If the platform to build for is the user's native platform then we
        // can use the cargo build command.
        let native_target: &str = env!("TARGET");

        // Build the command to run with the native target and whether or not
        // it's a release build.
        let mut cargo_build_command = Command::new("cargo");
        cargo_build_command
            .arg("build")
            .arg("--target")
            .arg(&native_target);
        if is_release {
            cargo_build_command.arg("--release");
        }

        // Run the command and output the status.
        cargo_build_command
            .status()
            .expect("Unable to run the build, please try again");
    } else {
        // Otherwise we have to use the `cross` command to build the library
        // for another platform.
        // Just like with the cargo version we build the command with the
        // non-native target this time and whether or not it's a release build.
        let mut cross_build_command = Command::new("cross");
        cross_build_command
            .arg("build")
            .arg("--target")
            .arg(&toolchain);
        if is_release {
            cross_build_command.arg("--release");
        }

        // Run the command and output the status.
        cross_build_command
            .status()
            .expect("Unable to run the build please try again");
    }
    // Next, we build the path to where we should copy the dynmaic library
    // over using the godot project path and whether it is a plugin or not.
    let godot_project_bin_path = if is_plugin {
        PathBuf::from(godot_project_absolute_path)
            .join("addons")
            .join(rust_library_name)
            .join("gdnative")
            .join("bin")
            .join(platform)
    } else {
        PathBuf::from(godot_project_absolute_path)
            .join("gdnative")
            .join("bin")
            .join(platform)
    };

    // Make sure that the directory to the path we created above exists so
    // that we can copy the dynamic library over to it.
    create_dir_all(&godot_project_bin_path).expect("Unable to run the build, please try again");

    // Finally we can copy the dynamic library over to the Godot project.
    println!(
        "{:?} {:?}",
        dynamic_library_file_path, godot_project_bin_path
    );
    let mut copy_dynamic_library_command = Command::new("cp");
    copy_dynamic_library_command
        .arg(dynamic_library_file_path)
        .arg(godot_project_bin_path);
    copy_dynamic_library_command
        .output()
        .expect("Unable to copy file");

    log_success_to_console(&format!("Build complete for platform {}", &platform));
}

/// Builds the library and then watches for changes to the `src` directory of
/// the Rust library, rebuilding when changes happen.
///
/// # Arguments
///
/// `rust_library_name`             - The name of the Rust library.
/// `godot_project_absolute_path`   - The absolute path to the Godot project.
/// `platform`                      - The platform to target the build for.
/// `is_release`                    - Indicates whether the build is a release build or not. This is passed in by the user as an argument to the `build` command.
/// `is_plugin`                     - Indicates whether the Godot project is a plugin or not.
pub fn build_and_watch_for_changes(
    rust_library_name: &str,
    godot_project_absolute_path: &str,
    platform: &str,
    is_release: bool,
    is_plugin: bool,
) {
    // Create our sender and receiver and then run the initial build of the
    // Rust library.
    let (tx, rx) = channel();
    build_for_platform(
        rust_library_name,
        godot_project_absolute_path,
        platform,
        is_release,
        is_plugin,
    );

    let mut last_checked = Local::now();
    let mut watcher =
        raw_watcher(tx).expect("Unable to create watcher to watch the Rust library for changes");
    let current_dir = std::env::current_dir().expect(
        "Unable to get current directory while attempting to watch the Rust library for changes",
    );

    // Tell the watcher to watch the the `src` directory in the Rust library
    // for changes.
    watcher
        .watch(current_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch the Rust library directory for changes");

    loop {
        // Whenever something happened to the `src` directory...
        match rx.recv() {
            Ok(RawEvent {
                path: Some(_path),
                op: Ok(op),
                cookie: _,
            }) => {
                // Make sure that it's a write operation...
                if op.contains(op::WRITE) {
                    // Make sure that some time has passed since the last
                    // build so that we don't spam it.
                    let now = Local::now();
                    if (now - last_checked).num_seconds() == 0 {
                        // Run the build for the native platform!
                        build_for_platform(
                            rust_library_name,
                            godot_project_absolute_path,
                            platform,
                            is_release,
                            is_plugin,
                        );
                    }
                    last_checked = Local::now();
                }
            }
            Ok(_event) => log_error_to_console(
                "Broken event when trying to watch the Rust library for changes",
            ),
            Err(e) => log_error_to_console(&e.to_string()),
        }
    }
}
