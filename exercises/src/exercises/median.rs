pub fn median(numbers: &Vec<i32>) -> i32 {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    sorted_numbers[sorted_numbers.len() / 2]
}
