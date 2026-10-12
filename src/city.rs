// city.rs

use std::iter::repeat_n;
use std::io::{Write, stdout};

/// This Module helps to make the cli more structured
/// 
/// It also carries many major data structures in the City

// use constants from constants.rs
use crate::constants::{
    CITY_LENGTH,
    CITY_HEIGHT,
    DEFAULT_CHAR,
    MOVE_UP,
    ERASE_LINE,
    FLUSH_ERROR
};


/// Enum to mention the type of a building
// pub enum BuildingType {
//     HABITAT = 1,
//     BUILDING = 2
// }

// /// Struct for a thing on an Island
// struct Building {

// }

/// Used to store everything about a currently running city

// one lifetime for current_help
pub(crate) struct City<'a> {
    /// Carries the entire thing in the CLI that is being printed
    /// Also structures each char in a sepeerate array
    pub cli_city: [[char; CITY_LENGTH]; CITY_HEIGHT],
    
    /// The bottom line that helps users select what they want to do
    pub current_help: &'a str,

    // /// The items in the island
    // island: Vec<>
}

impl City<'_> {
    /// Constructs a new City
    /// 
    /// Currently does the following things:
    ///     cli_city: completely blank filled with DEFAULT_CHAR
    ///     help: a blank string
    pub fn new() -> Self {
        Self {
            cli_city: [[DEFAULT_CHAR; CITY_LENGTH]; CITY_HEIGHT],
            current_help: "",
            // island: Vec::from([]),
        }
    }

    /// Prints a single frame based on stats
    /// 
    /// no_clear specifies whetehr the screen shoudl be cleared
    pub fn display(&self, no_clear: bool) {
        // get stream so taht we can flush beacause sometimes
        // 🦀 doesn't flush
        let mut stdout = stdout();

        if !no_clear {
            // clear what was printed before
            // go back CITY_HEIGHT + 1(help message) lines while clearing each line

            // use repeat_n to make a iterator
            for _ in repeat_n((), CITY_HEIGHT + 1).into_iter() {
                print!("{}{}", MOVE_UP, ERASE_LINE);
            };

            // Flush it if rust doesn't flush for us
            stdout.flush().expect(FLUSH_ERROR);
        }

        // iterate through each char to display the content first
        for line in self.cli_city {
            for o in line {
                print!("{}", o);
            };
            // Add a nl at the end of each line
            print!("\r\n");
        };

        // print the help message
        print!("{}", self.current_help);

        // flush
        stdout.flush().expect(FLUSH_ERROR);
    }
}
