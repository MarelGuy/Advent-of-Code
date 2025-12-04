#![allow(clippy::similar_names)]
// Day 3: Lobby

use std::error::Error;

const DAY_DATA: &str = include_str!("../../inputs/day_3");

fn find_max_sequence(input: &str, k: usize) -> Result<usize, Box<dyn Error>> {
    let chars: Vec<char> = input.chars().collect();
    let n: usize = chars.len();

    let mut stack: Vec<char> = Vec::with_capacity(k);

    for (i, &digit) in chars.iter().enumerate() {
        let remaining_in_input: usize = n - 1 - i;

        while let Some(&top) = stack.last() {
            if digit > top && (stack.len() + remaining_in_input >= k) {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(digit);
        }
    }

    let result_string: String = stack.iter().collect();

    Ok(result_string.parse::<usize>()?)
}

pub fn lobby() -> Result<(), Box<dyn Error>> {
    let data: Vec<&str> = DAY_DATA.split('\n').collect();

    let mut total_joltage_2: usize = 0;
    let mut total_joltage_12: usize = 0;

    for b_pack in &data {
        total_joltage_2 += find_max_sequence(b_pack, 2)?;

        total_joltage_12 += find_max_sequence(b_pack, 12)?;
    }

    println!("The joltage for the first part of the problem is: {total_joltage_2}");
    println!("The joltage for the second part of the problem is: {total_joltage_12}");

    Ok(())
}
