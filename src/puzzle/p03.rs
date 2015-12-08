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
