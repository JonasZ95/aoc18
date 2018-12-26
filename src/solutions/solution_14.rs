use Solution;
use Result;

#[derive(Default)]
pub struct Solution14 {
    data: u64
}

fn get_num(data: &[u8]) -> u64 {
    let n = data.len();
    data.iter()
        .enumerate()
        .map(|(i, &d)| {
            let exp = (n - i - 1) as u32;
            (d as u64) * 10_u64.pow(exp)
        })
        .sum()
}

fn get_data(mut num: u64) -> Vec<u8> {
    let mut data = Vec::new();
    while num > 0 {
        data.insert(0, (num % 10) as u8);
        num /= 10;
    }

    data
}

fn combine(r: &mut Vec<u8>, e1: &mut usize, e2: &mut usize) {
    let r1 = r[*e1];
    let r2 = r[*e2];
    let sum = r1 + r2;

    if sum > 9 {
        r.push(sum / 10);
    }
    r.push(sum % 10);

    *e1 += (r1 as usize) + 1;
    *e1 %= r.len();
    *e2 += (r2 as usize) + 1;
    *e2 %= r.len();
}

fn part1(val: u64, n: usize) -> Result<u64> {
    let mut r = get_data(val);
    let recipes = n + 10;
    let (mut e1, mut e2) = (0, 1);

    while r.len() < recipes {
        combine(&mut r, &mut e1, &mut e2);
    }

    Ok(get_num(&r[n..n + 10]))
}

fn part2(val: u64, pat: u64) -> Result<usize> {
    let mut r = get_data(val);
    let (mut e1, mut e2) = (0, 1);
    let m = get_data(pat).len();

    let result = loop {
        combine(&mut r, &mut e1, &mut e2);

        let n = r.len();

        if r.len() > m + 1 {
            let n = n - 1;
            if get_num(&r[n - m..n]) == pat {
                break n - m;
            }
        }

        if n > m {
            if get_num(&r[n - m..]) == pat {
                break n - m;
            }
        }
    };

    Ok(result)
}

impl Solution for Solution14 {
    fn init(&mut self) -> Result<()> {
        self.data = 37;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        assert_eq!(5158916779, part1(self.data, 9)?);
        assert_eq!(0124515891, part1(self.data, 5)?);
        assert_eq!(9251071085, part1(self.data, 18)?);
        assert_eq!(5941429882, part1(self.data, 2018)?);

        let result = part1(self.data, 9)?;
        println!("sample1: {}", result);

        let result = part1(self.data, 260321)?;
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        assert_eq!(9, part2(self.data, 51589)?);
        assert_eq!(18, part2(self.data, 92510)?);
        assert_eq!(2018, part2(self.data, 59414)?);

        let result = part2(self.data, 260321)?;
        println!("result2: {}", result);
        Ok(())
    }
}