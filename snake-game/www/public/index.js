!async function(){const o={console:{log:()=>{console.log("Just logging something!")},error:()=>{console.log("I am just error")}}},s=await fetch("sum.wasm"),a=await s.arrayBuffer(),e=(0,(await WebAssembly.instantiate(a,o)).instance.exports.sum)(10,20);console.log(e)}();