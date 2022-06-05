use crate::snake::{Direction, Snake};
use core::time::Duration;
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    border: Rect,
    snake: Snake,
    apple: Rect,
}

impl Game {
    pub fn new(canvas: Canvas<Window>, event_pump: EventPump) -> Self {
        let window = canvas.window();
        let border_x: i32 = (window.drawable_size().0 / 4).try_into().unwrap();
        let border_y: i32 = (window.drawable_size().1 / 12).try_into().unwrap();

        Game {
            canvas,
            event_pump,
            border: Rect::new(border_x, border_y, 500, 500),
            snake: Snake::default(),
            apple: Rect::new(450, 450, 25, 25),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.process_input();
            self.snake.offset();

            match self.check_collisions() {
                Color::RED => {
                    self.snake.add_segment();
                    self.spawn_apple();
                }
                Color::WHITE => break,
                Color::GREEN => break,
                _ => {}
            }

            self.draw_screen();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));
        }

        self.snake = Snake::default();
        self.apple = Rect::new(450, 450, 25, 25);
    }

    fn check_collisions(&self) -> Color {
        if self.apple.contains_rect(self.snake.head()) {
            Color::RED
        } else if !self.border.contains_rect(self.snake.head()) {
            Color::WHITE
        } else if self.snake.has_body_collided() {
            Color::GREEN
        } else {
            Color::BLACK
        }
    }

    fn spawn_apple(&mut self) {
        let x_location =
            rand::thread_rng().gen_range(self.border.left()..self.border.right()) / 25 * 25;
        let y_location =
            rand::thread_rng().gen_range(self.border.top()..self.border.bottom()) / 25 * 25;

        self.apple = Rect::new(x_location, y_location, 25, 25);
    }

    fn draw_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RED);
        self.canvas.fill_rect(self.apple).unwrap();

        self.canvas.set_draw_color(Color::GREEN);
        self.snake.draw(&mut self.canvas);

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.draw_rect(self.border).unwrap();

        self.canvas.present();
    }

    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(key), ..
            } = event
            {
                match key {
                    Keycode::Escape => std::process::exit(0),
                    Keycode::Up => match self.snake.direction {
                        Direction::Down => {}
                        _ => self.snake.direction = Direction::Up,
                    },
                    Keycode::Down => match self.snake.direction {
                        Direction::Up => {}
                        _ => self.snake.direction = Direction::Down,
                    },
                    Keycode::Left => match self.snake.direction {
                        Direction::Right => {}
                        _ => self.snake.direction = Direction::Left,
                    },
                    Keycode::Right => match self.snake.direction {
                        Direction::Left => {}
                        _ => self.snake.direction = Direction::Right,
                    },
                    _ => {}
                }
            }
        }
    }
}
