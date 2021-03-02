<p align="center">
  <img width="250" height="250" src="https://raw.githubusercontent.com/robertcorponoi/graphics/60af78e2ae65013129571a273d2a9cb456c687f6/godot-rust-cli/logo/godot-rust-cli-logo.svg">
</p>

<h1 align="center">Godot Rust CLI</h1>

<p align="center">Godot Rust CLI is an easy to incorporate Rust modules into your Godot project.</p>

[![Build Status](https://www.travis-ci.com/robertcorponoi/godot-rust-cli.svg?branch=main)](https://www.travis-ci.com/robertcorponoi/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/v/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/d/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/l/godot-rust-cli)

## Documentation

Note: Godot Rust CLI is below v1.0.0 and may contain bugs, please report any bugs as issues in the GitHub repo.

**Table of Contents**

- [Introduction](#introduction)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Step By Step](#step-by-step)
- [API](#api)
  - [new](#new)
  - [create](#create)
  - [destroy](#destroy)
  - [build](#build)
  - [plugin](#plugin)
- [Compatibility](#compatibility)
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

What this will do is create a new cargo library in the current directory with everything needed to create and build Rust modules. This also contains a `project.toml` file used by Godot Rust CLI to keep track of what modules have been created and the name of the Godot project used when running commands.

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
godot-rust-cli build [--w, --watch]
```

where:

- `w, --watch` can be passed optionally to have Godot Rust CLI watch the library for changes and rebuild automatically.

**Examples:**

Building the library:

```sh
godot-rust-cli build
```

Building the library and watching for changes to the library to rebuild automatically:

```sh
godot-rust-cli build --watch
```

### plugin

Creates a module meant to be used as a plugin. This works about the same as `create` but this also creates the `addons` structure in the Godot project that Godot requires from plugins.

**Usage:**

```sh
godot-rust-cli plugin <plugin_name>
```

where:

- `plugin_name` is the name of the plugin. This should be the user friendly name of the plugin and Godot Rust CLI will handle normalizing it, like the example below.

**Examples:**

Creating a plugin named Directory Browser:

```sh
godot-rust-cli plugin "Directory Browser"
```

# Compatibility

| Godot Rust Version | Godot Rust CLI Version |
|--------------------|------------------------|
| 0.9.3              | >=0.1.1                |

## License

[MIT](./LICENSE)
