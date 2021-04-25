#[path = "./player.rs"]
pub mod player;

#[path = "./utils.rs"]
mod utils;

use crate::utils::abs;
use core::option::Option::Some;
use p::Player;
use player as p;
use rand::prelude::ThreadRng;
use rand::Rng;
use regex::Regex;
use std::cmp::max;
use std::io::{Result, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use u::Point;
use utils as u;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<char>>,
    pub player: Player,
    treasure: Point,
    pub(crate) treasure_found: bool,
}

impl Board {
    const BOARD_COLOR: Color = Color::White;
    pub(crate) const BOARD_WIDTH: u8 = 15;
    pub(crate) const BOARD_HEIGHT: u8 = 15;
    pub(crate) const MAX_DIST: u8 = 4;

    const EMPTY_CHAR: char = '.';
    pub(crate) const PLAYER_CHAR: char = '@';
    pub(crate) const SEARCHED_CHAR: char = 'X';

    pub fn new() -> Self {
        let size = (Self::BOARD_WIDTH, Self::BOARD_HEIGHT);
        let mut rng: ThreadRng = rand::thread_rng();
        let rand_point: Point = rng.gen();
        Self {
            board: vec![vec![Self::EMPTY_CHAR; size.0 as usize]; size.1 as usize],
            player: Player::new(),
            treasure: rand_point,
            treasure_found: false,
        }
    }

