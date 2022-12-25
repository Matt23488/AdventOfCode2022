use std::{collections::HashSet, fs};

fn main() {
    let rope_sim = match RopeSimulation::build_from_input() {
        Some(sim) => sim,
        None => {
            eprintln!("Couldn't parse input");
            return;
        }
    };

    let part_one_answer = rope_sim.simulate_unique_tail_positions(1);
    dbg!(part_one_answer);

    let part_two_answer = rope_sim.simulate_unique_tail_positions(9);
    dbg!(part_two_answer);
}

#[derive(Debug)]
struct RopeSimulation {
    head_movements: Vec<Direction>,
}

impl RopeSimulation {
    fn build_from_input() -> Option<Self> {
        let input = match fs::read_to_string("input.txt") {
            Ok(input) => input,
            Err(_) => return None,
        };

        Some(Self {
            head_movements: input
                .lines()
                .map(|line| Direction::build_from_line(line))
                .filter(|dir| dir.is_some())
                .map(|dir| dir.unwrap())
                .collect(),
        })
    }

    fn simulate_unique_tail_positions(&self, num_knots: u32) -> usize {
        let mut set = HashSet::new();

        let mut head_pos = Knot(0, 0);
        let mut knots: Vec<Knot> = (0..num_knots).map(|_| Knot(0, 0)).collect();

        for head_movement in &self.head_movements {
            let offset = head_movement.offset();
            for _ in 0..head_movement.count() {
                head_pos += offset;

                let mut prev_knot = head_pos;
                for knot in knots.iter_mut() {
                    let difference = prev_knot - *knot;
                    let difference_abs = difference.abs();
                    let normalized = difference.grid_normalized();

                    if difference_abs.x() > 1 {
                        *knot += Knot(normalized.x(), 0);
                        if difference_abs.y() > 0 {
                            *knot += Knot(0, normalized.y());
                        }
                    } else if difference_abs.y() > 1 {
                        *knot += Knot(0, normalized.y());
                        if difference_abs.x() > 0 {
                            *knot += Knot(normalized.x(), 0);
                        }
                    }

                    prev_knot = *knot;
                }

                set.insert(*knots.last().unwrap());
            }
        }

        set.len()
    }
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    fn build_from_line(line: &str) -> Option<Self> {
        let mut iter = line.split(' ');

        let dir = iter.next()?;
        let num = iter.next()?;

        let num: i32 = num.parse().unwrap_or(0);

        match dir {
            "U" => Some(Self::Up(num)),
            "D" => Some(Self::Down(num)),
            "L" => Some(Self::Left(num)),
            "R" => Some(Self::Right(num)),
            _ => None,
        }
    }

    fn count(&self) -> i32 {
        match self {
            Self::Up(count) => *count,
            Self::Down(count) => *count,
            Self::Left(count) => *count,
            Self::Right(count) => *count,
        }
    }

    fn offset(&self) -> Knot {
        match self {
            Self::Up(_) => Knot(0, 1),
            Self::Down(_) => Knot(0, -1),
            Self::Left(_) => Knot(-1, 0),
            Self::Right(_) => Knot(1, 0),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Knot(i32, i32);

impl Knot {
    fn abs(&self) -> Knot {
        Knot(self.0.abs(), self.1.abs())
    }

    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn grid_normalized(&self) -> Knot {
        Knot(
            if self.0 > 1 {
                1
            } else if self.0 < -1 {
                -1
            } else {
                self.0
            },
            if self.1 > 1 {
                1
            } else if self.1 < -1 {
                -1
            } else {
                self.1
            },
        )
    }
}

use std::ops;

impl ops::AddAssign<Knot> for Knot {
    fn add_assign(&mut self, rhs: Knot) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::Sub<Knot> for Knot {
    type Output = Knot;

    fn sub(self, rhs: Knot) -> Self::Output {
        Knot(self.0 - rhs.0, self.1 - rhs.1)
    }
}
