use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct CheckSum {
    twice: u32,
    thrice: u32,
}

impl CheckSum {
    fn new() -> CheckSum {
        CheckSum {
            twice: 0,
            thrice: 0,
        }
    }
    fn check(&self) -> u32 {
        self.twice * self.thrice
    }
}

fn part1() -> io::Result<()> {
    let f = File::open("day2/input.txt")?;

    let mut checksum = CheckSum::new();
    for line in BufReader::new(f).lines().map(|s| s.unwrap()) {
        let mut counts = HashMap::with_capacity(line.len());
        for ch in line.chars() {
            let count = counts.entry(ch).or_insert(0);
            *count += 1;
        }
        for val in counts.values().collect::<HashSet<_>>() {
            match val {
                2 => checksum.twice += 1,
                3 => checksum.thrice += 1,
                _ => (),
            }
        }
    }
    println!("{:?}", checksum);
    println!("{}", checksum.check());
    Ok(())
}

fn part2() -> io::Result<()> {
    let text = fs::read_to_string("day2/input.txt")?;
    let lines: Vec<_> = text.as_str().lines().collect();
    for line in lines.iter() {
        let matches: Vec<_> = lines
            .iter()
            .filter(|&l| {
                l != line && l.chars().zip(line.chars()).filter(|(a, b)| a != b).count() == 1
            })
            .collect();
        if !matches.is_empty() {
            for m in matches {
                println!(
                    "{}",
                    line.chars()
                        .zip(m.chars())
                        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                        .collect::<String>()
                )
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
