use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

/// Creates a library and a module and runs the build command ands checks to
/// make sure that the default debug build was created.
#[test]
fn build_default_to_debug_build() -> Result<(), Box<dyn Error>> {
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

    // 3. Assert that the build command was successful.
    let mut cmd_build = Command::new("cargo");
    cmd_build
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("build");
    cmd_build.assert().success();

    // 4. Assert the path to the debug build of the dynamic library is correct.
    let dynamic_library_debug_path = format!(
        "target/{}/debug//{}platformer_modules{}",
        env!("TARGET"),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );

    assert_eq!(Path::new(&dynamic_library_debug_path).exists(), true);

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a library and a module and runs the build command with the release
/// flag ands checks to make sure that the release build was created.
#[test]
fn build_create_release_build() -> Result<(), Box<dyn Error>> {
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

    // 3. Assert that the build command was successful.
    let mut cmd_build = Command::new("cargo");
    cmd_build
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("build")
        .arg("--release");
    cmd_build.assert().success();

    // 4. Assert the path to the debug build of the dynamic library is correct.
    let dynamic_library_debug_path = format!(
        "target/{}/release//{}platformer_modules{}",
        env!("TARGET"),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );

    assert_eq!(Path::new(&dynamic_library_debug_path).exists(), true);

    set_current_dir("../")?;

    cleanup_test_files();

    Ok(())
}

/// Creates a library and a module and runs the build command ands checks to
/// make sure that the dynamic library was created and copied to the Godot
/// project.
#[test]
fn build_godot_project() -> Result<(), Box<dyn Error>> {
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

    // 3. Assert that the build command was successful.
    let mut cmd_build = Command::new("cargo");
    cmd_build
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("build");
    cmd_build.assert().success();

    set_current_dir("../")?;

    // 4. Assert that the dynamic library was copied over.
    let dynamic_library_name = format!(
        "platformer/gdnative/bin/{}/{}platformer_modules{}",
        std::env::consts::OS.to_lowercase(),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );

    let dynamic_library_path = Path::new(&dynamic_library_name);
    assert_eq!(dynamic_library_path.exists(), true);

    cleanup_test_files();

    Ok(())
}
