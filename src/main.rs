#[allow(dead_code)]
use std::convert::From;
use std::io::{self, Write};

use read_input::prelude::*;
use regex::Regex;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

mod board;
mod player;
mod utils;

static RE: &str = r"^(y|n)$";

fn main() {
    let mut board = board::Board::new();

    // Start a game
    let usr_input = input()
        .repeat_msg("Would you like to start a game of Treasure Hunt? [y/n] ")
        .add_err_test(
            |x: &String| {
                Regex::new(RE)
                    .unwrap()
                    .is_match(&*String::from(x).to_lowercase())
            },
            "Incorrect input, please use 'Y', 'y' for 'yes' or 'N', 'n' for 'no'",
        )
        .get();

    if usr_input == "N" || usr_input == "n" {
        println!("Bye Bye !");
    } else {
        // TODO print rules
        match print_rules() {
            Ok(_) => {}
            Err(_) => println!("Something went wrong. Couldn't print the rules."),
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
                    let mut s: String = input().msg("Search coordinates:").get();
                    s = s.chars().filter(|c| !c.is_whitespace()).collect();
                    let split: Vec<&str> = s.split(&['(', ')', ','][..]).collect();
                    println!("{:?}", split);
                    search(
                        &mut board.board,
                        &(
                            split[0].parse::<u8>().unwrap(),
                            split[1].parse::<u8>().unwrap(),
                        ),
                    );
                    // let mut s: String = input().msg("Format for search is (x,y)").get();
                    // s = s.chars().filter(|c| !c.is_whitespace()).collect();
                    // let split: Vec<&str> = s.split(&['(', ')', ','][..]).collect();
                    // println!("{:?}", split);
                    // search(
                    //     &mut board.board,
                    //     &(
                    //         split[1].parse::<u8>().unwrap(),
                    //         split[2].parse::<u8>().unwrap(),
                    //     ),
                    // );
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
    write!(&mut buffer, "{}", "Welcome to the Treasure Hunt!\n")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}", "You can ")?;
    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Move")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    write!(&mut buffer, "{}", " around the place to ")?;

    buffer.set_color(ColorSpec::new().set_fg(HL))?;
    write!(&mut buffer, "{}", "Search")?;
    buffer.set_color(ColorSpec::new().set_fg(WHITE))?;
    writeln!(&mut buffer, "{}", " for the Treasure! Good Luck...")?;

    writeln!(&mut buffer, "{}", "\t[*] Search will take one action, it lets you search for the Treasure on your current coordinates.")?;
    writeln!(&mut buffer, "{}", "\t[*] \"Move (x,y)\" or \"Move [x,y]\" to go to a coordinate.\n\t[*] You can only move within the board and you can only Move 4 blocks away at most.")?;

    return bufwtr.print(&buffer);
}

fn search(board: &mut Vec<Vec<char>>, point: &(u8, u8)) {
    let x: usize = point.0 as usize;
    let y: usize = point.1 as usize;

    board[x][y] = board::Board::SEARCHED_CHAR;
}
