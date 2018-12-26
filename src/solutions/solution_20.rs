use Solution;
use Result;
use util::file::data_path;
use util::file::sample_path;
use util::file::load;
use std::str::FromStr;
use util::mat2::Mat2;
use util::mat2::Pos;
use std::fmt::Display;
use std::fmt::Formatter;
use std::usize;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub enum TileKind {
    VerticalDoor,
    HorizontalDoor,
    Wall,
    Room,
    Cur,
}

#[derive(Debug)]
pub enum Expression {
    Sequence(Vec<Direction>),
    OneOf(Vec<Expression>),
    Concat(Vec<Expression>)
}

#[derive(Default)]
pub struct Data {
    expr: Expression
}


#[derive(Default)]
pub struct Solution20 {
    data: Data,
    sample: Data,
}

pub struct Grid {
    tiles: Mat2<TileKind>,
    p: Pos,
    reach: Mat2<usize>,
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Sequence(Vec::new())
    }
}

impl Default for TileKind {
    fn default() -> Self {
        TileKind::Wall
    }
}

impl Direction {
    fn from_char(c: char) -> Result<Direction> {
        use self::Direction::*;
        let t = match c {
            'N' => North,
            'E' => East,
            'S' => South,
            'W' => West,
            _ => return Err("Invalid direction".into())
        };

        Ok(t)
    }

    fn to_char(&self) -> char {
        use self::Direction::*;
        match self {
            North => 'N',
            East => 'E',
            South => 'S',
            West => 'W'
        }
    }

    fn offset(&self) -> (isize, isize) {
        use self::Direction::*;
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0)
        }
    }

    fn is_horizontal(&self) -> bool {
        use self::Direction::*;
        match self {
            North | South => false,
            _ => true
        }
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    fn door_kind(&self) -> TileKind {
        if self.is_vertical() {
            TileKind::VerticalDoor
        } else {
            TileKind::HorizontalDoor
        }
    }
}


impl TileKind {
    #[allow(dead_code)]
    fn from_char(c: char) -> Result<TileKind> {
        use self::TileKind::*;
        let t = match c {
            '#' => Wall,
            '.' => Room,
            '-' => VerticalDoor,
            '|' => HorizontalDoor,
            'X' => Cur,
            _ => return Err("Invalid tile".into())
        };

        Ok(t)
    }

    fn to_char(&self) -> char {
        use self::TileKind::*;
        match self {
            Wall => '#',
            Room => '.',
            VerticalDoor => '-',
            HorizontalDoor => '|',
            Cur => 'X'
        }
    }
}


impl Grid {
    fn new(n: usize) -> Self {
        let center = Pos::new(n / 2, n / 2);
        let reach = Mat2::new_with(n, n, usize::MAX);

        Grid {
            tiles: Mat2::new(n, n),
            p: center,
            reach,
        }
    }

    fn walk_seq(&mut self, p: Pos, d: usize, seq: &[Direction]) -> (Pos, usize) {
        let mut p = p;
        let mut d = d;
        for dir in seq {
            let o = dir.offset();

            p = p.offset(o).unwrap();
            self.tiles[p] = dir.door_kind();

            p = p.offset(o).unwrap();
            self.tiles[p] = TileKind::Room;


            self.reach[p] = self.reach[p].min(d);
            d += 1;
        }

        (p, d)
    }


    fn fill(&mut self, e: &Expression) {
        let p = self.p;
        self.fill2(&e, p, 1);
    }

    fn fill2(&mut self, e: &Expression, p: Pos, d: usize) -> (Pos, usize) {
        match e {
            Expression::Sequence(seq) => self.walk_seq(p, d, &seq),
            Expression::Concat(exprs) => {

                let mut pd = (p, d);
                for expr in exprs.iter() {
                    pd = self.fill2(expr, pd.0, pd.1);
                }

                pd
            },
            Expression::OneOf(exprs) => {
                for expr in exprs.iter() {
                    self.fill2(expr, p, d);
                }

                (p, d)
            }

        }
    }

