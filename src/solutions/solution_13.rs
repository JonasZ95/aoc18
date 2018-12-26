use Solution;
use Result;
use util::file::data_path;
use std::str::FromStr;
use util::file::sample_path;
use util::file::load;
use std::io::Write;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Copy, Clone, Debug)]
enum Path {
    VerticalRoad,
    HorizontalRoad,
    UpCurve,
    DownCurve,
    Cross
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    dir: Direction,
    pos: (usize, usize),
    cross_counter: usize,
    crashed: bool
}

#[derive(Default, Debug)]
struct Grid {
    grid: Vec<Option<Path>>,
    carts: Vec<Cart>,
    h: usize,
    w: usize
}

impl Direction {
    fn from_char(c: char) -> Result<Direction> {
        Ok(match c {
            'v' => Direction::South,
            '^' => Direction::North,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => return Err("Unknown direction".into())
        })
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        use self::Direction::*;
        match self {
            North => '^',
            South => 'v',
            East => '>',
            West => '<'
        }
    }
    fn left(&self) -> Direction {
        use self::Direction::*;

        match self {
            North => West,
            West => South,
            South => East,
            East => North
        }
    }


    fn right(&self) -> Direction {
        use self::Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }
}

impl Path {
    fn from_char(c: char) -> Result<Path> {
        Ok(match c {
            '+' => Path::Cross,
            '-' => Path::HorizontalRoad,
            '|' => Path::VerticalRoad,
            '\\' => Path::DownCurve,
            '/' => Path::UpCurve,
            _ => return Err("Unknown path".into())
        })
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            Path::HorizontalRoad => '-',
            Path::VerticalRoad => '|',
            Path::UpCurve => '/',
            Path::DownCurve => '\\',
            Path::Cross => '+'
        }
    }

    fn move_to_dir(dir: Direction) -> ((isize, isize), Direction) {
        match dir {
            Direction::North => ((0, -1), dir),
            Direction::South => ((0, 1), dir),
            Direction::East => ((1, 0), dir),
            Direction::West => ((-1, 0), dir),
        }
    }

    fn move_cart(&self, cart: &mut Cart) -> Result<()> {
        use self::Direction::*;
        let (off, dir) = match self {
            Path::VerticalRoad => {
                match cart.dir {
                    North | South => Path::move_to_dir(cart.dir),
                    _ => return Err("Vertical out of bounds".into())
                }
            },

            Path::HorizontalRoad => {
                match cart.dir {
                    East | West => Path::move_to_dir(cart.dir),
                    _ => return Err("Horizontal out of bounds".into())
                }
            }


            // /
            Path::UpCurve => {
                match cart.dir {
                    North => Path::move_to_dir(East),
                    South => Path::move_to_dir(West),
                    West => Path::move_to_dir(South),
                    East => Path::move_to_dir(North),
                }
            }

            // \
            Path::DownCurve => {
                match cart.dir {
                    North => Path::move_to_dir(West),
                    South => Path::move_to_dir(East),
                    West => Path::move_to_dir(North),
                    East => Path::move_to_dir(South),
                }
            }

            Path::Cross => {
                let counter = cart.cross_counter;
                cart.cross_counter = (cart.cross_counter + 1) % 3;
                match counter {
                    0 => Path::move_to_dir(cart.dir.left()),
                    1 => Path::move_to_dir(cart.dir),
                    2 => Path::move_to_dir(cart.dir.right()),
                    _ => unreachable!()
                }
            }
        };

        cart.dir = dir;
        cart.pos.0 = ((cart.pos.0 as isize) + off.0) as usize;
        cart.pos.1 = ((cart.pos.1 as isize) + off.1) as usize;
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let h = s.lines().count();
        let w = s.lines()
            .max_by_key(|l| l.len())
            .unwrap()
            .len();

        let mut grid = Grid::new(h, w);

       for (y, line) in s.lines().enumerate() {
           for (x, c) in line.chars().enumerate() {
                let path = match Direction::from_char(c) {
                    Ok(dir) => {
                        grid.carts.push(Cart{
                            cross_counter: 0,
                            dir,
                            pos: (x, y),
                            crashed: false
                        });
                        match dir {
                            Direction::West | Direction::East => Some(Path::HorizontalRoad),
                            _ => Some(Path::VerticalRoad)
                        }
                    },
                    Err(_) => {
                        if c == ' ' {
                            None
                        } else {
                            Some(Path::from_char(c)?)
                        }
                    }
                };

               grid.set(x, y, path);
           }
       }

        Ok(grid)
    }
}

impl Grid {
    fn new(h: usize, w: usize) -> Grid {
        let grid = vec![None; h * w];
        let carts = Vec::new();

        Grid{
            h,
            w,
            grid,
            carts
        }
    }

    fn calc_index(&self, x: usize, y: usize) -> usize {
        y*self.w + x
    }

    fn get(&self, x: usize, y: usize) -> &Option<Path> {
        return &self.grid[self.calc_index(x, y)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<Path> {
        let ix = self.calc_index(x, y);
        return &mut self.grid[ix]
    }

    fn set(&mut self, x: usize, y: usize, path: Option<Path>) {
        *self.get_mut(x, y) = path;
    }

    fn tick(&mut self) {
        let w = self.w;
        self.carts.sort_by_key(|c| c.pos.1 * w + c.pos.0);

        for i in 0..self.carts.len() {
            if self.carts[i].crashed {
                continue;
            }

            let pos = self.carts[i].pos;
            let path = self.get(pos.0, pos.1).unwrap();
            path.move_cart(&mut self.carts[i]).unwrap();

            for j in 0..self.carts.len() {
                if i !=j && !self.carts[j].crashed && self.carts[j].pos == self.carts[i].pos {
                    self.carts[j].crashed = true;
                    self.carts[i].crashed = true;
                }
            }
        }


    }

    fn run_till_collision(&mut self) -> (usize, usize) {
        loop {
            if let Some(coll) = self.carts.iter().find(|c| c.crashed) {
                return coll.pos;
            }

            self.tick();
        }
    }

    fn run_till_last_cart(&mut self) -> (usize, usize) {
        while self.carts.iter().filter(|c| !c.crashed).count() > 1 {
            self.tick();
        }

        self.carts.iter().find(|c| !c.crashed).unwrap().pos
    }

    #[allow(dead_code)]
    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        for y in 0..self.h {
            for x in 0..self.w {
                let cart = self.carts.iter()
                    .find(|c| c.pos.0 == x && c.pos.1 == y);
                let c = match cart {
                    Some(cart) => cart.dir.to_char(),
                    None => self.get(x, y)
                        .map(|p| p.to_char())
                        .unwrap_or(' ')
                };
                write!(w, "{}", c)?;
            }
            writeln!(w, "")?;
        }

        w.flush()?;

        Ok(())
    }
}

#[derive(Default)]
pub struct Solution13 {
    grid: Grid,
    grid_sample: Grid
}


fn part1(g: &mut Grid) -> (usize, usize) {
    g.run_till_collision()
}

fn part2(g: &mut Grid) -> (usize, usize) {
    g.run_till_last_cart()
}

impl Solution for Solution13 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(13))?;
        self.grid = s.parse()?;
        let s = load(&sample_path(13))?;
        self.grid_sample = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&mut self.grid_sample);
        println!("sample1: {:?}", result);
        let result = part1(&mut self.grid);
        println!("result1: {:?}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = part2(&mut self.grid);
        println!("result2: {:?}", result);
        Ok(())
    }
}