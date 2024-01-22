# Tetris

This repository contains the source code for a classic Tetris game clone, built using the Bevy game engine.

The game is cross-platform and supports Web, Linux, Windows, and MacOS.

## Features

- **Classic Tetris gameplay**: Form complete lines to score points and prevent the block pile from reaching the top.
- **Multiple Difficulty Levels**: Catering to both beginners and seasoned players.
- **Block Projections**: See a projection of where the block will land, helping you plan your placements.
- **Pause Mechanism**: Need to take a break? You can pause the game at any time.
- **Cross-platform**: The game can be played on Web, Linux, Windows, and MacOS.


## Gameplay Instructions

Here are the controls for the game:

- **Up Arrow**: Rotate the block.
- **Down Arrow**: Soft drop.
- **Space**: Hard drop.
- **Left/Right Arrows**: Move the block left or right.
- **Esc**: Pause the game.

## Play It Online
- [itch](https://windysha.itch.io/tetris)
- [github page](https://windysha.github.io/tetris/)

## Getting Started

Get a local copy,  and then run the following steps.

### Linux/Mac/Win
```
$ cargo build
or
$ cargo run
```
### Web
First, install toolchains:
```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
```
Then, compile wasm and copy file to out directory:
```
$ ./build_wasm.sh
```

Run the website:
```
$ cd wasm/
$ python3 -m http.server
```
### Github Action
Execute the following commands:
```
git tag -a "tetris-1.6" -m "official release"
git push --tags
```
### Snapshoot
<img src="https://github.com/WindySha/tetris/blob/master/screenshot/01.png" width="580" height="300">
<img src="https://github.com/WindySha/tetris/blob/master/screenshot/02.png" width="580" height="300">

## Acknowledgements 
- [Bevy](https://bevyengine.org) - A refreshingly simple data-driven game engine built in Rust.
- [bevy_github_ci_template](https://github.com/bevyengine/bevy_github_ci_template)
- [Rotation_systems](https://strategywiki.org/wiki/Tetris/Rotation_systems)
- [play-tetris](https://tetris.com/play-tetris)
- [bevy-tetris](https://github.com/corbamico/bevy-tetris)

## Licence
This project is licensed under the Apache License 2.0 - see the [LICENSE](https://github.com/WindySha/tetris/blob/main/LICENSE-Apache-2.0) file for more details.
