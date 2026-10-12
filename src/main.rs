// main.rs
// Copyright (C) 2026 Kushaal Srinandan Bandi

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// This is my own version of Dragon City for the CLI,
/// ENJOY! 😃
///
/// This is SUPER ALPHA stage, there may be some bugs.
/// If you find any bugs, create an issue.
use std::io::{
    Error as IOError, ErrorKind::Other as OtherIOErrorKind, Read, Result as IOResult, Write, stdin,
    stdout,
};
use std::process::Command;

// make constants.rs available
mod constants;

// make city.rs available
mod city;

// state the constants that this file uses
use constants::{
    HIDE_CURSOR, INPUT_ERROR, LOADING_SCREEN, SHOW_CURSOR, STTY_ERROR, WELCOME_MESSAGE,
};

// get City
use city::City;

use crate::constants::FLUSH_ERROR;

/// RAII guard to ensure terminal settings are restored
struct Durf;

impl Durf {
    /// make a new guard LOL 😭😭😭
    ///
    /// Also does settings for raw mode
    /// Uses stty
    fn new() -> IOResult<Self> {
        // settings for raw mode
        let status = Command::new("stty").arg("raw").arg("-echo").status()?;

        if !status.success() {
            Err(IOError::new(OtherIOErrorKind, STTY_ERROR))
        } else {
            Ok(Self)
        }
    }
}

// Make sure it always exits correctly even on panic
impl Drop for Durf {
    /// Returns input mode to normal
    ///
    /// Also makes cursor visible
    fn drop(&mut self) {
        // completely ignore beacuse error printing is kind of useless here
        let _ = Command::new("stty").arg("-raw").arg("echo").status();

        let mut stdout = stdout();

        println!("{}", SHOW_CURSOR);
        stdout.flush().expect(FLUSH_ERROR)
    }
}

fn main() {
    // make a new city
    let mut city = City::new();

    // get stdout to flush
    let mut stdout = stdout();
    let mut stdin = stdin();

    // lets hide the cursor before starting the game
    print!("{}", HIDE_CURSOR);

    // lets now make input raw mode
    let _guard = Durf::new();

    // flush if rust doesn't flush
    stdout.flush().expect(FLUSH_ERROR);

    // GAME START

    city.current_help = WELCOME_MESSAGE;
    city.cli_city = LOADING_SCREEN;

    let mut input = [0u8; 1];

    // display and hang
    city.display(true);
    loop {
        stdin.read_exact(&mut input).expect(INPUT_ERROR);

        if input[0] == b'q' {
            return;
        }
    }
}
