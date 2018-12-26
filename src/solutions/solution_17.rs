use Solution;
use Result;
use util::file::data_path;
use std::str::FromStr;
use regex::Regex;
use std::ops::RangeInclusive;
use util::mat2::Mat2;
use std::fmt::Display;
use std::fmt::Formatter;
use std::usize;
use util::mat2::Pos;
use util::file::load;
use util::file::sample_path;
use std::collections::VecDeque;

struct ClaySquare {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum Direction {
    Bottom,
    Left,
    Right
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum TileKind {
    FlowingWater,
    RestingWater,
    Sand,
    Clay,
    Spring
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}

impl TileKind {
    fn is_water(&self) -> bool {
        match self {
            TileKind::RestingWater | TileKind::FlowingWater => true,
            _ => false
        }
    }

    fn is_free(&self) -> bool {
        match self {
            TileKind::Sand => true,
            _ => false
        }
    }
}

impl Default for TileKind {
    fn default() -> Self {
        TileKind::Sand
    }
}

#[derive(Debug, Default)]
struct Data {
    tiles: Mat2<TileKind>,
    source_x: usize
}

impl FromStr for ClaySquare {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE_SQR: Regex = Regex::new(r"^(y|x)(=)(\d+)(,\s*)(y|x)(=)(\d+)(\.\.)(\d+)$").unwrap();
        }

        let caps = RE_SQR.captures(s)
            .ok_or("Invalid capture Line")?;

        let first = caps[3].parse()?;

        let second_start = caps[7].parse()?;
        let second_end = caps[9].parse()?;

        let first = first..=first;
        let second = second_start..=second_end;

        let (x, y) = match caps[1].chars().next().unwrap() {
            'x' => (first, second),
            'y' => (second, first),
            _ => unreachable!()
        };

        Ok(ClaySquare{
            x,
            y
        })
    }
}

impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let sqrs: Vec<ClaySquare> = s.lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<ClaySquare>>>()?;

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (usize::MAX, usize::MAX, 0, 0);
        for sqr in sqrs.iter() {
            min_x = min_x.min(*sqr.x.start());
            max_x = max_x.max(*sqr.x.end());

            min_y = min_y.min(*sqr.y.start());
            max_y = max_y.max(*sqr.y.end());
        }

        let h = max_y - min_y + 2;
        let w = max_x - min_x  + 3;

        min_x -= 1;
        min_y -= 1;

        let mut tiles = Mat2::new(h, w);
        let source_x = 500-min_x;
        tiles[Pos::new(source_x, 0)] = TileKind::Spring;

        let off = (-(min_x as isize) , -(min_y as isize));

        for sqr in sqrs.iter() {
            for y in sqr.y.clone() {
                for x in sqr.x.clone() {
                    let p = Pos::new(x, y)
                        .offset(off)
                        .unwrap();

                    tiles[p] = TileKind::Clay;
                }
            }
        }

        Ok(Data{
            tiles,
            source_x
        })
    }
}

impl Display for TileKind {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            TileKind::RestingWater => '~',
            TileKind::Sand => '.',
            TileKind::Clay => '#',
            TileKind::Spring => '+',
            TileKind::FlowingWater => '|'
        };

        write!(f, "{}", c)
    }
}

impl Data {
    fn water_tiles(&self) -> usize {
        self.tiles.grids()
            .filter(|t| t.is_water())
            .count()
    }

    fn resting_water_tiles(&self) -> usize {
        self.tiles.grids()
            .filter(|&&t| t == TileKind::RestingWater)
            .count()
    }

    fn find_floor_bound(&self, p: Pos, dir: Direction) -> Option<Pos> {
        let mut p = p;

        loop {
            let next = p.offset(dir.offset())?;
            let t = self.tiles.get(next)?;

            match t {
                TileKind::Clay => return Some(p),
                TileKind::Sand => return None,
                TileKind::FlowingWater => {
                    if self.can_flow(next, Direction::Bottom) {
                        return None;
                    }
                },
                t => {
                    panic!("Invalid floor tile: {:?}", t);
                }
            }

            p = next;
        }
    }

    fn update_flow(&mut self, p: Pos, q: &mut VecDeque<Pos>) {

        let t = self.tiles[p];
        if t != TileKind::FlowingWater && t != TileKind::Spring {
            return;
        }

        let bot = self.can_flow(p, Direction::Bottom);
        let left = self.can_flow(p, Direction::Left);
        let right = self.can_flow(p, Direction::Right);
        match (bot, left, right) {
            (true, _, _) => {
                self.flow(p, Direction::Bottom);
                q.push_back(p.offset( Direction::Bottom.offset()).unwrap());
            },
            (_, true, _) | (_, _, true) => {
                if left {
                    self.flow(p, Direction::Left);
                    q.push_back(p.offset( Direction::Left.offset()).unwrap());
                }

                if right {
                    self.flow(p, Direction::Right);
                    q.push_back(p.offset( Direction::Right.offset()).unwrap());
                }
            },
            _ => {
                let left = self.find_floor_bound(p, Direction::Left);
                let right = self.find_floor_bound(p, Direction::Right);

                match (left, right) {
                    (Some(l), Some(r)) => {
                        for x in l.x..=r.x {
                            self.tiles[Pos::new(x, p.y)] = TileKind::RestingWater;

                            let p = Pos::new(x, p.y-1);
                            if self.tiles[p] == TileKind::FlowingWater {
                                q.push_back(p)
                            }
                        }
                    },
                    _ => {
                        //Do nothing
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        let h = self.tiles.height();
        let mut q = VecDeque::new();

        q.push_back(Pos::new(self.source_x, 0));

        while let Some(p) = q.pop_front() {
            if p.y == h {
                continue;
            }
            self.update_flow(p, &mut q);
        }
    }

    fn flow(&mut self, p: Pos, dir: Direction) {
        if dir == Direction::Bottom && p.y == self.tiles.height() - 1 {
            return;
        }
        let n = p.offset(dir.offset()).unwrap();
        self.tiles[n] = TileKind::FlowingWater;
    }

    fn can_flow(&self, p: Pos, dir: Direction) -> bool {
        if dir == Direction::Bottom && p.y == self.tiles.height() - 1 {
            return true;
        }

        if let Some(p) = p.offset(dir.offset()) {
            if let Some(t) = self.tiles.get(p) {
               return t.is_free();
            }
        }

        false
    }
}



#[derive(Default)]
pub struct Solution17 {
    data: Data,
    sample: Data
}

fn part1(d: &mut Data) -> usize {
    d.update();
    d.water_tiles()
}

impl Solution for Solution17 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(17))?;
        self.data = s.parse()?;

        let s = load(&sample_path(17))?;
        self.sample = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&mut self.sample);
        println!("sample1: {}", result);

        let result = part1(&mut self.data);
        println!("data1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        println!("sample2: {}", self.sample.resting_water_tiles());
        println!("result2: {}", self.data.resting_water_tiles());
        Ok(())
    }
}