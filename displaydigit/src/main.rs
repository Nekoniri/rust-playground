use std::{io, process};

const HORIZONTAL: char = '_';
const VERTICAL: char = '|';
const EMPTY: char = ' ';

fn render_segment(state: bool, on_value: char) -> char {
    if state {
        on_value
    } else {
        EMPTY
    }
}

fn main() {
    loop {
        println!("Enter a number between 0 and 9: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong");

        let digit = match input.trim().parse::<i8>() {
            Ok(0) => [true, true, true, true, true, true, false],
            Ok(1) => [false, false, false, false, true, true, false],
            Ok(2) => [true, true, false, true, true, false, true],
            Ok(3) => [true, true, true, true, false, false, true],
            Ok(4) => [false, true, true, false, false, true, true],
            Ok(5) => [true, false, true, true, false, true, true],
            Ok(6) => [true, false, true, true, true, true, true],
            Ok(7) => [true, true, true, false, false, false, false],
            Ok(8) => [true, true, true, true, true, true, true],
            Ok(9) => [true, true, true, true, false, true, true],
            _ => {
                eprintln!(
                    "Invalid input received, expected number between 0 and 10. Got: {}",
                    input
                );
                process::exit(1);
            }
        };

        println!("{}{}{}", EMPTY, render_segment(digit[0], HORIZONTAL), EMPTY);
        println!(
            "{}{}{}",
            render_segment(digit[5], VERTICAL),
            render_segment(digit[6], HORIZONTAL),
            render_segment(digit[1], VERTICAL),
        );
        println!(
            "{}{}{}\n",
            render_segment(digit[4], VERTICAL),
            render_segment(digit[3], HORIZONTAL),
            render_segment(digit[2], VERTICAL),
        );
    }
}
