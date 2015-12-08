use std::ops::Add;

use regex::Regex;

pub fn solve1(input: String) -> i32 {
    let re = Regex::new("^([0-9]+)x([0-9]+)x([0-9]+)$").expect("failed to parse input format regex");
    input.split_whitespace().fold(0, |sum, line| {
        let captures = re.captures(line).expect("failed to parse input");
        let width: i32 = captures.at(1).expect("failed to get width").parse().expect("failed to parse width");
        let length: i32 = captures.at(2).expect("failed to get length").parse().expect("failed to parse length");
        let height: i32 = captures.at(3).expect("failed to get height").parse().expect("failed to parse height");
        let sides = [width * length, width * height, length * height];
        sum + sides.iter().min().expect("failed to get smallest side") + sides.iter().fold(0, Add::add) * 2
    })
}

pub fn solve2(input: String) -> i32 {
    let re = Regex::new("^([0-9]+)x([0-9]+)x([0-9]+)$").expect("failed to parse input format regex");
    input.split_whitespace().fold(0, |sum, line| {
        let captures = re.captures(line).expect("failed to parse input");
        let width: i32 = captures.at(1).expect("failed to get width").parse().expect("failed to parse width");
        let length: i32 = captures.at(2).expect("failed to get length").parse().expect("failed to parse length");
        let height: i32 = captures.at(3).expect("failed to get height").parse().expect("failed to parse height");
        let perimeters = [2 * width + 2 * length, 2 * width + 2 * height, 2 * length + 2 * height];
        sum + perimeters.iter().min().expect("failed to get smallest perimeter") + width * length * height
    })
}
