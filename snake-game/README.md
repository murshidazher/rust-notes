# Snake Game

- A snake game with Rust WebAssembly
- Reference to pure [web assembly](https://www.assemblyscript.org/types.html)
- To try if the web assembly text is working, [wat2wasm](https://webassembly.github.io/wabt/demo/wat2wasm/)
- A list of [assembly instructions](https://webassembly.github.io/spec/core/binary/instructions.html)

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

- `export "<export-name>"` export the function with any name
- Paste the code in [wat2wasm](https://webassembly.github.io/wabt/demo/wat2wasm/) and download the `wasm` file.
- Download the file and move it to `www` directory
- We can see the individual instructions using the **xxd**, on the right it just tries to convert into **utf8** hence the gibberish

```sh
xxd -g1 www/sum.wasm # 67 bytes (67 instruction)
```

- if you see the `sum.wasm` size it would be **67B**.

### Webpack

- Will be using the webpack to bundle and load the wasm file dynamically

```sh
pnpm install
pnpm run dev # http://localhost:8080/
```

## LICENSE

2023 &copy; Murshid Azher.
