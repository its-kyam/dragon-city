/// This is my own version of Dragon City for the CLI,
/// ENJOY! 😃
/// 
/// HINT: Use: 
/// `dragon-city cheat # or target/debug/dragon-city or cargo run -q -- ` 
/// for command mode!
/// 
/// This is BETA stage, there may be some bugs.
/// If you find any bugs, create an issue.

// make constants.rs available
mod constants;

// make city.rs available
mod city;

// state the constants that this file uses
use constants::{
    WELCOME_MESSAGE
};

// get City
use city::{
    City
};

fn main() {
    // make a new city
    let mut city = City::new();

    city.current_help = WELCOME_MESSAGE;

    // display and hang
    city.display(true);
    loop {};
}
