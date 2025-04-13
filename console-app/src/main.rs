fn main() {
    println!("Hello, world!");
    println!("Press Enter to close the window...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
}
