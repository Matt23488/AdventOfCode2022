use std::fs;

fn main() {
    let mut crane = Crane::new();
    let start_pos = crane.read_top();
    dbg!(start_pos);

    crane.execute_9000();
    let part_one_answer = crane.read_top();
    dbg!(part_one_answer);

    let mut crane = Crane::new();

    crane.execute_9001();
    let part_two_answer = crane.read_top();
    dbg!(part_two_answer);
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Crane {
    fn new() -> Crane {
        let mut stack_lines = Vec::new();
        let contents = fs::read_to_string("input.txt").unwrap();
        let mut input_iter = contents.lines();
        loop {
            stack_lines.push(match input_iter.next() {
                Some("") => break,
                Some(line) => line,
                None => panic!(),
            });
        }

        let stack_nums = stack_lines.pop().unwrap();
        let mut stacks = Vec::new();
        for i in 1..stack_nums.len() {
            match stack_nums.chars().nth(i).map(|c| c.to_digit(10)).flatten() {
                Some(num) => num,
                None => continue,
            };

            let mut stack = Vec::new();
            stack_lines
                .iter()
                .rev()
                .map(|line| line.chars().nth(i).unwrap())
                .filter(|c| *c != ' ')
                .for_each(|c| stack.push(c));

            stacks.push(stack);
        }

        let instructions: Vec<Instruction> = input_iter
            .map(|line| {
                line.split(' ')
                    .map(|word| word.parse::<usize>())
                    .filter(|num| num.is_ok())
                    .map(|num| num.unwrap())
            })
            .map(|mut nums| Instruction {
                count: nums.next().unwrap(),
                src: nums.next().unwrap() - 1,
                dst: nums.next().unwrap() - 1,
            })
            .collect();

        Crane {
            stacks,
            instructions,
        }
    }

    fn execute_9000(&mut self) {
        for instruction in &self.instructions {
            for _ in 0..instruction.count {
                let crt = self
                    .stacks
                    .iter_mut()
                    .nth(instruction.src)
                    .unwrap()
                    .pop()
                    .unwrap();
                self.stacks
                    .iter_mut()
                    .nth(instruction.dst)
                    .unwrap()
                    .push(crt);
            }
        }
    }

    fn execute_9001(&mut self) {
        for instruction in &self.instructions {
            let mut tmp = Vec::new();
            for _ in 0..instruction.count {
                tmp.push(
                    self.stacks
                        .iter_mut()
                        .nth(instruction.src)
                        .unwrap()
                        .pop()
                        .unwrap(),
                );
            }

            for _ in 0..instruction.count {
                self.stacks
                    .iter_mut()
                    .nth(instruction.dst)
                    .unwrap()
                    .push(tmp.pop().unwrap());
            }
        }
    }

    fn read_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}
