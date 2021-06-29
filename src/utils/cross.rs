use crate::log_utils::{log_info_to_console, log_success_to_console};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Describes the structure of the Cross.toml configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cross {
    /// The override for the windows docker image.
    #[serde(rename = "target.x86_64-pc-windows-gnu")]
    windows: Option<CrossWindows>,
}

/// Describes the structure of the windows override section.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossWindows {
    /// The name:tag of the image to use to override the windows image.
    image: String,
}

/// Returns the path to the Cross.toml configuration file.
pub fn get_path_to_cross_config_file() -> PathBuf {
    let current_dir = current_dir()
        .expect("Unable to get current directory while getting the path to the Cross.toml file.");
    return Path::new(&current_dir).join("Cross.toml");
}

/// Creates the initial Cross configuration and saves it to a toml file if it
/// doesn't already exist.
pub fn create_cross_config_file_if_not_exists() {
    let cross_config_path = get_path_to_cross_config_file();

    if !cross_config_path.exists() {
        let cross = Cross { windows: None };

        save_cross_config_to_file(&cross);

        log_info_to_console("[cross] Created cross configuration file.");
    }
}

/// Saves the Cross configuration to a file.
///
/// # Arguments
///
/// `cross_config` - The cross configuration to save.
pub fn save_cross_config_to_file(cross_config: &Cross) {
    let cross_config_path = get_path_to_cross_config_file();
    let cross_config_as_string =
        toml::to_string_pretty(&cross_config).expect("Unable to create Cross configuration file.");
    write(
        cross_config_path,
        cross_config_as_string.replace("\"", "").replace("'", "\""),
    )
    .expect("Unable to write Cross configuration file.");
}

/// Returns the Cross configuration as an object.
pub fn get_cross_config_as_object() -> Cross {
    let cross_config_file_path = get_path_to_cross_config_file();
    let cross_config_as_string =
        read_to_string(cross_config_file_path).expect("Unable to read Cross configuration file.");

    return toml::from_str(&cross_config_as_string)
        .expect("Unable to parse Cross configuration file.");
}

/// Adds an image override to the Cross configuration file if necessary.
///
/// # Arguments
///
/// `platform` - The platform being added.
pub fn add_image_override_if_necessary(platform: &str) {
    create_cross_config_file_if_not_exists();

    // Add the windows image override to the the Cross.toml
    // configuration file.
    add_cross_config_image_override(platform);

    // Copy the windows override docker file to the library directory.
    create_docker_dir_if_not_exists(platform);
    copy_platform_dockerfile_to_library_if_needed(platform);

    // Build the docker image so it can be used by cross.
    build_docker_image_for_platform(platform);
}

/// Creates docker directory in the library if it needs to be created.
///
/// # Argument
///
/// `platform` - The platform being added.
fn create_docker_dir_if_not_exists(platform: &str) {
    let current_dir_path = current_dir()
        .expect("Unable to get current directory while creating the docker directory.");
    let docker_dir_path = current_dir_path.join("docker");

    match platform {
        "windows" => {
            if !docker_dir_path.exists() {
                std::fs::create_dir(current_dir_path.join("docker"))
                    .expect("Unable to create docker directory in library.");
            }
        }
        _ => (),
    }
}

/// Copies the dockerfile for the platform to build for to the library's
/// docker directory.
///
/// # Arguments
///
/// `platform` - The platform to add.
fn copy_platform_dockerfile_to_library_if_needed(platform: &str) {
    let current_dir = std::env::current_dir().expect("Unable to get current directory.");

    let docker_file_source = match platform {
        "windows" => include_str!("../../docker/Dockerfile.x86_64-pc-windows-gnu"),
        _ => "",
    };

    if !docker_file_source.is_empty() {
        let docker_file_name = match platform {
            "windows" => "Dockerfile.x86_64-pc-windows-gnu",
            _ => "",
        };
        let docker_file_destination = current_dir.join("docker").join(&docker_file_name);

        std::fs::write(docker_file_destination, docker_file_source)
            .expect("Unable to copy platform dockerfile to library.");
        log_info_to_console("[cross] Copied docker image to library directory.");
    }
}

/// Adds an override to the Cross configuration file.
///
/// # Arguments
///
/// `platform` - The platform to add the override for.
fn add_cross_config_image_override(platform: &str) {
    match platform {
        "windows" => {
            let cross_windows_override = CrossWindows {
                image: "godot-rust-cli-platform-windows:v2".to_string(),
            };
            let mut cross_config = get_cross_config_as_object();
            cross_config.windows = Some(cross_windows_override);
            save_cross_config_to_file(&cross_config);
            log_info_to_console("[cross] Added docker image override for windows.");
        }
        _ => (),
    }
}

/// Builds the docker image for the specified platform.
///
/// # Arguments
///
/// `platform` - The platform to build the docker image for.
fn build_docker_image_for_platform(platform: &str) {
    let current_directory =
        current_dir().expect("Unable to get current directory while building images.");
    let docker_images_directory = current_directory.join("docker");

    match platform {
        "windows" => {
            log_info_to_console("[cross] Building windows override docker image.");
            let windows_docker_image_path =
                docker_images_directory.join("Dockerfile.x86_64-pc-windows-gnu");
            let mut build_windows_image_command = Command::new("docker");
            build_windows_image_command
                .arg("build")
                .arg("-f")
                .arg(&windows_docker_image_path)
                .arg("-t")
                .arg("godot-rust-cli-platform-windows:v2")
                .arg(".");

            build_windows_image_command
                .status()
                .expect("Unable to build windows docker image.");
            log_success_to_console("[cross] Built windows override docker image.");
        }
        _ => (),
    }
}
