use assert_cmd::prelude::*;

use serde_json::{json, Value};
use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

/// Creates a library and then creates a module within the library and
/// checks that all files are there and their values are what they should be.
#[test]
fn create_module_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd_new = Command::new("cargo");
    cmd_new
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd_new.assert().success();

    set_current_dir("platformer_modules")?;

    // 2. Assert that the create command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Player");
    cmd_create.assert().success();

    // 3. Assert that the module has a mod file.
    let module_mod_path = Path::new("src/player.rs");
    assert_eq!(module_mod_path.exists(), true);

    // 4. Assert that the module contents are what we expect.
    let module_mod_string = read_to_string(module_mod_path)?;
    let module_mod_split = module_mod_string.split("\r\n").collect::<Vec<&str>>();
    assert_eq!(module_mod_split[0], "use gdnative::api::Node2D;");
    assert_eq!(module_mod_split[5], "pub struct Player;");
    assert_eq!(module_mod_split[8], "impl Player {");
    assert_eq!(module_mod_split[10].trim(), "Player {}");

    // 4. Assert that the module was added to the lib file.
    let lib_file_string = read_to_string("src/lib.rs")?;
    let lib_file_split = lib_file_string.split("\n").collect::<Vec<&str>>();
    assert_eq!(lib_file_split[0], "mod player;");
    assert_eq!(
        lib_file_split[4].trim(),
        "handle.add_class::<player::Player>();"
    );

    // 5: Assert that the module was added to the config.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["modules"], json!(["Player"]));

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a library and then creates a module within the library and
/// checks the Godot project to make sure all of the files exist and that their
/// values are what they should be.
#[test]
fn create_module_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd_new = Command::new("cargo");
    cmd_new
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd_new.assert().success();

    set_current_dir("platformer_modules")?;

    // 2. Assert that the create command was successful.
    let mut cmd_create = Command::new("cargo");
    cmd_create
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Player");
    cmd_create.assert().success();

    set_current_dir("../")?;

    // 3. Assert that the gdns file for the module was created.
    let module_gdns_path = Path::new("platformer/rust_modules/player.gdns");
    assert_eq!(module_gdns_path.exists(), true);

    // 4. Assert that the gnds file has the correct contents.
    let module_gdns_string = read_to_string(module_gdns_path)?;
    let module_gdns_split = module_gdns_string.split("\r\n").collect::<Vec<&str>>();
    assert_eq!(
        module_gdns_split[2],
        "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
    assert_eq!(module_gdns_split[6], "resource_name = \"Player\"");
    assert_eq!(module_gdns_split[7], "class_name = \"Player\"");

    cleanup_test_files();

    Ok(())
}