    /// Search for the treasure on the player space
    ///
    /// If the treasure is hidden there, we congratulate the player and .
    /// If not, display a message indicating the distance to the treasure in blocs.
    /// Save and display the searched position on the board.
    pub(crate) fn search(&mut self) {
        let x: usize = self.player.position.x as usize;
        let y: usize = self.player.position.y as usize;

        // save position searched, also used for the display
        self.board[x][y] = Self::SEARCHED_CHAR;

        // we found the treasure
        if self.player.position.as_tuple() == self.treasure.as_tuple() {
            println!("Congratulation you found the treasure!");
            self.treasure_found = true;
        } else {
            let distance = self.get_distance_to(self.treasure.x, self.treasure.y);
            let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
            let mut buffer = buffer_writer.buffer();
            // The Treasure is X blocs away. (with highlight on X)
            buffer
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .map_err(|err| println!("{:?}", err))
                .ok();
            write!(&mut buffer, "The treasure is ")
                .map_err(|err| println!("{:?}", err))
                .ok();
            buffer
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 102, 255))))
                .map_err(|err| println!("{:?}", err))
                .ok();
            write!(&mut buffer, "{}", distance)
                .map_err(|err| println!("{:?}", err))
                .ok();
            buffer
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .map_err(|err| println!("{:?}", err))
                .ok();
            writeln!(&mut buffer, " blocs away.")
                .map_err(|err| println!("{:?}", err))
                .ok();
            buffer_writer
                .print(&buffer)
                .map_err(|err| println!("{:?}", err))
                .ok();
        }
    }

    pub(crate) fn move_to(&mut self, x: u8, y: u8) {
        let distance = self.get_distance_to(x, y);
        if distance > Self::MAX_DIST {
            println!("You cannot move more than 4 blocs in a turn!");
            return;
        } else {
            self.player.position.set((x, y));
        }
    }

    /// gives the distance from the player
    /// return u8: Distance between player and (x,y)
    pub fn get_distance_to(&self, x: u8, y: u8) -> u8 {
        max(
            abs(x as i32 - self.player.position.x as i32) as u8,
            abs(y as i32 - self.player.position.y as i32) as u8,
        )
    }

    /// Verifies that the string if of the format [number,number] or (number,number)
    /// number : a base 10 or base 16 (with 0x prefix) number
    /// a coordinate cannot be outside of our map i.e.: x ϵ [0;BOARD_WIDTH), y ϵ [0;BOARD_HEIGHT)
    ///
    /// A coordinate cannot be more than MAX_DIST away from the player
    ///
    /// This function goes through multiple checkpoints to validate a set of coordinates
    /// It also ignores any space in the given string
    ///
    /// Return true On success.  false On failure to validate
    pub fn validate_move_coordinates(&self, coords: &str) -> (bool, u8, u8) {
        if coords == "" {
            return (false, 0, 0);
        }
        const MAX_DIMENSIONS: usize = 2;
        // remove whitespace for ease of use
        let s: String = coords.chars().filter(|c| !c.is_whitespace()).collect();

        // check that we use the proper coord syntax (x,y) or [x,y].
        // we do not yet check for mismatched parentheses or numbers out of bound
        // we basically only check that we have positive numbers and:
        // ( or [; number_b10 or number_b16; comma;  number_b10 or number_b16; ) or ]
        const COORD_REGEX: &str = r#"^[\[\(][(\d+|0x[0-9a-fA-F]+),(\d+|0x[0-9a-fA-F]+)]+[\]\)]$"#;

        let re = Regex::new(COORD_REGEX).unwrap();
        if !re.is_match(s.as_str()) {
            println!("Incorrect format, please check your input!");
            return (false, 0, 0);
        }

        // check parenthesis match
        match s.chars().next().unwrap() {
            '[' => {
                if s.chars().last().unwrap() != ']' {
                    println!("If you use '[' for your coordinates, do not forget to end your input with ']'!\n");
                    return (false, 0, 0);
                }
            }
            '(' => {
                if s.chars().last().unwrap() != ')' {
                    println!("If you use '(' for your coordinates, do not forget to end your input with ')'!\n");
                    return (false, 0, 0);
                }
            }
            _ => {
                println!("Incorrect first character, please check your input!");
                return (false, 0, 0);
            }
        }

        // parenthesis are ok, we now want to extract the coordinates and check them.
        let mut split: Vec<&str> = s.split(&['(', ')', '[', ']', ','][..]).collect();
        split.retain(|&i| i != "");

        if split.len() > MAX_DIMENSIONS {
            println!(
                "Wrong number of coordinates: {} coordinates provided instead of {}.",
                split.len(),
                MAX_DIMENSIONS
            );
            return (false, 0, 0);
        }

        // For each of the two coordinates we check the base and convert it if the range is valid
        let mut coords_as_u8 = vec![u8::MAX, u8::MAX];
        for i in 0..MAX_DIMENSIONS {
            if split.get(i) == None {
                return (false, 0, 0);
            }
            // dealing with hex
            if split.get(i).unwrap().len() > 2 && split.get(i).unwrap()[..2].contains("0x") {
                let without_prefix = split.get(i).unwrap().trim_start_matches("0x");
                coords_as_u8[i] = match u8::from_str_radix(without_prefix, 16) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("The number {} can not be converted to a coordinate", {
                            split.get(i).unwrap()
                        });
                        return (false, 0, 0);
                    }
                };
            } else {
                // base 10 number
                coords_as_u8[i] = match split.get(i).unwrap().parse::<u8>() {
                    Ok(res) => res,
                    Err(_) => {
                        println!("The number {} can not be converted to a coordinate", {
                            split.get(i).unwrap()
                        });
                        return (false, 0, 0);
                    }
                };
            }
        }
        if !self.is_within_bounds(coords_as_u8[0], coords_as_u8[1]) {
            return (false, 0, 0);
        };

        // if self.get_distance_to(coords_as_u8[0], coords_as_u8[1]) > Self::MAX_DIST {
        //     println!(
        //         "You can't move that far! Movement is limited to {} blocs",
        //         Self::MAX_DIST
        //     );
        //     return (false, 0, 0);
        // }

        return (true, coords_as_u8[0], coords_as_u8[1]);
    }

    fn is_within_bounds(&self, x: u8, y: u8) -> bool {
        if x >= Self::BOARD_WIDTH {
            println!("Please respect the map bounds!");
            println!(
                "Max width is {}, which is lower than your input of x={}",
                Self::BOARD_WIDTH - 1,
                x
            );
            return false;
        } else if y >= Self::BOARD_HEIGHT {
            println!("Please respect the map bounds!");
            println!(
                "Max height is {}, which is lower than your input of y={}",
                Self::BOARD_HEIGHT - 1,
                y
            );
            return false;
        }
        return true;
    }
}
// print functions
impl Board {
    /// Print the help
    pub fn print_help() {
        println!();
        println!("[*] Search command");
        println!(
            "This will tell you how far away you are from the treasure.\n\
    It will also mark the searched square.\n\
    If you use this command on the treasure square, you win the game."
        );
        println!();

        println!("[*] Move command");
        println!(
            "To move to a coordinate, please use one of the following formats:\
                    \n\t1. [x,y]: x ϵ [0;{0}), y ϵ [0;{1})\
                    \n\t2. (x,y): x ϵ [0;{0}), y ϵ [0;{1})\
                    \n\t[*] To use hex numbers, prefix them with '0x'.",
            Board::BOARD_WIDTH,
            Board::BOARD_HEIGHT
        );

        println!("[*] Help command");
        println!("Shows this help.");
        println!();

        println!("[*] Quit command");
        println!("Leave the program.");
        println!();
    }

