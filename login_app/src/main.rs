use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command]
struct Args{
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    println!("Hello, world!");
}
