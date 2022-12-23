use std::fs;

fn main() {
    let section_assignment_pairs = get_section_assignments();

    let part_one_answer = part_one(&section_assignment_pairs);
    dbg!(part_one_answer);

    let part_two_answer = part_two(&section_assignment_pairs);
    dbg!(part_two_answer);
}

#[derive(Debug)]
struct SectionAssignmentPair {
    a: (usize, usize),
    b: (usize, usize),
}

impl SectionAssignmentPair {
    fn is_extremely_inefficient(&self) -> bool {
        (self.a.0 >= self.b.0 && self.a.1 <= self.b.1)
            || (self.b.0 >= self.a.0 && self.b.1 <= self.a.1)
    }

    fn is_inefficient(&self) -> bool {
        (self.a.0 >= self.b.0 && self.a.0 <= self.b.1)
            || (self.a.1 >= self.b.0 && self.a.1 <= self.b.1)
            || (self.b.0 >= self.a.0 && self.b.0 <= self.a.1)
            || (self.b.1 >= self.a.0 && self.b.1 <= self.a.1)
    }
}

fn get_section_assignments() -> Vec<SectionAssignmentPair> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let (a, b) = match (iter.next(), iter.next()) {
                (Some(a), Some(b)) => (a, b),
                _ => panic!(),
            };

            let mut a_iter = a.split('-');
            let mut b_iter = b.split('-');
            let (a_start, a_end, b_start, b_end) =
                match (a_iter.next(), a_iter.next(), b_iter.next(), b_iter.next()) {
                    (Some(a_start), Some(a_end), Some(b_start), Some(b_end)) => {
                        (a_start, a_end, b_start, b_end)
                    }
                    _ => panic!(),
                };

            let (a_start, a_end, b_start, b_end) = match (
                a_start.parse::<usize>(),
                a_end.parse::<usize>(),
                b_start.parse::<usize>(),
                b_end.parse::<usize>(),
            ) {
                (Ok(a_start), Ok(a_end), Ok(b_start), Ok(b_end)) => {
                    (a_start, a_end, b_start, b_end)
                }
                _ => panic!(),
            };

            SectionAssignmentPair {
                a: (a_start, a_end),
                b: (b_start, b_end),
            }
        })
        .collect()
}

fn part_one(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> usize {
    section_assignment_pairs
        .iter()
        .filter(|pair| pair.is_extremely_inefficient())
        .count()
}

fn part_two(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> usize {
    section_assignment_pairs
        .iter()
        .filter(|pair| pair.is_inefficient())
        .count()
}
