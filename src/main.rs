mod days;

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Hello! This is Marel's implementation of each day of the advent of code (https://adventofcode.com)"
    );
    println!(
        "Please, input a number ranging from 1 to 12, each number represents each day of the challenge:"
    );

    let input: u8 = read_input()?;

    match input {
        1 => days::secret_entrance()?,
        2 => days::gift_shop()?,
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
