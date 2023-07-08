async function init() {
  const importObject = {
    console: {
      log: () => {
        console.log("Just logging something!");
      },
      error: () => {
        console.log("I am just error");
      }
    }
  }

  const response = await fetch("sum.wasm");
  const buffer = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer, importObject);

  const sumFunction = wasm.instance.exports.sum;
  const result = sumFunction(10, 20);
  console.log(result);
}

init();
