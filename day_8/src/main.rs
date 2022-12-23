use std::fs;

fn main() {
    let grid = Grid::build_from_input();

    let part_one_answer = grid.count_visible_trees();
    dbg!(part_one_answer);

    let part_two_answer = grid.calculate_highest_tree_scenic_score();
    dbg!(part_two_answer);
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<usize>>,
}

impl Grid {
    fn build_from_input() -> Grid {
        Grid {
            rows: fs::read_to_string("input.txt")
                .unwrap()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        }
    }

    fn tree_scenic_score(&self, x: usize, y: usize) -> u32 {
        let height = self.tree_height(x, y);
        let num_rows = self.rows.len();
        let num_cols = self.rows[0].len();

        let mut up = 0;
        for y2 in (0..y).rev() {
            up += 1;
            if self.tree_height(x, y2) >= height {
                break;
            }
        }

        let mut down = 0;
        for y2 in (y + 1)..num_rows {
            down += 1;
            if self.tree_height(x, y2) >= height {
                break;
            }
        }

        let mut left = 0;
        for x2 in (0..x).rev() {
            left += 1;
            if self.tree_height(x2, y) >= height {
                break;
            }
        }

        let mut right = 0;
        for x2 in (x + 1)..num_cols {
            right += 1;
            if self.tree_height(x2, y) >= height {
                break;
            }
        }

        up * down * left * right
    }

    fn tree_height(&self, x: usize, y: usize) -> usize {
        self.rows[y][x]
    }

    fn tree_is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.tree_height(x, y);

        let mut visible = true;
        for x2 in 0..x {
            if self.tree_height(x2, y) >= height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }
        visible = true;

        for x2 in (x + 1)..(self.rows[0].len()) {
            if self.tree_height(x2, y) >= height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }
        visible = true;

        for y2 in 0..y {
            if self.tree_height(x, y2) >= height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }
        visible = true;

        for y2 in (y + 1)..(self.rows.len()) {
            if self.tree_height(x, y2) >= height {
                visible = false;
                break;
            }
        }

        visible
    }

    fn count_visible_trees(&self) -> usize {
        let mut count = 0;
        let num_rows = self.rows.len();
        let num_cols = self.rows[0].len();

        for y in 1..(num_rows - 1) {
            for x in 1..(num_cols - 1) {
                if self.tree_is_visible(x, y) {
                    count += 1;
                }
            }
        }

        count + 2 * (num_rows - 1) + 2 * (num_cols - 1)
    }

    fn calculate_highest_tree_scenic_score(&self) -> u32 {
        let mut scores: Vec<u32> = self
            .rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, _)| self.tree_scenic_score(x, y))
            })
            .collect();

        scores.sort();
        *scores.last().unwrap()
    }
}
