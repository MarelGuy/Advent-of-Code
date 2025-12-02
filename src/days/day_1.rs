// Day 1: Secret Entrance

use std::error::Error;

const DAY_DATA: &str = include_str!("../../inputs/day_1");

struct Dial {
    val: isize,
    part_one_zeros: usize,
    part_two_zeros: isize,
}

impl Dial {
    fn new() -> Self {
        Self {
            val: 50,
            part_one_zeros: 0,
            part_two_zeros: 0,
        }
    }

    fn right(&mut self, amount: isize) {
        self.part_two_zeros += (self.val + amount) / 100;
        self.val = (self.val + amount) % 100;

        if self.val == 0 {
            self.part_one_zeros += 1;
        }
    }

    fn left(&mut self, amount: isize) {
        let dist_to_zero: isize = if self.val == 0 { 100 } else { self.val };

        if amount >= dist_to_zero {
            self.part_two_zeros += 1;

            let remaining: isize = amount - dist_to_zero;

            self.part_two_zeros += remaining / 100;
        }

        self.val = (self.val - amount).rem_euclid(100);

        if self.val == 0 {
            self.part_one_zeros += 1;
        }
    }
}

pub fn secret_entrance() -> Result<(), Box<dyn Error>> {
    let data: Vec<&str> = DAY_DATA.split('\n').collect();

    let mut dial: Dial = Dial::new();

    for op in data {
        let split_op: (&str, &str) = op.split_at(1);

        let op_turns: isize = split_op.1.parse()?;

        match split_op.0 {
            "R" => dial.right(op_turns),
            "L" => dial.left(op_turns),
            _ => unreachable!(),
        }
    }

    println!(
        "The password fpr the first part of the problem is: {:?}",
        dial.part_one_zeros
    );

    println!(
        "The password fpr the second part of the problem is: {:?}",
        dial.part_two_zeros
    );

    Ok(())
}
