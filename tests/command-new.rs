use assert_cmd::prelude::*;

use serde_json::{json, Value};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test, Gdnlib};

/// Creates a library and checks that all of the files in the library exist
/// and that their values are what they should be.
#[test]
fn new_create_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd.assert().success();

    // 2. Assert that the library directory was created.
    let library_dir = Path::new("platformer_modules");
    assert_eq!(library_dir.exists(), true);

    // 3: Assert that the initial config is what it should be.
    let config = read_to_string("platformer_modules/godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["name"], "PlatformerModules");
    assert_eq!(
        config_json["cli_version"],
        env!("CARGO_PKG_VERSION").to_string()
    );
    assert_eq!(config_json["godot_project_dir_name"], "platformer");
    assert_eq!(config_json["is_plugin"], false);
    assert_eq!(config_json["modules"], json!([]));
    assert_eq!(config_json["platforms"], json!([]));

    // 4. Assert that the lib file exists.
    let lib_file_path = Path::new("platformer_modules/src/lib.rs");
    assert_eq!(lib_file_path.exists(), true);

    // 5. Assert that the lib file is what it should be.
    let lib_file_string = read_to_string(lib_file_path)?;
    let lib_file_split = lib_file_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(lib_file_split[0], "use gdnative::prelude::*;");
    assert_eq!(lib_file_split[1], "");
    assert_eq!(lib_file_split[2], "fn init(handle: InitHandle) {}");
    assert_eq!(lib_file_split[3], "");
    assert_eq!(lib_file_split[4], "godot_init!(init);");

    // 6. Assert that the Cargo.toml file of the library is what we expect.
    let cargo_toml_string = read_to_string("platformer_modules/Cargo.toml")?;
    let cargo_toml_split = cargo_toml_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(cargo_toml_split[0], "[package]");
    assert_eq!(cargo_toml_split[1], "name = \"platformer_modules\"");

    assert_eq!(cargo_toml_split[cargo_toml_split.len() - 6], "[lib]");
    assert_eq!(
        cargo_toml_split[cargo_toml_split.len() - 5],
        "crate-type = [\"cdylib\"]"
    );
    assert_eq!(cargo_toml_split[cargo_toml_split.len() - 4], "");
    assert_eq!(
        cargo_toml_split[cargo_toml_split.len() - 3],
        "[dependencies]"
    );
    assert_eq!(
        cargo_toml_split[cargo_toml_split.len() - 2],
        "gdnative = \"0.9.3\""
    );

    cleanup_test_files();

    Ok(())
}

/// Creates a library and checks that all of the files in the Godot project
/// exist and that their values are what they should be.
#[test]
fn new_create_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer");
    cmd.assert().success();

    // 2. Assert that the dynamic library for the library exists in the Godot project's bin directory.
    let dynamic_library_name = format!(
        "platformer/gdnative/bin/{}/{}platformer_modules{}",
        std::env::consts::OS.to_lowercase(),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let dynamic_library_path = Path::new(&dynamic_library_name);
    assert_eq!(dynamic_library_path.exists(), true);

    // 3. Assert that the gdnlib file exists.
    let gdnlib_path = Path::new("platformer/gdnative/platformer_modules.gdnlib");
    assert_eq!(gdnlib_path.exists(), true);

    // 4. Assert that the contents of the gdnlib file are what we expect.
    let gdnlib_string = read_to_string(gdnlib_path)?;
    let gdnlib_toml: Gdnlib = toml::from_str(&gdnlib_string)?;
    assert_eq!(gdnlib_toml.general.singleton, false);
    assert_eq!(gdnlib_toml.general.load_once, true);
    assert_eq!(gdnlib_toml.general.symbol_prefix, "godot_");
    assert_eq!(gdnlib_toml.general.reloadable, true);

    assert_eq!(
        gdnlib_toml.entry.get("Android.x86_64"),
        Some(
            &"res://gdnative/bin/android/x86_64-linux-android/libplatformer_modules.so".to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("Android.arm64-v8a"),
        Some(
            &"res://gdnative/bin/android/aarch64-linux-android/libplatformer_modules.so".to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("Windows.64"),
        Some(&"res://gdnative/bin/windows/platformer_modules.dll".to_owned())
    );
    assert_eq!(
        gdnlib_toml.entry.get("OSX.64"),
        Some(&"res://gdnative/bin/macos/libplatformer_modules.dylib".to_owned())
    );
    assert_eq!(
        gdnlib_toml.entry.get("X11.64"),
        Some(&"res://gdnative/bin/linux/libplatformer_modules.so".to_owned())
    );
    assert_eq!(
        gdnlib_toml.dependencies.get("Android.x86_64"),
        Some(&vec![])
    );
    assert_eq!(
        gdnlib_toml.dependencies.get("Android.arm64-v8a"),
        Some(&vec![])
    );
    assert_eq!(gdnlib_toml.dependencies.get("Windows.64"), Some(&vec![]));
    assert_eq!(gdnlib_toml.dependencies.get("OSX.64"), Some(&vec![]));
    assert_eq!(gdnlib_toml.dependencies.get("X11.64"), Some(&vec![]));

    cleanup_test_files();

    Ok(())
}
