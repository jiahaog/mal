use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();

    print!("user> ");
    io::stdout().flush().unwrap();

    for line in stdin.lock().lines() {
        print!("{}\n", line.unwrap());

        print!("user> ");
        io::stdout().flush().unwrap();
    }
}
