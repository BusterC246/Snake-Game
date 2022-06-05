pub mod game;
pub mod snake;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake Game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let mut game = game::Game::new(canvas, event_pump);

    loop {
        game.run();
    }
}
