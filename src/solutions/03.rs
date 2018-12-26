#[macro_use]
extern crate nom;

use nom::types::CompleteStr;


use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;



fn main() -> Result<()> {
    let r = File::open("in/03.txt")?;
    let r = BufReader::new(r);

    let mut claims: Vec<Claim> = Vec::new();
    for line in r.lines() {
        claims.push(line?.parse()?);
    }

    let max_width = claims.iter().map(|c| c.left_offset + c.width).max().unwrap();
    let max_height = claims.iter().map(|c| c.top_offset + c.height).max().unwrap();

    let mut grid = Grid::new(max_width, max_height);
    for claim in &claims {
        grid.set_claim(claim);
    }

    let result1 = part1(&claims, &grid)?;
    let result2 = part2(&claims, &grid)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}

fn part1(claims: &[Claim], grid: &Grid) -> Result<usize> {
    Ok(grid.crossovers())
}

fn part2(claims: &[Claim], grid: &Grid) -> Result<usize> {
    Ok(
        claims.iter()
            .find(|c| grid.is_single_crossover_claim(c)).unwrap().id
    )
}

struct Claim {
    id: usize,
    left_offset: usize,
    top_offset: usize,
    width: usize,
    height: usize
}

struct Grid {
    field: Vec<usize>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            field: vec![0; width * height],
            width,
            height
        }
    }

    fn set(&mut self, x: usize, y: usize,)  {
        self.field[y*self.height + x] += 1;
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.field[y*self.height + x]
    }

    fn set_claim(&mut self, claim: &Claim) {
        for x in 0..claim.width {
            for y in 0..claim.height {
                self.set(claim.left_offset + x, claim.top_offset + y);
            }
        }
    }

    fn crossovers(&self) -> usize {
        self.field.iter()
            .filter(|&&i| i > 1)
            .count()
    }

    fn is_single_crossover_claim(&self, claim: &Claim) -> bool {
        for x in 0..claim.width {
            for y in 0..claim.height {
                if self.get(claim.left_offset + x, claim.top_offset + y) > 1 {
                    return false
                }
            }
        }

        true
    }
}

impl FromStr for Claim {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        claim(CompleteStr(s))
            .map(|(_, c)| c)
            .map_err(|e| e.to_string().into())
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}



named!(num<CompleteStr, u64>,
       map_res!(take_while!(is_digit), |s: CompleteStr| u64::from_str_radix(s.0, 10))
);



named!(claim<CompleteStr, Claim>,
  do_parse!(
    tag!("#")   >>
    id: num >>

    tag!(" @ ") >>
    left_offset: num >>
    tag!(",") >>
    top_offset: num >>

    tag!(": ") >>

    width: num >>
    tag!("x") >>
    height: num >>

    (Claim { id, left_offset, top_offset, width, height })
  )
);

