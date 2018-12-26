extern crate regex;
#[macro_use]
extern crate lazy_static;

extern crate itertools;
extern crate z3;

pub mod solutions;
pub mod util;

pub type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub trait Solution {
    fn init(&mut self) -> Result<()>;
    fn part1(&mut self) -> Result<()>;
    fn part2(&mut self) -> Result<()>;

    fn run(&mut self) -> Result<()> {
        self.init()?;
        self.part1()?;
        self.part2()?;
        Ok(())
    }
}

