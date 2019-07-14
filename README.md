# isometric-fighting
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This is a playground for Rust programming language and idea of fighting game implemented with isometric perspective.

## Build instructions

Install rust via rustup.

Git clone this repo.

Run this inside cloned project:
```
cargo run
```
## Usage instructions

Keyboard arrows to move.

Space to reset to default state.

Esc to exit.

## History

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