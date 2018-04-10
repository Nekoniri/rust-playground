pub fn strings() {
    let ykcin = "Ykcin";
    let name = ykcin.to_string();
    let name2 = String::from(ykcin);

    let mut hello_world = String::new();
    hello_world.push_str("Hello, ");
    hello_world.push_str("world!");

    let name_chars = name.chars();
    let name_bytes = name.bytes();

    println!("to_string {}", name);
    println!("String::from {}", name2);
    println!("Bytes of name {:?}", name_bytes);
    println!("Chars of name {:?}", name_chars);
    println!("Mutable string {}", hello_world);
}
