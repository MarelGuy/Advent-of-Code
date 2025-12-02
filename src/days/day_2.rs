// Day 2: Gift Shop

use std::{error::Error, str::from_utf8};

const DAY_DATA: &str = include_str!("../../inputs/day_2");

pub fn gift_shop() -> Result<(), Box<dyn Error>> {
    let data: Vec<&str> = DAY_DATA.split(',').collect();

    let mut result_first_part: usize = 0;
    let mut result_second_part: usize = 0;

    for range in data {
        let range: Vec<&str> = range.split('-').collect();

        let min: usize = range[0].parse()?;
        let max: usize = range[1].parse()?;

        for i in min..=max {
            let i_string: String = i.to_string();
            let i_string_len: usize = i_string.len();

            let cap: usize = i_string_len / 2;

            let split_i_string: (&str, &str) = i_string.split_at(cap);

            if split_i_string.0 == split_i_string.1 {
                result_first_part += i;
            }

            for j in 1..=cap {
                let chunks: Vec<&str> = i_string
                    .as_bytes()
                    .chunks(j)
                    .map(|chunk| from_utf8(chunk).expect("chunk should be utf8"))
                    .collect();

                let first_chunk: &str = chunks[0];

                if chunks.iter().all(|&chunk| chunk == first_chunk) {
                    result_second_part += i;

                    break;
                }
            }
        }
    }

    println!("The result for the first part of the problem is: {result_first_part:?}");
    println!("The result for the second part of the problem is: {result_second_part:?}");

    Ok(())
}
