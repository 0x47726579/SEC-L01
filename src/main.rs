use std::io::{self, stdout, Write};

use crate::board::Board;

mod board;
mod player;
mod utils;

fn main() {
    // Start a game
    let mut board = board::Board::new();

    // print the rules
    utils::print_rules()
        .map_err(|err| println!("{:?}", err))
        .ok();

    ask_for_color(&mut board);

    // print the updated board at the start of the round + simple error handling
    board.print().map_err(|err| println!("{:?}", err)).ok();
    while !board.treasure_found {
        let usr_input = utils::ask_for_action().to_lowercase();
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
