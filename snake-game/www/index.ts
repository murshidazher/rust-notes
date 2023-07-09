import init, { World, Direction } from "snake_game";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  const mapKeyToDirection: Record<string, Direction> = {
    "ArrowUp": Direction.Up,
    "ArrowRight": Direction.Right,
    "ArrowDown": Direction.Down,
    "ArrowLeft": Direction.Left,
  }

  document.addEventListener("keydown", e => world.change_snake_dir(mapKeyToDirection[e.code]));

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

  function drawReward() {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    )

    snakeCells.forEach((cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);

      ctx.fillStyle = i === 0 ? "#7878db" : "#000000";

      ctx.beginPath();
      ctx.fillRect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    });

    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
  }

  function update() {
    const fps = 3; // frames per second
    // setTimeout will be only called once
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height); // Clean canvas before redrawing
      world.step(); // move the body
      paint();
      // the method takes a callback to invoked before the next repaint function of the browser
      // this will synchronize animation with the display refresh rate
      // https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  update();
});
