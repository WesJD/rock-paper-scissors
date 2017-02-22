use std::io;
use std::io::{Write};

pub fn print_flush(out: &str) {
    print!("{}", out);
    io::stdout().flush().unwrap();
}

pub fn format_count(base: String, amount: usize) -> String {
    if amount != (1 as usize) {
        base + "s"
    } else {
        base
    }
}