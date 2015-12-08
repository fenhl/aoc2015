#![warn(trivial_casts)]
#![forbid(unused, unused_extern_crates, unused_import_braces)]

extern crate regex;

mod puzzle;

fn main() {
    for puzzle in puzzle::Iter::default() {
        print!("{}:", puzzle.display());
        print!(" solving...");
        println!(" {}", puzzle.solve());
    }
}
