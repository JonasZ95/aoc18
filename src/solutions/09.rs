extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate linked_list;

use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::cell::RefCell;
use std::rc::Rc;
use linked_list::LinkedList;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


struct Game {
    marbles: usize,
    players: usize
}

fn parse(s: &str) -> Result<Game> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)( players; last marble is worth )(\d+)( points)$").unwrap();
    }

    let caps = RE.captures(s)
        .ok_or("Invalid Game Line")?;

    let marbles: usize = caps[3].parse()?;
    let players: usize = caps[1].parse()?;

    Ok(Game{
        marbles,
        players
    })
}

fn load(file: &str) -> Result<Game> {
    let mut r = File::open(file)?;
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    parse(&s)
}

fn offset(cur: usize, off: isize, n: usize) -> usize {
    let n = n as isize;
    let off = off%n;

    let cur = (cur as isize) + off;
    if cur < 0 {
        (n + cur)  as usize
    } else {
        cur as usize
    }
}

fn part1(g: &Game) -> Result<usize> {
    let mut circle = LinkedList::new();
    let mut cur = circle.cursor();
    let mut scores = HashMap::<usize, usize>::new();

    let mut player = 0;
    cur.insert(0);
    cur.next();

    for marble in 1..=g.marbles {
        match marble {
            m if m%23 == 0 => {
                for i in 0..8 {
                    while cur.prev().is_none() {}
                }
                let points = cur.remove().unwrap();

                *scores.entry(player)
                    .or_insert(0) += points + m;

                cur.next();

            },
            m => {
                while cur.next().is_none() {}
                cur.insert(m);
                cur.next();
            }
        };

        player = (player + 1) % g.players;

        if marble % 10000 == 0 {
            println!("m: {}/{}", marble, g.marbles);
        }
    }


    Ok(*scores.iter()
        .max_by_key(|(_, &s)| s)
        .unwrap().1)
}

fn part2(s: &Game) -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    const N: usize = 9;
    let data = load(&format!("in/0{}.txt", N))?;
    let sample = load(&format!("in/0{}_sample.txt", N))?;

    let sample1 = part1(&sample)?;
    println!("sample 1: {}", sample1);

    let new_game = Game {
        marbles: data.marbles * 100,
        players: data.players
    };

    let result1 = part1(&data)?;
    let result2 = part1(&new_game)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}