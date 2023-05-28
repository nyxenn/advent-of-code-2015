use regex::Regex;

use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let sol1: u32 = part_one();
    let sol2: u32 = part_two();

    (Solution::from(sol1), Solution::from(sol2))
}

struct Operations {
    turn_on: Box<dyn Fn(u32) -> u32>,
    turn_off: Box<dyn Fn(u32) -> u32>,
    toggle: Box<dyn Fn(u32) -> u32>,
}

fn part_one() -> u32 {
    let turn_on = Box::new(|_x: u32| 1);
    let turn_off = Box::new(|_x: u32| 0);
    let toggle = Box::new(|x: u32| if x == 0 { 1 } else { 0 });

    execute_instructions(Operations {
        turn_on,
        turn_off,
        toggle,
    })
}

fn part_two() -> u32 {
    let turn_on = Box::new(|x| x + 1);
    let turn_off = Box::new(|x| if x > 1 { x - 1 } else { 0 });
    let toggle = Box::new(|x| x + 2);

    execute_instructions(Operations {
        turn_on,
        turn_off,
        toggle,
    })
}

fn execute_instructions(operations: Operations) -> u32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"(.*) ([0-9]+,[0-9]+).* ([0-9]+,[0-9]+)").unwrap();
    let input_str = read_input(6, 1);

    for line in input_str.split('\n').filter(|x| !x.is_empty()) {
        let captures = re.captures(line).unwrap();
        let (operation, start, end) = (
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str(),
        );

        let (x1, y1) = split_point(start);
        let (x2, y2) = split_point(end);
        let operation = match operation {
            "turn on" => &operations.turn_on,
            "turn off" => &operations.turn_off,
            _ => &operations.toggle,
        };

        for x in x1..=x2 {
            for y in y1..=y2 {
                grid[x][y] = operation(grid[x][y]);
            }
        }
    }

    grid.iter().map(|col| col.iter().sum::<u32>()).sum()
}

fn split_point(s: &str) -> (usize, usize) {
    let mut split = s.split(',');
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}
