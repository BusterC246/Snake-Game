use std::collections::VecDeque;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Snake {
    body: VecDeque<(usize, usize)>,
    direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            body: VecDeque::from([(4, 3), (5, 3), (6, 3)]),
            direction: Direction::North,
        }
    }
}

impl Snake {
    pub fn offset(&mut self) -> Result<(), &'static str> {
        let mut x_offset: isize = 0;
        let mut y_offset: isize  = 0;

        match self.direction {
            Direction::North => y_offset = -1,
            Direction::South => y_offset = 1,
            Direction::East => x_offset = 1,
            Direction::West => x_offset = -1,
        }

        self.body.pop_back().unwrap();
        
        let new_front = if self.body.front().unwrap().0 as isize + y_offset >= 0 || self.body.front().unwrap().1 as isize + x_offset >= 0 {
            ((self.body.front().unwrap().0 as isize + y_offset) as usize, (self.body.front().unwrap().1 as isize + x_offset) as usize)
        } else {
            return Err("Out of bounds.");
        };

        self.body.push_front(new_front);

        Ok(())
    }

    pub fn add_segment(&mut self) {
        let new_segment = *self.body.back().unwrap();
        self.body.push_back(new_segment);
    }

    pub fn in_body(&self, y: usize, x: usize) -> bool {
        for segment in self.body.range(1..) {
            if y == segment.0 && x == segment.1 {
                return true;
            }
        }

        false
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                if let Direction::South = self.direction {}
                else { self.direction = Direction::North }
            }
            Direction::South => {
                if let Direction::North = self.direction {}
                else { self.direction = Direction::South }
            }
            Direction::East => {
                if let Direction::West = self.direction {}
                else { self.direction = Direction::East }
            }
            Direction::West => {
                if let Direction::East = self.direction {}
                else { self.direction = Direction::West }
            }
        }
    }

    pub fn get_body(&self) -> &VecDeque<(usize, usize)> {
        &self.body
    }
}