use dunce::canonicalize;
use std::{
    env::current_dir,
    io::Result,
    path::{Path, PathBuf},
    process::exit,
};

use crate::config_utils::get_path_to_config_file;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};

/// Exits if the current path is not the path to the library's directory.
pub fn exit_if_not_lib_dir() {
    if !get_path_to_config_file().exists() {
        log_styled_message_to_console(
            "This command must be used from the library directory",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Returns the path to the directory that contains the dynamic library for the
/// provided target.
///
/// # Arguments
///
/// `target` - The target to get the dynamic library path for.
/// `is_release` - Indicates whether it is a release build or not.
pub fn get_dynamic_library_directory_path(target: &str, is_release: bool) -> PathBuf {
    let current_dir =
        std::env::current_dir().expect("Unable to get the path to the current directory.");

    let debug_or_release_dir_name = if is_release { "release" } else { "debug" };

    return Path::new(&current_dir)
        .join(&target)
        .join(&debug_or_release_dir_name);
}

/// Returns the absolute path from a PathBuf.
///
/// # Arguments
///
/// `buf` - The PathBuf to return the absolute path of.
pub fn get_absolute_path(buf: &PathBuf) -> PathBuf {
    return if !buf.is_absolute() {
        absolute_path(buf)
            .expect("Unable to create absolute path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&buf).to_path_buf()
    };
}

/// Returns the absolute path from a relative path.
///
/// # Arguments
///
/// `path` - The relative path to get the absolute path of.
fn absolute_path<P>(path: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir()?.join(path)
    };

    match canonicalize(&absolute_path) {
        Ok(v) => absolute_path = v,
        Err(_e) => {
            let parent = &absolute_path
                .parent()
                .expect("Unable to get the absolute path's parent");
            let basename = &absolute_path
                .file_stem()
                .expect("Unable to get the absolute path's basename");
            let parent_canon =
                canonicalize(parent).expect("Unable to canonicalize parent directory");

            absolute_path = parent_canon.join(basename);
        }
    }

    Ok(absolute_path)
}
