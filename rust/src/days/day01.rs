use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let sol1: i32 = solve_part_one();
    let sol2: u32 = solve_part_two();

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part_one() -> i32 {
    let input_str = read_input(1, 1);
    input_str.chars().fold(0, |acc: i32, c: char| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn solve_part_two() -> u32 {
    let input_str = read_input(1, 1);
    let (mut steps, mut up, mut down) = (0, 0, 0);
    for c in input_str.chars() {
        if up < down {
            break;
        }

        steps += 1;
        match c {
            '(' => up += 1,
            ')' => down += 1,
            _ => (),
        };
    }

    steps
}