    /// Prints the `Board` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
    /// This functions requires definition of the `BOARD_WIDTH`, `BOARD_HEIGHT` and `BOARD_COLOR` constants
    pub fn print(&self) -> Result<()> {
        let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = buffer_writer.buffer();

        // Top row
        buffer.set_color(ColorSpec::new().set_fg(Some(Self::BOARD_COLOR)))?;
        write!(&mut buffer, "{:>4}", "#")?;
        for _ in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "###")?;
        }
        writeln!(&mut buffer, "#")?;

        // Main grid
        for y in (0..Self::BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} #", y)?; // Side coordinates

            for x in 0..Self::BOARD_WIDTH {
                let mut grid_c = self.board[x as usize][y as usize];
                let dist = self.get_distance_to(x, y);
                if grid_c == Self::SEARCHED_CHAR {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                }
                // depending on your terminal you will not see much difference
                if dist <= Self::MAX_DIST {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(102, 255, 255))))?;
                }
                if dist <= Self::MAX_DIST / 2 {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 255))))?;
                }
                if dist == 1 {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                }
                if (x, y) == self.player.position.as_tuple() {
                    buffer.set_color(ColorSpec::new().set_fg(Some(self.player.color)))?;
                    grid_c = Self::PLAYER_CHAR;
                }
                write!(&mut buffer, "{:^3}", grid_c)?;
                buffer.set_color(ColorSpec::new().set_fg(Some(Self::BOARD_COLOR)))?;
            }

            writeln!(&mut buffer, "#")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "#")?;
        for _ in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "###")?;
        }
        writeln!(&mut buffer, "#")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        writeln!(&mut buffer)?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return buffer_writer.print(&buffer);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        x,
        y,
        expected,
        case(0, 0, true), // edge case
        case(0, 14, true), // edge case
        case(14, 14, true), // edge case
        case(15, 15, false), // edge case Out of bounds
        case(255, 200, false), // Out of bounds
        ::trace
    )]
    fn within_bounds_tests(x: u8, y: u8, expected: bool) {
        let board = Board::new();
        assert_eq!(board.is_within_bounds(x, y), expected);
    }

    #[rstest(
    input,
    expected,
    case("(0,0)", (true,0,0)),  // edge case
    case("(0, 14)", (true,0,14)),  // edge case
    case("[14, 14]", (true,14,14)),  // edge case
    case("(13, 13]", (false,0,0)),  // mismatched parentheses
    case("{13, 13}", (false,0,0)),  // wrong parentheses
    case("13, 13)", (false,0,0)),  // missing parentheses
    case("[13, 13", (false,0,0)),  // missing parentheses
    case("13, 13", (false,0,0)),  // missing parentheses
    case("[13     ,   13   )", (false,0,0)),  // mismatched parentheses + spaces
    case("(   1 3     ,   13   )", (true,13,13)),  // spaces
    case("(-12,0)",(false,0,0)),  // negative values
    case("(-12,-0)",(false,0,0)),  // negative values
    case("(-1,-0xA)",(false,0,0)),  // negative values
    case("(-0x3,-0xa)",(false,0,0)),  // negative values
    case("(0,-0x0)",(false,0,0)),  // negative values
    case("(,0x0)",(false,0,0)),  // incorrect input
    case("(xa,0x0)",(false,0,0)),  // incorrect input
    case("(0xa,x1)",(false,0,0)),  // incorrect input
    case("(a,0x0)",(false,0,0)),  // incorrect input
    case("(0x100,0x0)",(false,0,0)),  // out of bounds
    ::trace
    )]
    fn coord_input_validation(input: &str, expected: (bool, u8, u8)) {
        let board = Board::new();
        assert_eq!(board.validate_move_coordinates(input), expected);
    }

    // Every search should only yield one searched square
    #[test]
    fn test_search() {
        let mut board = Board::new();
        for l in &board.board {
            for c in l {
                assert_ne!(c, &Board::SEARCHED_CHAR);
            }
        }
        board.search();
        let mut count = 0;
        for l in &board.board {
            for c in l {
                count += (c == &Board::SEARCHED_CHAR) as i32;
            }
        }
        assert_eq!(count, 1);
        board.player.position.x = board.player.position.x + 1 % 15;
        board.search();
        count = 0;
        for l in &board.board {
            for c in l {
                count += (c == &Board::SEARCHED_CHAR) as i32;
            }
        }
        assert_eq!(count, 2);
    }
}
