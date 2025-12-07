// Day 7: Laboratories

use std::collections::HashMap;

const DAY_DATA: &str = include_str!("../../inputs/day_7");

pub fn laboratories() {
    let data: Vec<Vec<char>> = DAY_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start_pos: (usize, usize) = (0, 0);

    for (r, row) in data.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch: &char| ch == 'S') {
            start_pos = (r, c);

            break;
        }
    }

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let total_timelines: usize = count_timelines(start_pos.0, start_pos.1, &data, &mut cache);

    let total_splitters: usize = data.iter().flatten().filter(|&&c: &&char| c == '^').count() - 1;

    println!("The times the beam was split in the first part of the problem is: {total_splitters}");
    println!("The timelines in the second part of the problem are: {total_timelines}");
}

fn count_timelines(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&count) = cache.get(&(row, col)) {
        return count;
    }

    let height: usize = grid.len();
    let width: usize = grid[0].len();

    let mut cur_row: usize = row;

    while cur_row < height {
        let tile: char = grid[cur_row][col];

        if tile == '^' {
            let mut paths: usize = 0;

            if col > 0 {
                paths += count_timelines(cur_row, col - 1, grid, cache);
            }

            if col + 1 < width {
                paths += count_timelines(cur_row, col + 1, grid, cache);
            }

            cache.insert((row, col), paths);

            return paths;
        }

        cur_row += 1;
    }

    cache.insert((row, col), 1);

    1
}
