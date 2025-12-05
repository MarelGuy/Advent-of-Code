// Day 5: Cafeteria

use std::{cmp, collections::HashSet, error::Error, num::ParseIntError, ops::RangeInclusive};

const DAY_DATA: &str = include_str!("../../inputs/day_5");

pub fn cafeteria() -> Result<(), Box<dyn Error>> {
    let data: Vec<&str> = DAY_DATA.split('\n').collect();

    let mut ids = data.split(|i| *i == "");

    let id_groups: Vec<&str> = ids
        .next()
        .ok_or_else(|| "Input was empty or did not contain the first group".to_string())?
        .to_vec();

    let ids: Vec<&str> = ids
        .next()
        .ok_or_else(|| "Input was empty or did not contain the first group".to_string())?
        .to_vec();

    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();

    for id_group in id_groups {
        let range_tuple: Vec<usize> = id_group
            .split('-')
            .map(|num: &str| num.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        ranges.push(range_tuple[0]..=range_tuple[1]);
    }

    ranges.sort_unstable_by_key(|r: &RangeInclusive<usize>| *r.start());

    let mut fresh_ingredients: HashSet<&str> = HashSet::new();
    let mut total_fresh_ingredients: usize = 0;

    let mut start: usize = 0;
    let mut end: usize = 0;

    for range in ranges {
        for id in &ids {
            if range.contains(&id.parse::<usize>()?) {
                fresh_ingredients.insert(id);
            }
        }

        let next_start: usize = *range.start();
        let next_end: usize = *range.end();

        if next_start <= end.saturating_add(1) {
            end = cmp::max(end, next_end);
        } else {
            total_fresh_ingredients += (end - start) + 1;

            start = next_start;
            end = next_end;
        }
    }

    total_fresh_ingredients += end - start;

    println!(
        "The number of fresh ingredients for the first part of the problem is: {}",
        fresh_ingredients.len()
    );

    println!(
        "The number of total fresh ingredients for the first part of the problem is: {total_fresh_ingredients}",
    );

    Ok(())
}
