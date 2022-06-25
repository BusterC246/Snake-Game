use crate::snake::{Snake, Direction};
use rand::prelude::*;

pub struct Game {
    grid: Vec<Vec<u8>>,
    snake: Snake,
    apple: (usize, usize),
}

impl Default for Game {
    fn default() -> Self {
        Game {
            grid: vec![vec![0; 32]; 24],
            snake: Snake::default(),
            apple: (5, 7),
        }
    }
}

impl Game {
    pub fn compute(&mut self) -> Option<&Vec<Vec<u8>>> {
        if self.snake.offset().is_err() {
            return None;
        }

        for segment in self.snake.get_body() {
            if segment.0 >= self.grid.len() || segment.1 >= self.grid[0].len() {
                return None;
            }
        }

        if self.snake.in_body(self.apple.0, self.apple.1) {
            self.snake.add_segment();
            self.spawn_apple();
        }

        let head = self.snake.get_body().front().unwrap();

        for segment in self.snake.get_body().range(1..) {
            if head == segment {
                return None;
            }
        }

        self.put_into_grid();

        Some(&self.grid)
    }

    fn put_into_grid(&mut self) {
        let mut y = 0;

        while y < self.grid.len() {
            let mut x = 0;

            while x < self.grid[y].len() {
                let mut unchanged = true;

                for segment in self.snake.get_body() {
                    if *segment == (y, x) {
                        self.grid[y][x] = 1;
                        unchanged = false;
                    }
                }

                if self.apple.0 == y && self.apple.1 == x {
                    self.grid[y][x] = 2;
                    unchanged = false;
                }

                if unchanged {
                    self.grid[y][x] = 0;
                }

                x += 1;
            }

            y += 1;
        }
    }

    fn spawn_apple(&mut self) {
        self.apple.0 = rand::thread_rng().gen_range(0..self.grid.len());
        self.apple.1 = rand::thread_rng().gen_range(0..self.grid[0].len());     
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }
}