use Solution;
use Result;
use util::file::load_and_parse_lines;
use util::file::data_path;
use std::collections::BTreeSet;

#[derive(Default)]
pub struct Solution01{
    data: Vec<i64>
}

impl Solution for Solution01 {
    fn init(&mut self) -> Result<()> {
        self.data = load_and_parse_lines(&data_path(1))?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = self.data.iter()
            .fold(0, |cur, offset| {
                cur + offset
            });

        println!("result: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut seen = BTreeSet::new();
        let mut freq: i64 = 0;
        let mut result = 0;

        for &offset in self.data.iter().cycle() {
            match seen.get(&freq) {
                Some(_) => {
                    result = freq;
                    break;
                },
                None => {
                    seen.insert(freq);
                    freq += offset;
                }
            }
        };

        println!("result: {}", result);
        Ok(())
    }
}