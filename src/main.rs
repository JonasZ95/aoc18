extern crate aoc;

use aoc::solutions;
use aoc::Result;


fn main() -> Result<()> {
    let n = 11;
    let mut solutions = solutions::get_solutions();

    match n {
        0 => {
            for solution in solutions.iter_mut() {
                solution.run()?;
            }
        },
        n => {
            solutions.get_mut(n-1).unwrap().run()?
        }
    };

    Ok(())
}
