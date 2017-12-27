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
use std::str;

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

fn main() {
    loop {
        let i: String = read!();
        let val = i.trim().parse::<Time>();
        println!("{:?}", val)
    }
}
