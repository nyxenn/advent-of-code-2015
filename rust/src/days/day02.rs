use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let boxes = parse_boxes();
    let sol1: u32 = solve_part_one(&boxes);
    let sol2: u32 = solve_part_two(&boxes);

    (Solution::from(sol1), Solution::from(sol2))
}

struct PresentBox {
    width: u32,
    height: u32,
    length: u32,
}

impl PresentBox {
    fn new(width: u32, height: u32, length: u32) -> PresentBox {
        PresentBox {
            width,
            height,
            length,
        }
    }

    fn area(&self) -> u32 {
        (2 * self.width * self.height)
            + (2 * self.length * self.height)
            + (2 * self.width * self.length)
    }

    fn volume(&self) -> u32 {
        self.width * self.height * self.length
    }

    fn smallest_side_dimensions(&self) -> (u32, u32) {
        let mut dimensions = vec![self.width, self.length, self.height];
        dimensions.sort();
        (dimensions[0], dimensions[1])
    }

    fn smallest_side_area(&self) -> u32 {
        let (a, b) = self.smallest_side_dimensions();
        a * b
    }

    fn ribbon_length(&self) -> u32 {
        let (a, b) = self.smallest_side_dimensions();
        2 * (a + b)
    }
}

fn parse_boxes() -> Vec<PresentBox> {
    let input_str = read_input(2, 1);
    let mut boxes: Vec<PresentBox> = Vec::new();

    for line in input_str.split('\n') {
        if line.is_empty() {
            continue;
        }

        let dimensions: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        boxes.push(PresentBox::new(dimensions[0], dimensions[1], dimensions[2]));
    }

    boxes
}

fn solve_part_one(boxes: &Vec<PresentBox>) -> u32 {
    boxes
        .iter()
        .fold(0, |acc, b| acc + b.area() + b.smallest_side_area())
}

fn solve_part_two(boxes: &Vec<PresentBox>) -> u32 {
    boxes
        .iter()
        .fold(0, |acc, b| acc + b.volume() + b.ribbon_length())
}
