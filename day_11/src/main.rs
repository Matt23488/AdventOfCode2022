fn main() {
    let mut hijinx = match the_code::Hijinx::build_from_input() {
        Some(hijinx) => hijinx,
        None => return,
    };

    let part_one_answer = hijinx.calculate_monkey_business(false, 20);
    hijinx.print();
    println!("Part One Answer: {part_one_answer}");

    let mut hijinx = match the_code::Hijinx::build_from_input() {
        Some(hijinx) => hijinx,
        None => return,
    };

    let part_two_answer = hijinx.calculate_monkey_business(true, 10_000);
    hijinx.print();
    println!("Part Two Answer: {part_two_answer}");
}

mod the_code {
    use std::{collections::HashMap, str::Lines};

    #[derive(Debug)]
    struct WorryLevel(u128);

    #[derive(Debug)]
    struct Test(u128, usize, usize);

    #[derive(Debug)]
    enum Operator {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    #[derive(Debug)]
    enum Operand {
        Constant(u128),
        Variable,
    }

    #[derive(Debug)]
    struct Operation {
        lhs: Operand,
        rhs: Operand,
        op: Operator,
    }

    #[derive(Debug)]
    struct Monkey {
        items: Option<Vec<WorryLevel>>,
        operation: Operation,
        test: Test,
        inspection_count: u128,
    }

    #[derive(Debug)]
    enum ParseResult<T> {
        Ok(T),
        FailedParse,
        EndOfInput,
    }

    #[derive(Debug)]
    struct Throw(WorryLevel, usize);

    #[derive(Debug)]
    pub struct Hijinx {
        monkeys: Vec<Monkey>,
    }

    impl WorryLevel {
        fn panic(&mut self, operation: &Operation, modulus: u128) {
            operation.apply(self, modulus);
        }

        fn relax(&mut self) {
            self.0 /= 3;
        }
    }

