// use assert_cmd::prelude::*;

// use serde_json::{json, Value};
// use std::env::set_current_dir;
// use std::error::Error;
// use std::fs::read_to_string;
// use std::path::Path;
// use std::process::Command;

// mod test_utilities;
// use test_utilities::{cleanup_test_files, init_test};

// /// Creates a library and a module and then destroys it and checks to make sure
// /// the library structure is correct.
// #[test]
// fn destroy_module_library_structure() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new = Command::new("cargo");
//     cmd_new
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the create command was successful.
//     let mut cmd_create = Command::new("cargo");
//     cmd_create
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Player");
//     cmd_create.assert().success();

//     // 3. Assert that the destroy command was successful.
//     let mut cmd_destroy = Command::new("cargo");
//     cmd_destroy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Player");
//     cmd_destroy.assert().success();

//     // 4. Assert that the module no longer has a mod file.
//     let module_mod_path = Path::new("src/player.rs");
//     assert_eq!(module_mod_path.exists(), false);

//     // 5. Assert that the module was removed from the lib file.
//     let lib_file_string = read_to_string("src/lib.rs")?;
//     let lib_file_split = lib_file_string
//         .split("\n")
//         .map(|x| x.replace("\r", ""))
//         .collect::<Vec<String>>();
//     assert_eq!(lib_file_split[0], "use gdnative::prelude::*;");
//     assert_eq!(lib_file_split[1], "");
//     assert_eq!(lib_file_split[2], "fn init(handle: InitHandle) {}");
//     assert_eq!(lib_file_split[3], "");
//     assert_eq!(lib_file_split[4], "godot_init!(init);");

//     // 6. Assert that the module was removed from the config.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(config_json["modules"], json!([]));

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and a module and then destroys it and checks to make sure
// /// the Godot project structure is correct.
// #[test]
// fn destroy_module_godot_structure() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new = Command::new("cargo");
//     cmd_new
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the create command was successful.
//     let mut cmd_create = Command::new("cargo");
//     cmd_create
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Player");
//     cmd_create.assert().success();

//     // 3. Assert that the destroy command was successful.
//     let mut cmd_destroy = Command::new("cargo");
//     cmd_destroy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Player");
//     cmd_destroy.assert().success();

//     set_current_dir("../")?;

//     // 4. Assert that the module no longer has a gdns file.
//     let module_gdns_path = Path::new("platformer/gdnative/player.gdns");
//     assert_eq!(module_gdns_path.exists(), false);

//     Ok(())
// }

// /// Creates a library and 5 modules and then destroys 2 of them and checks to
// /// make sure the library structure is correct.
// #[test]
// fn destroy_modules_library_structure() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new = Command::new("cargo");
//     cmd_new
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the create commands were successful.
//     let mut cmd_create_player = Command::new("cargo");
//     cmd_create_player
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Player");
//     cmd_create_player.assert().success();
//     let mut cmd_create_enemy = Command::new("cargo");
//     cmd_create_enemy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Enemy");
//     cmd_create_enemy.assert().success();
//     let mut cmd_create_level = Command::new("cargo");
//     cmd_create_level
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Level");
//     cmd_create_level.assert().success();
//     let mut cmd_create_environment = Command::new("cargo");
//     cmd_create_environment
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Environment");
//     cmd_create_environment.assert().success();
//     let mut cmd_create_space = Command::new("cargo");
//     cmd_create_space
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Space");
//     cmd_create_space.assert().success();

//     // 3. Assert that the destroy commands were successful.
//     let mut cmd_destroy_enemy = Command::new("cargo");
//     cmd_destroy_enemy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Enemy");
//     cmd_destroy_enemy.assert().success();
//     let mut cmd_destroy_space = Command::new("cargo");
//     cmd_destroy_space
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Space");
//     cmd_destroy_space.assert().success();

//     // 4. Assert that the modules that weren't destroyed have mod files.
//     let player_module_mod_path = Path::new("src/player.rs");
//     let level_module_mod_path = Path::new("src/level.rs");
//     let environment_module_mod_path = Path::new("src/environment.rs");
//     assert_eq!(player_module_mod_path.exists(), true);
//     assert_eq!(level_module_mod_path.exists(), true);
//     assert_eq!(environment_module_mod_path.exists(), true);

//     // 5. Assert that the modules that were deleted no longer have mod files.
//     let enemy_module_mod_path = Path::new("src/enemy.rs");
//     let space_module_mod_path = Path::new("src/space.rs");
//     assert_eq!(enemy_module_mod_path.exists(), false);
//     assert_eq!(space_module_mod_path.exists(), false);

