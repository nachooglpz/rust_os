# Learning how to code an OS in Rust
Based on the following guide:<br>
[Writing an OS in Rust](https://os.phil-opp.com/)

## Initial instructions
### Implement the target triple
To run this project it is important to use a bare metal environment.<br>
The guide recommends the ARM Cortex-M (embedded) with hardware floating point (hf).<br><br>

To install the target triple: `rustup target add thumbv7em-none-eabihf`.<br><br>

The `/.cargo/config.toml` file is already configured to use the ARM Cortex-M, so it is safe to just run the project using: `cargo build`.<br><br>

Or to `cargo build` using this target triple directly: `cargo build --target thumbv7em-none-eabihf`.<br><br>

### Setting up the Nightly Channel
To build the OS, we need experimental features only available using the Rust Nightly Channel.<br><br>

To configure Nightly on the working directory using `rustup`: `rustup override set nightly`.<br><br>

You can check that you have a Nightly version installed by running: `rustc --version`.<br>
The version number should contain `-nightly` at the end.<br><br>

If you want to return to the Stable Channel on the directory: `rustup override unset`.
