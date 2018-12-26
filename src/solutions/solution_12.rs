use Solution;
use Result;
use util::file::data_path;
use regex::Regex;
use util::file::load;
use std::str::FromStr;
use util::file::sample_path;

type Pattern = [Pot; 5];


#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Pot {
    NoPlant,
    Plant,
}

#[derive(Default)]
struct PatternMatch {
    pattern: Pattern,
    pot: Pot,
}

#[derive(Default)]
struct Data {
    state: Vec<Pot>,
    pats: Vec<(u16, Pot)>
}

impl Pot {
    fn from_char(c: char) -> Result<Pot> {
        match c {
            '.' => Ok(Pot::NoPlant),
            '#' => Ok(Pot::Plant),
            _ => Err("Unknown pot state".into())
        }
    }
}

impl Default for Pot {
    fn default() -> Self {
        Pot::NoPlant
    }
}


fn parse_pot_pattern(s: &str) -> Result<Vec<Pot>> {
    s.chars()
        .map(Pot::from_char)
        .collect()
}


impl FromStr for PatternMatch {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE_PATTERN: Regex = Regex::new(r"^([#\.]{5})( => )([#\.])$").unwrap();
        }

        let caps = RE_PATTERN.captures(s)
            .ok_or("Invalid init line")?;

        let vec_pat = parse_pot_pattern(&caps[1]).unwrap();
        let pot = Pot::from_char(caps[3].chars().next().unwrap()).unwrap();

        let mut pattern = [Pot::NoPlant; 5];
        pattern.clone_from_slice(&vec_pat);

        Ok(PatternMatch {
            pattern,
            pot,
        })
    }
}

impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
        static ref RE_INIT: Regex = Regex::new(r"^(initial state: )([#\.]*)$").unwrap();
    }

        let mut lines = s.lines();

        let init_line = lines.next()
            .ok_or("No init line")?;
        let caps = RE_INIT.captures(init_line)
            .ok_or("Invalid init line")?;
        let state = parse_pot_pattern(&caps[2]).unwrap();

        lines.next()
            .ok_or("No divide line")?;

        let patterns = lines
            .map(|s| s.parse())
            .collect::<Result<Vec<PatternMatch>>>()?;

        Ok(Data::new(state, patterns))
    }
}

impl Data {
    fn new(state: Vec<Pot>, patterns: Vec<PatternMatch>) -> Data {
        let mut pats: Vec<(u16, Pot)> = patterns.iter()
            .map(|p| (Data::pattern_to_num(&p.pattern) as u16, p.pot))
            .collect();

        pats.sort_by_key(|(k, _)| *k);


        Data {
            state,
            pats
        }
    }
    fn find_match(&self, data: Pattern) -> Option<Pot> {
        let num = Data::pattern_to_num(&data) as u16;
        let ix = self.pats
            .binary_search_by_key(&num, |(k,_)| *k);

        match ix {
            Ok(ix) => Some(self.pats[ix].1),
            Err(_) => None
        }
    }

    fn pot_to_num(pot: Pot) -> u32 {
        match pot {
            Pot::Plant => 1,
            Pot::NoPlant => 0
        }
    }

    fn pattern_to_num(data: &Pattern) -> u32 {
        Data::pot_to_num(data[0]) |
            Data::pot_to_num(data[1]) << 1 |
            Data::pot_to_num(data[2]) << 2 |
            Data::pot_to_num(data[3]) << 3 |
            Data::pot_to_num(data[4]) << 4
    }

    fn run(&self, gens: usize) -> isize {
        let mut buffer = self.state.clone();
        let mut back_buffer = self.state.clone();

        let mut left_index = 0;

        for _ in 1..=gens {
            let n = buffer.len();

            let first = buffer.iter().position(|&p| p == Pot::Plant).unwrap();
            let last = buffer.iter().rposition(|&p| p == Pot::Plant).unwrap();


            let right_left = (n - last - 1) as isize;
            let left_fill = (4-(first as isize)).max(0) as usize;
            let right_fill = (4 - right_left).max(0) as usize;

            let m = n + left_fill + right_fill;
            left_index += left_fill;
            back_buffer.resize(m, Pot::NoPlant);

            for i in 0..m {
                let i = i as isize;
                let p = get_pattern(&buffer, i - (left_fill as isize) - 2);
                let c = self.find_match(p)
                    .unwrap_or(Pot::NoPlant);

                back_buffer[i as usize] = c;
            }


            std::mem::swap(&mut buffer, &mut back_buffer);
        }

        let sum: isize = buffer.iter()
            .enumerate()
            .map(|(i, p)| {
                match p {
                    Pot::NoPlant => 0,
                    Pot::Plant => {
                        (i as isize) - (left_index as isize)
                    }
                }
            })
            .sum();

        sum
    }
}

#[derive(Default)]
pub struct Solution12 {
    data: Data,
    sample: Data,
}

fn get_pattern(data: &[Pot], ix: isize) -> Pattern {
    let mut pat = [Pot::NoPlant; 5];
    match ix {
        ix if ix < 0  => {
            let fill = (5+ix).max(0) as usize;
            let off = ix.abs() as usize;
            for i in 0..fill {
                pat[off+i] = data[i];
            }

        },
        ix if (ix as usize)+5 > data.len() => {
            let fill = ((data.len() as isize) - ix).max(0) as usize;
            let ix = ix as usize;
            for i in 0..fill {
                pat[i] = data[ix+i];
            }
        },
        ix => {
            let ix = ix as usize;
            pat.copy_from_slice(&data[ix..ix+5]);
        }
    };

    pat
}

impl Solution for Solution12 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(12))?;
        self.data = s.parse()?;
        let s = load(&sample_path(12))?;
        self.sample = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let sample_sum = self.sample.run(20);
        let sum = self.data.run(20);
        println!("sample-result: {}, result: {}", sample_sum, sum);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut n: usize = 500;
        for _ in 0..3 {
            println!("result[{}] : {}", n, self.data.run(n));
            n *= 10;
        }
        Ok(())
    }
}