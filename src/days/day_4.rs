// Day 4: Printing Department

const DAY_DATA: &str = include_str!("../../inputs/day_4");

fn search_row(row_opt: Option<&Vec<char>>, i: usize) -> usize {
    let row: &Vec<char> = match row_opt {
        Some(r) => r,
        None => return 0,
    };

    let start: usize = i.saturating_sub(1);

    let end: usize = (i + 2).min(row.len());

    row[start..end].iter().filter(|&&c| c == '@').count()
}

pub fn printing_department() -> () {
    let mut data: Vec<Vec<char>> = DAY_DATA
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut accessible_rolls: usize = 0;
    let mut deleted_rolls: usize = 0;

    let mut is_first_try: bool = false;
    let mut cells_to_delete: Vec<(usize, usize)> = Vec::new();

    loop {
        for (i, current_row) in data.iter().enumerate() {
            let top_row_opt: Option<&Vec<char>> = i.checked_sub(1).and_then(|idx| data.get(idx));
            let bot_row_opt: Option<&Vec<char>> = i.checked_add(1).and_then(|idx| data.get(idx));

            for (j, roll) in current_row.iter().enumerate() {
                if *roll == '@' {
                    const MAX_ADJ_ROLLS: usize = 4;

                    let mut found_adj_rolls: usize = 0;

                    found_adj_rolls += search_row(top_row_opt, j);
                    found_adj_rolls += search_row(Some(current_row), j);
                    found_adj_rolls += search_row(bot_row_opt, j);

                    if found_adj_rolls <= MAX_ADJ_ROLLS {
                        accessible_rolls += 1;

                        cells_to_delete.push((i, j));
                    }
                }
            }
        }

        if cells_to_delete.is_empty() {
            println!(
                "The number of removed rolls for the second part of the problem is: {deleted_rolls}"
            );

            break;
        }

        for (i, j) in cells_to_delete.drain(..) {
            deleted_rolls += 1;

            data[i][j] = '.';
        }

        if !is_first_try {
            println!(
                "The number of accessible rolls for the first part of the problem is: {accessible_rolls}"
            );

            is_first_try = true;
        }
    }
}
