# Learning how to code an OS in Rust
Based on the following guide:<br>
[Writing an OS in Rust](https://os.phil-opp.com/)

## Initial instructions
### Setting up the Nightly Channel
- To build the OS, we need experimental features only available using the Rust Nightly Channel.

- To configure Nightly on the working directory using `rustup`: `rustup override set nightly`.

- You can check that you have a Nightly version installed by running: `rustc --version`.
The version number should contain `-nightly` at the end.

- If you want to return to the Stable Channel on the directory: `rustup override unset`.

### Run target triple configuration
- The kernel runs on a custom target configuration, so in order to tell `cargo` that it should recomplie the `core` and `compiler_builtins` libraries, it needs access to the rust source code.

- To install the rust source code: `rustup component add rust-src`.

### Implement the target triple
- This is only for informational purposes.

- The target specification is already implemented on the `x86_64-nacho_os.son`, and embeded on the `.cargo/config.toml` configuration.<br>

- So the project can be run with just `cargo build`

- To run this project it is important to use a bare metal environment.
The guide recommends (while setting up the freestanding rust binary) the ARM Cortex-M (embedded) with hardware floating point (hf).

- To install the target triple: `rustup target add thumbv7em-none-eabihf`.

- The `.cargo/config.toml` has already a line commented to use the ARM Cortex-M, only disable the other line specifying to use the `json` specification and uncomment the other target specification. Then it is safe to run: `cargo build`.

- Or to `cargo build` using this target triple directly: `cargo build --target thumbv7em-none-eabihf`.