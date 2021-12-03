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

    let mut oxygen_values = buffered
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();
    let mut co2_values = oxygen_values.clone();

    let mut o2_idx = 0;
    while oxygen_values.len() > 1 {
        let (ones, zeroes) = oxygen_values
            .iter()
            .map(|val| {
                if val.chars().nth(o2_idx).unwrap() == '1' {
                    (1, 0)
                } else {
                    (0, 1)
                }
            })
            .reduce(|(acc_1, acc_0), (cur_1, cur_0)| (acc_1 + cur_1, acc_0 + cur_0))
            .unwrap();

        let most_common_bit = if ones >= zeroes { '1' } else { '0' };

        oxygen_values = oxygen_values
            .into_iter()
            .filter(|val| val.chars().nth(o2_idx).unwrap() == most_common_bit)
            .collect();
        o2_idx += 1;
    }

    let oxygen_reading = u16::from_str_radix(oxygen_values.pop().as_ref().unwrap(), 2).unwrap();

    let mut co2_idx = 0;
    while co2_values.len() > 1 {
        let (ones, zeroes) = co2_values
            .iter()
            .map(|val| {
                if val.chars().nth(co2_idx).unwrap() == '1' {
                    (1, 0)
                } else {
                    (0, 1)
                }
            })
            .reduce(|(acc_1, acc_0), (cur_1, cur_0)| (acc_1 + cur_1, acc_0 + cur_0))
            .unwrap();

        let least_common_bit = if ones >= zeroes { '0' } else { '1' };

        co2_values = co2_values
            .into_iter()
            .filter(|val| val.chars().nth(co2_idx).unwrap() == least_common_bit)
            .collect();
        co2_idx += 1;
    }

    let co2_reading = u16::from_str_radix(co2_values.pop().as_ref().unwrap(), 2).unwrap();

    let life_support_rating: u32 = (oxygen_reading as u32) * (co2_reading as u32);
    println!("{}", life_support_rating);

    Ok(())
}
