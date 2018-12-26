use Solution;
use Result;
use util::file::data_path;
use util::file::sample_path;
use util::file::load;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashSet;


#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    w: i64,
    x: i64,
    y: i64,
    z: i64
}

///Union Find with Path compression
pub struct UnionFind {
    rank: Vec<usize>,
    parent: Vec<Option<usize>>,

}

#[derive(Default)]
pub struct Data {
    points: Vec<Vec4>
}

#[derive(Default)]
pub struct Solution25 {
    data: Data,
    sample: Data
}


impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            rank: vec![1; n],
            parent: vec![None; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        match self.parent[x] {
            None => x,
            Some(p) => {
                let p = self.find(p);
                self.parent[x] = Some(p);
                p
            }
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        let x_rank = self.rank[x];
        let y_rank = self.rank[y];

        match x_rank.cmp(&y_rank) {
            Ordering::Equal => {
                self.parent[y] = Some(x);
                self.rank[x] += 1;
            },
            Ordering::Greater => {
                self.parent[y] = Some(x);
            },
            Ordering::Less => {
                self.parent[x] = Some(y);
            }
        }
    }

    fn sets(&mut self) -> usize {
        let n = self.parent.len();
        let s: HashSet<_> = (0..n)
            .map(|i| self.find(i))
            .collect();

        s.len()
    }
}


impl Vec4 {
    fn dist(&self, other: &Vec4) -> usize {
        let d = |p: i64, q: i64| (p - q).abs();

        (d(self.w, other.w) +
            d(self.x, other.x) +
            d(self.y, other.y) +
            d(self.z, other.z)) as usize
    }
}


fn part1(d: &Data) -> Result<usize> {
    let mut uf = UnionFind::new(d.points.len());

    for (i, p) in d.points.iter().enumerate() {
        for (j, other) in d.points.iter().take(i).enumerate() {
            if p.dist(&other) <= 3 {
                uf.union(i, j)
            }
        }
    }


    Ok(uf.sets())
}

fn part2(_: &Data) -> Result<usize> {
    Ok(0)
}

impl Solution for Solution25 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(25))?;
        self.data = s.parse()?;

        let s = load(&sample_path(25))?;
        self.sample = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample)?;
        println!("sample1: {}", result);

        let result =  part1(&self.data)?;
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

impl FromStr for Vec4 {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(',');

        let w = split.next()
            .ok_or("No w coord")?
            .parse()?;

        let x = split.next()
            .ok_or("No x coord")?
            .parse()?;

        let y = split.next()
            .ok_or("No y coord")?
            .parse()?;

        let z = split.next()
            .ok_or("No z coord")?
            .parse()?;


        Ok(Vec4{
            w,
            x,
            y,
            z
        })
    }
}

impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let points = s.lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Vec4>>>()?;

        Ok(Data{
            points
        })
    }
}