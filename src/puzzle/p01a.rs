use std::io::prelude::*;

pub fn solve(input: String) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        ' ' | '\n' => acc, // ignore whitespace
        c => panic!("unexpected character: {}", c)
    })
}
