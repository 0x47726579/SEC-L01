use rand::Rng;

use termcolor::Color;
#[path = "./utils.rs"]
mod utils;
use utils::Point;
#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) position: Point,
    pub(crate) color: Color,
}

impl Player {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rand_point: Point = rng.gen();
        Self {
            position: rand_point,
            color: Color::Green,
        }
    }
}
