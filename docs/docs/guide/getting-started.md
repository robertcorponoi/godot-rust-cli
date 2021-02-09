# Getting Started

## Prerequisites

Godot Rust CLI requires the following prerequisites:

- `bindgen` - This is required to build the required `gdnative` crate so you should follow the [instructions](https://rust-lang.github.io/rust-bindgen/requirements.html) to install it for your operating system.

- The latest version of [Rust](https://www.rust-lang.org/tools/install).

## Installation

To install Godot Rust CLI, use:

```sh
cargo install godot-rust-cli
```

To upgrade your version of Godot Rust CLI to the latest version, use:

```sh
cargo install --force godot-rust-cli
```

## Step-by-Step

This detailed guide will walk you through creating a library of Rust modules for your Godot project. If you would rather just look at the API, check it out [below]().

1. The first step is to create a library to hold all of the Rust modules for your Godot project with the `new` command. Since libraries are meant to be easy to share, this has to be in the same directory as your Godot project.

Let's assume that you have a Godot project named `platformer` and we wanted to make a library named `platformer_modules`. To create a library of modules for this project, you would use:

```sh
godot-rust-cli new platformer_modules platformer
```

What this will do is create a new cargo library in the current directory with everything needed to create and build Rust modules. This also contains a `project.toml` file used by Godot Rust CLI to keep track of what modules have been created and the name of the Godot project used when running commands.

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