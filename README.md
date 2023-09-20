# Hack Assembler in Rust

![GitHub last commit](https://img.shields.io/github/last-commit/cuspymd/hack-assembler-rust)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/cuspymd/hack-assembler-rust)

This is a Rust implementation of the Hack computer's assembler, as described in Chapter 6 of the [Nand to Tetris](https://www.nand2tetris.org/) course. With this assembler, you can translate Hack Assembly Language code into Hack Machine Language code.

## Installation

To use this assembler, you'll need Rust and Cargo installed on your system. If you don't have them installed, you can get them from the [official Rust website](https://www.rust-lang.org/).

Once Rust and Cargo are installed, you can clone this repository and build the assembler:

```bash
git clone https://github.com/cuspymd/hack-assembler-rust.git
cd hack-assembler-rust
cargo build --release
```

## Usage
After building the assembler, you can use it to assemble Hack Assembly Language code. 
Here's how you can run it:

```bash
./target/release/hack_assembler input.asm
```

Replace input.asm with the path to your Hack Assembly Language code.
