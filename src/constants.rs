// constants.rs

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
pub(crate) const ERASE_LINE: &str = "\033[2K";
pub(crate) const CITY_LENGTH: usize = 60;
pub(crate) const CITY_HEIGHT: usize = 25;
pub(crate) const DEFAULT_CHAR: char = '.';
pub(crate) const MOVE_UP: &str = "\033[1A";
pub(crate) const HIDE_CURSOR: &str = "\x1b[?25l";
pub(crate) const SHOW_CURSOR: &str = "\x1b[?25h";

// Images (BETA)
pub const LOADING_SCREEN: [[char; CITY_LENGTH]; CITY_HEIGHT] = [['🍳'; CITY_LENGTH]; CITY_HEIGHT];
