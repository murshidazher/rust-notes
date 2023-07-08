import init, { World } from "snake_game";

init().then(() => {
  const CELL_SIZE = 20;

  const world = World.new();
  const worldWidth = world.width();

  const canvas = document.getElementById("snake-canvas");
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

  drawWorld();
  drawSnake();

  // Every 100ms, re-draw the world and snake
  setInterval(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height); // Clean canvas before redrawing
    drawWorld();
    drawSnake();
    world.update(); // move the snake head to the right
  }, 100);
});
