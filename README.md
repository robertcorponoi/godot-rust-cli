<p align="center">
  <img width="250" height="250" src="https://raw.githubusercontent.com/robertcorponoi/graphics/60af78e2ae65013129571a273d2a9cb456c687f6/godot-rust-cli/logo/godot-rust-cli-logo.svg">
</p>

<h1 align="center">Godot Rust CLI</h1>

<p align="center">Godot Rust CLI is an easy to incorporate Rust modules into your Godot project.</p>

![main workflow](https://github.com/robertcorponoi/godot-rust-cli/actions/workflows/main.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/d/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/l/godot-rust-cli)
[![Discord](https://img.shields.io/discord/853728834519040030.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/kr9EkBp7)

## Documentation

Note: Godot Rust CLI is below v1.0.0 and may contain bugs, please report any bugs as issues in the GitHub repo or feel free to ask questions in the [Discord](https://discord.gg/kr9EkBp7).

Also keep in mind that the main branch will usually be ahead of the version on [crates.io](https://crates.io/crates/godot-rust-cli).

**Table of Contents**

- [Introduction](#introduction)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Step By Step](#step-by-step)
- [API](#api)
  - [new](#new)
    - [plugin](#plugin)
  - [create](#create)
  - [destroy](#destroy)
  - [build](#build)
  - [add-platform](#add-platform)
  - [remove-platform](#remove-platform)
- [Compatibility](#compatibility)
- [Updating](#updating)
- [Questions](#questions)
- [License](#license)

## Introduction

**Note:** Godot Rust CLI is below v1.0.0 and may contain bugs, please report any bugs as issues in the [GitHub repo](https://github.com/robertcorponoi/godot-rust-cli).

Godot Rust CLI is a simple command line interface to help you create and update Rust components for your Godot projects.

Using Rust in your Godot project is great for performance critical code and with Godot Rust CLI, you can create your entire project in Rust or you can mix it with new or existing gdscript.

Currently, Godot Rust CLI supports creating Rust modules for Windows, MacOS, and Linux. If you need support for any other platform just put in a request as an issue in the GitHub repo and it'll be added if possible.

**Note:** Godot Rust CLI is the successor to [Godot Rust Helper](https://github.com/robertcorponoi/godot_rust_helper). Godot Rust CLI aims to be much more simple but more strict. This also means that Godot Rust CLI doesn't have as many features and it enforces a much more strict project structure.

## Getting Started

### Prerequisites

Godot Rust CLI requires the following prerequisites:

- `bindgen` - This is required to build the required `gdnative` crate so you should follow the [instructions](https://rust-lang.github.io/rust-bindgen/requirements.html) to install it for your operating system.

- The latest version of [Rust](https://www.rust-lang.org/tools/install).

- `rustfmt` - This is need to format files after creating/editing them. You can install this with `rustup component add rustfmt`.

### Installation

To install Godot Rust CLI, use:

```sh
cargo install godot-rust-cli
```

To upgrade your version of Godot Rust CLI to the latest version, use:

```sh
cargo install --force godot-rust-cli
```

### Step-by-Step

This detailed guide will walk you through creating a library of Rust modules for your Godot project. If you would rather just look at the API, check it out [below]().

1. The first step is to create a library to hold all of the Rust modules for your Godot project with the `new` command. Since libraries are meant to be easy to share, libraries have to be created in the directory that contains your Godot project but **NOT** in your Godot project's directory.

For this guide, we're assuming that we're in a directory called Game and in that directory is a Godot project named `platformer` and we want to create a library of Rust modules named `platformer_modules` like so:

```
Documents
    Game
        platformer
```

To do this, we need to navigate to the Game directory and use:

```sh
godot-rust-cli new platformer_modules platformer
```

What this will do is create a new cargo library in the current directory with everything needed to create and build Rust modules. This also contains a `godot-rust-cli.json` file used by Godot Rust CLI to keep track of what modules have been created and the name of the Godot project used when running commands.

Your directory structure should now look like:

```
Documents
    Game
        platformer
        platformer_modules
```

2. Switch to the newly created `platformer_modules` directory:

```sh
cd platformer_modules
```

Now we're ready to start creating modules with the `create` command. Let's say that you wanted to create a Rust module for the player. Godot Rust CLI tries to normalize names as much as possible but since modules correspond to class names in Godot, you should use PascalCase like so:

```sh
godot-rust-cli create Player
```

This will create the `player.rs` script in the `src` directory and add it to the `lib.rs` file so that it's recognized by Godot. The `player.rs` script contains a basic "Hello World!" example that can be attached to a `Node2D` but you should change it to suit your needs.

This will also create a `player.gdns` script in a directory named` rust_modules` in your Godot project. All modules will have their gdns files output here.

3. At this point you're ready to build the project and create the dynamic library using the `build` command like so:

```sh
godot-rust-cli build
```

This runs `cargo build` and when finished, it copies the dynamic library file to a `bin` folder in the Godot project directory.

You can also use the `--watch` flag to have Godot Rust CLI watch for changes to the Rust modules and rebuild the library automatically like so:

```sh
godot-rust-cli build --watch
```

4. Now that you created and built the module, you should head over to your Godot project to use it. 

First you have to add the appropriate node to the scene tree. This node type should correspond to the node type that your script inherits.

Next, with the node selected, you have to go to the right sidebar and in the Script dropdown choose load, and then choose the module from the `rust_modules` folder. Now when you press play, you should see your module work.

Anytime you make changes to your Rust module and build, you don't have to do anything else in the Godot project, you can just hit play and see the latest changes you applied.

## API

### new

Creates a new library to house the Rust modules for a Godot project.

**Usage:**

```sh
godot-rust-cli new <library_name> <godot_project_name>
```

where:

- `library_name` is the name of the library. Note that libraries are cargo packages so you should stick to cargo naming standards.

- `godot_project_name` - The name of the directory of the Godot project. This will be used by Godot Rust CLI to reference when it needs to write files to the Godot project.

**Examples::**

Creating a library named `platformer_modules` for a Godot project named `platformer`:

```sh
godot-rust-cli new platformer_modules platformer
```

**Note:** The library cannot be created in the same directory as the Godot project, this is not allowed because there are known performance issues with large projects created like this.

**Note:** To make Godot Rust CLI easy to use when importing other people's projects, the library and Godot project must have the same parent directory. For example, if your Godot project is named `platformer` and it lives under a directory called `Games`, then the library of Rust modules must also be created in the `Games` directory.

### plugin

The `new` command can also be used to create a plugin. A plugin is itself a library and should only contain modules necessary for that plugin. A plugin has the same arguments as `new`, since it is an extension of the `new` command but you have to pass `plugin` as a flag.

**Example:**

Creating a library for a plugin named "Directory Browser" for a Godot project named `platformer`.

```sh
godot-rust-cli new "Directory Browser" platformer --plugin
```

**Note:** The plugin doesn't have to be tied to a Godot project outside of just using it for development/testing. All of the plugin's files are contained with the plugin itself and it can be moved around after development on it is finished.

### create

Creates a Rust module and it's corresponding file structure in the library and the Godot project.

**Usage:**

```sh
godot-rust-cli create <module_name>
```

where:

- `module_name` is the name of the module to create.

**Examples:**

Creating a module named Player:

```sh
godot-rust-cli create Player
```

Creating a module named MainScene:

```sh
godot-rust-cli create MainScene
```

**Note:** The name of the module to create should be PascalCase and Godot Rust CLI will attempt to normalize it for you. However, in cases where the module name is multiple words like `MainScene`, and if you use `mainscene`, then it will not be normalized correctly so you should try to use the correct casing whenever possible.

**Note:** All modules will be placed in a `rust_modules` directory in the Godot project by default but you can move it wherever you need it.

### destroy

Removes a module from the library and the Godot project.

**Usage:**

```sh
godot-rust-cli destroy <module_name>
```

where:

- `module_name` is the name you used when creating the module.

**Example:**

Destroying a module named Player:

```sh
godot-rust-cli destroy Player
```

### build

Builds the library to generate the dynamic libraries and then copies them to the Godot project's `bin` directory.

**Usage:**

```sh
godot-rust-cli build [--w, --watch] [-r, --release]
```

where:

- `w, --watch` can be passed optionally to have Godot Rust CLI watch the library for changes and rebuild automatically.
- `r, --release` can be passed optionally to create a release build instead of the default debug build.

**Examples:**

Building the library:

```sh
godot-rust-cli build
```

Building the library and watching for changes to the library to rebuild automatically:

```sh
godot-rust-cli build --watch
```

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

**Example:**

Adding windows as a platform to cross-compile the library for:

```sh
godot-rust-cli add-platform windows
```

### remove-platform

Removes a platform from the list of platforms that godot-rust-cli can build the library for.

This will remove the docker file for that platform from the library directory and the built image.

**Example:**

Removing the windows platform added above.

```sh
godot-rust-cli remove-platform windows
```

# Compatibility

| Godot Rust Version | Godot Rust CLI Version |
|--------------------|------------------------|
| 0.9.1              | >=0.1.1                |
| 0.9.3              | >=0.2.0                |

# Updating

## 0.1.x to 0.2.x

To update your project to be compatible with 0.2.x versions from 0.1.x versions, you will need to rename your `project.toml` file to `godot-rust-cli.toml`.

## 0.2.x to 0.3.x

Libraries have switched from a toml config to a json config so if you want to update your library it is recommended to check out what the json config looks like and update your local one to match. 

A tool is in development to make upgrading between major changes easier. If you need help, questions and concerns are always welcome on [Discord](https://discord.gg/kr9EkBp7).

If you have been developing a plugin, there is unfortunately no way to upgrade without creating a new project as there was a major overhaul to plugin creation.

# Questions

Check out the [Discord](https://discord.gg/kr9EkBp7) to ask any questions or concerns about the cli or Godot + Rust in general.

## License

[MIT](./LICENSE)
