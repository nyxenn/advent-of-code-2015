use md5::Digest;

use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_str = read_input(4, 1).trim_end_matches('\n').to_string();
    let sol1: u32 = find_num_for_leading_zeros(&input_str, 5);
    let sol2: u32 = find_num_for_leading_zeros(&input_str, 6);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_num_for_leading_zeros(input: &str, zeros: usize) -> u32 {
    let mut num = 0;

    loop {
        let hash = md5::compute(format!("{}{}", input, num));
        match matches_start_pattern(hash, zeros) {
            true => break,
            false => num += 1,
        }
    }

    num
}

fn matches_start_pattern(hash: Digest, zeros: usize) -> bool {
    let bytes_to_check = (zeros / 2) + (zeros % 2);
    let last_byte_lt_16 = zeros % 2 == 1;
    let mut success = true;

    for i in 0..bytes_to_check {
        if i + 1 == bytes_to_check && last_byte_lt_16 {
            success = hash.0[i] < 16
        } else {
            success = hash.0[i] == 0
        }

        if !success {
            break;
        }
    }

    success
}
