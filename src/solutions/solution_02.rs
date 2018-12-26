use Solution;
use Result;
use util::file::data_path;
use util::file::load_lines;

#[derive(Default)]
pub struct Solution02{
    data: Vec<String>
}

type CharCountMap = [usize; 256];

fn count_chars(s: &str) -> Result<CharCountMap> {
    let mut counter = [0 as usize; 256];

    if !s.is_ascii() {
        return Err("A Line can only contain ascii characters".into());
    }

    for b in s.bytes() {
        counter[b as usize] += 1;
    }

    Ok(counter)
}

fn part2(lines: &[String]) -> Result<String> {
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(i) {
            let zipped = line1.chars()
                .zip(line2.chars());


            let neq: usize = zipped.clone()
                .map(|(c1, c2)| (c1 != c2) as usize)
                .sum();


            if neq == 1 {
                return Ok(
                    zipped.filter(|(c1, c2)| c1 == c2)
                        .map(|(c1, _)| c1)
                        .collect()
                );
            }
        }
    }

    Ok(String::new())
}

impl Solution for Solution02 {
    fn init(&mut self) -> Result<()> {
        self.data = load_lines(&data_path(2))?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let (mut twos, mut threes) = (0, 0);

        for counter in self.data.iter()
            .map(|line| count_chars(&line)) {
            let counter = counter?;

            if counter.iter().find(|&&n| n == 2).is_some() {
                twos += 1;
            }

            if counter.iter().find(|&&n| n == 3).is_some() {
                threes += 1;
            }
        }

        println!("result: #2: {}, #3: {}", twos, threes);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
       let result = part2(&self.data)?;
        println!("result: {}", result);
        Ok(())
    }
}