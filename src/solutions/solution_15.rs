use Solution;
use Result;
use std::str::FromStr;
use util::file::data_path;
use util::mat2::Mat2;
use std::usize;
use util::mat2::Pos;
use std::collections::VecDeque;
use util::file::sample_path;
use util::file::load;
use std::fmt::Display;
use std::fmt::Formatter;

/*
- wall: #, open cavern: ., goblin: G, elf: E
- round: turn(attack if in range, move if not)
- only vert/hor attack and move
- order per starting position( y prio >> x)
- end if no targets remain
*/

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum UnitKind {
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Unit {
    kind: UnitKind,
    hp: u64,
    atk: u64,
    id: usize,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum GridKind {
    Wall,
    Open,
}

#[derive(Debug, Clone)]
struct Grid {
    unit: Option<Unit>,
    kind: GridKind,
}


#[derive(Debug, Clone, Default)]
struct Cave {
    grids: Mat2<Grid>,
    w: usize,
    h: usize
}

impl Unit {
    fn from_char(c: char, id: usize) -> Result<Unit> {
        let kind = match c {
            'G' => UnitKind::Goblin,
            'E' => UnitKind::Elf,
            _ => return Err("Invalid unit kind".into())
        };

        Ok(Unit {
            kind,
            id,
            hp: 200,
            atk: 3,
        })
    }

    fn can_attack(&self, other: &Unit) -> bool {
        self.kind != other.kind
    }

    fn to_char(&self) -> char {
        let n = (self.id & 0xf) as u8;

        let base = match self.kind {
            UnitKind::Goblin => b'A',
            UnitKind::Elf => b'0'
        };
        let n = base + n;
        n as char
    }

    fn damage(&mut self, dmg: u64) -> bool {
        let atk = dmg.min(self.hp);
        self.hp -= atk;
        self.hp == 0
    }

    fn opponent_kind(&self) -> UnitKind {
        match self.kind {
            UnitKind::Goblin => UnitKind::Elf,
            UnitKind::Elf => UnitKind::Goblin
        }
    }
}


impl Default for Grid {
    fn default() -> Self {
        Grid {
            kind: GridKind::Wall,
            unit: None,
        }
    }
}

impl Grid {
    fn from_char(c: char, id: usize) -> Result<Grid> {
        let (unit, kind) = match c {
            '.' => (None, GridKind::Open),
            '#' => (None, GridKind::Wall),
            c => {
                let unit = Unit::from_char(c, id)?;
                (Some(unit), GridKind::Open)
            }
        };

        Ok(Grid {
            unit,
            kind,
        })
    }

    fn to_char(&self) -> char {
        match (&self.unit, &self.kind) {
            (Some(u), _) => u.to_char(),
            (None, GridKind::Wall) => '#',
            (None, GridKind::Open) => '.'
        }
    }

    fn is_blocked(&self) -> bool {
        self.kind == GridKind::Wall || self.unit.is_some()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Cave {
    fn new(h: usize, w: usize) -> Cave {
        Cave {
            grids: Mat2::new(h, w),
            h,
            w
        }
    }

    fn elfs(&self) -> usize {
        self.units()
            .filter(|u| u.kind == UnitKind::Elf)
            .count()
    }

    fn winner(&self) -> Option<UnitKind> {
        let (mut elfs, mut gobs) = (0 , 0);
        for u in self.units() {
            match u.kind {
                UnitKind::Elf => elfs += 1,
                UnitKind::Goblin => gobs += 1
            };
        }

        match (elfs, gobs) {
            (0, _) => Some(UnitKind::Goblin),
            (_, 0) => Some(UnitKind::Elf),
            _ => None
        }
    }

    fn is_done(&self) -> bool {
        self.winner().is_some()
    }


    fn upgrade_elf_weapons(&mut self, atk: u64) {
        for p in self.grids.coords() {
            let g = &mut self.grids[p];
            if let Some(ref mut unit) = g.unit {
                if unit.kind == UnitKind::Elf {
                    unit.atk = atk;
                }
            }
        }
    }

    fn neighbors(&self, p: Pos) -> impl Iterator<Item=Pos> + '_ {
        [(0, -1), (-1, 0), (1, 0), (0, 1)].iter()
            .flat_map(move |off| p.offset(*off))
            .filter(move |&p| self.grids.has_pos(p))
    }


    fn units(&self) -> impl Iterator<Item=&Unit> + '_ {
        self.grids.grids()
            .filter_map(|g| match &g.unit {
                Some(ref u) => Some(u),
                None => None
            })
    }

    fn hp_sum(&self) -> u64 {
        self.units().map(|u| u.hp).sum()
    }

    fn move_unit(&mut self, from: Pos, to: Pos) {
        let unit = self.grids[from]
            .unit.take().unwrap();

        self.grids[to].unit = Some(unit);
    }

    fn attack(&mut self, p: Pos, enemy_pos: Pos) {
        let atk = self.grids[p]
            .unit.as_ref().unwrap()
            .atk;

        let enemy = &mut self.grids[enemy_pos];
        if enemy.unit.as_mut().unwrap().damage(atk) {
            enemy.unit.take();
        }
    }


    fn update(&mut self) {
        let units: Vec<_> = self.grids.coords()
            .map(|p| (p, &self.grids[p]))
            .filter(|(_, g)| g.unit.is_some())
            .map(|(p, g)| (p, g.unit.as_ref().unwrap().id))
            .collect();

        for (p, _) in units {
            //Check if unit just died
            if self.grids[p].unit.is_none() {
                continue
            }

            //Find closest enemy
            match self.find_close_enemy(p) {
                Some(enemy_pos) => {
                    self.attack(p, enemy_pos);
                },
                None => {
                    if let Some(next_pos) = self.find_nearest_enemy(p) {
                        self.move_unit(p, next_pos);

                        if let Some(enemy_pos) = self.find_close_enemy(next_pos) {
                            self.attack(next_pos, enemy_pos);
                        }
                    }
                }
            }
        }
    }

    fn find_close_enemy(&self, p: Pos) -> Option<Pos> {
        let unit = self.grids[p]
            .unit
            .as_ref()
            .unwrap();

        self.neighbors(p)
            .filter_map(|p| {
                match &self.grids[p] {
                    Grid { unit: Some(other), .. } => {
                        if unit.can_attack(&other) {
                            Some((p, other.id, other.hp))
                        } else {
                            None
                        }
                    }
                    _ => None
                }
            })
            .min_by_key(|(_, _, hp)| *hp)
            .map(|(t, _, _)| t)
    }

    fn find_nearest_enemy(&mut self, p: Pos) -> Option<Pos> {
        let mut search = Mat2::new(self.h, self.w);

        let g = &self.grids[p];
        let enemy = g.unit.as_ref().unwrap().opponent_kind();

        let (mut min_id, mut pos, mut min_d) = (usize::MAX, Pos::new(0, 0), usize::MAX);

        let mut q = VecDeque::new();
        q.push_back((1, p));
        search[p] = (p, 1);

        while let Some((d, p)) = q.pop_front() {
            if d > min_d {
                break
            }

            for n in self.neighbors(p) {
                match &self.grids[n] {
                    Grid { unit: Some(Unit { kind,  .. }), .. } if *kind == enemy => {
                        let idx = n.x + self.h * n.y;
                        if idx < min_id && d < min_d {
                            search[n] = (p, d + 1);
                            min_id = idx;
                            min_d = d+1;
                            pos = n;
                        }
                    }
                    g if !g.is_blocked() && search[n].1 == 0 => {
                        q.push_back((d+1, n));
                        search[n] = (p, d + 1);
                    }
                    _ => {
                        //Ignore neighbor If the field is blocked
                    }
                };
            }
        }

        if min_id == usize::MAX {
            return None;
        }


        let mut last_pos = pos;
        loop {
            let (last, d) = search.get(last_pos).unwrap();
            if *d == 2 {
                return Some(last_pos);
            }
            last_pos = *last;
        }
    }

    fn run_to_end(&mut self) -> (usize, UnitKind) {
        let mut r = 0;

        while !self.is_done() {
            self.update();
            r +=1;
        }

        (r, self.winner().unwrap())
    }
}

impl FromStr for Cave {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let h = s.lines().count();
        let w = s.lines().next().unwrap().len();

        let mut cave = Cave::new(h, w);
        let mut id = 1;
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let grid = Grid::from_char(c, id)?;
                if grid.unit.is_some() {
                    id += 1;
                }

                cave.grids[Pos::new(x, y)] = grid;
            }
        }

        Ok(cave)
    }
}


