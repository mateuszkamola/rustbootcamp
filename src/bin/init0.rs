use std::io;
use std::io::Error;
use std::io::BufRead;

fn parse(line: Result<String, Error>) {
    match line {
        Ok(line) => println!("Got this line: {}", line),
        Err(x) => println!("Error when reading line: {}", x)
    }
}

fn main() {
    let input = io::stdin().lock();
    for line in input.lines() {
        parse(line);
    }
}
