use std::collections::HashMap;

pub fn hmaps() {
    let mut scores_map = HashMap::new();
    scores_map.insert(String::from("Blue"), 2);
    scores_map.insert(String::from("Red"), 2);

    println!("scores_map {:#?}", scores_map);

    let teams = vec![String::from("Red"), String::from("Blue")];
    let initial_scores = vec![2, 2];

    println!("teams {:#?}", teams);
    println!("initial_scores {:#?}", initial_scores);

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores).collect();

    scores_map.entry(String::from("Yellow")).or_insert(50);
    scores_map.entry(String::from("Blue")).or_insert(50);

    println!("scores {:#?}", scores);

    for (key, value) in &scores_map {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
