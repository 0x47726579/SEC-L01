#![allow(dead_code)]

use rand::Rng;

use termcolor::Color;
#[path = "./utils.rs"]
mod utils;
use crate::utils::check_color;
use regex::Regex;
use utils::Point;

#[derive(Debug)]
pub struct Player {
    pub(crate) position: Point,
    pub(crate) color: Color,
}

impl Player {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rand_point: Point = rng.gen();
        Self {
            position: rand_point,
            color: Color::Green,
        }
    }

    ///
    /// Tries to set a colour for the player
    ///
    /// Tries to convert the given str to a known colour or to a RGB tuple
    ///
    /// Known colours:
    ///    - "Black", "Blue", "Green", "Red", "Cyan", "Magenta", "Yellow", "White"
    /// RGB Tuple:
    ///    - (u8,u8,u8)
    ///
    /// returns true on success, false on failure
    pub fn set_color(&mut self, color: &str) -> bool {
        if color == "" {
            return false;
        }
        let s: String = color.chars().filter(|c| !c.is_whitespace()).collect();
        const RGB_REGEX: &str = r#"^\d{1,3},\d{1,3},\d{1,3}$"#;
        let re = Regex::new(RGB_REGEX).unwrap();
        if re.is_match(s.as_str()) {
            let split: Vec<&str> = s.split(&[','][..]).collect();
            // we want to make sure we don't try to parse a value 0>n>255
            for i in &split {
                match i.parse::<u8>() {
                    Ok(_) => {}
                    Err(_) => {
                        println! {"The value entered must be between 0 and 255!"};
                        return false;
                    }
                };
            }

            self.color = Color::Rgb(
                split[0].parse::<u8>().unwrap(),
                split[1].parse::<u8>().unwrap(),
                split[2].parse::<u8>().unwrap(),
            );
            return true;
        }

        if check_color(s.as_str()) {
            self.color = s.parse().unwrap();
            return true;
        }

        return false;
    }
}
