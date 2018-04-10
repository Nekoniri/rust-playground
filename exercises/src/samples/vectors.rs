pub fn vectors() {
    let numbers = vec![1, 2, 3, 4];
    let third_number: Option<&i32> = numbers.get(2);

    println!("numbers = {:#?}", numbers);
    println!("third_number = {:?}", third_number);

    for number in &numbers {
        println!("number: {}", number);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
