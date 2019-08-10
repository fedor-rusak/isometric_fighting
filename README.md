# isometric-fighting
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This is a playground for Rust programming language and idea of fighting game implemented with isometric perspective.

To have some info about Rust language please check [Rust Language Cheat Sheet](https://cheats.rs/).

## Build instructions

Install rust via rustup.

Git clone this repo.

Run this inside cloned project:
```
cargo run
```

## Build instructions for Mac

Checked it on Mojave and everything worked after this:

```
sudo installer -pkg /Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg -target /
```

## Usage instructions

Keyboard arrows to move.

Space to reset to default state.

Esc to exit.

## History


### 0.1.20
 - Simplified some code with pattern matchings (more like destructure usage)

### 0.1.19
 - Docs updated!

### 0.1.18
 - Refactored and fixed to_map_index logic

### 0.1.17
 - GGEZ update
 - bugfix for negative coords and tile visiting

### 0.1.16
 - Now we can't step in them. So they are more like rocks than pits :)

### 0.1.15
 - Now we can do pits in our system

### 0.1.14
 - License fix

### 0.1.13
 - GGEZ version updated
 - Some comments
 - Removed deprecated gamepad support

### 0.1.12
 - GGEZ version updated!
 - Sound volume can be set before playing!

### 0.1.11
 - Camera as established mechanism!
 - Camera moves with avatar!

### 0.1.10
 - 3d coords (with z=0) for describing avatar movement!
 - rendering each tile separately
 - funny mode where moving over tile changes its color and space resets all!

### 0.1.9
 - Background audio part!

### 0.1.8
 - Hit space to get back to center!

### 0.1.7
 - Sound added!

### 0.1.6
 - Different images for different angles!
 - small snake_case fix

### 0.1.5
 - Speed for avatar!
 - Now avatar moves across the lines!

### 0.1.4
 - Floor is drawn!
 - Fresh avatar pic!
 - DesignDoc update

### 0.1.3
 - Temporary ggez dependency via git. Fixed version not published yet.
 - Moving picture via keyboard!

### 0.1.2
 - Esc to exit!
 - Checked controllers buttons and they worked!

### 0.1.1
 - Image loading, drawing
 - FPS cap

### 0.1.0
 - initial code with event loop setup