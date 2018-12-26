use Solution;
use Result;
use util::file::data_path;
use util::file::sample_path;
use util::file::load;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use util::mat2::Mat2;
use util::mat2::Pos;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Tile {
    Lumberyard,
    OpenGround,
    Tree
}

#[derive(Default, Clone)]
pub struct Data {
    tiles: Mat2<Tile>,
    back_buffer: Mat2<Tile>
}

#[derive(Default)]
pub struct Solution18 {
    data: Data,
    sample: Data
}

impl Default for Tile {
    fn default() -> Self {
        Tile::OpenGround
    }
}

impl Tile {
    fn from_char(c: char ) -> Result<Tile> {
        let t = match c {
            '#' => Tile::Lumberyard,
            '|' => Tile::Tree,
            '.' => Tile::OpenGround,
            _ => return Err("Invalid tile".into())
        };

        Ok(t)
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Lumberyard => '#',
            Tile::Tree => '|',
            Tile::OpenGround => '.'
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let h = s.lines().count();
        let w = s.lines().max_by_key(|l| l.len()).unwrap().len();

        let mut tiles = Mat2::new(h, w);
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                tiles[Pos::new(x, y)] = Tile::from_char(c)?;
            }
        }

        Ok(Data{
            back_buffer: tiles.clone(),
            tiles
        })
    }
}

impl Data {
    fn adjacent(&self, p: Pos) -> impl Iterator<Item=Pos> + '_ {
        [(-1, -1), (0, -1), (1, -1),
            (-1, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1)].iter()
            .filter_map(move |&o| p.offset(o))
            .filter( move |&p| self.tiles.has_pos(p))
    }

    fn count_resources(&self) -> (usize, usize, usize) {
        let (mut trees, mut lumberyards, mut open) = (0, 0, 0);
        for p in self.tiles.coords() {
            match self.tiles[p] {
                Tile::OpenGround => open += 1,
                Tile::Tree =>  trees += 1,
                Tile::Lumberyard => lumberyards += 1
            }
        }

        (trees, lumberyards, open)
    }

    fn count_adjacent(&self, p: Pos) -> (usize, usize, usize) {
        let (mut trees, mut lumberyards, mut open) = (0, 0, 0);
        for p in self.adjacent(p) {
            match self.tiles[p] {
                Tile::OpenGround => open += 1,
                Tile::Tree =>  trees += 1,
                Tile::Lumberyard => lumberyards += 1
            }
        }

        (trees, lumberyards, open)
    }

    fn update(&mut self) {
        for p in self.tiles.coords() {
            let (trees, lumberyards, _) = self.count_adjacent(p);
            let t = match self.tiles[p] {
                Tile::OpenGround if trees >= 3 => Tile::Tree,
                Tile::OpenGround => Tile::OpenGround,

                Tile::Tree if lumberyards >= 3 => Tile::Lumberyard,
                Tile::Tree => Tile::Tree,

                Tile::Lumberyard if lumberyards >= 1 && trees >= 1 => Tile::Lumberyard,
                Tile::Lumberyard => Tile::OpenGround
            };

            self.back_buffer[p] = t;
        }

        std::mem::swap(&mut self.back_buffer, &mut self.tiles);
    }

    fn update_mins(&mut self, mins: usize) {
        for _ in 0..mins {
            self.update();
        }
    }

    fn resource_value(&self) -> usize {
        let (trees, lumberyards, _) = self.count_resources();
        trees * lumberyards
    }
}

fn part1(d: &Data) -> Result<usize> {
    let mut d = d.clone();
    d.update_mins(10);
    Ok(d.resource_value())
}

fn part2(d: &Data) -> Result<usize> {
    let min = 1_000_000_000;
    let mut d = d.clone();

    let v: Vec<usize> = (0..1_000)
        .map(|_| {
            d.update();
            d.resource_value()
        })
        .collect();

    let last = *v.last().unwrap();
    let period = v.iter()
        .enumerate().rev().skip(1)
        .find(|(_, &n)| n == last)
        .map(|(i, _)| v.len() - 1 - i )
        .unwrap();

    let ix = v.len() - 1 - ((min - v.len()) % period);
    Ok(v[ix])
}

impl Solution for Solution18 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(18))?;
        self.data = s.parse()?;

        let s = load(&sample_path(18))?;
        self.sample = s.parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample)?;
        println!("sample1: {}", result);

        let result = part1(&self.data)?;
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = part2(&self.sample)?;
        println!("sample2: {}", result);

        let result = part2(&self.data)?;
        println!("result2: {}", result);
        Ok(())
    }
}