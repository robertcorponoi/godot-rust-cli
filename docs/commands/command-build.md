## build

Builds the library to generate the dynamic libraries and then copies them over to the Godot project's `/gdnative/bin` directory.

**Note:** This command has to be run within the library directory and will fail outside of it.

**Usage**

```sh
godot-rust-cli build [-w, --watch] [-r, --release] [-a, --all]
```

where:

- `w, --watch` can be passed optionally to have godot-rust-cli watch the library for changes and rebuild + copy the files over auotmatically.

- `r, --release` can be passed optionally to have godot-rust-cli create a release build instead of the default debug build. This is passed directly to the `cargo build` command so you can check the documentation on that on the cargo build [documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html).

- `a, --all` can be passed optionally to have godot-rust-cli build for all of the platforms defined in the configuration. This is a more advanced feature so make sure to check the documentation on [platforms](./command-platform.md) first.

**Examples:**

- Building the library normally:

```sh
godot-rust-cli build
```

- Building the library and watching for changes to trigger automatic rebuilds:

```sh
godot-rust-cli build --watch
```

- Building the release build of the library:

```sh
godot-rust-cli build --release
```

[Back to top](#build)