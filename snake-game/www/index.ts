import init, { World } from "snake_game";

init().then(() => {
  const CELL_SIZE = 20;

  const world = World.new();
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  function drawWorld() {
    ctx.beginPath();

    // Draw vertical lines of the world ↓
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    // Draw horizontal lines of the word →
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke(); // Draw the line stroke
  }

  function drawSnake() {
    const snakeIdx = world.snake_head_idx();
    const col = snakeIdx % worldWidth; // 10 % 8 = 2
    const row = Math.floor(snakeIdx / worldWidth); // (10/8) = 1

    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
  }

  function update() {
    // setTimeout will be only called once
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height); // Clean canvas before redrawing
      world.update(); // move the snake head to the right
      paint();
      // the method takes a callback to invoked before the next repaint function of the browser
      // this will synchronize animation with the display refresh rate
      // https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
      requestAnimationFrame(update);
    }, 100);
  }

  paint();
  update();
});
