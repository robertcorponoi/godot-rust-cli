## 0.3.1 / 2020-06-21
- Made the authors field in the parsed Cargo.toml to be optional to account for setups where the user might not have a name or email.

## 0.3.0 / 2020-06-15
- Fixed issue with Godot throwing an error because it couldn't find the dynamic library before a build was run.
- Changed plugin architecture so now plugins are libraries in themselves with modules that are a part of that plugin.
- Modules can now be moved around in the Godot project.
- Migrated from Travis CI to GitHub Actions.
- Improved error messages and code comments.

## 0.2.0 / 2021-05-25
- Updated crates
- Updated gdnative to 0.9.3
- Changed from using project.toml to godot-rust-cli.json

## 0.1.2 / 2021-02-11
- Moved docs into their own branch.

## 0.1.1 / 2021-02-8
- Removed unnecessary dependencies.

## 0.1.0 / 2021-02-08
- Initial release