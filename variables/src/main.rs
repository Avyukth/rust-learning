// fn greet(s: String)  {
//     println!("Hello  {s}");
// }

// fn greet_borrow(s: &String)  {
//     println!("Hello  {s}");
// }

fn greet_borrow_mut(s: &mut String) {
    *s = format!("hello borrow {s}");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

// fn main() {
//     let mut name: String = "Hello world".to_string();
//     greet_borrow_mut(&mut name);
//     // greet(name);
//     println!("{name}");
// }

fn main() {
    let input: String = read_line();
    println!("you Typed: [{input}] ")
}
