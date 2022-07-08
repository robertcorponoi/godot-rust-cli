use std::path::PathBuf;

/// The structure of a gdns file that is used to describe a script.
#[derive(Debug)]
pub struct GdnsFile {
    /// The name of the script.
    pub name: String,
    /// The path to the gdnlib file.
    pub gdnlib_path: String,
}

impl GdnsFile {
    /// Creates a new instance of the GdnsFile with the provided
    /// properties.
    ///
    /// # Arguments
    ///
    /// `name` - The pascal case version of the script name.
    /// `gdnlib_path` - The path to the gdnlib file.
    pub fn new(name: &str, gdnlib_path: &str) -> GdnsFile {
        GdnsFile {
            name: name.to_string(),
            gdnlib_path: gdnlib_path.to_string(),
        }
    }

    /// Returns the GdnsFile as a pretty printed string.
    pub fn to_string(&mut self) -> String {
        let gdnlib_file_string = format!(
            r#"[gd_resource type="NativeScript" load_steps=2 format=2]

[ext_resource path="res://{}.gdnlib" type="GDNativeLibrary" id=1]

[resource]

resource_name = "{}"
class_name = "{}"
library = ExtResource( 1 )
"#,
            self.gdnlib_path, self.name, self.name
        );

        gdnlib_file_string
    }

    /// Writes the provided GdnsFile to the gdns file.
    ///
    /// # Arguments
    ///
    /// `path` - The path to write the file to.
    pub fn write(&mut self, path: PathBuf) {
        std::fs::write(path, self.to_string()).expect("Unable to update contents of the gdns file");
    }
}
