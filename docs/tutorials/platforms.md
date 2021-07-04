## Platforms

Adding and removing platforms is a new experimental feature to godot-rust-cli.

Before we get into how to add and remove platforms, let's get into the pre-requisites and how they work.

A platform, in the context of godot-rust-cli, is something to build the library for other than the user's default platform. If you don't ever plan on or need to cross-compile, you don't have to worry about platforms at all. Platforms are use solely for cross-compiling your library to work on other systems, like windows (if you're not already developing on windows).

### Pre-requisites

Before you can use platforms you need the following this:

- [cross](https://github.com/rust-embedded/cross). The cross binary is used for cross-compilation so having this command available to be used by godot-rust-cli is necessary.
- As a dependency of cross, you also need [docker](https://www.docker.com/).

### How it Works

When you specify another platform to build for, godot-rust-cli will create a Docker directory within your library (if it doesn't already exist) along with a `Cross.toml` configuration file needed by cross. This docker directory is going to contain the docker images needed to build the library for that platform and the cross configuration file will contain the name of the image to use when cross compiling for that platform.

### Supported Platforms

Currently the list of platforms that can be specified are:

- Windows

The list of upcoming platforms, in what is most likely the order of their release, are:

- Linux
- Android
- MacOS

### Adding a Platform

To add a new platform, you have to use the `add-platform` command along with the name of the platform to add:

```sh
godot-rust-cli add-platform windows
```

This will create the docker folder within your library and the `Cross.toml` configuration file. This docker file extends the cross windows docker file but adds extra tools that we need to compile Godot Rust. This will also run the build for the newly added image so that it can be used when building the platform so it can take a little while for this to complete.

### Building for the Platform

TO build for the newly added windows platform, you have to use the `build` command and pass the `--all` flag to it. This will build for the native platfrom and any platforms added. After the build is complete, you should be able to open the Godot project in that platform and run it the same as you would with your native platform.