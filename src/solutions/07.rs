extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate id_tree;
extern crate petgraph;

use std::fs::File;
use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use petgraph::prelude::*;
use petgraph::visit::Topo;
use std::collections::HashSet;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

struct Order(char, char);

fn parse_order(s: &str) -> Result<Order> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(Step )(\w)( must be finished before step )(\w)( can begin.)$").unwrap();
    }

    let caps = RE.captures(s)
        .ok_or("Invalid Order Line")?;

    let before = caps[2].chars().next().unwrap();
    let after = caps[4].chars().next().unwrap();

    Ok(Order(before, after))
}

fn parse_orders(file: &str) -> Result<Vec<Order>> {
    let r = File::open(file)?;
    let r = BufReader::new(r);
    let mut s = Vec::new();

    for l in r.lines() {
        let l = l?;
        s.push(parse_order(&l)?);
    }

    s.sort_by_key(|o| o.0);

    Ok(s)
}

fn build_graph(orders: &[Order]) -> DiGraph<char, ()> {
    let mut keys: HashSet<char> = HashSet::<char>::new();
    for o in orders.iter() {
        keys.insert(o.0);
        keys.insert(o.1);
    }

    let mut node_idxs = HashMap::new();
    let mut deps = DiGraph::<char, ()>::new();
    for &key in keys.iter() {
        node_idxs.insert(key, deps.add_node(key));
    }

    for o in orders {
        let before = node_idxs.get(&o.0).unwrap();
        let after = node_idxs.get(&o.1).unwrap();
        deps.add_edge(*before, *after, ());
    }

    deps
}


fn part1(s: &[Order]) -> Result<String> {
    //Build tree

    let mut rel = BTreeMap::<char, BTreeSet<char>>::new();
    let mut refs = BTreeMap::<char, usize>::new();

    for o in s.iter() {
        rel.entry(o.0)
            .or_insert(BTreeSet::new())
            .insert(o.1);

        rel.entry(o.1)
            .or_insert(BTreeSet::new());

        *refs.entry(o.1)
            .or_insert(0) += 1;

        refs.entry(o.0)
            .or_insert(0);
    }

    let mut q: BTreeSet<char> = refs.iter()
        .filter(|(_, &n)| n == 0)
        .map(|(&c, _)| c)
        .collect();

    let mut result = String::new();

    loop {
        //find key
        let k =
            {
                let k = q.iter()
                    .find(|&k| *refs.get(k).unwrap() == 0);
                match k {
                    Some(k) => *k,
                    None => return Ok(result)
                }
            };

        q.remove(&k);

        if result.chars().find(|&c| c == k).is_some() {
            continue;
        }

        for &after in rel.get(&k).unwrap().iter().rev() {
            q.insert(after);
            *refs.get_mut(&after).unwrap() -= 1;
        }

        result.push(k);
    }
}

fn part2(s: &[Order], seq: &str) -> Result<usize> {
    let mut rel = BTreeMap::<char, BTreeSet<char>>::new();
    let mut refs = BTreeMap::<char, usize>::new();

    for o in s.iter() {
        rel.entry(o.0)
            .or_insert(BTreeSet::new())
            .insert(o.1);

        rel.entry(o.1)
            .or_insert(BTreeSet::new());

        *refs.entry(o.1)
            .or_insert(0) += 1;

        refs.entry(o.0)
            .or_insert(0);
    }

    let mut tasks: Vec<char> = seq.chars().collect();
    let mut workers: [(usize, Option<char>); 5] = [(0, None); 5];
    let mut tick = 0;

    loop {
        //Work
        for worker in workers.iter_mut()
            .filter(|w| w.0 != 0) {

            worker.0 -= 1;
            if worker.0 == 0 {
                let work = worker.1.take().unwrap();

                for after in rel.get(&work).unwrap().iter() {
                    *refs.get_mut(&after).unwrap() -= 1;
                }
            }
        }


        loop {
            if let Some((i, &task)) = tasks.iter()
                .enumerate()
                .filter(|(_, task)| *refs.get(task).unwrap() == 0)
                .next() {

                if let Some(worker) = workers.iter_mut().find(|(t, _)| *t == 0) {
                    worker.1 = Some(task);
                    worker.0 = (task as u8 - b'A') as usize + 61;

                    tasks.remove(i);
                } else {
                    break
                }
            } else {
                break
            }
        }

        if workers.iter().find(|(t, _)| *t != 0).is_none() {
            return Ok(tick);
        }

        tick += 1;
    }

}

fn main() -> Result<()> {
    let data = parse_orders("in/07.txt")?;
    let sample = parse_orders("in/07_sample.txt")?;

    let result1 = part1(&sample)?;
    let result2 = part2(&sample, &result1)?;
    println!("SAMPLE - #1: {}, 2: {}", result1, result2);

    let gr = build_graph(&data);
    let mut topo = Topo::new(&gr);
    while let Some(node) = topo.next(&gr) {
        println!("n: {}", gr.node_weight(node).unwrap())
    }


    let result1 = part1(&data)?;
    let result2 = part2(&data, &result1)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}