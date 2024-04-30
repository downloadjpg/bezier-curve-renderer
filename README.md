# Bezier Curves Demo
This is a simple Rust application that allows you to interactively create and manipulate Bezier curves. It uses the Piston game engine for rendering and user input.

## Building and Running
Rust needs to be installed to build the program, so a prebuilt executable is provided at `/target/release/bezier-curves.exe`

If you don't have Rust installed, you can install it by following the [instructions on the official Rust website](https://www.rust-lang.org/learn/get-started).
To build and run the project, navigate to the project directory in your terminal and run `cargo build` or `cargo run`.


## Controls
- Left click to select a point. Drag to move the selected point.
- Right click to delete a point. Right click anywhere else to add a point.
- Press the space bar to add a new curve.
- Use the left and right arrow keys to scrub along time.