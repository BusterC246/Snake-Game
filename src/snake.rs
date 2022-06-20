use std::collections::VecDeque;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: VecDeque<Rect>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            body: VecDeque::from([
                Rect::new(300, 300, 25, 25), 
                Rect::new(300, 325, 25, 25)
            ]),
            direction: Direction::Up,
        }
    }
}

impl Snake {
    pub fn offset(&mut self) {
        let mut tail = self.body.pop_back().unwrap();
        tail.reposition(self.body.front().unwrap().top_left());

        self.body.push_front(tail);

        match self.direction {
            Direction::Up => self.body.front_mut().unwrap().offset(0, -25),
            Direction::Down => self.body.front_mut().unwrap().offset(0, 25),
            Direction::Left => self.body.front_mut().unwrap().offset(-25, 0),
            Direction::Right => self.body.front_mut().unwrap().offset(25, 0),
        }
    }

    pub fn add_segment(&mut self) {
        self.body.push_back(Rect::new(
            self.body.back().unwrap().x(),
            self.body.back().unwrap().y(),
            25,
            25,
        ));
    }

    pub fn head(&self) -> Rect {
        *self.body.front().unwrap()
    }

    pub fn has_body_collided(&self) -> bool {
        for segment in self.body.range(1..) {
            if self.body.front().unwrap().contains_rect(*segment) {
                return true;
            }
        }

        false
    }

    pub fn apple_in_body(&self, apple: &Rect) -> bool {
        for segment in &self.body {
            if segment.contains_rect(*apple) {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for segment in &self.body {
            canvas.set_draw_color(Color::GREEN);
            canvas.fill_rect(*segment)?;
            canvas.set_draw_color(Color::BLACK);
            canvas.draw_rect(*segment)?;
        }

        Ok(())
    }
}
