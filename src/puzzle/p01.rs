use std::io::prelude::*;

pub fn solve1(input: String) -> i32 {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        ' ' | '\n' => floor, // ignore whitespace
        c => panic!("unexpected character: {}", c)
    })
}

pub fn solve2(input: String) -> i32 {
    input
        .chars()
        .fold(Err((1, 0)), |acc, c| acc.or_else(|(pos, floor)| {
            let next_floor = match c {
                '(' => floor + 1,
                ')' => floor - 1,
                ' ' | '\n' => floor, // ignore whitespace
                c => panic!("unexpected character: {}", c)
            };
            if next_floor < 0 {
                Ok(pos)
            } else {
                Err((pos + 1, next_floor))
            }
        }))
        .expect("Santa does not enter the basement")
}
