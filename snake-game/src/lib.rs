use rand::Rng;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

fn rnd(max: usize) -> usize {
  rand::thread_rng().gen_range(0..max)
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
  Won,
  Lost,
  Played,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

struct Snake {
  body: Vec<SnakeCell>,
  direction: Direction,
}

impl Snake {
  fn new(spawn_index: usize, size: usize) -> Snake {
    let mut body = vec![];

    for i in 0..size {
      body.push(SnakeCell(spawn_index - i));
    }

    Snake {
      body,
      direction: Direction::Right,
    }
  }
}

#[wasm_bindgen]
pub struct World {
  width: usize,
  size: usize,
  snake: Snake,
  next_cell: Option<SnakeCell>,
  reward_cell: usize,
  status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
  pub fn new(width: usize, snake_idx: usize) -> World {
    let snake = Snake::new(snake_idx, 3);
    let size = width * width;

    World {
      width,
      size,
      reward_cell: World::gen_reward_cell(size, &snake.body),
      snake,
      next_cell: None,
      status: None,
    }
  }

  fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
    let mut reward_cell;

    loop {
      reward_cell = rnd(max);
      if !snake_body.contains(&SnakeCell(reward_cell)) {
        break;
      }
    }

    reward_cell
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn reward_cell(&self) -> usize {
    self.reward_cell
  }

  pub fn snake_head_idx(&self) -> usize {
    self.snake.body[0].0
  }

  pub fn start_game(&mut self) {
    self.status = Some(GameStatus::Played);
  }

  pub fn game_status(&self) -> Option<GameStatus> {
    self.status
  }

  pub fn change_snake_dir(&mut self, direction: Direction) {
    // guard to cover the edge cases of changing directing in the same lane
    let next_cell = self.gen_next_snake_cell(&direction);

    // if the change direction cell is the next cell, don't do any action
    if self.snake.body[1].0 == next_cell.0 {
      return;
    }

    self.next_cell = Some(next_cell);
    self.snake.direction = direction;
  }

  pub fn snake_length(&self) -> usize {
    self.snake.body.len()
  }

  pub fn snake_cells(&self) -> *const SnakeCell {
    self.snake.body.as_ptr()
  }

  pub fn step(&mut self) {
    match self.status {
      Some(GameStatus::Played) => {
        let temp = self.snake.body.clone();

        match self.next_cell {
          Some(cell) => {
            self.snake.body[0] = cell;
            self.next_cell = None;
          }
          None => {
            self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
          }
        }

        let len = self.snake.body.len();

        for i in 1..len {
          self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }

        if self.reward_cell == self.snake_head_idx() {
          if self.snake_length() < self.size {
            self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
          } else {
            self.reward_cell = 1000;
          }

          self.snake.body.push(SnakeCell(self.snake.body[1].0));
        }
      }
      _ => {}
    }
  }

  fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
    let snake_idx = self.snake_head_idx();
    let row = snake_idx / self.width;

    return match direction {
      Direction::Right => {
        let threshold = (row + 1) * self.width;
        if snake_idx + 1 == threshold {
          SnakeCell(threshold - self.width)
        } else {
          SnakeCell(snake_idx + 1)
        }
      }
      Direction::Left => {
        let threshold = row * self.width;
        if snake_idx == threshold {
          SnakeCell(threshold + (self.width - 1))
        } else {
          SnakeCell(snake_idx - 1)
        }
      }
      Direction::Up => {
        let threshold = snake_idx - (row * self.width);
        if snake_idx == threshold {
          SnakeCell((self.size - self.width) + threshold)
        } else {
          SnakeCell(snake_idx - self.width)
        }
      }
      Direction::Down => {
        let threshold = snake_idx + ((self.width - row) * self.width);
        if snake_idx + self.width == threshold {
          SnakeCell(threshold - ((row + 1) * self.width))
        } else {
          SnakeCell(snake_idx + self.width)
        }
      }
    };
  }
}
// wasm-pack build --target web
