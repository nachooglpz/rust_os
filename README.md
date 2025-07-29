# Learning how to code an OS in rust
Based on the following guide:<br>
[Writing an OS in Rust](https://os.phil-opp.com/)

## Initial instructions
To run this project it is important to use a bare metal environment.<br>
The guide recommends the ARM Cortex-M (embedded) with hardware floating point (hf).<br><br>

To install the target triple: `rustup target add thumbv7em-none-eabihf`.<br><br>

The `/.cargo/config.toml` file is already configured to use the ARM Cortex-M, so it is safe to just run the project using: `cargo build`.<br><br>

Or to `cargo build` using this target triple directly: `cargo build --target thumbv7em-none-eabihf`.
