use aoc2022_rust::problem;
use self::Direction::*;
use std::slice::Iter;

fn main() {
    let input = problem::load(8);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

struct Grid {
    width: usize,
    height: usize,
    items: Vec<u32>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u32 {
        self.items[x * self.width + y]
    }

    fn is_visible(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let height = self.get(x, y);
        match direction {
            // Remember range bounds are [lower, upper)
            Direction::Up => std::iter::IntoIterator::into_iter(0..y).all(|val| self.get(x, val) < height),
            Direction::Down => std::iter::IntoIterator::into_iter((y + 1)..self.height).all(|val| self.get(x, val) < height),
            Direction::Left => std::iter::IntoIterator::into_iter(0..x).all(|val| self.get(val, y) < height),
            Direction::Right => std::iter::IntoIterator::into_iter((x + 1)..self.width).all(|val| self.get(val, y) < height),
        }
    }

    fn number_visible(&self, x: usize, y: usize, direction: &Direction) -> usize {
        let height = self.get(x, y);
        match direction {
            // Remember range bounds are [lower, upper)
            Direction::Up => std::iter::IntoIterator::into_iter(0..y).rev().enumerate().find(|(_, val)| self.get(x, *val) >= height).map(|(index, _)| index + 1).unwrap_or_else(|| (0..y).len()),
            Direction::Down => std::iter::IntoIterator::into_iter((y + 1)..self.height).enumerate().find(|(_, val)| self.get(x, *val) >= height).map(|(index, _)| index + 1).unwrap_or_else(|| ((y + 1)..self.height).len()),
            Direction::Left => std::iter::IntoIterator::into_iter(0..x).rev().enumerate().find(|(_, val)| self.get(*val, y) >= height).map(|(index, _)| index + 1).unwrap_or_else(|| (0..x).len()),
            Direction::Right => std::iter::IntoIterator::into_iter((x + 1)..self.width).enumerate().find(|(_, val)| self.get(*val, y) >= height).map(|(index, _)| index + 1).unwrap_or_else(|| ((x + 1)..self.width).len()),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }
}

fn solve1(input: &str) -> usize {
    let mut rows = input.lines();
    let row = rows.next();
    let width = row.unwrap().len();
    let mut height: usize = 1;
    let mut trees: Vec<u32> = row.unwrap().chars().map(|x| x.to_digit(10).expect("expected a digit")).collect();
    while let Some(row) = rows.next() {
        trees.append(&mut row.chars().map(|x| x.to_digit(10).expect("expected a digit")).collect());
        height += 1;
    }

    let grid: Grid = Grid { width, height, items: trees };
    count_visible(grid)
}

fn count_visible(grid: Grid) -> usize {
    // Edges are always visible
    let mut visible_trees = grid.width * 2 + (grid.height - 2) * 2;
    for x in 1..(grid.width as usize - 1) {
        for y in 1..(grid.height as usize - 1) {
            let mut visible;
            for direction in Direction::iterator() {
                visible = grid.is_visible(x, y, direction);
                if visible {
                    visible_trees += 1;
                    break;
                }
            }
        }
    }
    visible_trees
}

fn solve2(input: &str) -> usize {
    let mut rows = input.lines();
    let row = rows.next();
    let width = row.unwrap().len();
    let mut height: usize = 1;
    let mut trees: Vec<u32> = row.unwrap().chars().map(|x| x.to_digit(10).expect("expected a digit")).collect();
    while let Some(row) = rows.next() {
        trees.append(&mut row.chars().map(|x| x.to_digit(10).expect("expected a digit")).collect());
        height += 1;
    }

    let grid: Grid = Grid { width, height, items: trees };
    get_highest_scenic_score(grid)
}


fn get_highest_scenic_score(grid: Grid) -> usize {
    let mut max_score: usize = 0;
    for x in 1..(grid.width as usize - 1) {
        for y in 1..(grid.height as usize - 1) {
            let mut score: usize = 1;
            for direction in Direction::iterator() {
                score *= grid.number_visible(x, y, direction);
            }
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(8);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 21);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 8);
    }
}