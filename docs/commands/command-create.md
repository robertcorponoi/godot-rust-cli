## create

Creates a Rust module and its corresponding file structure in the library and Godot project.

This will create the `*.rs` file for the module, add it to the `lib.rs` automatically, and create the `*.gdns` file for it in Godot so that when you're ready to use it in the Godot project, you can just assign that `*.gnds` file as a script to a node.

By default modules will be placed in a `gdnative` directory within your Godot project but you can move them around freely.

**Note:** This command has to be run within the library directory and will fail outside of it.

**Usage**

```sh
godot-rust-cli create <module_name>
```

where:

- `module_name` is the name of the module to create. This name of the module should be the name of the class so more generally speaking it should be PascalCase. Check out the examples below for examples on the naming convention.

**Examples:**

- Creating a module named "Player":

```sh
godot-rust-cli create Player
```

- Creating a module named "MainScene":

```sh
godot-rust-cli create MainScene
```

[Back to top](#create)