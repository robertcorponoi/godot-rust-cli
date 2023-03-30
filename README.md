<p align="center">
  <img width="250" height="250" src="https://raw.githubusercontent.com/robertcorponoi/graphics/60af78e2ae65013129571a273d2a9cb456c687f6/godot-rust-cli/logo/godot-rust-cli-logo.svg">
</p>

<h1 align="center">Godot Rust CLI</h1>

<p align="center">Godot Rust CLI is an easy to incorporate Rust modules into your Godot project.</p>

![main workflow](https://github.com/robertcorponoi/godot-rust-cli/actions/workflows/main.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/d/godot-rust-cli)
![Crates.io](https://img.shields.io/crates/l/godot-rust-cli)
[![Discord](https://img.shields.io/discord/853728834519040030.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/GSf8wvYzxe)

**Note:** As of March 30th, 2023 this project is archived. This could change in the future and the README will be updated when that happens.

Note: Godot Rust CLI is below v1.0.0 and may contain bugs, please report any bugs as issues in the GitHub repo or feel free to ask questions in the [Discord](https://discord.gg/GSf8wvYzxe).

Also keep in mind that the main branch will usually be ahead of the version on [crates.io](https://crates.io/crates/godot-rust-cli).

**Table of Contents**

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Tutorials](#tutorials)
- [Command Reference](#command-reference)
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

**Note:** Godot Rust CLI currently builds Rust modules using the [godot-rust](https://github.com/godot-rust/godot-rust) ([gdnative](https://crates.io/crates/gdnative)) crate.

**Note:** All refrences to `modules` refer to project-level modules. Godot Rust CLI does not build editor-level modules.

## Prerequisites

Godot Rust CLI requires the following prerequisites:

- `bindgen` - This is required to build the required `gdnative` crate so you should follow the [instructions](https://rust-lang.github.io/rust-bindgen/requirements.html) to install it for your operating system.

- The latest version of [Rust](https://www.rust-lang.org/tools/install).

- `rustfmt` - This is need to format files after creating/editing them. You can install this with `rustup component add rustfmt`.

## Installation

To install Godot Rust CLI, use:

```sh
cargo install godot-rust-cli
```

To upgrade your version of Godot Rust CLI to the latest version, use:

```sh
cargo install --force godot-rust-cli
```

## Tutorials

1. [Basic usage](docs/tutorials/basic-usage.md)
2. [Creating a Godot plugin](docs/tutorials/creating-a-godot-plugin.md)
3. [Platforms (experimental)](docs/tutorials/platforms.md)

## Command Reference

- [new/plugin](docs/commands/command-new.md)
- [create](docs/commands/command-create.md)
- [destroy](docs/commands/command-destroy.md)
- [build](docs/commands/command-build.md)
- [platform](docs/commands/command-platform.md)

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

A tool is in development to make upgrading between major changes easier. If you need help, questions and concerns are always welcome on [Discord](https://discord.gg/GSf8wvYzxe).

If you have been developing a plugin, there is unfortunately no way to upgrade without creating a new project as there was a major overhaul to plugin creation.

# Questions

Check out the [Discord](https://discord.gg/GSf8wvYzxe) to ask any questions or concerns about the cli or Godot + Rust in general.

## License

[MIT](./LICENSE)
