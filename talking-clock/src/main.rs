/*
 * This is a list of numbers the talking clock would need
 * Oh
 * One
 * Two
 * Three
 * Four
 * Five
 * Six
 * Seven
 * Eight
 * Nine
 * Ten
 * Eleven
 * Twelve
 * Thirteen
 * Fourteen
 * Fifteen
 * Sixteen
 * Seventeen
 * Eighteen
 * Nineteen
 * Twenty
 * Thirty
 * Forty
 * Fifty
*/
#![feature(slice_patterns)]
#[macro_use]
extern crate text_io;
use std::{fmt, str};

#[derive(Debug)]
struct Time {
    hour: u8,
    min: u8,
}



impl str::FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u8> = s.split(":").filter_map(|s| s.parse().ok()).collect();

        match parts[..] {
            [hour, min] => {
                if hour > 23 || min > 59 {
                    return Err(format!("invalid format: {}", s));
                }
                Ok(Time {
                    hour: hour,
                    min: min,
                })
            }
            _ => Err(format!("Invalid command: {}", s)),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_words(time: &Time) -> &'static str {
            let hour_in_words = match time.hour {
                0 | 12 => "Twelve",
                23 | 11 => "Eleven",
                22 | 10 => "Ten",
                21 | 9 => "Nine",
                20 | 8 => "Eight",
                19 | 7 => "Seven",
                18 | 6 => "Six",
                17 | 5 => "Five",
                16 | 4 => "Four",
                15 | 3 => "Three",
                14 | 2 => "Two",
                13 | 1 => "One",
                _ => unreachable!(),
            };
            hour_in_words
        }

        write!(f, "({}, {})", to_words(self), self.min)
    }
}



fn main() {
    loop {
        let i: String = read!();
        let val: Time = i.trim().parse::<Time>().ok().unwrap();
        println!("{}", val);
    }
}