/// Creates a library and then creates multiple modules within the library and
/// checks that all files are there and their values are what they should be.
#[test]
fn create_multiple_modules_library_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd_new = Command::new("cargo");
    cmd_new
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd_new.assert().success();

    set_current_dir("platformer_modules")?;

    // 2. Assert that the create commands were successful.
    let mut cmd_create_player = Command::new("cargo");
    cmd_create_player
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Player");
    cmd_create_player.assert().success();

    let mut cmd_create_enemy = Command::new("cargo");
    cmd_create_enemy
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Enemy");
    cmd_create_enemy.assert().success();

    let mut cmd_create_level = Command::new("cargo");
    cmd_create_level
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Level");
    cmd_create_level.assert().success();

    // 3. Assert that the modules have a mod file.
    let player_module_mod_path = Path::new("src/player.rs");
    let enemy_module_mod_path = Path::new("src/enemy.rs");
    let level_module_mod_path = Path::new("src/level.rs");
    assert_eq!(player_module_mod_path.exists(), true);
    assert_eq!(enemy_module_mod_path.exists(), true);
    assert_eq!(level_module_mod_path.exists(), true);

    // 4. Assert that the modules were added to the lib file.
    let lib_file_string = read_to_string("src/lib.rs")?;
    let lib_file_split = lib_file_string.split("\n").collect::<Vec<&str>>();
    assert_eq!(lib_file_split[0], "mod enemy;");
    assert_eq!(lib_file_split[1], "mod level;");
    assert_eq!(lib_file_split[2], "mod player;");
    assert_eq!(
        lib_file_split[6].trim(),
        "handle.add_class::<player::Player>();"
    );
    assert_eq!(
        lib_file_split[7].trim(),
        "handle.add_class::<level::Level>();"
    );
    assert_eq!(
        lib_file_split[8].trim(),
        "handle.add_class::<enemy::Enemy>();"
    );

    // 5: Assert that the modules were added to the config.
    let config = read_to_string("godot-rust-cli.json")?;
    let config_json: Value = serde_json::from_str(&config)?;
    assert_eq!(config_json["modules"], json!(["Player", "Enemy", "Level"]));

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a library and then creates multiple modules within the Godot project
/// and checks that all files are there and their values are what they should be.
#[test]
fn create_multiple_modules_godot_structure() -> Result<(), Box<dyn Error>> {
    init_test();

    // 1. Assert that the new command was successful.
    let mut cmd_new = Command::new("cargo");
    cmd_new
        .arg("run")
        .arg("--manifest-path=../Cargo.toml")
        .arg("new")
        .arg("PlatformerModules")
        .arg("platformer")
        .arg("--skip-build");
    cmd_new.assert().success();

    set_current_dir("platformer_modules")?;

    // 2. Assert that the create commands were successful.
    let mut cmd_create_player = Command::new("cargo");
    cmd_create_player
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Player");
    cmd_create_player.assert().success();

    let mut cmd_create_enemy = Command::new("cargo");
    cmd_create_enemy
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Enemy");
    cmd_create_enemy.assert().success();

    let mut cmd_create_level = Command::new("cargo");
    cmd_create_level
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Level");
    cmd_create_level.assert().success();

    set_current_dir("../")?;

    // 3. Assert that the gdns files for the modules were created.
    let player_module_gdns_path = Path::new("platformer/rust_modules/player.gdns");
    let enemy_module_gdns_path = Path::new("platformer/rust_modules/enemy.gdns");
    let level_module_gdns_path = Path::new("platformer/rust_modules/level.gdns");
    assert_eq!(player_module_gdns_path.exists(), true);
    assert_eq!(enemy_module_gdns_path.exists(), true);
    assert_eq!(level_module_gdns_path.exists(), true);

    // 4. Assert that the gnds files have the correct contents.
    let player_module_gdns_string = read_to_string(player_module_gdns_path)?;
    let player_module_gdns_split = player_module_gdns_string
        .split("\r\n")
        .collect::<Vec<&str>>();
    assert_eq!(
        player_module_gdns_split[2],
        "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
    assert_eq!(player_module_gdns_split[6], "resource_name = \"Player\"");
    assert_eq!(player_module_gdns_split[7], "class_name = \"Player\"");

    let enemy_module_gdns_string = read_to_string(enemy_module_gdns_path)?;
    let enemy_module_gdns_split = enemy_module_gdns_string
        .split("\r\n")
        .collect::<Vec<&str>>();
    assert_eq!(
        enemy_module_gdns_split[2],
        "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
    assert_eq!(enemy_module_gdns_split[6], "resource_name = \"Enemy\"");
    assert_eq!(enemy_module_gdns_split[7], "class_name = \"Enemy\"");

    let level_module_gdns_string = read_to_string(level_module_gdns_path)?;
    let level_module_gdns_split = level_module_gdns_string
        .split("\r\n")
        .collect::<Vec<&str>>();
    assert_eq!(
        level_module_gdns_split[2],
        "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
    assert_eq!(level_module_gdns_split[6], "resource_name = \"Level\"");
    assert_eq!(level_module_gdns_split[7], "class_name = \"Level\"");

    cleanup_test_files();

    Ok(())
}
