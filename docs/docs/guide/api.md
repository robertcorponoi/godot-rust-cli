# API

## new

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

## create

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

## destroy

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

## build

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

## plugin

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