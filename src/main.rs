/// This is my own version of Dragon City for the CLI,
/// ENJOY! 😃
/// 
/// HINT: Use: 
/// `dragon-city cheat # or target/debug/dragon-city or cargo run -q -- ` 
/// to use command mode!
/// 
/// This is BETA stage, there may be some bugs.
/// If you find any bugs, create an issue.

// make constants.rs available
mod constants;

// state the constants that this file uses
use constants::{
    WELCOME_MESSAGE
};

fn main() {
    println!("{}", WELCOME_MESSAGE);
}
