# Creating a Godot Plugin

This basic guide will walk you through creating a plugin library for your Godot project.

1. Just like in the basic usage, the first step is to navigate to the parent directory of the Godot project in your terminal of choice. For simplicity's sake, a library has to have the same parent directory as the Godot project. For example, a recommended directory structure can look like:

```
...
/plugin-development
    /plugins
        project.godot
        ...
```

Where `plugins` is the Godot project and `plugin-development` is the parent directory. Therefore, when you create the library, it will have the same parent directory as the Godot project. For this tutorial, we'll assume that we're using the above directory structure.

So first, `cd` into that directory:

```sh
cd ~/path/to/plugin-development
```

2. Now that we're in the right directory (godot-rust-cli will let you know if you're in the correct directory or not), we can run the `new` command to create the plugin library. To let godot-rust-cli know that the library is meant for a plugin, you have to also pass the `--plugin` flag:

```sh
godot-rust-cli new "Directory Browser" plugins --plugin
```

The first argument passed to `new` is the name of the library and plugin. This is different than normal as here it's recommend that it is the name of the plugin that should go in the plugin's `plugin.cfg` file. This means that this should be the name that you want other people to see when they install your plugin (if it's a public plugin). godot-rust-cli will take care of converting the name to various cases as needed.

The second argument is the name of the directory that contains the Godot project. Note that this is not the name of the Godot project. godot-rust-cli needs this to know what directory contains the Godot project in case there's other directories present.

Also, while the path to a Godot project needs to be passed, a plugin doesn't have to be tied to a Godot project outside of development/testing. All of the plugin's files are contained within the plugin itself and can be moved around after development is complete.

This will do various things:

- Create the plugin structure within the Godot project. For the above plugin, it would look like `addons/directory_browser`.

- Create the files needed for the plugin, like the plugin.cfg file.

- Create an initial module for the plugin.

After this command is complete (and it might take a little time, godot-rust-cli runs the initial build so that Godot doesn't complain about missing dynamic libraries), your directory structure should now look like:

```
...
/plugin-development
    /plugins
        /addons
            /directory_browser
                plugin.cfg
                directory_browser.gdns
                /gdnative
        project.godot
        ...
    /directory_browser
```

Check out the new command [documentation](../commands/command-new) for more information.

3. Switch to the newly created `directory_browser` directory for your library.

From now on, every command we go over is run from inside the library directory. Attempting to run any of the commands below outside of the library will error out and exit early.

```sh
cd directory_browser
```

4. At this point, the plugin is ready to go and when activated it will print "Hello, World" to the Godot console so let's try it out.

Open up your Godot project and go to the Project -> Project Settings menu option. From here you can go to the Plugins tab and you should see your plugin listed. Click on the checkbox next to enable to enable your plugin.

As soon as you enable your plugin, you should see the console in Godot log "Hello, World".

5. Let's add another module to our plugin.

The naming convention for modules is to use the name that would correspond to the class name in Godot. For example, for a module named directory it would be "Directory". These names are used in various casings by godot-rust-cli but it's best to initially pass the name of the module as PascalCase.

Creating the directory module would look like:

```sh
godot-rust-cli create Directory
```

This does several things:

- Creates the `directory.rs` file within the src directory of the library with boilerplate tool code.
- Adds the directory module to the `lib.rs` file within the src directory of the library.
- Creates the `directory.gdns` file within the Godot project's `/addons/directory_browser/gdnative` directory. The module can be moved anywhere you would like from here, it is just the default place of modules.

The default `directory.rs` file will contain code that when the plugin is enabled, it wll print "Hello, World" to the console in Godot. Since this tutorial isn't about writing Rust, we'll save that for the (upcoming) examples and instead focus on what you can do with godot-rust-cli. However, since our initial plugin module already prints "Hello, World", find that line in the new module and change it to something else like, "Hello from Directory".

Check out the create command [documentation](../commands/command-create) for more information.

5. Now we're ready to build the project and re-enable the plugin to see our new module in action.

You can build the project to generate the dynamic libraries for the library using the `build` command like so:

```sh
godot-rust-cli build
```

This will build the library and copy the dynamic libraries over to the gdnative/bin folder in the Godot project.

You can also use the `--watch` flag to watch for changes to the Rust modules and rebuild the library automatically. Check out the build command [documentation](../commands/command-build) for more information.

4. Now that the module has been created and the library has been built, let's see the module in action in the Godot project.

Back in your Godot project, go back to the Project -> Project Settings -> Plugins menu and uncheck and re-check the enable checkbox next to your module name so that Godot can pick up on the changes to it.

When you re-enable the plugin, you should see the console print two lines, "Hello, World" followed by "Hello from Directory".

5. Optionally, if you decide that you don't need the directory module anymore, you can remove all traces of it from the library and Godot project.

To remove a module, make sure that you're in the library directory and run the `destroy` command passing in the name of the module to remove:

```sh
godot-rust-cli destroy Directory
```

The name of the module to remove should be the same name that was used when the module was created. Check out the destroy command [documentation](../commands/command-destroy) for more information.

[Back to top](#creating-a-godot-plugin)