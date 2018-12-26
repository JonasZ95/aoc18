use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::usize;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialOrd, PartialEq)]
struct Vec2 {
    x: u64,
    y: u64,
}

struct Grid {
    field: Vec<usize>,
    h: usize,
    w: usize,
}

impl Grid {
    fn new(h: usize, w: usize) -> Grid {
        Grid {
            field: vec![0; h * w],
            h,
            w,
        }
    }

    fn from_coords_sum(coords: &[Vec2]) -> Grid {
        let w = coords.iter()
            .max_by_key(|c| c.x)
            .unwrap()
            .x + 2;

        let h = coords.iter()
            .max_by_key(|c| c.y)
            .unwrap()
            .y + 2;

        let mut grid = Grid::new(h as usize, w as usize);

        for x in 0..w {
            for y in 0..h {
                let v = Vec2 { x, y };
                grid.set(v, v.sum_dist(coords));
            }
        }

        grid

    }

    fn from_coords(coords: &[Vec2]) -> Grid {
        let w = coords.iter()
            .max_by_key(|c| c.x)
            .unwrap()
            .x + 2;

        let h = coords.iter()
            .max_by_key(|c| c.y)
            .unwrap()
            .y + 2;


        let mut grid = Grid::new(h as usize, w as usize);
        for x in 0..w {
            for y in 0..h {
                let v = Vec2{x, y};
                let mut min = usize::MAX;


                for (i, d) in coords.iter()
                    .map(|c| c.dist(&v))
                    .enumerate() {

                    match min.cmp(&d) {
                        Ordering::Equal => grid.set(v, 0),
                        Ordering::Greater => {
                            min = d;
                            grid.set(v, i + 1);
                        },
                        _ => continue
                    }
                }
            }
        }

        grid
    }



    fn set(&mut self, v: Vec2, i: usize) {
        self.field[(v.y as usize * self.w) + v.x as usize] = i;
    }

    fn get(&self, v: Vec2) -> usize {
        self.field[(v.y as usize * self.w) + v.x as usize]
    }

    fn finite_coords(&self, s: &[Vec2]) -> HashSet<usize> {
        let mut finite: HashSet<usize> = s.iter()
            .enumerate()
            .map(|(i, _)| i + 1)
            .collect();

        let row_0 = (0..self.w)
            .zip(iter::repeat(0));
        let row_h = (0..self.w)
            .zip(iter::repeat(self.h-1));

        let col_0 = iter::repeat(0)
            .zip(0..self.h);
        let col_h = iter::repeat(self.w - 1)
            .zip(0..self.h);

        for (x, y) in row_0
            .chain(row_h)
            .chain(col_0)
            .chain(col_h)  {

            let v = Vec2{
                x: x as u64,
                y: y as u64
            };

            let i = self.get(v) as usize;
            if i != 0 {
                finite.remove(&i);
            }
        }

        finite
    }

    fn count_coords(&self, i: usize) -> usize {
        self.field.iter()
            .filter(|&&j| j == i)
            .count()
    }
}


impl Vec2 {
    fn dist(&self, other: &Vec2) -> usize {
        let x_d = self.x as i64 - other.x as i64;
        let y_d = self.y as i64 - other.y as i64;

        (x_d.abs() + y_d.abs())  as usize
    }

    fn sum_dist(&self, other: &[Vec2]) -> usize {
        other.iter()
            .map(|v|  self.dist(v))
            .sum()
    }
}

fn parse_coord(s: &str) -> Result<Vec2> {
    let split = s.find(", ")
        .ok_or("No comma found")?;

    let (x, y) = s.split_at(split);
    let x: u64 = x.parse()?;
    let y: u64 = y[2..].parse()?;

    Ok(Vec2 { x, y })
}

fn load_coords(file: &str) -> Result<Vec<Vec2>> {
    let r = File::open(file)?;
    let r = BufReader::new(r);
    let mut coords = Vec::new();

    for l in r.lines() {
        let l = parse_coord(l?.as_ref())?;
        coords.push(l);
    }

    Ok(coords)
}

fn part1(s: &[Vec2]) -> Result<usize> {
    let grid = Grid::from_coords(s);
    let finite = grid.finite_coords(s);

    let max = finite.iter()
        .map(|&i| (i, grid.count_coords(i)))
        .max_by_key(|(_, n)| *n)
        .unwrap()
        .1;
    Ok(max)
}

fn part2(s: &[Vec2], max: usize) -> Result<usize> {
    let grid = Grid::from_coords_sum(s);
    let n = grid.field.iter()
        .filter(|&&n| n < max)
        .count();
    Ok(n)
}


fn main() -> Result<()> {
    let data = load_coords("in/06.txt")?;
    let sample = load_coords("in/06_sample.txt")?;

    let sample1 = part1(&sample)?;
    let sample2 = part2(&sample, 32)?;
    println!("sample 1: {}, sample2: {}", sample1, sample2);

    let result1 = part1(&data)?;
    let result2 = part2(&data, 10_000)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}