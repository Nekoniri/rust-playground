mod samples;
mod exercises;

use exercises::*;
use samples::*;

fn main() {
    // vectors::vectors();
    // strings::strings();
    // hmaps::hmaps();
    // error_handling::open_file();
    // generics::generics();
    // traits::traits();
    // lifetimes::lifetimes();

    let numbers = vec![1, 4, 5, 2, 2, 2, 3];
    println!("Numbers: {:?}", numbers);

    println!("Mean: {}", mean::mean(&numbers));
    println!("Median: {}", median::median(&numbers));
    println!("Mode: {}", mode::mode(&numbers));
}
