## platform

The platform commands are split into two commands: `add-platform` and `remove-platform`.

### add-platform

Adds a new platform to the platforms that godot-rust-cli can build the library for.

Note that platforms are experimental, new, and an advanced feature. Platform support is minimal and new platforms will be added as they are tested and confirmed to be working.

Currently the only platform supported is windows.

The list of platforms being worked on include:
- Android
- Linux
- MacOS

Note that you don't need to add a platform if you're just buliding for your native platform. For example, if you're on windows then you don't need to add windows as a platform as you will build for windows by default. Platforms are only used if you want to cross-compile your library.

To cross-compile the library, the [cross](https://github.com/rust-embedded/cross) cli is used. This means that if you want to add platforms to cross-compile to, you will need to follow the instructions for setting it up, which is essentially just installing the crate and making sure that you have docker or podman.

Also, since we need extra utilities to cross-compile, we have to extend the docker images used by cross so when you add a platform, you'll notice the following:

- A directory will be created in the library directory named `docker`. This directory will contain the images used by cross for cross-compilation.

- A Cross.toml configuration file will be created to let cross know the custom images to use.

Any platforms added will not be cross-compiled for by default unless you pass the `--all` flag to the `build` command. For more information check the build command [documentation](./command-build.md)

**Note:** This command has to be run within the library directory and will fail outside of it.

**Usage**

```sh
godot-rust-cli add-platform <platform_name>
```

where:

- `platform_name` is the name of the platform to add, from the list of supported platforms. Specifying an unsupported platform will return an error message.

**Example:**

- Adding windows as a platform to cross-compile the library for:

```sh
godot-rust-cli add-platform windows
```

### remove-platform

Removes a platform from the list of platforms that godot-rust-cli can build the library for.

This will also remove the docker file for that platform from the library directory and the built image.

**Usage**

```sh
godot-rust-cli remove-platform <platform_name>
```

where:

- `platform_name` is the name of the platform to remove. This should be the same name that was used when the platform was added with `platform-add`.

**Example:**

Removing the windows platform added above.

```sh
godot-rust-cli remove-platform windows
```

[Back to top](#platform)