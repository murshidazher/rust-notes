# Snake Game

- A snake game with Rust WebAssembly
- Reference to pure [web assembly](https://www.assemblyscript.org/types.html)
- To try if the web assembly text is working, [wat2wasm](https://webassembly.github.io/wabt/demo/wat2wasm/)

## Why WebAssembly

- We need to compile Rust to WebAssembly code.
- WebAssembly can be executed in the browser.
- It runs at native speed (almost) compared to executing JavaScript.
- We can write wasm in any language given that the language has web assembly compilation. Rust is one of such language.
- Will take the Rust code and compile it into web assembly instructions.
- This code can be loaded just like the JavaScript loaded into the browser.
- Used for 3d applications, cad, virtual reality, system application, games.

## Getting Started

The html and javascript code goes into the `www` directory.