//     // 6. Assert that the modules were removed from the lib file.
//     let lib_file_string = read_to_string("src/lib.rs")?;
//     let lib_file_split = lib_file_string
//         .split("\n")
//         .map(|x| x.replace("\r", ""))
//         .collect::<Vec<String>>();
//     assert_eq!(lib_file_split[0], "mod environment;");
//     assert_eq!(lib_file_split[1], "mod level;");
//     assert_eq!(lib_file_split[2], "mod player;");
//     assert_eq!(lib_file_split[3], "use gdnative::prelude::*;");
//     assert_eq!(lib_file_split[4], "");
//     assert_eq!(lib_file_split[5], "fn init(handle: InitHandle) {");
//     assert_eq!(
//         lib_file_split[6].trim(),
//         "handle.add_class::<player::Player>();"
//     );
//     assert_eq!(
//         lib_file_split[7].trim(),
//         "handle.add_class::<environment::Environment>();"
//     );
//     assert_eq!(
//         lib_file_split[8].trim(),
//         "handle.add_class::<level::Level>();"
//     );
//     assert_eq!(lib_file_split[9], "}");
//     assert_eq!(lib_file_split[10], "");
//     assert_eq!(lib_file_split[11], "godot_init!(init);");

//     // 7. Assert that the modules were removed from the config.
//     let config = read_to_string("godot-rust-cli.json")?;
//     let config_json: Value = serde_json::from_str(&config)?;
//     assert_eq!(
//         config_json["modules"],
//         json!(["Player", "Level", "Environment"])
//     );

//     set_current_dir("../")?;

//     cleanup_test_files();

//     Ok(())
// }

// /// Creates a library and 5 modules and then destroys 2 of them and checks to
// /// make sure the Godot project structure is correct.
// #[test]
// fn destroy_modules_godot_structure() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new = Command::new("cargo");
//     cmd_new
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the create commands were successful.
//     let mut cmd_create_player = Command::new("cargo");
//     cmd_create_player
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Player");
//     cmd_create_player.assert().success();
//     let mut cmd_create_enemy = Command::new("cargo");
//     cmd_create_enemy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Enemy");
//     cmd_create_enemy.assert().success();
//     let mut cmd_create_level = Command::new("cargo");
//     cmd_create_level
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Level");
//     cmd_create_level.assert().success();
//     let mut cmd_create_environment = Command::new("cargo");
//     cmd_create_environment
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Environment");
//     cmd_create_environment.assert().success();
//     let mut cmd_create_space = Command::new("cargo");
//     cmd_create_space
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Space");
//     cmd_create_space.assert().success();

//     // 3. Assert that the destroy commands were successful.
//     let mut cmd_destroy_enemy = Command::new("cargo");
//     cmd_destroy_enemy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Enemy");
//     cmd_destroy_enemy.assert().success();
//     let mut cmd_destroy_space = Command::new("cargo");
//     cmd_destroy_space
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Space");
//     cmd_destroy_space.assert().success();

//     set_current_dir("../")?;

//     // 4. Assert that the modules that weren't deleted have a gdns file still.
//     let player_gdns_path = Path::new("platformer/gdnative/player.gdns");
//     let level_gdns_path = Path::new("platformer/gdnative/level.gdns");
//     let environment_gdns_path = Path::new("platformer/gdnative/environment.gdns");
//     assert_eq!(player_gdns_path.exists(), true);
//     assert_eq!(level_gdns_path.exists(), true);
//     assert_eq!(environment_gdns_path.exists(), true);

//     // 4. Assert that the modules that were deleted no longer have a gdns file.
//     let enemy_gdns_path = Path::new("platformer/gdnative/enemy.gdns");
//     let space_gdns_path = Path::new("platformer/gdnative/space.gdns");
//     assert_eq!(enemy_gdns_path.exists(), false);
//     assert_eq!(space_gdns_path.exists(), false);

//     Ok(())
// }

// /// Creates a library and then a module and then moves the module from the
// /// gdnative folder into its own folder and makes sure that it still gets
// /// deleted.
// #[test]
// fn destroy_moved_module_godot_structure() -> Result<(), Box<dyn Error>> {
//     init_test();

//     // 1. Assert that the new command was successful.
//     let mut cmd_new = Command::new("cargo");
//     cmd_new
//         .arg("run")
//         .arg("--manifest-path=../Cargo.toml")
//         .arg("new")
//         .arg("PlatformerModules")
//         .arg("platformer")
//         .arg("--skip-build");
//     cmd_new.assert().success();

//     set_current_dir("platformer_modules")?;

//     // 2. Assert that the create command was successful.
//     let mut cmd_create = Command::new("cargo");
//     cmd_create
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("create")
//         .arg("Player");
//     cmd_create.assert().success();

//     Command::new("mkdir")
//         .arg("../platformer/player")
//         .output()
//         .expect("Unable to create player dir");

//     Command::new("mv")
//         .arg("../platformer/gdnative/player.gdns")
//         .arg("../platformer/player/player.gdns")
//         .output()
//         .expect("Unable to move player script");

//     // 3. Assert that the destroy command was successful.
//     let mut cmd_destroy = Command::new("cargo");
//     cmd_destroy
//         .arg("run")
//         .arg("--manifest-path=../../Cargo.toml")
//         .arg("destroy")
//         .arg("Player");
//     cmd_destroy.assert().success();

//     set_current_dir("../")?;

//     let module_gdns_path_old = Path::new("platformer/gdnative/player.gdns");
//     let module_gdns_path_new = Path::new("platformer/player/player.gdns");

//     assert_eq!(module_gdns_path_old.exists(), false);
//     assert_eq!(module_gdns_path_new.exists(), false);

//     cleanup_test_files();

//     Ok(())
// }
