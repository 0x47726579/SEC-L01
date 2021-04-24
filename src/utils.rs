#[allow(dead_code)]
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::board::Board;
use core::convert::From;

#[derive(Debug)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    #[allow(dead_code)]
    pub(crate) fn as_tuple(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}

impl From<(u8, u8)> for Point {
    fn from(item: (u8, u8)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = (
            rng.gen_range(0..Board::BOARD_WIDTH),
            rng.gen_range(0..Board::BOARD_HEIGHT),
        );
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}
