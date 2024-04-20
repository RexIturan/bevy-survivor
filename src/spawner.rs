use bevy::math::Vec2;

pub struct PositionIterator {
    pub width: u32,
    pub height: u32,
    x: u32,
    y: u32,
}

impl PositionIterator {
    pub fn new(width: u32, height: u32) -> PositionIterator {
        return PositionIterator {
            width,
            height,
            x: 0,
            y: 0,
        };
    }
    fn increment_indices(&mut self) {
        if self.x == (self.width - 1) {
            self.y += 1;

            if self.y == self.height {
                self.y = 0;
            }
        }
        self.x = (self.x + 1) % self.width;
    }
}

impl Iterator for PositionIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(Vec2::new(self.x as f32, self.y as f32));

        self.increment_indices();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use crate::spawner::PositionIterator;

    #[test]
    fn basic_10x10() {
        let mut position_iter = PositionIterator::new(10, 10);

        for y in 0..10 {
            for x in 0..10 {
                let next = position_iter.next();
                assert_eq!(next.unwrap(), Vec2::new(x as f32, y as f32));
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                let next = position_iter.next();
                assert_eq!(next.unwrap(), Vec2::new(x as f32, y as f32));
            }
        }
    }
}