#[derive(Default)]
pub struct Solution15 {
    data: Cave,
    sample: Cave
}
fn part1(g: &mut Cave) -> u64 {
    let (r, _) = g.run_to_end();

    for u in g.units() {
        println!("unit {:?}", u);
    }
    let hp: u64 = g.hp_sum();
    println!("rounds: {}, hp: {}", r, hp);
    (r as u64) * hp
}

fn part2(g: &mut Cave) -> u64 {
    let elfs = g.elfs();
    for i in 4.. {
        let mut g = g.clone();
        g.upgrade_elf_weapons(i);
        let (r, winner) = g.run_to_end();

        if winner == UnitKind::Elf && elfs == g.elfs() {
            println!("atk: {}, e: {}, ee: {}", i, elfs, g.elfs());
            let hp: u64 = g.hp_sum();
            return ((r as u64) - 1) * hp;
        }
    }

    0

}

impl Solution for Solution15 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(15))?;
        self.data = s.parse()?;

        let s = load(&sample_path(15))?;
        self.sample = s.parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let sample1 = part1(&mut self.sample.clone());
        println!("s1: {}", sample1);

        let result1 = part1(&mut self.data.clone());
        println!("result1: {}", result1);

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let sample1 = part2(&mut self.sample.clone());
        println!("s2: {}", sample1);

        let result1 = part2(&mut self.data.clone());
        println!("result2: {}", result1);
        Ok(())
    }
}