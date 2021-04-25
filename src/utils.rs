#![allow(unused_variables)]
#![allow(dead_code)]

use core::convert::From;

#[allow(dead_code)]
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::board::Board;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Point {
    pub(crate) fn as_tuple(&self) -> (u8, u8) {
        (self.x, self.y)
    }
    pub fn set(&mut self, p: (u8, u8)) {
        self.x = p.0;
        self.y = p.1;
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

/// Takes a string of a colour in English and checks if it's known
pub fn check_color(color: &str) -> bool {
    let colors = [
        "Black", "Blue", "Green", "Red", "Cyan", "Magenta", "Yellow", "White",
    ];

    if colors.contains(&color) {
        return true;
    } else {
        println!("\"{}\" was not understood. The known colours are:", color);
        for c in &colors {
            println!("   * {}", c);
        }
        return false;
    }
}

/// simple function to return the absolute value of an i32
pub fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case("Green", true),
        case("Blue", true),
        case("Cyam", false),  // misspelled 
        case("Black", true),
        case("0,0,0", false),  // not tested here
        ::trace
    )]
    fn utils_color_tests(input: &str, expected: bool) {
        assert_eq!(check_color(input), expected);
    }
}
