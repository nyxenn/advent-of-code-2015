use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_str = read_input(3, 1);
    let sol1: u32 = solve_part_one(&input_str);
    let sol2: u32 = solve_part_two(&input_str);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn solve_part_one(directions: &str) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut visited_houses: Vec<Point> = vec![Point { x, y }];

    for direction in directions.chars() {
        match direction {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        };

        let current_point = Point { x, y };
        if !visited_houses.contains(&current_point) {
            visited_houses.push(current_point);
        }
    }

    visited_houses.len() as u32
}

fn solve_part_two(directions: &str) -> u32 {
    let mut loc_santa = Point { x: 0, y: 0 };
    let mut loc_robo = Point { x: 0, y: 0 };
    let mut visited_houses: Vec<Point> = vec![Point { x: 0, y: 0 }];

    for (i, direction) in directions.chars().enumerate() {
        let mut loc = match i % 2 == 0 {
            true => &mut loc_santa,
            false => &mut loc_robo,
        };

        match direction {
            '^' => loc.y += 1,
            'v' => loc.y -= 1,
            '>' => loc.x += 1,
            '<' => loc.x -= 1,
            _ => (),
        };

        if !visited_houses.contains(loc) {
            visited_houses.push(loc.clone());
        }
    }

    visited_houses.len() as u32
}
