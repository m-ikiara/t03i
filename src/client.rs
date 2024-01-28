use std::io::stdout;
use crossterm::{QueueableCommand, cursor};
use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello from Client!");
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5,5)).unwrap();
    thread::sleep(Duration::from_secs(5));
}
