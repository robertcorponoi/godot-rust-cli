use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The structure of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gdnlib {
    general: GdnlibGeneral,
    entry: HashMap<String, String>,
    dependencies: HashMap<String, Vec<String>>,
}

/// The structure of the general section of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GdnlibGeneral {
    pub singleton: bool,
    pub load_once: bool,
    pub symbol_prefix: String,
    pub reloadable: bool,
}

impl Gdnlib {
    /// Creates a new instance of the Gdnlib with the default properties.
    ///
    /// # Arguments
    ///
    /// `rust_library_name_normalized`  - The snake_case version of the Rust library name.
    /// `is_plugin`                     - Indicates whether the Gdnlib is for a Godot project that is a plugin or not.
    pub fn new(rust_library_name_normalized: &str, is_plugin: bool) -> Gdnlib {
        // The base path to the binaries for each operating system. If the
        // Godot project is a plugin this will always be under the `addons`
        // directory. Otherwise, the base path is just the root of the Godot
        // project.
        let gdnlib_base_path = if is_plugin {
            format!("res://addons/{}", rust_library_name_normalized)
        } else {
            "res:/".to_owned()
        };

        // Defines the path to the various popular binaries that could exist.
        // We're allowed to define them all here as Godot (currently) doesn't
        // complain if they're not being used.
        let osx_bin_path = format!(
            "{}/gdnative/bin/macos/lib{}.dylib",
            gdnlib_base_path, rust_library_name_normalized
        );
        let windows_bin_path = format!(
            "{}/gdnative/bin/windows/{}.dll",
            gdnlib_base_path, rust_library_name_normalized
        );
        let linux_bin_path = format!(
            "{}/gdnative/bin/linux/lib{}.so",
            gdnlib_base_path, rust_library_name_normalized
        );
        let android_arm_bin_path = format!(
            "{}/gdnative/bin/android/aarch64-linux-android/lib{}.so",
            gdnlib_base_path, rust_library_name_normalized
        );
        let android_64_bin_path = format!(
            "{}/gdnative/bin/android/x86_64-linux-android/lib{}.so",
            gdnlib_base_path, rust_library_name_normalized
        );

        // The locations to the binaries above are inserted into a `HashMap`
        // which is then used as the contents under the `[entries]` tag in the
        // gdnlib file.
        let mut entries: HashMap<String, String> = HashMap::new();
        entries.insert("OSX.64".to_owned(), osx_bin_path);
        entries.insert("Windows.64".to_owned(), windows_bin_path);
        entries.insert("X11.64".to_owned(), linux_bin_path);
        entries.insert("Android.arm64-v8a".to_owned(), android_arm_bin_path);
        entries.insert("Android.x86_64".to_owned(), android_64_bin_path);

        // Another tag we have to create the content for is the
        // `[dependencies]` tag. Here we define the same popular operating
        // systems as above and set them to an empty array since we don't
        // start with any dependencies yet.
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        deps.insert("OSX.64".to_owned(), vec![]);
        deps.insert("Windows.64".to_owned(), vec![]);
        deps.insert("X11.64".to_owned(), vec![]);
        deps.insert("Android.arm64-v8a".to_owned(), vec![]);
        deps.insert("Android.x86_64".to_owned(), vec![]);

        Gdnlib {
            general: GdnlibGeneral {
                singleton: false,
                load_once: true,
                symbol_prefix: "godot_".to_string(),
                reloadable: true,
            },
            entry: entries,
            dependencies: deps,
        }
    }

    /// Returns the Gdnlib as a pretty printed toml string.
    ///
    /// # Arguments
    ///
    /// `self` - The current Gdnlib object.
    pub fn to_string(&mut self) -> String {
        // We have to replace single quotes with double quotes here as that is
        // what is expected by Godot.
        // TOOD: Replace the quotes earlier in the process.
        toml::to_string_pretty(self)
            .expect("Unable to convert gdnlib to string")
            .replace("'", "\"")
    }
}
