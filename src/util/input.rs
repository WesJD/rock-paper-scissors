use std::io;

pub fn read_line() -> String {
    let mut read = String::new();
    io::stdin().read_line(&mut read).expect("Unable to read line.");
    read.pop(); //remove the \n
    read
}