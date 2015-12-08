use std::collections::HashSet;

pub fn solve1(input: String) -> i32 {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input.chars().fold((0, 0), |(x, y), c| {
        let next_coords = match c {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            ' ' | '\n' => (x, y), // ignore whitespace
            c => panic!("unexpected character: {}", c)
        };
        visited.insert(next_coords);
        next_coords
    });
    visited.len() as i32
}

pub fn solve2(input: String) -> i32 {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    for (i, c) in input.chars().enumerate() {
        let (x, y) = if i % 2 == 0 { (santa_x, santa_y) } else { (robo_x, robo_y) };
        let next_coords = match c {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            ' ' | '\n' => (x, y), // ignore whitespace
            c => panic!("unexpected character: {}", c)
        };
        visited.insert(next_coords);
        if i % 2 == 0 {
            santa_x = next_coords.0;
            santa_y = next_coords.1;
        } else {
            robo_x = next_coords.0;
            robo_y = next_coords.1;
        }
    }
    visited.len() as i32
}
