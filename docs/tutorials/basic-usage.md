# Basic Usage

This basic guide will walk you through creating a library for your Godot project.

1. The first step is to navigate to the parent directory of the Godot project in your terminal of choice. For simplicity's sake, a library has to have the same parent directory as the Godot project. For example, a recommended directory structure can look like:

```
...
/platformer-game
    /platformer
        project.godot
        ...
```

Where `platformer` is the Godot project and `platformer-game` is the parent directory. Therefore, when you create the library, it will have the same parent directory as the Godot project. For this tutorial, we'll assume that we're using the above directory structure.

So first, `cd` into that directory:

```sh
cd ~/path/to/platformer-game
```

2. Now that we're in the right directory (godot-rust-cli will let you know if you're in the correct directory or not), we can run the `new` command to create the library:

```sh
godot-rust-cli new platformer_modules platformer
```

The first argument passed to `new` is the name of the library. This can generally be whatever you like but it will be normalized to snake case by godot-rust-cli. This is a bit different for plugins as you can see in the [creating a Godot plugin](./creating-a-godot-plugin.md) documentation.

The second argument is the name of the directory that contains the Godot project. Note that this is not the name of the Godot project. godot-rust-cli needs this to know what directory contains the Godot project in case there's other directories present.

This will create the cargo library in with everything needed to create and build Rust modules. This will also create a `godot-rust-cli.json` configuration file used by godot-rust-cli to keep track of what modules have been created, whether the library is for a plugin or not, platforms to cross-compile for, and more.

After this command is complete (and it might take a little time, godot-rust-cli runs the initial build so that Godot doesn't complain about missing dynamic libraries), your directory structure should now look like:

```
...
/platformer-game
    /platformer
        project.godot
        ...
    /platformer_modules
```

Check out the new command [documentation](../commands/command-new) for more information.

3. Switch to the newly created `platformer_modules` directory.

From now on, every command we go over is run from inside the library directory. Attempting to run any of the commands below outside of the library will error out and exit early.

```sh
cd platformer_modules
```

4. At this point we're ready to start creating modules so let's create our first module, our player. 

The naming convention for modules is to use the name that would correspond to the class name in Godot. For example, for player it would be "Player", for main scene it would be "MainScene". These names are used in various casings by godot-rust-cli but it's best to initially pass the name of the module as PascalCase.

Creating the player module would look like:

```sh
godot-rust-cli create Player
```

This does several things:

- Creates the `player.rs` file within the src directory of the library with boilerplate code.
- Adds the player module to the `lib.rs` file within the src directory of the library.
- Creates the `player.gdns` file within the Godot project's gdnative directory. The module can be moved anywhere you would like from here, it is just the default place of modules.

The default `player.rs` file will contain code meant to be attached to a `Node2D` within Godot and when the project is run, it wll print "Hello, World" to the console in Godot. Since this tutorial isn't about writing Rust, we'll save that for the (upcoming) examples and instead focus on what you can do with godot-rust-cli.

Check out the create command [documentation](../commands/command-create) for more information.

5. Now we're ready to build the project and attach the player script to a node and see it work.

You can build the project to generate the dynamic libraries for the library using the `build` command like so:

```sh
godot-rust-cli build
```

This will build the library and copy the dynamic libraries over to the gdnative/bin folder in the Godot project.

You can also use the `--watch` flag to watch for changes to the Rust modules and rebuild the library automatically. Check out the build command [documentation](../commands/command-build) for more information.

4. Now that the module has been created and the library has been built, let's see the module in action in the Godot project.

Open up your Godot project and attach the node type that the script corresponds to in the scene tree. If you didn't change the default `player.rs` code, it will extend a `Node2D` so add a `Node2D` to your scene.

Next, with the `Node2D` selected, scroll down on the properties panel on the right and under the script section, select to add an existing script. Nativate to to the location of the `Player.gdns` file and select it to be added as the script for the `Node2D`.

Now you can simply run your Godot project and see the line "Hello, World" printed to the console. If you make changes to the `player.rs` module, like printing a different line of text, and build the library again you can see it print the new text when you run the Godot project. All you have to do is make sure to run a build after every change or use the `--watch` flag to have godot-rust-cli automatically run builds for you.

5. Optionally, if you decide that you don't need the player module anymore, you can remove all traces of it from the library and Godot project.

To remove a module, make sure that you're in the library directory and run the `destroy` command passing in the name of the module to remove:

```sh
godot-rust-cli destroy Player
```

The name of the module to remove should be the same name that was used when the module was created. Check out the destroy command [documentation](../commands/command-destroy) for more information.

[Back to top](#basic-usage)