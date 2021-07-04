## destroy

Removes a Rust module created with the `create` command and its corresponding files in the library and Godot project.

This will remove the `*.rs` file for the module, remove it from the `lib.rs` automatically, and remove the `*.gdns` file for it in Godot.

**Note:** This command has to be run within the library directory and will fail outside of it.

**Usage**

```sh
godot-rust-cli destroy <module_name>
```

where:

- `module_name` is the name of the module to remove. This should be the same name used when the module was created with the `create` command.

**Examples:**

- Destroying a module named "Player":

```sh
godot-rust-cli destroy Player
```

- Destroying a module named "MainScene":

```sh
godot-rust-cli destroy MainScene
```

[Back to top](#destroy)