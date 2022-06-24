pub mod game;
pub mod snake;
pub mod newsnake;
pub mod logic;
pub mod render;

use std::time::Duration;
use newsnake::Direction;
use sdl2::{event::Event, keyboard::Keycode, EventPump};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake Game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // let canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut game = game::Game::new(canvas, event_pump);
    
    let mut screen = render::Screen::new(window.into_canvas().build().unwrap());
    let mut game = logic::Game::default();

    loop {
        if let Some(direction) = process_input(&mut event_pump) {
            game.set_snake_direction(direction);
            // break;
        }

        if let Ok(grid) = game.compute() {
            if let Result::Err(error) = screen.draw_screen(grid) {
                eprintln!("{}", error);
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

fn process_input(event_pump: &mut EventPump) -> Option<Direction> {
    let mut direction = None;

    for event in event_pump.poll_iter() {
        if let Event::KeyDown {
            keycode: Some(key), ..
        } = event
        {
            direction = match key {
                Keycode::Escape => std::process::exit(0),
                Keycode::Up => Some(Direction::North),
                Keycode::Down => Some(Direction::South),
                Keycode::Left => Some(Direction::West),
                Keycode::Right => Some(Direction::East),
                _ => None,
            }
        }
    }

    direction
}