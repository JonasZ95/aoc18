extern crate id_tree;

use std::fs::File;
use std::usize;
use std::io::Read;
use id_tree::NodeId;
use id_tree::InsertBehavior;
use std::collections::VecDeque;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

type Tree = id_tree::Tree<NodeData>;

struct NodeData {
    meta: Vec<u64>,
    id: usize
}

#[derive(Debug)]
enum State {
    NewNode(NodeId),
    Meta(NodeId, usize)
}

fn load_tree(file: &str) -> Result<Tree> {
    let mut r = File::open(file)?;
    let mut s = String::new();
    r.read_to_string(&mut s)?;

    let mut tree = Tree::new();
    let mut node_stack = VecDeque::<State>::new();

    let mut it = s.split_whitespace()
        .map(|s| s.parse::<u64>());

    let mut id = 1;
    let node = NodeData {
        meta: Vec::new(),
        id
    };
    let node = tree.insert(id_tree::Node::new(node),InsertBehavior::AsRoot)?;
    node_stack.push_back(State::NewNode(node));

    while let Some(state) = node_stack.pop_front() {
        match state {
            State::NewNode(node_id) => {
                let childs = it.next().unwrap()?;
                let meta_len = it.next().unwrap()?;

                node_stack.push_front(State::Meta(node_id.clone(),meta_len as usize));

                for _i in 0..childs {
                    id += 1;
                    let node = NodeData {
                        meta: Vec::new(),
                        id
                    };
                    tree.insert(id_tree::Node::new(node), InsertBehavior::UnderNode(&node_id))?;
                }

                for child in tree.get(&node_id).unwrap().children().iter().rev() {
                    node_stack.push_front(State::NewNode(child.clone()));
                }
            },
            State::Meta(node_id, meta_len) => {
                let node = tree.get_mut(&node_id)?;
                let data = node.data_mut();

                for _i in 0..meta_len {
                    data.meta.push(it.next().unwrap()?);
                }


            }
        }
    }

    Ok(tree)
}

fn part1(t: &Tree) -> Result<u64> {
    let r = t.root_node_id().unwrap();
    Ok(t.traverse_level_order(r)?
        .flat_map(|n| n.data().meta.iter())
        .sum())
}

fn part2(t: &Tree) -> Result<u64> {
    let mut ids = HashMap::<usize, u64>::new();
    let r = t.root_node_id().unwrap();

    for node in t.traverse_post_order(r).unwrap() {
        let data = node.data();
        let t: u64 = {
            if node.children().is_empty() {
                data.meta.iter().sum()
            } else {
                data.meta.iter()
                    .map(|&ix| {
                        let ix = (ix - 1) as usize;
                        node.children()
                            .get(ix)
                            .map( |id| t.get(id).unwrap().data().id)
                            .map(|id| *ids.get(&id).unwrap())
                            .unwrap_or(0)
                    })
                    .sum()
            }
        };

        ids.insert(data.id, t);
    }

    Ok(*ids.get(&1).unwrap())
}


fn main() -> Result<()> {
    let data = load_tree("in/08.txt")?;
    let sample = load_tree("in/08_sample.txt")?;

    let sample1 = part1(&sample)?;
    let sample2 = part2(&sample)?;
    println!("sample 1: {}, sample2: {}", sample1, sample2);

    let result1 = part1(&data)?;
    let result2 = part2(&data)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}