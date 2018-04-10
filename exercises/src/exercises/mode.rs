use std::collections::HashMap;

pub fn mode(numbers: &[i32]) -> i32 {
    *numbers
        .iter()
        .fold(HashMap::new(), |mut acc, number| {
            *acc.entry(number).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
        .expect("Could not compute the mode of empty vector")
}
