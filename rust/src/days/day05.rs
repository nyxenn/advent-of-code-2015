use crate::{utils::read_input::read_input, Solution, SolutionPair};
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_input(5, 1);
    let sol1: u32 = count_nice_words(&input, part_one);
    let sol2: u32 = count_nice_words(&input, part_two);

    (Solution::from(sol1), Solution::from(sol2))
}

fn count_nice_words(input: &str, is_word_nice: fn(&str) -> bool) -> u32 {
    let mut nice = 0;

    for word in input.split('\n') {
        if word.is_empty() {
            continue;
        }

        if is_word_nice(word) {
            nice += 1
        }
    }

    nice
}

fn part_one(word: &str) -> bool {
    let mut vowel_count = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut previous_letter: char = char::from(0);
    let mut identical_successive_letters = 0;

    let mut contains_forbidden_string = false;
    let forbidden_strings: Vec<String> = vec!["ab", "cd", "pq", "xy"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    for c in word.chars() {
        if vowels.contains(&c) {
            vowel_count += 1;
        }

        if c == previous_letter {
            identical_successive_letters += 1;
        }

        if forbidden_strings.contains(&format!("{}{}", previous_letter, c)) {
            contains_forbidden_string = true
        }

        previous_letter = c;
    }

    vowel_count >= 3 && identical_successive_letters >= 1 && !contains_forbidden_string
}

fn part_two(word: &str) -> bool {
    let mut pair_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut previous_letter: Option<char> = None;

    let mut two_pairs = false;
    let mut mirrored = false;

    for (i, c) in word.chars().enumerate() {
        // Skip first iteration
        if previous_letter.is_none() {
            previous_letter = Some(c);
            continue;
        }

        let pair = format!("{}{}", previous_letter.unwrap(), c);

        if i > 1 && c == word.chars().nth(i - 1).unwrap() {
            mirrored = true;
        }

        match pair_map.get_mut(&pair) {
            Some(indexes) => {
                // Filter out 3 consecutive identical letters

                // AoC considers this to be incorrect
                // if *indexes.iter().next().unwrap() < i - 1 {
                //     println!("Word: {}", word);
                //     two_pairs = true;
                // }

                // Even though this one (correct) does not consider "rxexcbwhiywwwwnu" to contain
                // two pairs (4 consecutive w's)
                if *indexes.iter().last().unwrap() < i - 1 {
                    two_pairs = true;
                }

                indexes.push(i)
            }
            None => {
                pair_map.insert(pair, vec![i]);
            }
        }

        previous_letter = Some(c);
    }

    two_pairs && mirrored
}
