use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let mut iter = buffered
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .into_iter()
        .peekable();

    let mut total_increasing_depths = 0_usize;
    
    while let Some(current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if next > current {
                total_increasing_depths += 1;
            }
        }
    }

    println!("Total increasing depths: {}", total_increasing_depths);

    Ok(())
}
