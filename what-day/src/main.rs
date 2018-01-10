#[macro_use]
extern crate text_io;
use what_day::*;

fn main() {
    loop {
        let i: String = read!();
        let dt = Date::new(i).unwrap();
        println!("{}", dt.give_day());
    }
}

mod what_day {
    use std::error::Error;
    #[derive(Debug)]
    pub struct Date {
        year: u16,
        month: u16,
        date: u16,
        dayrange: [(u16, &'static str); 7],
    }

    impl Date {
        pub fn new(s: String) -> Result<Self, Box<Error>> {
            let d_comps: Vec<u16> = s.split("-").filter_map(|s| s.parse().ok()).collect();
            if d_comps.len() != 3 {
                return Err(From::from("invalid format"));
            };
            Ok(Date {
                year: d_comps[0],
                month: d_comps[1],
                date: d_comps[2],
                dayrange: [
                    (2, "Monday"),
                    (3, "Tuesday"),
                    (4, "Wednesday"),
                    (5, "Thursday"),
                    (6, "Friday"),
                    (0, "Saturday"),
                    (1, "Sunday"),
                ],
            })
        }

        pub fn give_day(self) -> &'static str {
            let (month, year) = if self.month < 3 {
                (self.month + 12, self.year - 1)
            } else {
                (self.month, self.year)
            };

            let q = self.date;
            let m = month;
            let k = year % 100;
            let j = year / 100;
            // source : https://en.wikipedia.org/wiki/Zeller%27s_congruence
            let h = (q + (13 * (m + 1) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
            self.dayrange.iter().find(|&s| s.0 == h).unwrap().1
        }
    }
}
