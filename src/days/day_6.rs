// Day 6: Trash Compactor

use std::{cmp, error::Error};

const DAY_DATA: &str = include_str!("../../inputs/day_6");

pub fn trash_compactor() -> Result<(), Box<dyn Error>> {
    let data: Vec<Vec<&str>> = DAY_DATA
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut col_widths: Vec<usize> = Vec::new();

    if let Some(first) = data.first() {
        for i in 0..first.len() {
            let max: usize = data
                .iter()
                .map(|row: &Vec<&str>| row.get(i).map_or(0, |s: &&str| s.len()))
                .max()
                .unwrap_or(0);

            col_widths.push(max);
        }
    }

    let mut lines: Vec<Vec<String>> = Vec::new();

    for line in DAY_DATA.lines() {
        let mut row_items: Vec<String> = Vec::new();
        let mut cursor: usize = 0;

        for &width in &col_widths {
            let end: usize = cmp::min(cursor + width, line.len());

            let chunk: &str = if cursor < line.len() {
                &line[cursor..end]
            } else {
                ""
            };

            row_items.push(format!("{chunk:<width$}"));

            cursor += width + 1;
        }
        lines.push(row_items);
    }

    let mut rtl_final_result: usize = 0;
    let mut ltr_final_result: usize = 0;

    for i in (0..lines[0].len()).rev() {
        let (mut numbers, mut operator): (Vec<&str>, String) = (Vec::new(), String::new());

        for j in 0..lines.len() {
            let current: &str = &lines[j][i];

            if !current.contains('+') && !current.contains('*') {
                numbers.push(current);
            } else {
                operator = current.split_whitespace().collect::<String>();
            }
        }

        let longest_string_len: usize = numbers
            .iter()
            .max_by_key(|s: &&&str| s.len())
            .expect("Array should not be empty")
            .len();

        let num_chars: Vec<Vec<char>> = numbers
            .iter()
            .map(|str: &&str| str.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut new_nums: Vec<String> = Vec::new();

        for j in (0..longest_string_len).rev() {
            let num: String = num_chars
                .iter()
                .filter_map(|chars: &Vec<char>| chars.get(j))
                .filter(|c: &&char| !c.is_whitespace())
                .collect();

            new_nums.push(num);
        }

        let (mut rtl_op_result, mut ltr_op_result): (usize, usize) = match operator.as_str() {
            "*" => (1, 1),
            "+" => (0, 0),
            _ => unreachable!(),
        };

        for i in 0..numbers.len() {
            let rtl: usize = numbers[i]
                .chars()
                .filter(|c: &char| !c.is_whitespace())
                .collect::<String>()
                .parse::<usize>()?;

            let ltr: Option<usize> = new_nums.get(i).map(|s| s.parse::<usize>()).transpose()?;

            match operator.as_str() {
                "*" => {
                    rtl_op_result *= rtl;

                    if let Some(l) = ltr {
                        ltr_op_result *= l;
                    }
                }
                "+" => {
                    rtl_op_result += rtl;

                    if let Some(l) = ltr {
                        ltr_op_result += l;
                    }
                }
                _ => unreachable!(),
            }
        }

        rtl_final_result += rtl_op_result;
        ltr_final_result += ltr_op_result;
    }

    println!(
        "The final result of the operations in the first part of the problem is: {rtl_final_result}",
    );

    println!(
        "The final result of the operations in the second part of the problem is: {ltr_final_result}",
    );

    Ok(())
}