    const TEST_VAL_TEXT: &str = "  Test: divisible by ";
    const TEST_TRUE_TEXT: &str = "    If true: throw to monkey ";
    const TEST_FALSE_TEXT: &str = "    If false: throw to monkey ";
    impl Test {
        fn parse_test(iter: &mut Lines) -> ParseResult<Test> {
            let val = match iter.next() {
                Some(line) => match line.get(..TEST_VAL_TEXT.len()) {
                    Some(TEST_VAL_TEXT) => match line.get(TEST_VAL_TEXT.len()..) {
                        Some(val) => match val.parse::<u128>() {
                            Ok(val) => val,
                            Err(_) => return ParseResult::FailedParse,
                        },
                        None => return ParseResult::FailedParse,
                    },
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            let monkey_if_true = match iter.next() {
                Some(line) => match line.get(..TEST_TRUE_TEXT.len()) {
                    Some(TEST_TRUE_TEXT) => match line.get(TEST_TRUE_TEXT.len()..) {
                        Some(val) => match val.parse::<usize>() {
                            Ok(val) => val,
                            Err(_) => return ParseResult::FailedParse,
                        },
                        None => return ParseResult::FailedParse,
                    },
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            let monkey_if_false = match iter.next() {
                Some(line) => match line.get(..TEST_FALSE_TEXT.len()) {
                    Some(TEST_FALSE_TEXT) => match line.get(TEST_FALSE_TEXT.len()..) {
                        Some(val) => match val.parse::<usize>() {
                            Ok(val) => val,
                            Err(_) => return ParseResult::FailedParse,
                        },
                        None => return ParseResult::FailedParse,
                    },
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            ParseResult::Ok(Test(val, monkey_if_true, monkey_if_false))
        }

        fn it(&self, worry_level: &WorryLevel) -> usize {
            if worry_level.0 % self.0 == 0 {
                self.1
            } else {
                self.2
            }
        }
    }

    impl Operand {
        fn build_from_text(text: Option<&str>) -> Option<Operand> {
            match text {
                Some("old") => Some(Operand::Variable),
                Some(constant) => match constant.parse::<u128>() {
                    Ok(val) => Some(Operand::Constant(val)),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }

    impl Operation {
        fn build_from_text(text: &str) -> Option<Operation> {
            let mut iter = text.split(' ');

            let lhs = match Operand::build_from_text(iter.next()) {
                Some(lhs) => lhs,
                None => return None,
            };

            let op = match iter.next() {
                Some("+") => Operator::Add,
                Some("-") => Operator::Subtract,
                Some("*") => Operator::Multiply,
                Some("/") => Operator::Divide,
                _ => return None,
            };

            let rhs = match Operand::build_from_text(iter.next()) {
                Some(rhs) => rhs,
                None => return None,
            };

            Some(Operation { lhs, op, rhs })
        }

        fn apply(&self, worry_level: &mut WorryLevel, modulus: u128) {
            let lhs = match self.lhs {
                Operand::Constant(c) => c,
                Operand::Variable => worry_level.0,
            };

            let rhs = match self.rhs {
                Operand::Constant(c) => c,
                Operand::Variable => worry_level.0,
            };

            worry_level.0 = match self.op {
                Operator::Add => lhs + rhs,
                Operator::Subtract => lhs - rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Divide => lhs / rhs,
            } % modulus;
        }
    }

    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Worry level is multiplied by {}",
                match self.rhs {
                    Operand::Constant(c) => format!("{c}"),
                    Operand::Variable => String::from("itself"),
                }
            )
        }
    }

    const HEADER_TEXT: &str = "Monkey ";
    const STARTING_ITEMS_TEXT: &str = "  Starting items: ";
    const OPERATION_TEXT: &str = "  Operation: new = ";
    impl Monkey {
        fn build_from_text(text: &str) -> Vec<Monkey> {
            let mut monkeys = Vec::new();

            let mut iter = text.lines();

            loop {
                match Monkey::parse_monkey(&mut iter) {
                    ParseResult::Ok(monkey) => monkeys.push(monkey),
                    ParseResult::FailedParse => continue,
                    ParseResult::EndOfInput => break,
                };
            }

            monkeys
        }

        fn parse_monkey(iter: &mut Lines) -> ParseResult<Monkey> {
            match iter.next() {
                Some(line) => match line.get(..HEADER_TEXT.len()) {
                    Some(HEADER_TEXT) => (),
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            let line = match iter.next() {
                Some(line) => match line.get(..STARTING_ITEMS_TEXT.len()) {
                    Some(STARTING_ITEMS_TEXT) => line.get(STARTING_ITEMS_TEXT.len()..),
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            let starting_items: Vec<WorryLevel> = match line {
                Some(list) => list
                    .split(", ")
                    .map(|worry_level| worry_level.parse::<u128>())
                    .filter(|worry_level| worry_level.is_ok())
                    .map(|worry_level| WorryLevel(worry_level.unwrap()))
                    .collect(),
                None => return ParseResult::FailedParse,
            };

            let line = match iter.next() {
                Some(line) => match line.get(..OPERATION_TEXT.len()) {
                    Some(OPERATION_TEXT) => match line.get(OPERATION_TEXT.len()..) {
                        Some(line) => line,
                        None => return ParseResult::FailedParse,
                    },
                    _ => return ParseResult::FailedParse,
                },
                None => return ParseResult::EndOfInput,
            };

            let operation = match Operation::build_from_text(line) {
                Some(operation) => operation,
                None => return ParseResult::FailedParse,
            };

            let test = match Test::parse_test(iter) {
                ParseResult::Ok(test) => test,
                ParseResult::FailedParse => return ParseResult::FailedParse,
                ParseResult::EndOfInput => return ParseResult::EndOfInput,
            };

            ParseResult::Ok(Monkey {
                items: Some(starting_items),
                operation,
                test,
                inspection_count: 0,
            })
        }

        fn take_turn(&mut self, very_worried: bool, modulus: u128) -> Vec<Throw> {
            let throws = self
                .items
                .take()
                .unwrap()
                .into_iter()
                .map(|mut item| {
                    // inspect
                    item.panic(&self.operation, modulus);
                    self.inspection_count += 1;

                    // get bored
                    if !very_worried {
                        item.relax();
                    }

                    // throw
                    let recipient = self.test.it(&item);
                    Throw(item, recipient)
                })
                .collect::<Vec<_>>();

            self.items = Some(Vec::new());

            throws
        }

        fn catch(&mut self, item: WorryLevel) {
            if let Some(items) = &mut self.items {
                items.push(item);
            }
        }
    }

    impl std::fmt::Display for Monkey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|i| i.0.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    impl Hijinx {
        pub fn _build_from_example() -> Option<Hijinx> {
            Hijinx::build("example.txt")
        }

        pub fn build_from_input() -> Option<Hijinx> {
            Hijinx::build("input.txt")
        }

        fn build(file: &str) -> Option<Hijinx> {
            let input = match std::fs::read_to_string(file) {
                Ok(input) => input,
                Err(_) => return None,
            };

            Some(Hijinx {
                monkeys: Monkey::build_from_text(&input),
            })
        }

        pub fn calculate_monkey_business(&mut self, very_worried: bool, num_rounds: u32) -> u128 {
            let modulus = self
                .monkeys
                .iter()
                .map(|monkey| monkey.test.0)
                .reduce(|acc, test| acc * test)
                .unwrap();

            for _ in 0..num_rounds {
                let mut map = HashMap::<usize, Option<Vec<WorryLevel>>>::with_capacity(50);
                for (i, monkey) in self.monkeys.iter_mut().enumerate() {
                    // catch items thrown to current monkey
                    match map.get_mut(&i) {
                        Some(items @ Some(_)) => {
                            items
                                .take()
                                .unwrap()
                                .into_iter()
                                .for_each(|item| monkey.catch(item));
                        }
                        _ => (),
                    };

                    monkey
                        .take_turn(very_worried, modulus)
                        .into_iter()
                        .for_each(|Throw(item, recipient)| {
                            match map.get_mut(&recipient) {
                                Some(Some(items)) => {
                                    items.push(item);
                                }
                                _ => {
                                    let items = vec![item];
                                    map.insert(recipient, Some(items));
                                }
                            };
                        });
                }

                // catch any remaining uncaught items
                for (i, monkey) in self.monkeys.iter_mut().enumerate() {
                    match map.get_mut(&i) {
                        Some(items @ Some(_)) => {
                            items
                                .take()
                                .unwrap()
                                .into_iter()
                                .for_each(|item| monkey.catch(item));
                        }
                        _ => (),
                    };
                }
            }

            let mut inspection_counts: Vec<u128> = self
                .monkeys
                .iter()
                .map(|monkey| monkey.inspection_count)
                .collect();

            inspection_counts.sort();
            inspection_counts.reverse();

            inspection_counts[0] * inspection_counts[1]
        }

        pub fn print(&self) {
            for (i, monkey) in self.monkeys.iter().enumerate() {
                println!("Monkey {i}: {monkey}");
            }
        }
    }
}
