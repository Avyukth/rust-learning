use std::{mem::size_of, thread};

fn my_thread() {
    println!("Hello, From Thread named {}!",
        thread::current().name().unwrap()
    );
}



fn main() {
    thread::Builder::new()
     .name("Name Thread".to_string())
     .stack_size(size_of::<usize>() * 4)
     .spawn(my_thread)
      .unwrap();
}    
