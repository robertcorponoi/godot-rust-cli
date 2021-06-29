use assert_cmd::prelude::*;

use serde_json::{json, Value};
use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_docker_images, cleanup_test_files, init_test};

// /// Creates a library and adds linux as a supported platform that the library
// /// can be built for.
// #[test]
// fn platform_add_linux_platform() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new_library = Command::new("cargo");
//     cmd_new_library
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new_library.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the add platform command was successful.
//     let mut cmd_add_platform = Command::new("cargo");
//     cmd_add_platform
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("linux");
//     cmd_add_platform.assert().success();

//     // 3. Assert that the config contains the added platform.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["platforms"], json!(["linux"]));

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and attempts to add linux twice which should only add it
// /// once.
// #[test]
// fn platform_add_linux_platform_twice() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new_library = Command::new("cargo");
//     cmd_new_library
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new_library.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the add platform command was successful.
//     let mut cmd_add_platform = Command::new("cargo");
//     cmd_add_platform
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("linux");
//     cmd_add_platform.assert().success();
//     let mut cmd_add_platform_2 = Command::new("cargo");
//     cmd_add_platform_2
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("linux");
//     cmd_add_platform_2.assert().failure();

//     // 3. Assert that the config contains the added platform.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["platforms"], json!(["linux"]));

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and attempts to add an unsupported platform.
// #[test]
// fn platform_add_unsupported_platform() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new_library = Command::new("cargo");
//     cmd_new_library
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new_library.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the add platform command was successful.
//     let mut cmd_add_platform = Command::new("cargo");
//     cmd_add_platform
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("macos");
//     cmd_add_platform.assert().failure();

//     // 3. Assert that the config contains the added platform.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["platforms"], json!([]));

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and adds windows as a supported platform that the library
// /// can be built for.
// #[test]
// fn platform_add_windows_platform() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new_library = Command::new("cargo");
//     cmd_new_library
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new_library.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the add platform command was successful.
//     let mut cmd_add_platform = Command::new("cargo");
//     cmd_add_platform
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("windows");
//     cmd_add_platform.assert().success();

//     // 3. Assert that the config contains the added platform.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["platforms"], json!(["windows"]));

//     // 4. Since the windows platform needs a custom docker image we want to
//     // make sure that the docker image was copied over.
//     let windows_docker_file_path = Path::new("docker/Dockerfile.x86_64-pc-windows-gnu");
//     assert_eq!(windows_docker_file_path.exists(), true);

//     // 5. Assert that the Cross.toml file exists and that its contents are what
//     // we expect.
//     let cross_config_file_path = Path::new("Cross.toml");
//     assert_eq!(cross_config_file_path.exists(), true);

//     let cross_config_string = read_to_string(&cross_config_file_path)?;
//     let cross_config_split = cross_config_string
//         .split("\n")
//         .map(|x| x.replace("\r", ""))
//         .collect::<Vec<String>>();
//     assert_eq!(cross_config_split[0], "[target.x86_64-pc-windows-gnu]");
//     assert_eq!(
//         cross_config_split[1],
//         "image = \"godot-rust-cli-platform-windows:v2\""
//     );

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and adds windows as a supported platform that the library
// /// can be built for and then builds the library.
// #[test]
// fn platform_add_windows_platform_and_build() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new_library = Command::new("cargo");
//     cmd_new_library
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new_library.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the add platform command was successful.
//     let mut cmd_add_platform = Command::new("cargo");
//     cmd_add_platform
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("add-platform")
//         .arg("windows");
//     cmd_add_platform.assert().success();

//     // 3. Assert the build --all command was successful.
//     let mut cmd_build_all = Command::new("cargo");
//     cmd_build_all
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("build")
//         .arg("--all");
//     cmd_build_all.assert().success();

//     // 4. Assert that the config contains the added platform.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["platforms"], json!(["windows"]));

//     // 5. Since the windows platform needs a custom docker image we want to
//     // make sure that the docker image was copied over.
//     let windows_docker_file_path = Path::new("docker/Dockerfile.x86_64-pc-windows-gnu");
//     assert_eq!(windows_docker_file_path.exists(), true);

//     // 6. Assert that the Cross.toml file exists and that its contents are what
//     // we expect.
//     let cross_config_file_path = Path::new("Cross.toml");
//     assert_eq!(cross_config_file_path.exists(), true);

//     let cross_config_string = read_to_string(&cross_config_file_path)?;
//     let cross_config_split = cross_config_string
//         .split("\n")
//         .map(|x| x.replace("\r", ""))
//         .collect::<Vec<String>>();
//     assert_eq!(cross_config_split[0], "[target.x86_64-pc-windows-gnu]");
//     assert_eq!(
//         cross_config_split[1],
//         "image = \"godot-rust-cli-platform-windows:v2\""
//     );

//     // 7. Assert that the dll file was copied to the Godot project.
//     let godot_project_dll_path = Path::new("../platformer/gdnative/bin/windows/platformer_modules.dll");
//     assert_eq!(godot_project_dll_path.exists(), true);

//     set_current_dir("../")?;

//     cleanup_docker_images();
//     cleanup_test_files();

//     Ok(())
// }

/// Creates a library, adds a supported platform and then removes it.
#[test]
fn platform_add_and_remove_platform() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd_new_library = Command::new("cargo");
    cmd_new_library
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd_new_library.assert().success();

    set_current_dir("platformer_modules")?;

    // 2. Assert that the add platform command was successful.
    let mut cmd_add_platform = Command::new("cargo");
    cmd_add_platform
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("add-platform")
        .arg("linux");
    cmd_add_platform.assert().success();

    // 3. Assert that the config contains the added platform.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["platforms"], json!(["linux"]));

    // 4. Assert that the remove platform command was successful.
    let mut cmd_add_platform = Command::new("cargo");
    cmd_add_platform
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("remove-platform")
        .arg("linux");
    cmd_add_platform.assert().success();

    // 5. Assert that the config contains the added platform.
    let config_updated = read_to_string("godot-rust-cli.json")?;
    let config_updated_json: Value = serde_json::from_str(&config_updated)?;
    assert_eq!(config_updated_json["platforms"], json!([]));

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}
