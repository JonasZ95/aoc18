use Solution;
use Result;
use util::file::data_path;
use util::file::sample_path;
use util::file::load;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashSet;
use std::cmp::Reverse;

#[derive(Default, Debug)]
pub struct Data {
    infection: Vec<Group>,
    immune_system: Vec<Group>,
}


#[derive(Debug, Clone)]
pub struct Group {
    units: usize,
    hp: usize,
    atk_damage: usize,
    atk_type: String,
    initiative: usize,

    weakness: HashSet<String>,
    immunities: HashSet<String>,
}

#[derive(Default)]
pub struct Solution24 {
    data: Data,
    sample: Data,
}


/*fn get_2_mut<T>(v: &mut Vec<T>, i: usize, j: usize) -> (&mut T, &mut T) {
    if i == j {
        panic!("Same indices for 2 mut");
    }

    unsafe  {
        //v.get
        let x = &mut v[i];
        let y = &mut v[j];

        (x, y)
    }
}*/

impl Group {
    fn attack(&self, target: &Group) -> usize {
        let dmg = self.effective_power();
        let total_dmg = dmg * target.type_factor(&self.atk_type);
        let destroyed_units = (total_dmg / target.hp).min(target.units);

        destroyed_units
    }

    fn effective_power(&self) -> usize {
        self.atk_damage * self.units
    }


    fn type_factor(&self, atk_type: &str) -> usize {
        if self.immunities.contains(atk_type) {
            0
        } else if self.weakness.contains(atk_type) {
            2
        } else {
            1
        }
    }

    fn possible_damage(&self, atk_damage: usize, atk_type: &str) -> usize {
        self.type_factor(atk_type) * atk_damage
    }

    fn is_dead(&self) -> bool {
        self.units == 0
    }
}

fn simulate_combat(d: &Data, boost: usize) -> (bool, bool, usize) {
    type Groups = Vec<(bool, Group)>;

    let mut groups: Groups = d.immune_system.iter()
        .map(|g| {
            let mut g = g.clone();
            g.atk_damage += boost;
            (true, g)
        })
        .chain(
            d.infection.iter()
                .map(|g| (false, g.clone()))
        )
        .collect();

    let mut attacked_by = vec![None; groups.len()];
    let mut targets = Vec::new();


    loop {
        groups.sort_by_key(|(_, g)| Reverse((g.effective_power(), g.initiative)));

        if let Some(first_dead) = groups.iter().position(|(_, g)| g.is_dead()) {
            groups.truncate(first_dead);
        }

        let imms = groups.iter()
            .filter(|(imm, _)| *imm)
            .count();

        if imms == 0 || imms == groups.len() {
            let n = groups.iter()
                .map(|(_, g)| g.units)
                .sum();
            return (true, imms != 0, n);
        }


        //find targets
        for i in 0..groups.len() {
            attacked_by[i] = None;
        }
        targets.clear();

        for (i, (imm, g)) in groups.iter().enumerate() {
            let ad = g.atk_damage * g.units;
            let at = &g.atk_type;

            //Find target
            let target = groups.iter()
                .enumerate()
                .filter(|(_, (is_imm, _))| is_imm != imm)
                .filter(|(i, _)| attacked_by[*i].is_none())
                .max_by_key(|(_, (_, g))| (g.possible_damage(ad, &at), g.effective_power(), g.initiative));

            //If there is a target make a link
            if let Some((j, _)) = target {
                attacked_by[j] = Some(i);


                targets.push((i, j));
            }
        }

        //attack targets
        let mut destroyed = 0;
        targets.sort_by_key(|&(attacker, _)| Reverse(groups[attacker].1.initiative));
        for &(attacker, target) in targets.iter() {
            let destroyed_units = {
                let atk = &groups[attacker];
                let target = &groups[target];

                atk.1.attack(&target.1)
            };

            groups[target].1.units -= destroyed_units;
            destroyed += destroyed_units;
        }

        if destroyed == 0 {
            return (false, false, 0);
        }
    }
    /*
       round:
        1. target selection
            choose a group by dec. eff power, init
            select order most dmg a->d, highest eff power, init
            d group can be only choosen once
            a group (0..1)-> d group
            d group (0..1) -> a group
        2. attacking
            attack in dec init order
            cannot attack if group has no units
    */
}

fn part1(d: &Data) -> usize {
    simulate_combat(&d, 0).2
}

fn part2(d: &Data) -> usize {
    const N: usize = 10000;

    let (mut l, mut r) = (1, N);
    loop {

        if l > r {
            panic!("no boost found");
        }

        let m = l + (r - l) / 2;

        let (has_winner, imm_won, rem) = simulate_combat(&d, m);
        //increase boost
        if !has_winner || !imm_won {
            l = m+1;
            continue;
        }

        let (_, imm_won, _) = simulate_combat(&d, m-1 );
        //decrease boost
        if imm_won {
            r = m - 1;
            continue;
        }

        break rem
    }
}

impl Solution for Solution24 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(24))?;
        self.data = s.parse()?;

        let s = load(&sample_path(24))?;
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
        let result = part2(&self.sample);
        println!("sample2: {}", result);

        let result = part2(&self.data);
        println!("result2: {}", result);
        Ok(())
    }
}

impl FromStr for Data {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut data = Data {
            immune_system: Vec::new(),
            infection: Vec::new(),
        };

        let mut infection = false;

        for (i, l) in s.lines().enumerate() {
            match (i, l) {
                (0, "Immune System:") => continue,
                (0, _) => return Err("No immune system".into()),

                (_, "") => continue,

                (_, "Infection:") => infection = true,

                (_, l) => {
                    let g = l.parse()?;
                    if infection {
                        data.infection.push(g);
                    } else {
                        data.immune_system.push(g);
                    }
                }
            }
        }

        Ok(data)
    }
}

impl FromStr for Group {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE_GROUP: Regex = Regex::new(r"^(?P<units>\d+) units each with (?P<hp>\d+) hit points (\((?P<ts>[^\)]*)\) )?with an attack that does (?P<ad>\d+) (?P<at>\w+) damage at initiative (?P<init>\d+)$").unwrap();

            static ref RE_WEAK: Regex = Regex::new(r"weak to ((\w+(, )?)+)").unwrap();
            static ref RE_IMM: Regex = Regex::new(r"immune to ((\w+(, )?)+)").unwrap();
        }

        let re: &Regex = &RE_GROUP;

        let caps = re.captures(s)
            .ok_or("Invalid group line")?;

        let units = caps["units"].parse()?;
        let hp = caps["hp"].parse()?;
        let initiative = caps["init"].parse()?;

        let atk_damage = caps["ad"].parse()?;
        let atk_type = caps["at"].to_string();


        let type_chart = caps.name("ts")
            .map(|ts| ts.as_str())
            .unwrap_or("");

        let weakness = match RE_WEAK.captures(type_chart) {
            Some(caps) => caps[1].split(", ")
                .map(|s| s.to_string())
                .collect(),
            None => HashSet::new()
        };

        let immunities = match RE_IMM.captures(type_chart) {
            Some(caps) => caps[1].split(", ")
                .map(|s| s.to_string())
                .collect(),
            None => HashSet::new()
        };

        Ok(Group {
            units,
            hp,
            initiative,
            atk_damage,
            atk_type,

            weakness,
            immunities,
        })
    }
}