use clap::Parser;
use std::path::PathBuf;

// --- structs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// --- functions
fn greeting(name: String ) -> String {
    format!("Hello, {}!", name)
}

// --- core app
fn main() {
    println!("Hello, world!");
}

// --- TDD
#[test]
fn greeting_test() {
    let want = String::from("Hello, Rusty! ");
    let name: String = String::from("Rusty");
    let result = greeting(name);
    assert_eq!(want, result);
}

// ----- end of file -----