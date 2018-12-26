use Solution;
use Result;
use util::file::data_path;
use util::file::sample_path;
use util::file::load;
use std::str::FromStr;
use regex::Regex;
use util::file::in_path;
use std::fs::File;
use std::io::Write;


#[derive(Default, Debug)]
pub struct Data {
    nano_bots: Vec<NanoBot>
}


#[derive(Debug, PartialOrd, PartialEq)]
pub struct NanoBot {
    x: i64,
    y: i64,
    z: i64,

    r: usize,
}

#[derive(Default)]
pub struct Solution23 {
    data: Data,
    sample: Data,
}

impl FromStr for NanoBot {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE_NANOBOT: Regex = Regex::new(r"^(pos=<)(-?\d*)(,)(-?\d*)(,)(-?\d*)(>,\sr=)(\d*)$").unwrap();
        }

        let caps = RE_NANOBOT.captures(s)
            .ok_or("Invalid nano bot Line")?;

        let x = caps[2].parse()?;
        let y = caps[4].parse()?;
        let z = caps[6].parse()?;

        let r = caps[8].parse()?;

        Ok(NanoBot {
            x,
            y,
            z,
            r,
        })
    }
}


impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let bots = s.lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<NanoBot>>>()?;

        Ok(Data {
            nano_bots: bots
        })
    }
}

impl NanoBot {
    fn dist(&self, other: &NanoBot) -> usize {
        let d = |p: i64, q: i64| (p - q).abs();

        (d(self.x, other.x) +
            d(self.y, other.y) +
            d(self.z, other.z)) as usize
    }


    fn is_in_range(&self, other: &NanoBot) -> bool {
        self.dist(other) <= self.r
    }
}

fn part1(d: &Data) -> usize {
    let max_r = d.nano_bots.iter()
        .max_by_key(|n| n.r)
        .unwrap();

    d.nano_bots.iter()
        .filter(|&n| max_r.is_in_range(n))
        .count()
}

fn part2(d: &Data, is_sample: bool) -> Result<()> {
    static SPLIT: &str = "+++";
    let tmpl = load(&format!("{}/23_z3.tmpl", in_path()))?;

    let split = tmpl.find(SPLIT).unwrap();

    let head = &tmpl[..split];
    let tail = &tmpl[split+SPLIT.len()+1..];


    let mut  out = File::create(format!("23_z3_{}.z3", is_sample as u8))?;
    out.write_all(head.as_bytes())?;

    // (if (<= (dist x {] y {} z {}) {}) 1 0)
    for n in  d.nano_bots.iter() {
        out.write_fmt(format_args!("(if (<= (dist x {x} y {y} z {z}) {r}) 1 0)\n",
                                   x=n.x, y=n.y, z=n.z, r=n.r))?;
    }

    out.write_all(tail.as_bytes())?;

    Ok(())
}

impl Solution for Solution23 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(23))?;
        self.data = s.parse()?;

        let s = load(&sample_path(23))?;
        self.sample = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample);
        println!("sample1: {}", result);

        let result = part1(&self.data);
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        part2(&self.sample, true)?;

        part2(&self.data, false)?;
        Ok(())
    }
}