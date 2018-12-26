
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::isize;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

//position=<10, -3> velocity=<-1,  1>

#[derive(Debug, Clone)]
struct Vec2 {
    x: isize,
    y: isize
}

#[derive(Debug, Clone)]
struct Point {
    pos: Vec2,
    velocity: Vec2
}

impl Point {
    fn update(&mut self, delta: usize) {
        let delta = delta as isize;
        self.pos.x += self.velocity.x * delta;
        self.pos.y += self.velocity.y * delta;
    }

    fn is_neighbor(&self, other: &Point) -> bool {
        let x = (self.pos.x - other.pos.x).abs();
        let y = (self.pos.y - other.pos.y).abs();

        x <= 1 && y <= 1
    }
}

fn parse_point(s: &str) -> Result<Point> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(position=<\s*)(-?\d+)(,\s*)(-?\d+)(>\s*velocity=<\s*)(-?\d+)(,\s*)(-?\d+)").unwrap();
     }

    let caps = RE.captures(s)
        .ok_or("Invalid Point Line")?;

    let pos_x: isize = caps[2].parse()?;
    let pos_y: isize = caps[4].parse()?;


    let vel_x: isize = caps[6].parse()?;
    let vel_y: isize = caps[8].parse()?;

    Ok(Point {
        pos: Vec2{
            x: pos_x,
            y: pos_y
        },
        velocity: Vec2 {
            x: vel_x,
            y: vel_y
        }
    })
}

fn load(file: &str) -> Result<Vec<Point>> {
    let r = File::open(file)?;
    let r = BufReader::new(r);
    let mut points = Vec::new();

    for l in r.lines() {
        let l = l?;
        points.push(parse_point(&l)?);
    }

    Ok(points)
}

fn print_grid(p: &[Point], base_x: isize, base_y: isize, h: usize, w: usize) {
    let mut grid = vec![false; h*w];
    for p in p.iter() {
        let p = &p.pos;
        let x = (p.x - base_x).abs() as usize;
        let y = (p.y - base_y).abs() as usize;
        grid[y*w + x] = true;
    }

    println!("--grid--");
    for y in 0..h {
        for x in 0..w {
            let c = if grid[y * w + x] {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!("--------");
}

fn part1(s: &[Point]) -> Result<usize> {
    const THRESHOLD: usize = 65;
    let mut points = s.to_vec();


    for second in 1.. {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);

        //Update
        for p in points.iter_mut() {
            p.update(1);

            min_x = min_x.min(p.pos.x);
            max_x = max_x.max(p.pos.x);

            min_y = min_y.min(p.pos.y);
            max_y = max_y.max(p.pos.y);
        }

        let w = (max_x - min_x).abs() as usize + 1;
        let h = (max_y - min_y).abs() as usize + 1;
        if h < THRESHOLD && w < THRESHOLD {
            print_grid(&points, min_x, min_y, h, w);
            return Ok(second);
        }
    }

    Ok(0)
}

fn main() -> Result<()> {
    const N: usize = 10;

    let sample = load(&format!("in/{}_sample.txt", N))?;
    let data = load(&format!("in/{}.txt", N))?;

    let result2 = part1(&data)?;
    println!("2: {}", result2);
    Ok(())
}