use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn is_collision(a: u8, b: u8) -> bool {
    if a.is_ascii_lowercase() != b.is_ascii_lowercase() {
        return a.to_ascii_uppercase() == b.to_ascii_uppercase()
    }

    false
}

fn react_poly<'a>(it: impl Iterator<Item=&'a u8>) -> usize {
    it.fold(Vec::new(), |mut v, &c| {
            if !v.is_empty() && is_collision(*v.last().unwrap(), c) {
                v.pop();
            } else {
                v.push(c);
            }

            v
        }).len()
}

fn part1(s: &[u8]) -> Result<usize> {
    Ok(react_poly(s.iter()))
}

fn part2(s: &[u8]) -> Result<usize> {
    let min = (b'a'..=b'z')
        .map(|c| react_poly(s.iter()
            .filter(|&&a| a.to_ascii_lowercase() != c))
        )
        .min()
        .unwrap();

    Ok(min)
}

fn main() -> Result<()> {
    let mut r = File::open("in/05.txt")?;
    let mut s = Vec::new();
    r.read_to_end(&mut s)?;

    let sample1 = part1(b"dabAcCaCBAcCcaDA")?;
    let sample2 = part2(b"dabAcCaCBAcCcaDA")?;
    println!("sample 1: {}, sample2: {}", sample1, sample2);

    let result1 = part1(&s)?;
    let result2 = part2(&s)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}