use std::env::set_current_dir;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::process::{exit, Command};

use convert_case::{Case, Casing};

use crate::command_build::build_library;
use crate::command_create::create_module;
use crate::config_utils::{create_initial_config, Config};
use crate::definitions::CargoToml;
use crate::file_utils::write_and_fmt;
use crate::gdnlib_utils::create_initial_gdnlib;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::get_absolute_path;

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project the plugin or modules are for.
/// `plugin` - Indicates whether the library is for a plugin or not.
/// `skip_build` - Indicates whether the build should be skipped after creating the library or not.
pub fn create_library(name: &str, godot_project_dir: PathBuf, plugin: bool, skip_build: bool) {
    log_styled_message_to_console("Creating library", ConsoleColors::WHITE);

    let library_name_normalized = name.to_case(Case::Snake);

    let library_absolute_path = get_absolute_path(&PathBuf::from(&name));
    let godot_project_absolute_path = get_absolute_path(&godot_project_dir);

    check_if_library_already_exists(library_absolute_path);
    check_if_godot_project_valid(&godot_project_absolute_path);

    create_cargo_library(&library_name_normalized);

    set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

    create_library_cargo_toml();

    let godot_project_dir_name = godot_project_absolute_path
        .file_name()
        .unwrap()
        .to_str()
        .expect("Unable to convert Godot file name to str")
        .to_string();
    let config = create_initial_config(name.to_owned(), godot_project_dir_name, plugin);
    create_initial_lib_file();

    log_styled_message_to_console(
        "running initial build to generate Godot project structure",
        ConsoleColors::CYAN,
    );

    if plugin {
        create_plugin_structure_in_godot(&name, &godot_project_absolute_path);
    }

    create_gdnative_dir_in_godot(
        &library_name_normalized,
        &godot_project_absolute_path,
        &config,
    );

    create_initial_gdnlib(&config);

    // For testing we skip building the library so that tests won't take a
    // long time to run. We already test building on its own so it isn't
    // necessary to run here.
    if !skip_build {
        // Otherwise, in normal environments, we want to run the initial build
        // or else Godot will throw errors stating it can't find the dynamic
        // library for the project.
        build_library(false, false);
    }

    log_styled_message_to_console("library created", ConsoleColors::GREEN);
}

/// Creates the `Cargo.toml` file for the library.
fn create_library_cargo_toml() {
    // Get the base Cargo.toml contents of the library.
    let library_cargo_toml_string = read_to_string("Cargo.toml")
        .expect("Unable to read library's Cargo.toml file while creating the library");

    // Add the necessary dependencies to the base contents.
    let new_library_cargo_toml: CargoToml = toml::from_str(&library_cargo_toml_string)
        .expect("Unable to parse the library's Cargo.toml file");

    // Turn the new contents of the library's Cargo.toml into a string so that we
    // can write it back to the library. We also need to normalize some things here
    // because when we turn the Cargo toml contents to a string, extra symbols get
    // added.
    let new_library_cargo_toml_string = toml::to_string(&new_library_cargo_toml)
        .expect(
            "Unable to convert the library's new Cargo.toml to a string while creating the library",
        )
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");

    // Next we overwrite the contents of the Cargo.toml file with our contents.
    write("Cargo.toml", new_library_cargo_toml_string).expect(
        "Unable to update contents of the library's Cargo.toml file while creating the library",
    );
}

/// Checks to see if a directory with the name of the library to create already
/// exists and if it does, then we log it to the console and exit early.
///
/// # Arguments
///
/// `library_absolute_path` - The absolute path to the library directory to create.
fn check_if_library_already_exists(library_absolute_path: PathBuf) {
    if library_absolute_path.exists() {
        // If there's already a directory with the library name then we print an
        // error to the console and exit early.
        log_styled_message_to_console(
            "Cannot create library, directory with the same name already exists",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Checks to see if the Godot project is valid by checking whether it has a
/// project.godot file or not.
///
/// # Arguments
///
/// `godot_project_absolute_path` - The absolute path to the Godot project.
fn check_if_godot_project_valid(godot_project_absolute_path: &PathBuf) {
    if !godot_project_absolute_path.join("project.godot").exists() {
        // If there's not a project.godot file at the root of the provided Godot
        // project directory then we print an error to the console and exit early.
        log_styled_message_to_console(
            "The Godot project dir provided is not valid",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Creates the library using the `cargo new --lib` command.
///
/// # Arguments
///
/// `library_name` - The name of the library to pass to the `cargo new` command.
fn create_cargo_library(library_name: &String) {
    match Command::new("cargo")
        .arg("new")
        .arg(&library_name)
        .arg("--lib")
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Creates the gdnative directory within the Godot project.
///
/// # Arguments
///
/// `library_name_snake_case` - The snake case version of the library name.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
/// `config` - The config.
fn create_gdnative_dir_in_godot(
    library_name_snake_case: &str,
    godot_project_absolute_path: &PathBuf,
    config: &Config,
) {
    let gdnative_path = if config.is_plugin {
        godot_project_absolute_path
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
    } else {
        godot_project_absolute_path.join("gdnative")
    };

    match create_dir_all(&gdnative_path) {
        Ok(_) => (),
        Err(e) => {
            // If there was a problem creating the directory then we print the error
            // to the console and exit early.
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Creates the initial `lib.rs` file in the library directory.
fn create_initial_lib_file() {
    let lib_template = include_str!("../templates/lib.rs");
    write_and_fmt("src/lib.rs", lib_template).expect(
        "Unable to create the initial lib.rs file in the library while creating the library",
    );
}

/// Creates the plugin structure for a plugin library with the Godot project by
/// creating an addons folder if it doesn't exist, and then the plugin folder
/// with the configuration needed.
///
/// # Arguments
///
/// `plugin_name` - The name of the plugin.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
fn create_plugin_structure_in_godot(plugin_name: &str, godot_project_path: &PathBuf) {
    let module_name_snake_case = &plugin_name.to_case(Case::Snake);

    let godot_plugin_dir = godot_project_path
        .join("addons")
        .join(&module_name_snake_case);
    let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");
    create_dir_all(&godot_plugin_dir)
        .expect("Unable to create the plugin directory structure in Godot project while creating the library");

    create_module(&plugin_name);

    let plugin_cfg = include_str!("../templates/plugin-cfg.txt");
    let plugin_cfg_with_name = plugin_cfg.replace("PLUGIN_NAME", &plugin_name);
    let plugin_cfg_with_script = plugin_cfg_with_name.replace(
        "PLUGIN_GDNS_LOCATION",
        &format!("{}.gdns", &module_name_snake_case),
    );
    write(godot_plugin_cfg, plugin_cfg_with_script)
        .expect("Unable to write plugin.cfg file in the Godot project while creating the library");
}
