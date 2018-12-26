use std::fs::File;
use std::str::FromStr;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;

use Result;

pub fn in_path() -> &'static str {
    "in"
}

pub fn data_path(n: usize) -> String {
    format!("/home/jonas/CLionProjects/aoc/in/{:02}.txt", n)
}

pub fn sample_path(n: usize) -> String {
    format!("/home/jonas/CLionProjects/aoc/in/{:02}_sample.txt", n)
}

pub fn load(path: &str) -> Result<String> {
    let mut r = File::open(path)?;
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    Ok(s)
}

pub fn load_lines(path: &str) -> Result<Vec<String>> {
    let r = File::open(path)?;
    let r = BufReader::new(r);
    let mut lines = Vec::new();
    for l in r.lines() {
        lines.push(l?);
    }

    Ok(lines)
}

pub fn load_and_parse<T: FromStr>(path: &str) -> Result<T>
    where <T as FromStr>::Err: std::error::Error,
          <T as FromStr>::Err: 'static {
    let mut r = File::open(path)?;
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    let r = s.parse()?;
    Ok(r)
}

pub fn load_and_parse_lines<T: FromStr>(path: &str) -> Result<Vec<T>>
    where <T as FromStr>::Err: std::error::Error,
          <T as FromStr>::Err: 'static {
    let r = File::open(path)?;
    let r = BufReader::new(r);
    let mut result = Vec::new();

    for l in r.lines() {
        let e = l?.parse()?;
        result.push(e);
    }

    Ok(result)
}