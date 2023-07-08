import init, { greet } from "snake_game";

init().then(_ => {
  greet("snake");
  console.log("OK!");
})
