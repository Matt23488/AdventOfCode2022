use std::fs;

fn main() {
    let processor = Processor::build_from_input();

    let part_one_answer = processor.get_signal_strength();
    dbg!(part_one_answer);

    let part_two_answer = processor.render_crt();
    println!("\n{part_two_answer}");
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn build_from_line(line: &str) -> Instruction {
        let mut iter = line.split(' ');
        match iter.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(iter.next().unwrap().parse().unwrap()),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Processor {
    instructions: Vec<Instruction>,
}

impl Processor {
    fn build_from_input() -> Processor {
        let input = fs::read_to_string("input.txt").unwrap();

        Processor {
            instructions: input
                .lines()
                .map(|line| Instruction::build_from_line(line))
                .collect(),
        }
    }

    fn get_signal_strength(&self) -> i32 {
        let mut strength = 0;

        self.execute(|x, cycle| {
            let cycle = cycle + 1;
            if self.is_signal_strength_cycle(cycle) {
                strength += cycle * x;
            }
        });

        strength
    }

    fn render_crt(&self) -> String {
        let mut screen = Vec::new();

        self.execute(|x, cycle| {
            let pos = cycle % 40;

            screen.push(if (pos - x as i32).abs() <= 1 {
                '#'
            } else {
                '.'
            });

            if pos == 39 {
                screen.push('\n');
            }
        });

        screen.iter().collect()
    }

    fn execute<F>(&self, mut callback: F)
    where
        F: FnMut(i32, i32),
    {
        let mut x = 1;
        let mut cycle = 0;
        let mut instruction_iter = self.instructions.iter();

        loop {
            let instruction = instruction_iter.next();
            let (instruction_cycles, x_inc) = match instruction {
                Some(Instruction::Noop) => (1, 0),
                Some(Instruction::AddX(inc)) => (2, *inc),
                None => break,
            };

            for _ in 0..instruction_cycles {
                callback(x, cycle);

                cycle += 1;
            }

            x += x_inc;
        }
    }

    fn is_signal_strength_cycle(&self, cycle: i32) -> bool {
        cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220
    }
}
