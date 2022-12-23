use std::{collections::HashSet, fs};

fn main() {
    let buffer = Buffer::new();

    match buffer.start_of_marker(4) {
        Some(part_one_answer) => {
            dbg!(part_one_answer);
        }
        None => println!("Something went wrong with part one"),
    };

    match buffer.start_of_marker(14) {
        Some(part_two_answer) => {
            dbg!(part_two_answer);
        }
        None => println!("Something went wrong with part two"),
    };
}

#[derive(Debug)]
struct Buffer(String);

impl Buffer {
    fn new() -> Buffer {
        Buffer(fs::read_to_string("input.txt").unwrap())
    }

    fn start_of_marker(&self, marker_size: usize) -> Option<usize> {
        for pos in (marker_size - 1)..self.0.len() {
            if marker_size
                == (&self.0[(pos + 1 - marker_size)..=pos])
                    .chars()
                    .collect::<HashSet<_>>()
                    .len()
            {
                return Some(pos + 1);
            }
        }

        None
    }
}
