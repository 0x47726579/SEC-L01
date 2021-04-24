/*! Visual only `Board` functions

Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

#[path = "./player.rs"]
pub mod player;

#[path = "./utils.rs"]
mod utils;

use core::option::Option::Some;
use p::Player;
use player as p;
use rand::Rng;
use std::io::{Result, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use u::Point;
use utils as u;

#[derive(Debug)]
pub struct Board {
    //TODO
    pub board: Vec<Vec<char>>,
    _player: Player,
    treasure: Point,
}

impl Board {
    const BOARD_COLOR: Color = Color::White;
    pub(crate) const BOARD_WIDTH: u8 = 15;
    pub(crate) const BOARD_HEIGHT: u8 = 15;

    const EMPTY_CHAR: char = '.';
    const PLAYER_CHAR: char = '@';
    pub const SEARCHED_CHAR: char = 'S';

    pub fn new() -> Self {
        let size = (Self::BOARD_WIDTH, Self::BOARD_HEIGHT);
        let mut rng = rand::thread_rng();
        let rand_point: Point = rng.gen();
        Self {
            board: vec![vec![Self::EMPTY_CHAR; size.0 as usize]; size.1 as usize],
            _player: Player::new(),
            treasure: rand_point,
        }
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
                // TODO
                let mut grid_c = self.board[x as usize][y as usize];
                if (x, y) == self.treasure.as_tuple() {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                    grid_c = 'X';
                }
                if (x, y) == self._player.position.as_tuple() {
                    buffer.set_color(ColorSpec::new().set_fg(Some(self._player.color)))?;
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

        // Debug prints
        const DEBUG: bool = true;
        if DEBUG {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut buffer)?;
            writeln!(&mut buffer, "[=== DEBUG ===]")?;
            write!(
                &mut buffer,
                "Treasure infos: {:?}\nPlayer infos: {:?}",
                self.treasure, self._player
            )?;
            writeln!(&mut buffer, "[=== DEBUG ===]")?;
        }

        writeln!(&mut buffer)?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return buffer_writer.print(&buffer);
    }
}
