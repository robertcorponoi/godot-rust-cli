use chrono::offset::Local;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::create_dir_all;
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use crate::config_utils::get_config_as_object;
use crate::dynamic_library_utils::get_dynamic_library_ext;
use crate::file_utils::copy_file_to_location;
use crate::log_utils::{log_styled_message_to_console, log_version, ConsoleColors};
use crate::path_utils::get_dynamic_libraries_path;
use crate::time_utils::get_current_datetime_formatted;

/// Runs the cargo build command to build the library and generate the dynamic
/// libraries needed to run the modules in the Godot project.
pub fn build_library() {
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while building the library");
    let library_and_godot_project_shared_dir = current_dir
        .parent()
        .expect("Unable to get shared directory while building the library");

    let config = get_config_as_object();
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let dynamic_libraries_path = get_dynamic_libraries_path();
    let dynamic_library_ext = get_dynamic_library_ext();

    // On linux and macos the dynamic library build files will have a prefix of
    // "lib".
    let dynamic_library_extra = if cfg!(windows) { "" } else { "lib" };
    let dynamic_library_file_name = format!(
        "{}{}.{}",
        dynamic_library_extra, &library_name_snake_case, dynamic_library_ext
    );
    let dynamic_library_file = dynamic_libraries_path.join(dynamic_library_file_name);

    let godot_project_dir = library_and_godot_project_shared_dir.join(&config.godot_project_name);

    log_version();
    log_styled_message_to_console("building...", ConsoleColors::CYAN);

    run_cargo_build_command();

    let godot_project_bin_path = if config.is_plugin {
        godot_project_dir
            .join("addons")
            .join(&library_name_snake_case)
            .join("bin")
    } else {
        library_and_godot_project_shared_dir
            .join(&config.godot_project_name)
            .join("bin")
    };

    // The dynamic library gets copied to the `bin` directory in the root of
    // the Godot project so we want to make sure that is created if it doesn't
    // already exist.
    create_dir_all(&godot_project_bin_path)
        .expect("Unable to create bin directory in the Godot project");

    copy_file_to_location(&dynamic_library_file, &godot_project_bin_path);

    log_styled_message_to_console("Build complete", ConsoleColors::GREEN);
}

/// Watches the src directory in the library for changes and rebuilds the
/// library when changes occur.
///
/// Everytime that the library is rebuilt, it logs the time that the build
/// completed and lets the user know that it is waiting for changes before
/// rebuilding.
pub fn build_library_and_watch_for_changes() {
    let (tx, rx) = channel();

    build_library_with_timestamp();

    let mut last_checked = Local::now();
    let mut watcher =
        raw_watcher(tx).expect("Unable to create watcher to watch library for changes");
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while attempting to watch library for changes");

    watcher
        .watch(current_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch library directory for changes");
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(_path),
                op: Ok(op),
                cookie: _,
            }) => {
                if op.contains(op::WRITE) {
                    let now = Local::now();
                    if (now - last_checked).num_seconds() == 0 {
                        build_library_with_timestamp();
                    }
                    last_checked = Local::now();
                }
            }
            Ok(_event) => log_styled_message_to_console("broken event", ConsoleColors::RED),
            Err(e) => log_styled_message_to_console(&e.to_string(), ConsoleColors::RED),
        }
    }
}

/// Runs the `build_library` function to build the library and copy the
/// dynamic library file to the Godot project.
///
/// In addition to that, it also logs the datetime that the build was
/// completed as `YYYY-MM-DD HH:MM::SS and lets the user know that it is
/// waiting for changes before building again.
pub fn build_library_with_timestamp() {
    build_library();

    // After the build we want to show a message letting the user know that the
    // build has finished and is waiting for futher changes before rebuilding.
    log_styled_message_to_console("", ConsoleColors::WHITE);
    log_styled_message_to_console(
        &format!(
            "[{}] {}",
            get_current_datetime_formatted(),
            "waiting for changes..."
        ),
        ConsoleColors::WHITE,
    );
}

/// Runs the cargo build command in the library directory to build the dynamic
/// libraries. If the build failed, a message is logged to the console to let
/// the user know.
fn run_cargo_build_command() {
    let cargo_build_command = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Unable to run cargo build");

    let cargo_build_command_was_successful = cargo_build_command.success();
    if !cargo_build_command_was_successful {
        log_styled_message_to_console("Build failed, please try again", ConsoleColors::RED);
        exit(1);
    }
}
