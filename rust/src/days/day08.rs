use std::str::FromStr;

use regex::{Match, Regex};

use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_str = read_input(8, 1);
    let sol1: u16 = part_one(&input_str);
    let sol2: u16 = part_two(&input_str);

    (Solution::from(sol1), Solution::from(sol2))
}

struct StrLen {
    raw: u16,
    in_mem: u16,
}

fn part_one(input_str: &str) -> u16 {
    let mut raw_len: u16 = 0;
    let mut in_mem_len: u16 = 0;
    let ascii_re =
        Regex::new(r#"\\"|\\x[0-9a-f]{2}|\\\\"#).expect("Could not compile ASCII code regex");

    for line in input_str.split('\n').filter(|x| !x.is_empty()) {
        let str_len = count_chars(&ascii_re, line);
        raw_len += str_len.raw;
        in_mem_len += str_len.in_mem;
    }

    raw_len - in_mem_len
}

fn part_two(input_str: &str) -> u16 {
    let mut extra_chars_len = 0;

    for line in input_str.split('\n').filter(|x| !x.is_empty()) {
        let chars_need_escaping = (line.split(['"', '\\']).count() - 1) as u16;
        extra_chars_len += chars_need_escaping + 2;
    }

    extra_chars_len
}

fn count_chars(escaped_chars_re: &Regex, s: &str) -> StrLen {
    let matches: Vec<Match> = escaped_chars_re.find_iter(s).collect();
    let excess_char_len: u16 = matches
        .iter()
        // Calculate length diff between escaped character and value it represents
        // e.g. "\x27" (4 chars) representing an apostrophe "'" (1 char)
        .map(|x| x.end() as u16 - x.start() as u16 - 1)
        .sum();

    StrLen {
        raw: s.len() as u16,
        in_mem: s.len() as u16 - 2 - excess_char_len,
    }
}
