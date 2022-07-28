use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env::{consts, current_dir, set_current_dir};
use std::fs::{create_dir_all, read_to_string, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use convert_case::{Case, Casing};
use rust_codegen::Scope;
use walkdir::WalkDir;

use crate::build_utils::{build_and_watch_for_changes, build_for_platform};
use crate::cargo_config::CargoConfig;
use crate::config_utils::{
    add_module_to_config, add_platform_to_config, create_initial_config, get_config_as_object,
    is_module_in_config, remove_module_from_config_if_exists,
    remove_platform_from_config_if_exists,
};
use crate::cross_utils::add_image_override_for_platform;
use crate::definitions::CargoToml;
use crate::file_utils::write_and_fmt;
use crate::gdnlib::Gdnlib;
use crate::gdns_file::GdnsFile;
use crate::lib_utils::add_module_to_lib;
use crate::log_utils::{log_error_to_console, log_info_to_console, log_success_to_console};
use crate::path_utils::{exit_if_not_lib_dir, get_absolute_path};
use crate::plugin_config::PluginConfig;

lazy_static! {
    static ref VALID_PLATFORMS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // m.insert("android.arm", "aarch64-linux-android");
        // m.insert("android", "x86_64-linux-android");
        m.insert("windows", "x86_64-pc-windows-gnu");
        // m.insert("linux", "x86_64-unknown-linux-gnu");
        // m.insert("macos", "x86_64-apple-darwin");
        m
    };
}

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project the plugin or modules are for.
/// `plugin` - Indicates whether the library is for a plugin or not.
/// `skip_build` - Indicates whether the build should be skipped after creating the library or not.
pub fn command_new(name: &str, godot_project_dir: PathBuf, plugin: bool, skip_build: bool) {
    log_info_to_console("Creating library");

    // The input from the user could be in any format but as is standard with
    // Rust, we want to make sure that the library has a snake_case name so we
    // enforce that here in case it is not already.
    let library_name_normalized = name.to_case(Case::Snake);

    // Create an absolute path from the current path and then the normalized
    // version of the Rust library name provided by the user.
    let library_absolute_path = get_absolute_path(&PathBuf::from(&name));

    // To make it easier to write to the Godot project we also want to create
    // an absolute path to it.
    let godot_project_absolute_path = get_absolute_path(&godot_project_dir);

    // If there's already a directory with the library name then we print an
    // error to the console and exit early.
    if library_absolute_path.exists() {
        log_error_to_console("Cannot create library, directory with the same name already exists");
        exit(1);
    }

    // If there's not a project.godot file at the root of the provided Godot
    // project directory then we print an error to the console and exit early.
    if !godot_project_absolute_path.join("project.godot").exists() {
        log_error_to_console("The Godot project dir provided is not valid");
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
            log_error_to_console(&e.to_string());
            exit(1);
        }
    }

    set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

    // Since we have custom configuration for the Godot project that needs to
    // be used as env variables, we have to create the initial config.toml
    // file similarly to how we replaced the Cargo.toml file.
    let godot_project_absolute_path_as_string = godot_project_absolute_path
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    let mut cargo_config = CargoConfig::new(&godot_project_absolute_path_as_string);
    cargo_config.write();

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
    create_initial_config(name.to_owned(), godot_project_dir_name, plugin);

    // Build the initial contents of the Rust library's `lib.rs` file which
    // is used to initialize Godot.
    log_info_to_console("Creating initial lib.rs file");
    let mut scope = Scope::new();
    scope.import("gdnative::prelude", "*");
    scope.new_fn("init").arg("handle", "InitHandle");
    scope.raw("godot_init!(init);");
    write("src/lib.rs", scope.to_string()).expect("Unable to create the initial lib.rs file");

    log_info_to_console("running initial build to generate Godot project structure");

    if plugin {
        let module_name_snake_case = &name.to_case(Case::Snake);

        let godot_plugin_dir = godot_project_absolute_path
            .join("addons")
            .join(&module_name_snake_case);
        let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");
        create_dir_all(&godot_plugin_dir)
        .expect("Unable to create the plugin directory structure in Godot project while creating the library");

        command_create(&name);

        // Every Godot plugin needs to have a config file that describes the
        // plugin.
        // More about this can be found at: https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html
        let mut plugin = PluginConfig::new(&name, &format!("{}.gdns", &library_name_normalized));
        plugin.write(godot_plugin_cfg);
    }

    // Create the initial gdnlib file for the Godot project. This file points
    // to the binaries for popular operating systems so that Godot knows which
    // one to use.
    let mut gdnlib = Gdnlib::new(&library_name_normalized, plugin);
    let gdnlib_pretty_printed = gdnlib.to_string();

    // Next, we create the directory to where the gndlib file will be saved in
    // the Godot project. As with most operations in the Godot project we have
    // to handle this differently if the Godot project is a plugin.
    let gdnlib_dir: PathBuf = if plugin {
        godot_project_absolute_path
            .join("addons")
            .join(&library_name_normalized)
            .join("gdnative")
    } else {
        godot_project_absolute_path.join("gdnative")
    };
    create_dir_all(&gdnlib_dir).expect("Unable to create directory for the gdnlib file");

    // Using the directory defined above we can create the path to the gdnlib
    // file which we will write in the next step.
    let gdnlib_file_path = gdnlib_dir.join(format!("{}.gdnlib", &library_name_normalized));

    // Finally we can write the gndlib file to the Godot project. As with most
    // of the write operations if something goes wrong we log the error to the
    // terminal and exit early.
    log_info_to_console("Creating the gdnlib file in the Godot project");
    match write(&gdnlib_file_path, gdnlib_pretty_printed) {
        Ok(_) => (),
        Err(e) => {
            log_error_to_console(&e.to_string());
            exit(1);
        }
    };

    // For testing we skip building the library so that tests won't take a
    // long time to run. We already test building on its own so it isn't
    // necessary to run here.
    if !skip_build {
        // Otherwise, in normal environments, we want to run the initial build
        // or else Godot will throw errors stating it can't find the dynamic
        // library for the project.
        command_build(false, false);
    }

    log_success_to_console("library created");
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

    let mut config = get_config_as_object();

    // Read the cargo config so that we can get the path to the Godot project
    // from the env vars.
    let cargo_config = CargoConfig::read();

    log_info_to_console("Creating module");

    if is_module_in_config(name, &mut config) {
        // If there's already a module with the same name in the config, then
        // we exist early to avoid creating duplicates.
        log_error_to_console("A module with the same name already exists");
    }

    // Next we build the script based on whether the Godot project is a plugin
    // or not.
    let mut scope = Scope::new();
    if config.is_plugin {
        scope.import("gdnative::prelude", "*");
        scope.import("gdnative::api", "EditorPlugin");

        let script_struct = scope.new_struct(&module_name_pascal_case);
        script_struct.vis("pub");
        script_struct.derive("gdnative::NativeClass");
        script_struct.attr("#[inherit(EditorPlugin)]");
        script_struct.attr(&format!(
            "#[user_data(user_data::LocalCellData<{}>)]",
            &module_name_pascal_case
        ));

        let script_impl = scope.new_impl(&module_name_pascal_case);
        script_impl.r#macro("#[gdnative::methods]");

        let new_fn = script_impl.new_fn("new");
        new_fn.arg("_owner", "&EditorPlugin");
        new_fn.ret("Self");
        new_fn.line(format!("{} {}", &module_name_pascal_case, "{}"));

        let ready_fn = script_impl.new_fn("_ready");
        ready_fn.attr("export");
        ready_fn.arg_mut_self();
        ready_fn.arg("_owner", "&EditorPlugin");
        ready_fn.line("godot_print!(\"Hello world!\")");
    } else {
        scope.import("gdnative::api", "Node2D");
        scope.import("gdnative::prelude", "*");

        let script_struct = scope.new_struct(&name);
        script_struct.vis("pub");
        script_struct.attr("#[inherit(Node2D)]");
        script_struct.derive("NativeClass");

        let script_impl = scope.new_impl(&name);
        script_impl.r#macro("#[methods]");

        let new_fn = script_impl.new_fn("new");
        new_fn.arg("_owner", "&Node2D");
        new_fn.ret("Self");
        new_fn.line(format!("{} {}", &name, "{}"));

        let ready_fn = script_impl.new_fn("_ready");
        ready_fn.attr("export");
        ready_fn.arg_mut_self();
        ready_fn.arg("_owner", "&Node2D");
        ready_fn.line("godot_print!(\"Hello world!\")");

        let process_fn = script_impl.new_fn("_process");
        process_fn.attr("export");
        process_fn.arg_mut_self();
        process_fn.arg("_owner", "&Node2D");
        process_fn.arg("_delta", "f32");
    };

    // Stringify the code and write it out to a file in the Godot project.
    write_and_fmt(
        format!("src/{}.rs", &module_name_snake_case),
        scope.to_string(),
    )
    .expect("Unable to create the initial script file in the library while creating a module");

    add_module_to_lib(name, &config);

    // Creates the gdns file for the module from the template and places it either
    // in the gdnative directory at the root of the Godot project if it is a
    // normal library or in the gdnative directory at the root of the plugin
    // directory in the Godot project if it is a plugin library.
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let gdns_dir: PathBuf = if config.is_plugin {
        PathBuf::from(&cargo_config.env.godot_project_path)
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
    } else {
        PathBuf::from(&cargo_config.env.godot_project_path).join("gdnative")
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

    // Create the gdns file which defines the script in the Godot project.
    let mut gdns_file = GdnsFile::new(&&module_name_pascal_case, &gdnlib_path);
    gdns_file.write(gdns_dir.join(&gdns_file_name));

    add_module_to_config(name, &mut config);

    log_success_to_console("Module created");
}

