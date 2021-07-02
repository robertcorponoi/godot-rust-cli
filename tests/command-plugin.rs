use assert_cmd::prelude::*;

use serde_json::{json, Value};
use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test, Gdnlib};

/// Creates a plugin and checks that all of the files in the library exist and
/// that their values are what they should be.
#[test]
fn plugin_create_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd.assert().success();

    // 2. Assert that the library directory for the plugin was created.
    let plugin_library_dir = Path::new("directory_browser");
    assert_eq!(plugin_library_dir.exists(), true);

    set_current_dir(plugin_library_dir)?;

    // 3: Assert that the config is what it should be.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["name"], "Directory Browser");
    assert_eq!(config_json["godot_project_name"], "platformer");
    assert_eq!(config_json["is_plugin"], true);
    assert_eq!(config_json["modules"], json!([]));
    assert_eq!(config_json["platforms"], json!([]));

    // 4. Assert that by default the plugin should have a module with the name of the plugin.
    let plugin_module_path = Path::new("src/directory_browser.rs");
    assert_eq!(plugin_module_path.exists(), true);

    // 5. Assert that the contents of the plugin's initial module matches the initial tool module.
    let plugin_module_string = read_to_string(plugin_module_path)?;
    let plugin_module_split = plugin_module_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(plugin_module_split[0], "use gdnative::api::EditorPlugin;");
    assert_eq!(
        plugin_module_split[5],
        "#[user_data(user_data::LocalCellData<DirectoryBrowser>)]"
    );
    assert_eq!(plugin_module_split[6], "pub struct DirectoryBrowser;");
    assert_eq!(plugin_module_split[9], "impl DirectoryBrowser {");
    assert_eq!(plugin_module_split[11].trim(), "DirectoryBrowser");

    // 6. Assert that the lib file exists.
    let lib_file_path = Path::new("src/lib.rs");
    assert_eq!(lib_file_path.exists(), true);

    // 7. Assert that the plugin's initial module is added to the lib file.
    let lib_file_string = read_to_string(lib_file_path)?;
    let lib_file_split = lib_file_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(lib_file_split[0], "mod directory_browser;");
    assert_eq!(
        lib_file_split[4].trim(),
        "handle.add_tool_class::<directory_browser::DirectoryBrowser>();"
    );

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a plugin and checks that all of the files in the Godot project
/// exist and that their values are what they should be.
#[test]
fn plugin_create_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin");
    cmd.assert().success();

    // 2. Assert that the plugin directory in the Godot project was created.
    let plugin_godot_dir = Path::new("platformer/addons/directory_browser");
    assert_eq!(plugin_godot_dir.exists(), true);

    // 3. Assert that the dynamic library for the plugin exists in the plugin's bin directory.
    let plugin_dynamic_library_name = format!(
        "platformer/addons/directory_browser/gdnative/bin/{}/{}directory_browser{}",
        std::env::consts::OS.to_lowercase(),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let plugin_dynamic_library_path = Path::new(&plugin_dynamic_library_name);
    assert_eq!(plugin_dynamic_library_path.exists(), true);

    // 4. Assert that the `gdnative` directory was created in the plugin's directory.
    let plugin_gdnative_path = Path::new("platformer/addons/directory_browser/gdnative");
    assert_eq!(plugin_gdnative_path.exists(), true);

    // 5. Assert that the `plugin.cfg` file exists.
    let plugin_cfg_path = Path::new("platformer/addons/directory_browser/plugin.cfg");
    assert_eq!(plugin_cfg_path.exists(), true);

    // 6. Assert that the contents of the `plugin.cfg` are what we expect.
    let plugin_cfg_string = read_to_string(plugin_cfg_path)?;
    let plugin_cfg_split = plugin_cfg_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(plugin_cfg_split[0], "[plugin]");
    assert_eq!(plugin_cfg_split[1], "name = \"Directory Browser\"");
    assert_eq!(plugin_cfg_split[5], "script = \"directory_browser.gdns\"");

    // 7. Assert that the plugin's gdns file exists.
    let plugin_gdns_path =
        Path::new("platformer/addons/directory_browser/gdnative/directory_browser.gdns");
    assert_eq!(plugin_gdns_path.exists(), true);

    // 8. Assert that the contents of the plugin's gdns file are what we expect.
    let plugin_gdns_string = read_to_string(plugin_gdns_path)?;
    let plugin_gdns_split = plugin_gdns_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(plugin_gdns_split[2], "[ext_resource path=\"res://addons/directory_browser/gdnative/directory_browser.gdnlib\" type=\"GDNativeLibrary\" id=1]");
    assert_eq!(plugin_gdns_split[6], "resource_name = \"DirectoryBrowser\"");
    assert_eq!(plugin_gdns_split[7], "class_name = \"DirectoryBrowser\"");

    // 9. Assert that the plugin's gdnlib file exists.
    let plugin_gdnlib_path =
        Path::new("platformer/addons/directory_browser/gdnative/directory_browser.gdnlib");
    assert_eq!(plugin_gdnlib_path.exists(), true);

    // 10. Assert that the contents of the plugin's gdnlib file are what we expect.
    let gdnlib_string = read_to_string(plugin_gdnlib_path)?;
    let gdnlib_toml: Gdnlib = toml::from_str(&gdnlib_string)?;
    assert_eq!(gdnlib_toml.general.singleton, false);
    assert_eq!(gdnlib_toml.general.load_once, true);
    assert_eq!(gdnlib_toml.general.symbol_prefix, "godot_");
    assert_eq!(gdnlib_toml.general.reloadable, true);

    assert_eq!(
        gdnlib_toml.entry.get("Android.x86_64"),
        Some(
            &"res://addons/directory_browser/gdnative/bin/android/x86_64-linux-android/libdirectory_browser.so".to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("Android.arm64-v8a"),
        Some(
            &"res://addons/directory_browser/gdnative/bin/android/aarch64-linux-android/libdirectory_browser.so".to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("Windows.64"),
        Some(
            &"res://addons/directory_browser/gdnative/bin/windows/directory_browser.dll".to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("OSX.64"),
        Some(
            &"res://addons/directory_browser/gdnative/bin/macos/libdirectory_browser.dylib"
                .to_owned()
        )
    );
    assert_eq!(
        gdnlib_toml.entry.get("X11.64"),
        Some(
            &"res://addons/directory_browser/gdnative/bin/linux/libdirectory_browser.so".to_owned()
        )
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

/// Creates a plugin and then creates a module within the plugin and checks to
/// make sure the module is added to the library.
#[test]
fn plugin_create_module_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd_plugin = Command::new("cargo");
    cmd_plugin
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd_plugin.assert().success();

    set_current_dir("directory_browser")?;

    // 2. Assert the create module command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Explorer");
    cmd_create.assert().success();

    // 3: Assert that the config includes the new module.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["modules"], json!(["Explorer"]));

    // 4. Assert that the plugin module has a mod file.
    let module_path = Path::new("src/explorer.rs");
    assert_eq!(module_path.exists(), true);

    // 5. Assert that the contents of the module's file matches the initial tool module.
    let module_string = read_to_string(module_path)?;
    let module_split = module_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(module_split[0], "use gdnative::api::EditorPlugin;");
    assert_eq!(
        module_split[5],
        "#[user_data(user_data::LocalCellData<Explorer>)]"
    );
    assert_eq!(module_split[6], "pub struct Explorer;");
    assert_eq!(module_split[9], "impl Explorer {");
    assert_eq!(module_split[11].trim(), "Explorer");

    // 6. Assert that the module is added to the lib file.
    let lib_file_string = read_to_string("src/lib.rs")?;
    let lib_file_split = lib_file_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(lib_file_split[0], "mod directory_browser;");
    assert_eq!(lib_file_split[1], "mod explorer;");
    assert_eq!(
        lib_file_split[5].trim(),
        "handle.add_tool_class::<explorer::Explorer>();"
    );
    assert_eq!(
        lib_file_split[6].trim(),
        "handle.add_tool_class::<directory_browser::DirectoryBrowser>();"
    );

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a plugin and then creates a module within the plugin and checks to
/// make sure the module is added to Godot project.
#[test]
fn plugin_create_module_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd_plugin = Command::new("cargo");
    cmd_plugin
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd_plugin.assert().success();

    set_current_dir("directory_browser")?;

    // 2. Assert the create module command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Explorer");
    cmd_create.assert().success();

    // 3. Assert the build command was successful.
    let mut cmd_build = Command::new("cargo");
    cmd_build
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("build");
    cmd_build.assert().success();

    set_current_dir("../")?;

    // 3. Assert that the dynamic library for the plugin exists in the plugin's bin directory.
    let plugin_dynamic_library_name = format!(
        "platformer/addons/directory_browser/gdnative/bin/{}/{}directory_browser{}",
        std::env::consts::OS.to_lowercase(),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let plugin_dynamic_library_path = Path::new(&plugin_dynamic_library_name);
    assert_eq!(plugin_dynamic_library_path.exists(), true);

    // 4. Assert that the plugin's gdns file exists.
    let module_gdns_path = Path::new("platformer/addons/directory_browser/gdnative/explorer.gdns");
    assert_eq!(module_gdns_path.exists(), true);

    // 8. Assert that the contents of the plugin's gdns file are what we expect.
    let module_gdns_string = read_to_string(module_gdns_path)?;
    let module_gdns_split = module_gdns_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(module_gdns_split[2], "[ext_resource path=\"res://addons/directory_browser/gdnative/directory_browser.gdnlib\" type=\"GDNativeLibrary\" id=1]");
    assert_eq!(module_gdns_split[6], "resource_name = \"Explorer\"");
    assert_eq!(module_gdns_split[7], "class_name = \"Explorer\"");

    cleanup_test_files();

    Ok(())
}

/// Creates a plugin and then creates a module within the plugin and lastly
/// deletes the module and checks the library structure.
#[test]
fn plugin_destroy_module_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd_plugin = Command::new("cargo");
    cmd_plugin
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd_plugin.assert().success();

    set_current_dir("directory_browser")?;

    // 2. Assert the create module command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Explorer");
    cmd_create.assert().success();

    // 3: Assert that the config includes the new module.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["modules"], json!(["Explorer"]));

    // 4. Assert that the destroy module command was successful.
    let mut cmd_destroy = Command::new("cargo");
    cmd_destroy
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("destroy")
        .arg("Explorer");
    cmd_destroy.assert().success();

    // 5: Assert that the config no longer includes the new module.
    let config_updated = read_to_string("godot-rust-cli.json")?;
    let config_updated_json: Value = serde_json::from_str(&config_updated)?;
    assert_eq!(config_updated_json["modules"], json!([]));

    // 4. Assert that the plugin module no longer has a mod file.
    let module_path = Path::new("src/explorer.rs");
    assert_eq!(module_path.exists(), false);

    // 5. Assert that the module is removed from the lib file.
    let lib_file_string = read_to_string("src/lib.rs")?;
    let lib_file_split = lib_file_string
        .split("\n")
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    assert_eq!(lib_file_split[0], "mod directory_browser;");
    assert_eq!(
        lib_file_split[4].trim(),
        "handle.add_tool_class::<directory_browser::DirectoryBrowser>();"
    );

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a plugin and then creates a module within the plugin and lastly
/// deletes the module and checks the Godot project structure.
#[test]
fn plugin_destroy_module_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd_plugin = Command::new("cargo");
    cmd_plugin
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd_plugin.assert().success();

    set_current_dir("directory_browser")?;

    // 2. Assert the create module command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Explorer");
    cmd_create.assert().success();

    // 3. Assert the destroy module command was successful.
    let mut cmd_destroy = Command::new("cargo");
    cmd_destroy
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("destroy")
        .arg("Explorer");
    cmd_destroy.assert().success();

    set_current_dir("../")?;

    // 4. Assert that the plugin's gdns file no longer exists.
    let module_gdns_path =
        Path::new("platformer/addons/directory_browser/rust_modules/explorer.gdns");
    assert_eq!(module_gdns_path.exists(), false);

    cleanup_test_files();

    Ok(())
}

/// Creates a plugin and then attempts to delete the root plugin module.
#[test]
fn plugin_destroy_root_module() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the plugin command was successful.
    let mut cmd_plugin = Command::new("cargo");
    cmd_plugin
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("Directory Browser")
        .arg("platformer")
        .arg("--plugin")
        .arg("--skip-build");
    cmd_plugin.assert().success();

    set_current_dir("directory_browser")?;

    // 2. Assert the destroy module command was not successful.
    let mut cmd_destroy = Command::new("cargo");
    cmd_destroy
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("destroy")
        .arg("Directory Browser");
    cmd_destroy.assert().failure();

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}
