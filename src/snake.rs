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
    body: Vec<Rect>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            body: vec![Rect::new(300, 300, 25, 25)],
            direction: Direction::Up,
        }
    }
}

impl Snake {
    pub fn offset(&mut self) {
        let mut i = self.body.len() - 1;
        while i > 0 {
            let next_point = self.body[i - 1].top_left();
            self.body[i].reposition(next_point);
            i -= 1;
        }

        match self.direction {
            Direction::Up => self.body[0].offset(0, -25),
            Direction::Down => self.body[0].offset(0, 25),
            Direction::Left => self.body[0].offset(-25, 0),
            Direction::Right => self.body[0].offset(25, 0),
        }
    }

    pub fn add_segment(&mut self) {
        let mut x_offset = 0;
        let mut y_offset = 0;

        match self.direction {
            Direction::Up => y_offset = 25,
            Direction::Down => y_offset = -25,
            Direction::Left => x_offset = -25,
            Direction::Right => x_offset = 25,
        }
        self.body.push(Rect::new(
            self.body.last().unwrap().x() + x_offset,
            self.body.last().unwrap().y() + y_offset,
            25,
            25,
        ));
    }

    pub fn head(&self) -> Rect {
        self.body[0]
    }

    pub fn has_body_collided(&self) -> bool {
        for segment in &self.body[1..] {
            if self.body[0].contains_rect(*segment) {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.fill_rects(&self.body).unwrap();
    }
}
