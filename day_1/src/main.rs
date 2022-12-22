use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut aggregated_calories = aggregate_calories()?;
    let max_calories = part_one(&aggregated_calories);
    dbg!(max_calories);

    if let Some(max_calories) = part_two(&mut aggregated_calories) {
        dbg!(max_calories);
    }

    Ok(())
}

fn aggregate_calories() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut counts = Vec::new();

    let text = fs::read_to_string("input.txt")?;

    let mut total = 0;
    for line in text.lines() {
        if line.is_empty() {
            counts.push(total);
            total = 0;
            continue;
        }

        total += line.parse::<i32>().unwrap_or_default();
    }

    if total > 0 {
        counts.push(total);
    }

    Ok(counts)
}

fn part_one(counts: &Vec<i32>) -> i32 {
    match counts.iter().max() {
        Some(val) => *val,
        None => -1
    }
}

fn part_two(counts: &mut Vec<i32>) -> Option<i32> {
    counts.sort();
    if let [ .., a, b, c ] = counts[..] {
        Some(a + b + c)
    } else {
        None
    }
}