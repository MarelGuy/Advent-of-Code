mod days;

use std::{
    error::Error,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Hello! This is Marel's implementation of each day of the advent of code (https://adventofcode.com)"
    );
    println!(
        "Please, input a number ranging from 1 to 12, each number represents each day of the challenge:"
    );
    println!("1: --- Day 1: Secret Entrance ---     (https://adventofcode.com/2025/day/1)");
    println!("2: --- Day 2: Gift Shop ---           (https://adventofcode.com/2025/day/2)");
    println!("3: --- Day 3: Lobby ---               (https://adventofcode.com/2025/day/3)");
    println!("4: --- Day 4: Printing Department --- (https://adventofcode.com/2025/day/4)");
    println!("5: --- Day 5: Cafeteria ---           (https://adventofcode.com/2025/day/5)");
    println!("6: --- Day 6: Trash Compactor ---     (https://adventofcode.com/2025/day/6)");
    println!("7: --- Day 7: Laboratories ---        (https://adventofcode.com/2025/day/7)");
    println!("8: --- Day 8: Playground ---          (https://adventofcode.com/2025/day/8)");

    print!("> ");

    io::stdout().flush()?;

    let input: u8 = read_input()?;

    match input {
        1 => days::secret_entrance()?,
        2 => days::gift_shop()?,
        3 => days::lobby()?,
        4 => days::printing_department(),
        5 => days::cafeteria()?,
        6 => days::trash_compactor()?,
        7 => days::laboratories(),
        8 => days::playground(),
        _ => println!("This day is not yet implemented, please try again"),
    }

    Ok(())
}

fn read_input() -> Result<u8, Box<dyn Error>> {
    loop {
        let mut input: String = String::new();

        io::stdin().read_line(&mut input)?;

        let number: u8 = input.trim().parse()?;

        if !(1..=12).contains(&number) {
            println!("You need to input a number ranging from 1 to 12");
        }

        return Ok(number);
    }
}
