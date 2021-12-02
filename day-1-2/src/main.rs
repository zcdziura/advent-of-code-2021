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

    let lines = buffered
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let mut measurement_windows = Vec::<(usize, usize, usize)>::new();
    let mut idx = 0_usize;
    while let Some(&first) = lines.get(idx) {
        let second = if let Some(&second) = lines.get(idx + 1) {
            second
        } else {
            break;
        };

        let third = if let Some(&third) = lines.get(idx + 2) {
            third
        } else {
            break;
        };

        measurement_windows.push((first, second, third));
        idx += 1;
    }

    let mut peekable = measurement_windows.into_iter()
        .map(|(first, second, third)| first + second + third)
        .peekable();

    let mut total_increasing_sliding_window_depths = 0;
    while let Some(current) = peekable.next() {
        if let Some(&next) = peekable.peek() {
            if next > current {
                total_increasing_sliding_window_depths += 1;
            }
        }
    }

    println!("Total increasing sliding window depths: {}", total_increasing_sliding_window_depths);
    
    Ok(())
}
