use crate::log_utils::log_info_to_console;
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
    windows: Option<CrossImage>,
    // /// The override for the linux docker image.
    // #[serde(rename = "target.x86_64-unknown-linux-gnu")]
    // linux: Option<CrossImage>,
}

/// Describes the structure of an image definition in the cross config.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossImage {
    /// The name:tag of the image to use to override the default image.
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

    // Create the default config with no overrides.
    if !cross_config_path.exists() {
        let cross = Cross {
            windows: None,
            // linux: None,
        };

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

    // For some reason toml wraps our tags so we need to replace those and
    // also replace any instances of single quotes with double quotes.
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

/// Adds an image override to the cross configuration for the provided platform.
///
/// # Arguments
///
/// `platform` - The platform to add the override for.
pub fn add_image_override_for_platform(platform: &str) {
    // Since this could be the first platform being added we want to create the
    // cross config if it doesn't exist yet.
    create_cross_config_file_if_not_exists();

    // Add the entry for this platform into the cross configuration file.
    add_docker_image_to_cross_config(platform);

    // Create the docker directory in the library to hold the docker files for
    // the custom images if it doesn't already exist.
    create_docker_dir_if_not_exists();

    // Now we want to copy the docker image override from the cli to the
    // library directory.
    copy_platform_dockerfile_to_library(platform);

    // Build the docker image so it can be used by cross when cross compiling.
    build_docker_image_for_platform(platform);
}

/// Creates docker directory in the library if it doesn't exist yet. The docker
/// directory holds all of the docker images needed to cross compile the
/// library.
fn create_docker_dir_if_not_exists() {
    let current_dir_path = current_dir()
        .expect("Unable to get current directory while creating the docker directory.");
    let docker_dir_path = current_dir_path.join("docker");

    if !docker_dir_path.exists() {
        std::fs::create_dir(current_dir_path.join("docker"))
            .expect("Unable to create docker directory in library.");
    }
}

/// Copies the docker file for the platform to build for to the library's
/// docker directory.
///
/// # Arguments
///
/// `platform` - The platform to add.
fn copy_platform_dockerfile_to_library(platform: &str) {
    let current_dir = std::env::current_dir().expect("Unable to get current directory.");

    // Get the contents of the docker file that needs to be copied from the
    // cli's local docker directory.
    let docker_file_source = match platform {
        "windows" => Some(include_str!(
            "../../docker/Dockerfile.x86_64-pc-windows-gnu"
        )),
        // "linux" => Some(include_str!("../../docker/Dockerfile.x86_64-unknown-linux-gnu")),
        _ => None,
    };

    // Set the name of the docker file to use when copying the contents over to
    // the library's docker directory.
    let docker_file_name = match platform {
        "windows" => Some("Dockerfile.x86_64-pc-windows-gnu"),
        // "linux" => Some("Dockerfile.x86_64-unknown-linux-gnu"),
        _ => None,
    };

    match (docker_file_source, docker_file_name) {
        (Some(docker_file_source_value), Some(docker_file_name_value)) => {
            // The directory to copy the contents of the docker file over to.
            let docker_file_destination = current_dir.join("docker").join(&docker_file_name_value);

            std::fs::write(docker_file_destination, docker_file_source_value)
                .expect("Unable to copy platform dockerfile to library.");
            log_info_to_console("[cross] Copied docker image to library directory.");
        }
        _ => (),
    }
}

/// Adds an image that should be used over the default cross image in the cross
/// configuration file.
///
/// # Arguments
///
/// `platform` - The platform to add the image for.
fn add_docker_image_to_cross_config(platform: &str) {
    // Get the configuration so that we can save the image override to it.
    let mut cross_config = get_cross_config_as_object();

    // Depending on the platform being added we create the entry for the docker
    // file in the cross config.
    match platform {
        "windows" => {
            let cross_windows = CrossImage {
                image: get_docker_image_name(platform).unwrap().to_string(),
            };
            cross_config.windows = Some(cross_windows);
        }
        // "linux" => {
        //     let cross_linux = CrossImage {
        //         image: get_docker_image_name(platform).unwrap().to_string(),
        //     };
        //     cross_config.linux = Some(cross_linux);
        // }
        _ => (),
    };

    // Save the new value to config.
    save_cross_config_to_file(&mut cross_config);

    log_info_to_console(&format!("[cross] Added {} docker image.", &platform));
}

/// Builds the custom docker image for the specified platform.
///
/// # Arguments
///
/// `platform` - The platform to build the docker image for.
fn build_docker_image_for_platform(platform: &str) {
    // Get the path to where the docker images are stored in the library
    // directory.
    let current_directory =
        current_dir().expect("Unable to get current directory while building images.");
    let docker_images_directory = current_directory.join("docker");

    // Get the path to the docker image in the library docker directory.
    let docker_file_path = match platform {
        "windows" => Some(docker_images_directory.join("Dockerfile.x86_64-pc-windows-gnu")),
        "linux" => Some(docker_images_directory.join("Dockerfile.x86_64-unknown-linux-gnu")),
        _ => None,
    };

    // Create the tag for the image.
    let docker_image_tag = match platform {
        "windows" => Some("godot-rust-cli-platform-windows:v1"),
        "linux" => Some("godot-rust-cli-platform-linux:v1"),
        _ => None,
    };

    log_info_to_console(&format!("[cross] Building {} docker image.", &platform));

    match (docker_file_path, docker_image_tag) {
        (Some(docker_file_path_value), Some(docker_image_tag_value)) => {
            // Run the docker build command for the image passing in the docker file
            // and the tag.
            let mut docker_build_command = Command::new("docker");
            docker_build_command
                .arg("build")
                .arg("-f")
                .arg(&docker_file_path_value)
                .arg("-t")
                .arg(&docker_image_tag_value)
                .arg(".");

            docker_build_command
                .status()
                .expect("Unable to build windows docker image.");

            log_info_to_console(&format!(
                "[cross] Finished building {} docker image.",
                &platform
            ));
        }
        _ => (),
    };
}

/// Returns the name for the docker image used for the provided platform.
///
/// # Arguments
///
/// `platform` - The platform to get the docker image name for.
pub fn get_docker_image_name(platform: &str) -> Option<&str> {
    return match platform {
        "windows" => Some("godot-rust-cli-platform-windows:v1"),
        _ => None,
    };
}
