# [rust-notes](https://github.com/murshidazher/rust-notes)

> 🦀 A series of notes on rust language

- Google's rust [handbook](https://google.github.io/comprehensive-rust/)
- [Rust documentations](https://doc.rust-lang.org/book/ch01-01-installation.html)

## Table of Contents

- [rust-notes](#rust-notes)
  - [Table of Contents](#table-of-contents)
  - [Why Rust ?](#why-rust-)
  - [Installation](#installation)
  - [Visual Studio Code Extensions](#visual-studio-code-extensions)
    - [Common Errors](#common-errors)
  - [Manage Rust Versions](#manage-rust-versions)
    - [Create project](#create-project)
  - [LICENSE](#license)

## Why Rust ?

Rust is

- fast and predictable performance like C and C++ (no garbage collector) as well as access to low-level hardware.
- safe (typed language)
- built-in dependency management
- performant (memory can be optimized)
- can be compiled into WebAssembly (WaSM) -> it is a set of instruction that can be executed inside the browser just like JS.

## Installation

> 📖 Official installation [docs](https://doc.rust-lang.org/book/ch01-01-installation.html) and installation for [macOS using homebrew](https://sourabhbajaj.com/mac-setup/Rust/).

```sh
brew install rustup-init
rustup-init --profile minimal --default-toolchain nightly --no-modify-path
```

## Visual Studio Code Extensions

- Rust analyzer - `rust-lang.rust-analyzer`
- WebAssembly - `dtsvet.vscode-wasm`

### Common Errors

> rust analyzer failed to discover workspace in vscode

You need the `Config.toml` file for vscode extension to work properly with rust. More info [here](https://stackoverflow.com/a/72066369)

## Manage Rust Versions

> 📖 [managing rust versions using rustup](https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)

shows the rust compiler installed

```sh
rustc --version
rustup show # shows all the installed versions
```

installing a version

```sh
rustup install nightly-2021-11-29
rustup default nightly-2021-11-29
```

### Create project

> 💡 `Cargo` is the Rust package manager equivalent to `npm` for Javascript.

You could create a new rust project with `cargo`:

```sh
cargo new project-name
# or you can init the file
mkdir project-name && cd project-name
cargo init
```

since rust is a compiled language we need to compile it binary code so it can be executed,

```sh
rustc hello-world/src/main.rs
./main # execute the binary
```

## LICENSE

2023 &copy; Murshid Azher.
