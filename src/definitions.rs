use serde::{Deserialize, Serialize};

/// The structure of a library's Cargo.toml file.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToml {
  /// A reference to the CargoPackage struct.
  pub package: CargoPackage,
  /// A reference to the CargoLib struct. If it doesn't exist
  /// (which it most likely won't) then we use the `create_cargo_lib` function
  /// to set the default value.
  #[serde(default = "create_cargo_lib")]
  pub lib: CargoLib,
  /// A reference to the CargoDependencies struct.
  pub dependencies: CargoDependencies,
}

/// The fields of the Cargo.toml that are under the [package] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoPackage {
  pub name: String,
  pub version: String,
  pub authors: Vec<String>,
  pub edition: String,
}

/// The fields of the Cargo.toml that are under the [lib] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoLib {
  #[serde(rename = "crate-type")]
  pub crate_type: Vec<String>,
}

/// The fields of the Cargo.toml that are under the [dependencies] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoDependencies {
  #[serde(default = "add_gdnative_dep")]
  pub gdnative: String,
}

/// Returns the contents of what should appear under the [lib] tag. This is
/// used by the Cargo struct to create the default value for the [lib] tag
/// if no value is present.
fn create_cargo_lib() -> CargoLib {
  return CargoLib {
    crate_type: vec!["cdylib".to_string()],
  };
}

/// Returns the gdnative dependency to add to the Cargo.toml dependencies. This
/// is used by the CargoDependencies struct to add the gdnative dependency that
/// is necessary.
fn add_gdnative_dep() -> String {
  return String::from("0.9.1");
}