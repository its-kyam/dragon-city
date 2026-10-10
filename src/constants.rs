/// Module to store all the constants and strings in this project
/// 
/// Also this changes frame after input

// Strings
/// Basic Message
pub const WELCOME_MESSAGE: &str = "Welcome to Dragon City";
pub const FLUSH_ERROR: &str = "An Error Occured while flushing";

// Constants
pub(crate) const ERASE_LINE: &str = "\033[2K";
pub(crate) const CITY_LENGTH: usize = 50;
pub(crate) const CITY_HEIGHT: usize = 10;
pub(crate) const DEFAULT_CHAR: char = ' ';
pub(crate) const MOVE_UP: &str = "\033[1A";
