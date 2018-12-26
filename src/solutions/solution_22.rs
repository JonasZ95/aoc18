use Solution;
use Result;
use util::mat2::Pos;
use util::mat2::Mat2;
use std::fmt::Display;
use std::fmt::Formatter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Debug)]
enum RegionKind {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
enum Equipment {
    Neither,
    Torch,
    ClimbingGear,
}


#[derive(Default, Clone)]
struct Region {
    geo_index: usize,
    depth: usize,
}

#[derive(Default)]
pub struct Data {
    depth: usize,
    target: Pos,
}

#[derive(Default)]
pub struct Solution22 {
    data: Data,
    sample: Data,
}

impl Default for RegionKind {
    fn default() -> Self {
        RegionKind::Rocky
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let c = match self.kind() {
            RegionKind::Rocky => '.',
            RegionKind::Wet => '=',
            RegionKind::Narrow => '|'
        };

        write!(f, "{}", c)
    }
}

impl Region {
    fn kind(&self) -> RegionKind {
        match self.erosion_level() % 3 {
            0 => RegionKind::Rocky,
            1 => RegionKind::Wet,
            _ => RegionKind::Narrow
        }
    }

    fn erosion_level(&self) -> usize {
        (self.geo_index + self.depth) % 20183
    }

    fn risk_level(&self) -> usize {
        match self.kind() {
            RegionKind::Rocky => 0,
            RegionKind::Wet => 1,
            RegionKind::Narrow => 2
        }
    }

    fn can_enter(&self, e: Equipment) -> bool {
        match (self.kind(), e) {
            (RegionKind::Rocky, Equipment::Neither) => false,
            (RegionKind::Narrow, Equipment::ClimbingGear) => false,
            (RegionKind::Wet, Equipment::Torch) => false,
            _ => true
        }
    }

    fn swap_equipment(&self,e: Equipment) -> Option<Equipment> {
        use self::RegionKind::*;
        use self::Equipment::*;

        match (self.kind(), e) {
            (Rocky, Torch) => Some(ClimbingGear),
            (Rocky, ClimbingGear) => Some(Torch),

            (Narrow, Torch) => Some(Neither),
            (Narrow, Neither) => Some(Torch),

            (Wet, Neither) => Some(ClimbingGear),
            (Wet, ClimbingGear) => Some(Neither),

            _ => None
        }
    }
}

impl Data {
    fn build_grid(&self, extra: usize) -> Mat2<Region> {
        let h = (self.target.y + 1) * extra;
        let w = (self.target.x + 1) * extra;
        let mut m = Mat2::<Region>::new(h, w);

        for y in 0..h {
            for x in 0..w {
                let p = Pos::new(x, y);

                let geo_index = match (x, y) {
                    (0, 0) => 0,
                    (_, _) if self.target == p => 0,
                    (x, 0) => x * 16807,
                    (0, y) => y * 48271,
                    (x, y) => m[Pos::new(x - 1, y)].erosion_level() * m[Pos::new(x, y - 1)].erosion_level()
                };

                let mut r = &mut m[p];
                r.geo_index = geo_index;
                r.depth = self.depth;
            }
        }

        m
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QEntry {
    d: usize,
    p: Pos,
    e: Equipment,
}


impl PartialOrd for QEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.d.partial_cmp(&self.d)
    }
}

impl Ord for QEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
    }
}


fn part1(d: &Data) -> usize {
    let grid = d.build_grid(1);

    println!("grid:\n{}", grid);

    let risks = grid
        .grids()
        .map(|r| r.risk_level())
        .sum();

    risks
}

fn part2(data: &Data) -> Result<usize> {
    let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let grid = data.build_grid(10);
    let mut dists = [
        Mat2::<usize>::new_with(grid.height(), grid.width(), usize::MAX),
        Mat2::<usize>::new_with(grid.height(), grid.width(), usize::MAX),
        Mat2::<usize>::new_with(grid.height(), grid.width(), usize::MAX)
    ];

    let dist_ix = |e| match e {
        Equipment::Neither => 0,
        Equipment::Torch => 1,
        Equipment::ClimbingGear => 2,
    };


    // Dijkstra
    let mut q = BinaryHeap::new();
    let p = Pos::new(0, 0);
    q.push(QEntry {
        d: 0,
        p,
        e: Equipment::Torch,
    });

    while let Some(QEntry{d, p, e}) = q.pop() {
        let mut dist = &mut dists[dist_ix(e)];
        if dist[p] <= d {
            continue;
        }

        if p == data.target && e == Equipment::Torch {
            return Ok(d);
        }

        let r = &grid[p];

        q.push(QEntry {
            d: d + 7,
            p,
            e: r.swap_equipment(e).unwrap(),
        });

        dist[p] = d;

        for n in ns.iter()
            .filter_map(|&n| p.offset(n))
            .filter(|&n| grid.has_pos(n)) {

            if grid[n].can_enter(e) {
                q.push(QEntry {
                    d: d + 1,
                    p: n,
                    e,
                });
            }
        }
    }


    Err("No path to target".into())
}

impl Solution for Solution22 {
    fn init(&mut self) -> Result<()> {
        self.data = Data {
            depth: 4002,
            target: Pos::new(5, 746),
        };

        self.sample = Data {
            depth: 510,
            target: Pos::new(10, 10),
        };

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample);
        println!("sample1: {}", result);

        let result = 0;//part1(&self.data);
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