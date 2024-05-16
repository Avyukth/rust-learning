
use std::{sync::RwLock, thread};
use once_cell::sync::Lazy;

static  USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(buid_users()));

fn buid_users() -> Vec<String> {
    vec!["Allice".to_string(), "Bob".to_string(), "Carol".to_string()]
}

fn read_line() -> String {
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
        loop{
            println!("Current users in thread.");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a Name to add to the user List:");
        let input = read_line();
        if input == "q" {
            break;
        }else{
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }
    }
}