/// Removes a module by deleting its module file from the library and searching
/// the Godot project for the corresponding gdns file to remove.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
pub fn command_destroy(name: &str) {
    exit_if_not_lib_dir();

    log_info_to_console("destroying module...");

    let mut config = get_config_as_object();

    let module_name_snake_case = name.to_case(Case::Snake);
    let module_name_pascal_case = name.to_case(Case::Pascal);

    let current_dir_path =
        current_dir().expect("Unable to get current directory while destroying the module");

    // Read the cargo config so that we can get the path to the Godot project
    // from the env vars.
    let cargo_config = CargoConfig::read();

    remove_module_from_config_if_exists(&module_name_pascal_case, &mut config);

    // Removes the module's gdns file from Godot project.
    let library_name_snake_case = &config.name.to_case(Case::Snake);
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);

    // The first place we should check for the module to remove is either the
    // gdnative folder in the plugin directory if it's a plugin or just the
    // gdnative folder in the root directory of the Godot project otherwise.
    let possible_gdns_path = if config.is_plugin {
        PathBuf::from(&cargo_config.env.godot_project_path)
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
            .join(&gdns_file_name)
    } else {
        PathBuf::from(&cargo_config.env.godot_project_path)
            .join("gdnative")
            .join(&gdns_file_name)
    };

    if possible_gdns_path.exists() {
        // If this path exists, then we can just remove the module.
        remove_file(possible_gdns_path).expect("Unable to remove the module's gdns file from the Godot project while destroying the module");
    } else {
        // Otherwise, we want to search a directory for the module. If the
        // module is a plugin, we can limit our search to the plugin directory.
        // Otherwise, we search the entire project since the user might have
        // moved it around.
        let search_dir = if config.is_plugin {
            PathBuf::from(&cargo_config.env.godot_project_path)
                .join("addons")
                .join(&library_name_snake_case)
        } else {
            PathBuf::from(&cargo_config.env.godot_project_path).to_owned()
        };

        for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
            let file_name = entry
                .file_name()
                .to_str()
                .expect("Unable to get file name while finding module to remove in Godot project");
            if file_name == gdns_file_name {
                remove_file(entry.path()).expect("Unable to remove module's gdns file");
            }
        }
    }

    // Removes all traces of a module from the lib.rs file.
    let lib_file_contents = read_to_string(current_dir_path.join("src").join("lib.rs"))
        .expect("Unable to read the contents of the lib file while destroying the module");

    // Create the mod and handle strings that we want to search for and remove
    // from the lib file.
    let module_mod_search_query = format!("mod {};", &module_name_snake_case);
    let module_handle_search_query = format!(
        "handle.add_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );
    let module_plugin_handle_search_query = format!(
        "handle.add_tool_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );

    // Remove the `mod` declaration for the module.
    let file_contents_no_mod = lib_file_contents
        .lines()
        .filter(|&line| line.trim() != module_mod_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    // Removes the turbofish handle for a module in a normal library.
    let file_contents_no_mod_no_handle = file_contents_no_mod
        .lines()
        .filter(|&line| line.trim() != module_handle_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    // Removes the turbofish handle for a module in a plugin library.
    let file_contents_no_mod_no_handles = file_contents_no_mod_no_handle
        .lines()
        .filter(|&line| line.trim() != module_plugin_handle_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    write_and_fmt("src/lib.rs", file_contents_no_mod_no_handles)
        .expect("Unable to write the new contents to the lib.rs file while destroying the module");

    // Removes the module's file from the library.
    let module_file_name = format!("src/{}.rs", &module_name_snake_case);
    let module_file_path = Path::new(&module_file_name);
    remove_file(module_file_path)
        .expect("Unable to remove the module file from the library while destroying the module");

    log_success_to_console("Module destroyed");
}

/// Runs the command to build the library and then copies over the dynamic
/// libraries to the Godot project.
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_platforms` - Indicates whether all platforms should be built or just the native one.
pub fn command_build(is_release: bool, build_all_platforms: bool) {
    log_info_to_console("[build] build starting...");

    let config = get_config_as_object();

    // Normalize the name of the library as snake case as that is what is
    // needed to construct the path to the dynamic library.
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    // We need to load up the cargo config to get the Godot project's path and
    // also the godot-rust-cli config to get the other details about the
    // project.
    let cargo_config = CargoConfig::read();

    // Build for the native platform by default.
    let native_platform = std::env::consts::OS.to_lowercase();

    build_for_platform(
        &library_name_snake_case,
        &cargo_config.env.godot_project_path,
        &native_platform,
        is_release,
        config.is_plugin,
    );

    // Build for all platforms if the flag is passed.
    if build_all_platforms {
        for platform in &config.platforms {
            build_for_platform(
                &library_name_snake_case,
                &cargo_config.env.godot_project_path,
                &platform,
                is_release,
                config.is_plugin,
            );
        }
    }

    // Let the user know that the build is complete.
    log_success_to_console("[build] build complete");
}

/// Runs the build and watch command from the build crate to run an initial
/// build on the native platform and then watch for changes to the `src`
/// directory of the Rust library and rebuild.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
pub fn command_build_and_watch(is_release: bool) {
    exit_if_not_lib_dir();

    // We need to load up the cargo config to get the Godot project's path and
    // also the godot-rust-cli config to get the other details about the
    // project.
    let cargo_config = CargoConfig::read();
    let godot_rust_cli_config = get_config_as_object();

    // Build and watch is only supported for the user's native platform.
    let native_platform = consts::OS.to_lowercase();
    build_and_watch_for_changes(
        &godot_rust_cli_config.name.to_case(Case::Snake),
        &cargo_config.env.godot_project_path,
        &native_platform,
        is_release,
        godot_rust_cli_config.is_plugin,
    );
}

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
pub fn command_platform_add(platform: &str) {
    let platform_normalized = platform.to_lowercase();

    if VALID_PLATFORMS.contains_key(&platform_normalized.as_str()) {
        let mut config = get_config_as_object();
        // Add the platform to the `platforms` array in the config.
        add_platform_to_config(&platform_normalized, &mut config);

        // Since we need images that do more than the default cross images, we
        // have to copy the docker file override into the user's library and
        // add it
        add_image_override_for_platform(&platform_normalized);
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
pub fn command_platform_remove(platform: &str) {
    let mut config = get_config_as_object();

    // Remove the platform from the `platforms` array in the configuration.
    remove_platform_from_config_if_exists(platform, &mut config);

    // Remove the docker image from the user's system since it's no longer
    // needed.
    let image_name = match platform {
        "windows" => Some("godot-rust-cli-platform-windows:v1"),
        _ => None,
    };

    match image_name {
        Some(image_tag) => {
            let mut remove_default_docker_image_command = std::process::Command::new("docker");
            remove_default_docker_image_command
                .arg("rmi")
                .arg("rustembedded/cross:x86_64-pc-windows-gnu");

            remove_default_docker_image_command
                .status()
                .expect("Unable to remove docker image rustembedded/cross:x86_64-pc-windows-gnu");

            let mut remove_custom_docker_image_command = std::process::Command::new("docker");
            remove_custom_docker_image_command.arg("rmi").arg(image_tag);

            remove_custom_docker_image_command
                .status()
                .expect(&format!("Unable to remove docker image {}", image_tag));
            log_info_to_console(&format!("Removed docker image for {}", &platform));
        }
        None => (),
    }
}
