use std::{fs::File, io::{BufReader, BufRead}};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let mut gamma_rate = String::with_capacity(12);
    let mut epsilon_rate = String::with_capacity(12);
    
    buffered
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line
            .chars()
            .map(|c| if c == '1' {
                (1_usize, 0_usize)
            } else {
                (0_usize, 1_usize)
            })
            .collect::<Vec<(usize, usize)>>()
        )
        .reduce(|acc, cur| acc
            .into_iter()
            .zip(cur.into_iter())
            .map(|((acc_1, acc_0), (cur_1, cur_0))| (acc_1 + cur_1, acc_0 + cur_0))
            .collect::<Vec<(usize, usize)>>()
        )
        .unwrap()
        .into_iter()
        .for_each(|(one, zero)| {
            if one > zero {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            } else {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            }
        });

    let gamma_rate: u16 = u16::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate: u16 = u16::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    let total: u32 = (gamma_rate as u32) * (epsilon_rate as u32);
    println!("{}", total);

    Ok(())
}
