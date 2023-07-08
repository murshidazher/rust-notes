# [rust-notes](https://github.com/murshidazher/rust-notes)

> 🦀 A series of notes on rust language

- Google's rust [handbook](https://google.github.io/comprehensive-rust/)
- [Rust documentations](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Package registry for rust in 📦 [crates.io](https://crates.io/)

## Table of Contents

- [rust-notes](#rust-notes)
  - [Table of Contents](#table-of-contents)
  - [Why Rust ?](#why-rust-)
  - [Installation](#installation)
  - [Visual Studio Code Extensions](#visual-studio-code-extensions)
    - [Common Errors](#common-errors)
  - [Manage Rust Versions](#manage-rust-versions)
    - [Create project](#create-project)
    - [Conventions \& Syntax](#conventions--syntax)
    - [How Rust Memory it works ?](#how-rust-memory-it-works-)
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
# to see the hex dump of the file
# see the compiled version with sequence of the bytes
xxd -g1 main
```

to run the file using `cargo`;

```sh
cd project-name
cargo run
```

### Conventions & Syntax

> 💡 Commonly used conventions in `rust`

- Naming -> snake case.
- Basic types -> [Scalar types doc](https://google.github.io/comprehensive-rust/basic-syntax/scalar-types.html)
- Moving -> happens with dynamic value of the heap
- Copy -> happens with static values
- The `match` keyword is used for pattern matching for enum.
- The `struct` keyword is like **class** and `trait` is similar to **interfaces**. - [more on this](https://betterprogramming.pub/rust-basics-structs-methods-and-traits-bb4839cd57bd)
- The `impl` makes the compiler determine type at the compile time

### How Rust Memory it works ?

> 🎥 Video on [Visualizing the rust memory](https://www.youtube.com/watch?v=rDoqT-a6UFg&t=581s)

- Sequence of the bytes are loaded into the virtual memory where the code is saved so the the instruction set can be read
- Stack -> grows high address to low address (much faster than heap)
  - Main function runs in the stack
- Heap -> grows low address to high address (bigger size than stack)
- It will allocate a stack frame for main function execution in advance.

## LICENSE

2023 &copy; Murshid Azher.
