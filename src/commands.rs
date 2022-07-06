use std::env::{current_dir, set_current_dir};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::process::{exit, Command};

use convert_case::{Case, Casing};

use crate::command_build::build_library;
use crate::config_utils::{
    add_module_to_config, create_initial_config, get_config_as_object, is_module_in_config,
};
use crate::definitions::CargoToml;
use crate::file_utils::write_and_fmt;
use crate::gdnlib_utils::create_initial_gdnlib;
use crate::lib_utils::add_module_to_lib;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::{exit_if_not_lib_dir, get_absolute_path};

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project the plugin or modules are for.
/// `plugin` - Indicates whether the library is for a plugin or not.
/// `skip_build` - Indicates whether the build should be skipped after creating the library or not.
pub fn command_new(name: &str, godot_project_dir: PathBuf, plugin: bool, skip_build: bool) {
    log_styled_message_to_console("Creating library", ConsoleColors::WHITE);

    let library_name_normalized = name.to_case(Case::Snake);

    let library_absolute_path = get_absolute_path(&PathBuf::from(&name));
    let godot_project_absolute_path = get_absolute_path(&godot_project_dir);

    // If there's already a directory with the library name then we print an
    // error to the console and exit early.
    if library_absolute_path.exists() {
        log_styled_message_to_console(
            "Cannot create library, directory with the same name already exists",
            ConsoleColors::RED,
        );
        exit(1);
    }

    // If there's not a project.godot file at the root of the provided Godot
    // project directory then we print an error to the console and exit early.
    if !godot_project_absolute_path.join("project.godot").exists() {
        log_styled_message_to_console(
            "The Godot project dir provided is not valid",
            ConsoleColors::RED,
        );
        exit(1);
    }

    // Creates the library using the `cargo new --lib` command.
    match Command::new("cargo")
        .arg("new")
        .arg(&library_name_normalized)
        .arg("--lib")
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }

    set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

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

    let godot_project_dir_name = godot_project_absolute_path
        .file_name()
        .unwrap()
        .to_str()
        .expect("Unable to convert Godot file name to str")
        .to_string();
    let config = create_initial_config(name.to_owned(), godot_project_dir_name, plugin);

    // Creates the initial `lib.rs` file in the library directory.
    let lib_template = include_str!("./templates/lib.rs");
    write_and_fmt("src/lib.rs", lib_template).expect(
        "Unable to create the initial lib.rs file in the library while creating the library",
    );

    log_styled_message_to_console(
        "running initial build to generate Godot project structure",
        ConsoleColors::CYAN,
    );

    if plugin {
        let module_name_snake_case = &name.to_case(Case::Snake);

        let godot_plugin_dir = godot_project_absolute_path
            .join("addons")
            .join(&module_name_snake_case);
        let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");
        create_dir_all(&godot_plugin_dir)
        .expect("Unable to create the plugin directory structure in Godot project while creating the library");

        command_create(&name);

        let plugin_cfg = include_str!("./templates/plugin-cfg.txt");
        let plugin_cfg_with_name = plugin_cfg.replace("PLUGIN_NAME", &name);
        let plugin_cfg_with_script = plugin_cfg_with_name.replace(
            "PLUGIN_GDNS_LOCATION",
            &format!("{}.gdns", &module_name_snake_case),
        );
        write(godot_plugin_cfg, plugin_cfg_with_script).expect(
            "Unable to write plugin.cfg file in the Godot project while creating the library",
        );
    }

    // Creates the gdnative directory within the Godot project.
    let gdnative_path = if config.is_plugin {
        godot_project_absolute_path
            .join("addons")
            .join(&library_name_normalized)
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

/// Creates a module by creating a module for it inside the library and a
/// corresponding gdns file in the Godot project.
///
/// # Arguments
///
/// `name` - The name of the module to create as pascal case.
pub fn command_create(name: &str) {
    exit_if_not_lib_dir();

    let module_name_snake_case = &name.to_case(Case::Snake);
    let module_name_pascal_case = &name.to_case(Case::Pascal);

    let current_dir_path =
        current_dir().expect("Unable to get current directory while creating the module");
    let parent_dir_path = current_dir_path
        .parent()
        .expect("Unable to get the shared directory while creating the module");

    let mut config = get_config_as_object();

    let path_to_godot_project = parent_dir_path.join(&config.godot_project_dir_name);

    log_styled_message_to_console("Creating module", ConsoleColors::WHITE);

    if is_module_in_config(name, &mut config) {
        // If there's already a module with the same name in the config, then
        // we exist early to avoid creating duplicates.
        log_styled_message_to_console(
            "A module with the same name already exists",
            ConsoleColors::RED,
        );
    }

    // Creates the initial file for the module in the library directory.
    // Get the template for the module depending whether it's a regular module
    // or a plugin module.
    let mod_template = if config.is_plugin {
        include_str!("./templates/mod-plugin.rs")
    } else {
        include_str!("./templates/mod.rs")
    };

    // Replace the values in the default module template with the pascal
    // version of the module name and write the file to the library's src
    // directory.
    let mod_template_with_module = mod_template.replace("MODULE_NAME", &module_name_pascal_case);
    write_and_fmt(
        format!("src/{}.rs", &module_name_snake_case),
        mod_template_with_module,
    )
    .expect("Unable to create the initial module file in the library while creating a module");

    add_module_to_lib(name, &config);

    // Creates the gdns file for the module from the template and places it either
    // in the gdnative directory at the root of the Godot project if it is a
    // normal library or in the gdnative directory at the root of the plugin
    // directory in the Godot project if it is a plugin library.
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let gdns_dir: PathBuf = if config.is_plugin {
        path_to_godot_project
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
    } else {
        path_to_godot_project.join("gdnative")
    };

    create_dir_all(&gdns_dir).expect("Unable to create directory for module file in Godot.");

    // The path to the gdnlib file in the Godot project.
    let gdnlib_path = if config.is_plugin {
        format!(
            "addons/{}/gdnative/{}",
            &library_name_snake_case, &library_name_snake_case
        )
    } else {
        format!("gdnative/{}", &library_name_snake_case)
    };

    // Replace the values in our template with the name of the library and the
    // pascal version of the module name.
    let gdns_template = include_str!("./templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("GDNLIB_PATH", &gdnlib_path)
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_dir.join(&gdns_file_name), gdns_with_module_name)
        .expect("Unable to create module's gdns file while creating the module");

    add_module_to_config(name, &mut config);

    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}
