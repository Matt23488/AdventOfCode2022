use std::{cell::RefCell, rc::Rc};

fn main() {
    let tree = communication_device::DirectoryTree::build_from_input();

    let part_one_answer = part_one(tree.clone());
    dbg!(part_one_answer);

    let part_two_answer = part_two(tree.clone());
    dbg!(part_two_answer);

    dbg!(part_two_answer + tree.borrow().total_size());
}

fn part_one(tree: Rc<RefCell<communication_device::DirectoryTree>>) -> u32 {
    tree.borrow()
        .get_dir_sizes_where(&|size| size <= 100000)
        .iter()
        .sum()
}

fn part_two(tree: Rc<RefCell<communication_device::DirectoryTree>>) -> u32 {
    let free_space_needed = 30_000_000 - (70_000_000 - tree.borrow().total_size());
    let mut candidates_for_deletion = tree
        .borrow()
        .get_dir_sizes_where(&|size| size >= free_space_needed);
    candidates_for_deletion.sort();

    *candidates_for_deletion.first().unwrap_or(&0)
}

mod communication_device {
    use std::{cell::RefCell, fs, rc::Rc};

    #[derive(Debug)]
    enum Command<'cmd> {
        ChangeDirectory(&'cmd str),
        List,
    }

    impl Command<'_> {
        fn build_from_line<'a>(line: &'a str) -> Option<Command<'a>> {
            let mut iter = line.split(' ');
            match iter.next() {
                Some("$") => {}
                _ => return None,
            };

            match iter.next() {
                Some("ls") => Some(Command::<'a>::List),
                Some("cd") => match iter.next() {
                    Some(dir) => Some(Command::<'a>::ChangeDirectory(dir)),
                    None => None,
                },
                _ => None,
            }
        }
    }

    #[derive(Debug)]
    struct File {
        name: String,
        size: u32,
    }

    impl File {
        fn build_from_line(line: &str) -> Option<File> {
            let mut iter = line.split(' ');

            let size = match iter.next() {
                Some(size) => size,
                None => return None,
            };

            let size = match size.parse::<u32>() {
                Ok(size) => size,
                Err(_) => return None,
            };

            let name = match iter.next() {
                Some(name) => String::from(name),
                None => return None,
            };

            Some(File { name, size })
        }
    }

    #[derive(Debug)]
    pub struct DirectoryTree {
        _name: String,
        parent: Option<Rc<RefCell<DirectoryTree>>>,
        directories: Vec<Rc<RefCell<DirectoryTree>>>,
        files: Vec<File>,
    }

    impl DirectoryTree {
        pub fn build_from_input() -> Rc<RefCell<DirectoryTree>> {
            let input = fs::read_to_string("input.txt").unwrap();
            let root = Rc::new(RefCell::new(DirectoryTree {
                _name: String::from("/"),
                parent: None,
                directories: Vec::new(),
                files: Vec::new(),
            }));
            let mut current = root.clone();
            let mut spaces = 0;

            println!("/");
            for line in input.lines() {
                if let Some(command) = Command::build_from_line(line) {
                    match command {
                        Command::ChangeDirectory("/") => {}
                        Command::ChangeDirectory("..") => match &current.clone().borrow().parent {
                            Some(rc) => {
                                current = rc.clone();
                                spaces -= 2;
                            }
                            None => panic!("No parent directory!"),
                        },
                        Command::ChangeDirectory(dir) => {
                            let new = Rc::new(RefCell::new(DirectoryTree {
                                _name: String::from(dir),
                                parent: Some(current.clone()),
                                directories: Vec::new(),
                                files: Vec::new(),
                            }));

                            current.borrow_mut().directories.push(new.clone());

                            current = new;

                            spaces += 2;
                            print!("{}", (0..spaces).map(|_| ' ').collect::<String>());
                            println!("{dir} (dir)");
                        }
                        _ => {}
                    };

                    continue;
                }

                if let Some(file) = File::build_from_line(line) {
                    print!("{}", (0..spaces).map(|_| ' ').collect::<String>());
                    println!("{} (file, {})", file.name, file.size);
                    current.borrow_mut().files.push(file);
                }
            }

            root
        }

        pub fn total_size(&self) -> u32 {
            self.files.iter().map(|f| f.size).sum::<u32>()
                + self
                    .directories
                    .iter()
                    .map(|d| d.borrow().total_size())
                    .sum::<u32>()
        }

        pub fn get_dir_sizes_where<F>(&self, predicate: &F) -> Vec<u32>
        where
            F: Fn(u32) -> bool,
        {
            let mut sizes = Vec::new();

            let dir_size = self.total_size();
            if predicate(dir_size) {
                sizes.push(dir_size);
            }

            for dir in &self.directories {
                sizes.append(&mut dir.borrow().get_dir_sizes_where(predicate));
            }

            sizes
        }
    }
}
