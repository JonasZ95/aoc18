use Solution;

pub mod solution_01;
pub mod solution_02;
pub mod solution_12;
pub mod solution_13;
pub mod solution_14;
pub mod solution_15;
pub mod solution_16;
pub mod solution_17;
pub mod solution_18;
pub mod solution_19;
pub mod solution_20;
pub mod solution_21;
pub mod solution_22;
pub mod solution_23;
pub mod solution_24;
pub mod solution_25;



use solutions::solution_01::Solution01;
use solutions::solution_02::Solution02;
use solutions::solution_12::Solution12;
use solutions::solution_13::Solution13;
use solutions::solution_14::Solution14;
use solutions::solution_15::Solution15;
use solutions::solution_16::Solution16;
use solutions::solution_17::Solution17;
use solutions::solution_18::Solution18;
use solutions::solution_19::Solution19;
use solutions::solution_20::Solution20;
use solutions::solution_21::Solution21;
use solutions::solution_22::Solution22;
use solutions::solution_23::Solution23;
use solutions::solution_24::Solution24;
use solutions::solution_25::Solution25;


pub fn get_solutions() -> Vec<Box<Solution>> {
    vec![
        Box::new(Solution01::default()),
        Box::new(Solution02::default()),
        Box::new(Solution12::default()),
        Box::new(Solution13::default()),
        Box::new(Solution14::default()),
        Box::new(Solution15::default()),
        Box::new(Solution16::default()),
        Box::new(Solution17::default()),
        Box::new(Solution18::default()),
        Box::new(Solution19::default()),
        Box::new(Solution20::default()),
        Box::new(Solution21::default()),
        Box::new(Solution22::default()),
        Box::new(Solution23::default()),
        Box::new(Solution24::default()),
        Box::new(Solution25::default())
    ]
}