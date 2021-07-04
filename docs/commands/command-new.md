## new

Creates a cargo library in which your Rust modules will reside.

In order to maintain an easy way to share a project with team members, this command must be run from the Godot project's parent directory. Therefore it is recommended to make a directory that contains just your Godot like so:

```
...
/platformer-game
    /platformer
        project.godot
        ...
```

In the example above, you can see there's a directory, `platformer-game` that contains just the Godot project. The `new` command would be run within the `platformer-game` directory. This ensures that the library is always able to access the path to the Godot project relative to the parent directory.

**Usage:**

```sh
godot-rust-cli new <library_name> <godot_project_dir_name> [-p, --plugin] [-s, --skio-build]
```

where:

- `library_name` is the name of the library. Since libraries are cargo packages, the name you provide will be kept in its original format in the configuration but the library directory and cargo package will be normalized. For example, if you name your library "PlatformerModules", it will be normalized to "platformer_modules".

- `godot_project_dir_name` - The directory that contains the Godot project.

- `plugin` - Indicates whether the library is for a Godot plugin or not. More on that below.

- `skip-build` Indicates whether godot-rust-cli should skip the initial library build or not. This is mostly used by tests since skipping this can cause Godot to complain about missing dynamic libraries until a build is run.

### -p, --plugin

The `--plugin` flag is used when developing a library meant for a Godot plugin. When creating a plugin, you want the library_name to be the name of the plugin as will be used by Godot and the plugin configuration file.

The plugin command will also set up the necessary directory structure and files needed for a plugin to work in Godot. Creating a plugin library changes the way the other commands work within that library so it cannot be used for both general Rust modules within your game and for plugin modules. Simply put, plugins are their own entities and should be developed outside of any other project. This is not its own command because there might be a misconception that it can be used within a regular library.

Also, while the path to a Godot project needs to be passed, a plugin doesn't have to be tied to a Godot project outside of development/testing. All of the plugin's files are contained within the plugin itself and can be moved around after development is complete.

**Examples:**

- Going by the directory structure shown above, creating a new library for the platformer game might look like:

```sh
# If needed:
# cd platformer-game

godot-rust-cli new platformer_modules platformer
```

- Creating a library for a plugin named "Directory Browser" for a Godot project named "plugins".

```sh
# If needed:
# cd plugins

godot-rust-cli new "Directory Browser" plugins --plugin
```

[Back to top](#new)