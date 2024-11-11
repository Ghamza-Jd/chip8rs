# Chip8rs

Chip-8 emulator in Rust

## Requirements

Along side having Rust to compile the emulator, SDL2 is needed to dynamically link to it. [SDL2.0 development libraries](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#sdl20-development-libraries)

## Running

Run

```
cargo chip8emu --rom ./roms/<name> [--scale <u32>]

# Example
cargo chip8emu --rom ./roms/TETRIS --scale 10
```