    fn furthest_room_dist(&self) -> usize {
        *self.reach.grids()
            .filter(|&&d| d != usize::MAX)
            .max()
            .unwrap()
    }

    #[allow(dead_code)]
    fn rooms_with_min_d(&self, min_d: usize) -> usize {
        self.reach.grids()
            .filter(|&&d| d != usize::MAX && d >= min_d)
            .count()
    }
}

fn part1(d: &Data, n: usize) -> Result<usize> {
    let mut grid = Grid::new(n);
    grid.fill(&d.expr);
    Ok(grid.furthest_room_dist())
}

fn part2(d: &Data, n: usize) -> Result<usize> {
    let mut grid = Grid::new(n);
    grid.fill(&d.expr);
    Ok(grid.rooms_with_min_d(1000))
}

impl Solution for Solution20 {
    fn init(&mut self) -> Result<()> {
        let s = load(&sample_path(20))?;
        self.sample = s.parse()?;

        let s = load(&data_path(20))?;
        self.data = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample, 500)?;
        println!("sample1: {}", result);
        let result = part1(&self.data, 1000)?;
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = part2(&self.data, 1000)?;
        println!("result2: {}", result);
        Ok(())
    }
}

fn parse_expression(s: &str) -> Result<(Expression, usize)> {
    let first = s.chars().next();

    match first {
        //One of
        Some('(') => {
            //Find end parenthesis
            let mut level = 0;
            let end = s
                .chars()
                .position(|c| {
                    match (level, c) {
                        (1, ')') => return true,
                        (_, '(') => level += 1,
                        (_, ')') => level -= 1,
                        (_, _) => {}
                    };

                    false
                })
                .ok_or("No End parenthesis")?;

            //Split options
            let inner = &s[1..end];
            let mut level = 0;

            let exprs = inner.split(|c| {
                match (level, c) {
                    (_, '(') => level += 1,
                    (_, ')') => level -= 1,
                    (0, '|') => return true,
                    (_, _) => {}
                };

                false
            })
                .map(|s| parse_expressions(s))
                .collect::<Result<Vec<Expression>>>()?;

            Ok((Expression::OneOf(exprs), end + 1))
        }
        //Empty sequence
        None => Ok((Expression::Sequence(Vec::new()), 0)),
        //Sequence
        Some(_) => {
            let seq = s.chars()
                .take_while(|&c| c != '(')
                .map(|c| Direction::from_char(c))
                .collect::<Result<Vec<Direction>>>()?;

            let n = seq.len();
            Ok((Expression::Sequence(seq), n))
        }
    }
}


fn parse_expressions(s: &str) -> Result<Expression> {
    let mut s = s;
    let mut exprs = Vec::new();

    while !s.is_empty() {
        let (expr, skip) = parse_expression(s)?;
        exprs.push(expr);
        s = &s[skip..];
    }

    Ok(Expression::Concat(exprs))
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Expression::Sequence(ref dirs) => {
                for d in dirs {
                    write!(f, "{}", d.to_char())?;
                }
            }
            Expression::OneOf(choices) => {
                write!(f, "(")?;
                for (i, e) in choices.iter().enumerate() {
                    if i != 0 {
                        write!(f, "|")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")?;
            }
            Expression::Concat(exprs) => {
                for e in exprs.iter() {
                    write!(f, "{}", e)?;
                }
            }
        }

        Ok(())
    }
}


impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let first = s.chars().next().ok_or("missing ^")?;
        if first != '^' {
            return Err("wrong first".into());
        }

        let end = s.chars().last().ok_or("missing $")?;
        if end != '$' {
            return Err("wrong end".into());
        }


        let expr = parse_expressions(&s[1..s.len() - 1])?;
        Ok(Data {
            expr
        })
    }
}


impl Display for TileKind {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}