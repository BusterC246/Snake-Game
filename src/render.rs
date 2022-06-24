use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Screen {
    canvas: Canvas<Window>,
}

impl Screen {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Screen { canvas }
    }

    pub fn draw_screen(&mut self, grid: &Vec<Vec<u8>>) -> Result<() , String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let mut y = 0;

        while y < grid.len() {
            let mut x = 0;

            while x < grid[y].len() {
                if grid[y][x] == 1 || grid[y][x] == 2 {
                    if grid[y][x] == 1 {
                        self.canvas.set_draw_color(Color::GREEN);
                    } else {
                        self.canvas.set_draw_color(Color::RED);
                    }

                    self.canvas.fill_rect(Rect::new(x as i32 * 25, y as i32 * 25, 22, 22))?;
                }

                x += 1;
            }

            y += 1;
        }

        self.canvas.present();

        Ok(())
    }
}