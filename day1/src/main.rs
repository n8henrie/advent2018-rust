use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, SeekFrom};

fn part1(inputfile: &str) -> io::Result<()> {
    let mut num: i32 = 0;
    let f = File::open(inputfile)?;
    for line in BufReader::new(f).lines() {
        let line = line?.parse::<i32>().unwrap();
        num += line;
    }
    println!("{}", num);
    Ok(())
}

fn part2(inputfile: &str) -> io::Result<()> {
    let mut num: i32 = 0;
    let mut seen = HashSet::new();
    let f = File::open(inputfile)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            reader.get_mut().seek(SeekFrom::Start(0))?;
            continue;
        }
        let line = line.trim().parse::<i32>().unwrap();
        num += line;
        if seen.insert(num.clone()) == false {
            println!("{}", num);
            return Ok(());
        }
    }
}

fn main() {
    let homedir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => "/".to_string(),
    };
    let inputfile = format!("{}/{}", homedir, "git/advent2018-rust/day1/input.txt");
    part1(&inputfile);
    part2(&inputfile);
}
