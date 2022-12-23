use std::{collections::HashMap, fs};

fn main() {
    let priority_map = get_priority_map();
    let rucksacks = get_rucksacks();

    let part_one_answer = part_one(&priority_map, &rucksacks);
    dbg!(part_one_answer);

    let part_two_answer = part_two(&priority_map, &rucksacks);
    dbg!(part_two_answer);
}

fn get_priority_map() -> HashMap<char, usize> {
    let mut priority_map = HashMap::new();

    ('a'..='z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .for_each(|(c, priority)| {
            priority_map.insert(c, priority);
        });

    ('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1 + 26))
        .for_each(|(c, priority)| {
            priority_map.insert(c, priority);
        });

    priority_map
}

#[derive(Debug)]
struct Rucksack(String);

impl Rucksack {
    fn compartments(&self) -> Compartments {
        let Rucksack(contents) = self;
        let mid_point = contents.len() / 2;
        Compartments(
            &contents[0..mid_point],
            &contents[mid_point..contents.len()],
        )
    }
}

#[derive(Debug)]
struct Compartments<'c>(&'c str, &'c str);

impl<'c> Compartments<'c> {
    fn find_common(&self) -> char {
        self.0
            .chars()
            .find(|c| self.1.chars().find(|oc| oc == c).is_some())
            .unwrap()
    }
}

fn get_rucksacks() -> Vec<Rucksack> {
    let text = fs::read_to_string("input.txt").unwrap();

    text.lines()
        .map(|line| Rucksack(String::from(line)))
        .collect()
}

fn part_one(priority_map: &HashMap<char, usize>, rucksacks: &Vec<Rucksack>) -> usize {
    rucksacks
        .iter()
        .map(|sack| sack.compartments())
        .map(|compartments| compartments.find_common())
        .map(|common| priority_map.get(&common).unwrap())
        .sum()
}

fn part_two(priority_map: &HashMap<char, usize>, rucksacks: &Vec<Rucksack>) -> usize {
    let mut iter = rucksacks.iter();
    let mut sum = 0usize;
    loop {
        let (sack_a, sack_b, sack_c) = match (iter.next(), iter.next(), iter.next()) {
            (Some(sack_a), Some(sack_b), Some(sack_c)) => (sack_a, sack_b, sack_c),
            _ => break sum,
        };

        let badge = sack_a
            .0
            .chars()
            .find(|a| {
                sack_b.0.chars().find(|b| b == a).is_some()
                    && sack_c.0.chars().find(|c| c == a).is_some()
            })
            .unwrap();

        sum += priority_map.get(&badge).unwrap();
    }
}
