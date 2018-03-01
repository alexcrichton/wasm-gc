# wasm-gc

A small command to gc a wasm module and remove all unneeded exports, imports,
functions, etc. This is effectively `--gc-sections` for wasm.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
  shall be dual licensed as above, without any additional terms or conditions.

### WSL (Windows Subsystem for Linux)

Install `gcc`:

`sudo apt update && sudo apt install gcc -y`

and then install the crate:

`cargo install wasm-gc`
