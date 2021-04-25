use std::io::{self, stdout, Write};

use read_input::prelude::*;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::board::Board;

mod board;
mod player;
mod utils;

fn main() {
    // Start a game
    let mut board = board::Board::new();

    // print the rules
    print_rules().map_err(|err| println!("{:?}", err)).ok();

    ask_for_color(&mut board);

    // print the updated board at the start of the round + simple error handling
    board.print().map_err(|err| println!("{:?}", err)).ok();
    while !board.treasure_found {
        let usr_input = ask_for_action().to_lowercase();
        match &*usr_input {
            "1" | "move" => {
                move_logic(&mut board);
            }
            "2" | "search" => {
                board.search();
            }
            "3" | "help" => Board::print_help(),
            "4" | "quit" => break,
            _ => {}
        };
    }
}

fn move_logic(board: &mut Board) {
    let mut result: (bool, u8, u8) = (false, 0, 0);
    let mut coord = String::new();
    while !result.0 {
        coord.clear();
        print!("Move: ");
        let _ = stdout().flush();
        match io::stdin().read_line(&mut coord) {
            Ok(_) => {}
            Err(_) => println!("Something went wrong, couldn't get input."),
        }
        // Clear line return from the line read
        coord = coord.trim_end_matches('\n').parse().unwrap();
        result = board.validate_move_coordinates(coord.as_str());
        if result.0 {
            result.0 = board.get_distance_to(result.1, result.2) <= Board::MAX_DIST;
            if !result.0 {
                println!(
                    "You can't move that far! Movement is limited to {} blocs",
                    Board::MAX_DIST
                );
            }
        }
    }
    board.move_to(result.1, result.2);
    board.print().map_err(|err| println!("{:?}", err)).ok();
}

fn ask_for_color(board: &mut Board) {
    let mut usr_color = String::new();
    while !board.player.set_color(usr_color.as_str()) {
        usr_color.clear();
        print!("Please choose your player colour, either in English or with an RGB value (e.g.: Green, 133,230,89): ");
        let _ = stdout().flush();
        match io::stdin().read_line(&mut usr_color) {
            Ok(_) => {}
            Err(_) => println!("Something went wrong, couldn't get input"),
        }
        // Clear line return from the line read
        usr_color = usr_color.trim_end_matches('\n').parse().unwrap();
    }
}

fn print_rules() -> io::Result<()> {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
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

    return bufwtr.print(&buffer);
}

/// At the start of each turn the player is asked for an action that can be chosen from a menu
/// This function enables us to print the menu and get the user's input
fn ask_for_action() -> String {
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
