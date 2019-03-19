# GTKonfig
GTK+and Rust powered config file manager for Linux

## Goal 
Provide an easier interface for beginner users to "rice" Linux. With GTKonfig, users will be able to configure their environments in a graphical interface. This interface will be not unlike the Gnome Settings program. Users will also be able to use the system to import configurations from other users. Users will also be able to create global variables to be used across configurations, used for things such as fonts and colors. This will probably not be a go to tool for experienced ricers, but this will definitely streamline the process for those less experienced.

On the backend, dotfiles will be stored similary to the way [wpgtk]:https://github.com/deviantfero/wpgtk handles them, with .base being generated for each program. A second file will also be included that defines both variables and the respective gui widgets that control them. 

## Todo
*Create Symlinks
*Create file structure to build UI on the fly
*Use UI to edit symlinked files
*Create script to autoadd supported programs

## Possibilities
*NCurses UI
*Repos to store popular programs

## Building
Run `cargo build` to build and `cargo run` to run. We have not yet made a makefile to install the binaries to path.


