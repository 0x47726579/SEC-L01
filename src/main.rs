/*! Visual only `Board` functions

Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

use read_input::prelude::*;
use regex::Regex;
use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

struct Board {
    //TODO
    pub board: Vec<Vec<char>>,
    player: Player,
}

struct Player {
    position: (u8, u8),
}

impl Player {
    pub fn new() -> Self {
        Self { position: (0, 0) }
    }
}

impl Board {
    const BOARD_COLOR: Color = Color::White;
    const BOARD_WIDTH: u8 = 15;
    const BOARD_HEIGHT: u8 = 15;

    const EMPTY_CHAR: char = '.';
    const PLAYER_CHAR: char = '@';
    pub const SEARCHED_CHAR: char = '*';

    pub fn new() -> Self {
        let size = (Self::BOARD_WIDTH, Self::BOARD_HEIGHT);
        Self {
            board: vec![vec![Self::EMPTY_CHAR; size.0 as usize]; size.1 as usize],
            player: Player::new(),
        }
    }
    /// Prints the `Board` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
    /// This functions requires definition of the `BOARD_WIDTH`, `BOARD_HEIGHT` and `BOARD_COLOR` constants
    pub fn print(&self) -> io::Result<()> {
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();

        // Top row
        buffer.set_color(ColorSpec::new().set_fg(Some(Self::BOARD_COLOR)))?;
        write!(&mut buffer, "{:>4}", "⌜")?;
        for _ in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..Self::BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} ∣", (y + ('A' as u8)) as char)?; // Side coordinates

            for x in 0..Self::BOARD_WIDTH {
                // TODO
                let mut grid_c = self.board[x as usize][y as usize];
                if (x, y) == self.player.position {
                    grid_c = Self::PLAYER_CHAR;
                }
                write!(&mut buffer, "{:^3}", grid_c)?;
                buffer.set_color(ColorSpec::new().set_fg(Some(Self::BOARD_COLOR)))?;
            }

            writeln!(&mut buffer, "∣")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "⌞")?;
        for _ in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "_⎽_")?;
        }
        writeln!(&mut buffer, "⌟")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..Self::BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x + 1)?;
        }
        writeln!(&mut buffer)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return bufwtr.print(&buffer);
    }
}

static RE: &str = r"^(O|o|N|n)$";

fn main() {
    let mut board = Board::new();

    // Démarrer ou non une partie
    let usr_input = input()
        .repeat_msg("Voulez vous démarrer une nouvelle partie de chasse au trésor ?[O-o/N-n] ")
        .add_err_test(
            |x: &String| Regex::new(RE).unwrap().is_match(x),
            "Saisie incorrecte, veuillez réessayer",
        )
        .get();

    if usr_input == "N" || usr_input == "n" {
        println!("Bye Bye !");
    } else {
        // TODO print rules
        match print_rules() {
            Ok(_) => {}
            Err(_) => println!("Something wrong during rules print"),
        };
        // TODO get player settings/color
        loop {
            let usr_input = input().repeat_msg("Choose one of the following:\n1. Move\n2. Search\n3. Quit\n")
                .inside(["1".to_string(), "2".to_string(), "3".to_string(), "Move".to_string(), "Search".to_string(), "Quit".to_string()])
                .err("You can only input a number from 1 to 3 included, or the command name displayed!").get();
            // TODO this but properly

            match &*usr_input {
                "1" | "Move" => {} // TODO move logic
                "2" | "Search" => {
                    let s: String = input().repeat_msg("Format for search is (x,y)").get();
                    let split: Vec<&str> = s.split(&['(', ')', ','][..]).collect();
                    println!("{:?}", split);
                    search(
                        &mut board.board,
                        &(
                            split[1].parse::<u8>().unwrap(),
                            split[2].parse::<u8>().unwrap(),
                        ),
                    );
                } // TODO Search logic
                "3" | "Quit" => break,
                _ => {}
            };
            match board.print() {
                Ok(_) => {}
                Err(_) => println!("Something wrong during board print"),
            };
        }
    }
}

fn print_rules() -> io::Result<()> {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    const WHITE: Option<Color> = Some(Color::White);
    const HL: Option<Color> = Some(Color::Cyan);

    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}", "Welcome to the Treasure Hunt !")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}\n{}", "!", "You can ")?;
    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Move")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}", " around the place to ")?;

    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Search")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    writeln!(&mut buffer, "{}", " for the Treasure! Good Luck...")?;

    writeln!(&mut buffer, "{}", "Search will take one action, it lets you search for the Treasure on your current coordinates.")?;
    writeln!(&mut buffer, "{}", "\"Move (x,y)\" or \"Move [x,y]\" to go to a coordinate, you can only move within the board and you can only Move 4 blocks away at most.")?;

    return bufwtr.print(&buffer);
}

fn search(board: &mut Vec<Vec<char>>, point: &(u8, u8)) {
    let x: usize = point.0 as usize;
    let y: usize = point.1 as usize;

    board[x][y] = Board::SEARCHED_CHAR;
}
