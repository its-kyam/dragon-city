// constants.rs
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

/// Module to store all the constants and strings in this project
///
/// Also this changes frame after input
// Strings
/// Basic Message
pub const WELCOME_MESSAGE: &str = "Welcome to Dragon City, press q to quit anytime";
pub const FLUSH_ERROR: &str = "An Error Occured while flushing";
pub const STTY_ERROR: &str = "Failed to set stty raw mode!";
pub const INPUT_ERROR: &str = "Failed to take input through stdin";

// Constants
pub(crate) const ERASE_LINE: &str = "\x1b[2K";
pub(crate) const CITY_LENGTH: usize = 60;
pub(crate) const CITY_HEIGHT: usize = 25;
pub(crate) const DEFAULT_CHAR: char = '.';
pub(crate) const MOVE_UP: &str = "\x1b[1A";
pub(crate) const HIDE_CURSOR: &str = "\x1b[?25l";
pub(crate) const SHOW_CURSOR: &str = "\x1b[?25h";

// Images (BETA)
pub const LOADING_SCREEN: [[char; CITY_LENGTH]; CITY_HEIGHT] = [['🍳'; CITY_LENGTH]; CITY_HEIGHT];
