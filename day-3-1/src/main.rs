use std::{fs::File, io::{BufReader, BufRead}};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    errors {
        InvalidDirection(direction: String) {
            description("invalid direction"),
            display("invalid direction: {}", direction),
        }
    }
}

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let lines = buffered
        .lines()
        .filter_map(|line| line.ok()|)
        .map(|line| );
    
    Ok(())
}
