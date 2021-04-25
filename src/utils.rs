use std::io::{self, Write};

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use read_input::prelude::input;
use read_input::InputBuild;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::board::Board;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Point {
    #[allow(dead_code)]
    pub(crate) fn as_tuple(&self) -> (u8, u8) {
        (self.x, self.y)
    }
    pub fn set(&mut self, p: (u8, u8)) {
        self.x = p.0;
        self.y = p.1;
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }
}

#[allow(dead_code)]
pub fn print_rules() -> io::Result<()> {
    let buffer_writer = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = buffer_writer.buffer();
    const WHITE: Option<Color> = Some(Color::White);
    const HL: Option<Color> = Some(Color::Green);

    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    writeln!(&mut buffer, "{}", "Welcome to the Treasure Hunt!\n")?;
    write!(&mut buffer, "{}", "You can ")?;
    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Move")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}", " around the place to ")?;

    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Search")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    writeln!(&mut buffer, "{}", " for the Treasure! Good Luck...")?;

    writeln!(&mut buffer, "\t[*] Search will take one action, it lets you search for the Treasure on your current coordinates.")?;
    writeln!(&mut buffer, "\t[*] \"Move (x,y)\" or \"Move [x,y]\" to go to a coordinate.\n\t[*] You can only move within the board and you can only Move {} blocs away at most.",Board::MAX_DIST)?;
    writeln!(&mut buffer, "You are represented by the character '{}' on the map, an '{}' signifies you have searched the area, and a '#' is a wall.\n",Board::PLAYER_CHAR, Board::SEARCHED_CHAR)?;

    return buffer_writer.print(&buffer);
}

/// At the start of each turn the player is asked for an action that can be chosen from a menu
/// This function enables us to print the menu and get the user's input
#[allow(dead_code)]
pub fn ask_for_action() -> String {
    input()
        .repeat_msg(
            "Choose one of the following:\n1. Move          3. Help\n2. Search        4. Quit\n",
        )
        .inside([
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "Move".to_string(),
            "Search".to_string(),
            "Help".to_string(),
            "Quit".to_string(),
        ])
        .err("You can only input a number from 1 to 4 included, or the command name displayed!")
        .get()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

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
