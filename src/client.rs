use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};

fn main() {
    println!("Hello from Client!");
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5,5)).unwrap();
}
