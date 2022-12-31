# Rusted Fortress
[![Rust](https://github.com/timeolord/Rusted-Fortress/actions/workflows/rust.yml/badge.svg)](https://github.com/timeolord/Rusted-Fortress/actions/workflows/rust.yml)

Dwarf Fortress-esque game made in Bevy. Currently WIP, but you can see the current build at https://www.allanwei.com/Rusted-Fortress/.

Compiling from source:
Use cargo and build or run the project. There are no external dependencies. If you want to compile for the browser, you can use trunk. The trunk configuration is setup for a *nix system, so you might have to change the file paths (change / to \\\\, etc.) if you want to serve it on Windows.

Controls:
WASD to move the camera around.
Z and X to change the current layer. Similar to dwarf fortress, but the map will render all tiles below the current layer, without a cutoff. 
