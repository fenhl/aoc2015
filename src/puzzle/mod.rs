use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod p01a;

pub struct Puzzle {
    day: u8,
    second: bool,
    solution: fn(String) -> i32
}

impl Puzzle {
    pub fn display(&self) -> String {
        format!("{second} puzzle of December {day}{ordinal_suffix}",
            second=if self.second { "second" } else { "first" },
            day=self.day,
            ordinal_suffix=match self.day { 1 => "st", 2 => "nd", 3 => "rd", _ => "th" }
        )
    }

    pub fn input_file(&self) -> File {
        File::open(Path::new(&format!("assets/input/p{day:02}{puzzle_id}.txt",
            day=self.day,
            puzzle_id=if self.second { "b" } else { "a" }
        ))).expect("input file not found")
    }

    pub fn solve(&self) -> i32 {
        let mut buf = String::default();
        self.input_file().read_to_string(&mut buf).expect("failed to read input");
        (self.solution)(buf)
    }
}

pub struct Iter {
    day: u8,
    second: bool
}

impl Default for Iter {
    fn default() -> Iter {
        Iter {
            day: 1,
            second: false
        }
    }
}

impl Iterator for Iter {
    type Item = Puzzle;

    fn next(&mut self) -> Option<Puzzle> {
        macro_rules! first {
            ($day:expr, $solution:expr) => {
                if self.day == $day && self.second == false {
                    self.second = true;
                    return Some(Puzzle {
                        day: $day,
                        second: false,
                        solution: $solution
                    });
                }
            };
        }
        first!(1, p01a::solve);
        None
    }
}
