use Solution;
use Result;
use util::file::data_path;
use util::file::load;

use regex::Regex;
use std::collections::HashSet;

type Instruction = [u64; 4];
type Capture = [u64; 4];

struct Sample {
    before: Capture,
    instr: Instruction,
    after: Capture,
}

#[derive(Default)]
pub struct Solution16 {
    samples: Vec<Sample>,
    prog: Vec<Instruction>,
}

pub struct Cpu {
    register: [u64; 4]
}

#[derive(Copy, Clone, Debug)]
enum Reg {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
}

#[derive(Debug)]
enum Opcode {
    Addr { a: Reg, b: Reg, c: Reg },
    Addi { a: Reg, b: u64, c: Reg },

    Mulr { a: Reg, b: Reg, c: Reg },
    Muli { a: Reg, b: u64, c: Reg },

    Banr { a: Reg, b: Reg, c: Reg },
    Bani { a: Reg, b: u64, c: Reg },

    Borr { a: Reg, b: Reg, c: Reg },
    Bori { a: Reg, b: u64, c: Reg },

    Setr { a: Reg, c: Reg },
    Seti { a: u64, c: Reg },

    Gtir { a: u64, b: Reg, c: Reg },
    Gtri { a: Reg, b: u64, c: Reg },
    Gtrr { a: Reg, b: Reg, c: Reg },

    Eqir { a: u64, b: Reg, c: Reg },
    Eqri { a: Reg, b: u64, c: Reg },
    Eqrr { a: Reg, b: Reg, c: Reg },
}

impl Reg {
    fn to_index(&self) -> usize {
        use self::Reg::*;
        match self {
            Reg0 => 0,
            Reg1 => 1,
            Reg2 => 2,
            Reg3 => 3
        }
    }

    fn from_index(ix: usize) -> Result<Reg> {
        use self::Reg::*;
        let reg = match ix {
            0 => Reg0,
            1 => Reg1,
            2 => Reg2,
            3 => Reg3,
            _ => return Err("Invalid index".into())
        };

        Ok(reg)
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { register: [0; 4] }
    }
    fn from_capture(cap: Capture) -> Cpu {
        Cpu { register: cap }
    }

    fn capture(&self) -> Capture {
        self.register.clone()
    }

    fn get_reg(&self, r: Reg) -> u64 {
        self.register[r.to_index()]
    }

    fn set_reg(&mut self, r: Reg, val: u64) {
        self.register[r.to_index()] = val
    }
}

impl Opcode {
    fn exec(&self, cpu: &mut Cpu) -> Result<()> {
        use self::Opcode::*;
        match *self {
            Addr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, a + b);
            }
            Addi { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, a + b);
            }

            Mulr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, a * b);
            }
            Muli { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, a * b);
            }

            Borr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, a | b);
            }
            Bori { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, a | b);
            }

            Banr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, a & b);
            }
            Bani { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, a & b);
            }

            Setr { a, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, a);
            }
            Seti { a, c } => {
                cpu.set_reg(c, a);
            }

            Gtir { a, b, c } => {
                let b = cpu.get_reg(b);
                cpu.set_reg(c, (a > b) as u64);
            }
            Gtri { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, (a > b) as u64);
            }
            Gtrr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, (a > b) as u64);
            }

            Eqir { a, b, c } => {
                let b = cpu.get_reg(b);
                cpu.set_reg(c, (a == b) as u64);
            }
            Eqri { a, b, c } => {
                let a = cpu.get_reg(a);
                cpu.set_reg(c, (a == b) as u64);
            }
            Eqrr { a, b, c } => {
                let a = cpu.get_reg(a);
                let b = cpu.get_reg(b);
                cpu.set_reg(c, (a == b) as u64);
            }
        }

        Ok(())
    }

    fn from_instr(instr: Instruction) -> Result<Opcode> {
        use self::Opcode::*;
        let (op, a, b, c) = (instr[0], instr[1], instr[2], instr[3]);
        let reg_a = Reg::from_index(a as usize);
        let reg_b = Reg::from_index(b as usize);
        let reg_c = Reg::from_index(c as usize);

        let op = match op {
            0 => Addr { a: reg_a?, b: reg_b?, c: reg_c? },
            1 => Addi { a: reg_a?, b, c: reg_c? },

            2 => Mulr { a: reg_a?, b: reg_b?, c: reg_c? },
            3 => Muli { a: reg_a?, b, c: reg_c? },

            4 => Banr { a: reg_a?, b: reg_b?, c: reg_c? },
            5 => Bani { a: reg_a?, b, c: reg_c? },

            6 => Borr { a: reg_a?, b: reg_b?, c: reg_c? },
            7 => Bori { a: reg_a?, b, c: reg_c? },

            8 => Setr { a: reg_a?, c: reg_c? },
            9 => Seti { a, c: reg_c? },

            10 => Gtir { a, b: reg_b?, c: reg_c? },
            11 => Gtri { a: reg_a?, b, c: reg_c? },
            12 => Gtrr { a: reg_a?, b: reg_b?, c: reg_c? },

            13 => Eqir { a, b: reg_b?, c: reg_c? },
            14 => Eqri { a: reg_a?, b, c: reg_c? },
            15 => Eqrr { a: reg_a?, b: reg_b?, c: reg_c? },
            _ => return Err("Invalid opcode".into())
        };

        Ok(op)
    }

    fn count() -> usize {
        16
    }
}



fn op_options(samples: &[Sample]) -> Vec<(u64, HashSet<u64>)> {
    let mut result = Vec::new();
    let n = Opcode::count();

    for (_, s) in samples.iter().enumerate() {
        let op = s.instr[0];

        let v = (0..n)
            .filter_map(|i| {
                let mut cpu  = Cpu::from_capture(s.before);
                let mut instr = s.instr;
                instr[0] = i as u64;

                if let Ok(op) = Opcode::from_instr(instr) {
                    op.exec(&mut cpu).unwrap();

                    if cpu.capture() == s.after {
                        return Some(instr[0]);
                    }
                }

                None
            })
            .collect();

        result.push((op, v));
    }

    result

}

fn part1(samples: &[Sample]) -> Result<usize> {
    let opts = op_options(&samples);
    let count = opts.iter().filter(|&c| c.1.len() >= 3).count();
    Ok(count)
}

fn part2(samples: &[Sample], p: &[Instruction]) -> Result<u64> {
    let mut t = [None; 16];
    let mut opts = op_options(&samples);

    while t.iter().filter(|o| o.is_none()).count() > 0 {
        for i in 0..opts.len() {
            if opts[i].1.len() != 1 {
                continue;
            }

            let real_op = {
                let can = &mut opts[i].1;
                let last = *can.iter().next().unwrap();
                can.remove(&last);
                last
            };
            let op = opts[i].0;
            t[op as usize] = Some(real_op);

            for i in 0..opts.len() {
                if opts[i].0 ==  op {
                    opts[i].1.clear();
                } else {
                    opts[i].1.remove(&real_op);
                }
            }
        }
    }

    //run program
    let prog: Vec<Opcode> = p.iter()
        .map(|i| {
            let mut instr = *i;
            instr[0] = t[instr[0] as usize].unwrap();
            Opcode::from_instr(instr).unwrap()
        })
        .collect();

    let mut cpu = Cpu::new();
    for op in prog.iter() {
        op.exec(&mut cpu).unwrap();
    }

    Ok(cpu.capture()[0])
}


impl Solution for Solution16 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(16))?;
        let split = s.find("\n\n\n\n").unwrap();

        let (samples, program) = s.split_at(split);
        self.samples = parse_samples(samples)?;
        self.prog = parse_program(&program[4..])?;

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.samples)?;
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = part2(&self.samples, &self.prog)?;

        println!("result2: {}", result);
        Ok(())
    }
}

fn parse_instruction(s: &str) -> Result<Instruction> {
    let mut instr = [0; 4];
    for (i, s) in s.split_whitespace().enumerate() {
        if i >= 4 {
            return Err("Invalid amount of numbers in instruction".into());
        }

        instr[i] = s.parse()?;
    }

    Ok(instr)
}

fn parse_capture(s: &str) -> Result<Capture> {
    lazy_static! {
        static ref RE_CAP: Regex = Regex::new(r"^((Before|After):\s*\[)(\d)(, )(\d)(, )(\d)(, )(\d)(\])$").unwrap();
    }

    let mut cap = [0; 4];

    let caps = RE_CAP.captures(s)
        .ok_or("Invalid capture Line")?;

    for i in 0..4 {
        cap[i] = caps[3 + i * 2].parse()?;
    }


    Ok(cap)
}

fn parse_program(s: &str) -> Result<Vec<Instruction>> {
    s.lines()
        .map(|l| parse_instruction(l))
        .collect()
}


fn parse_samples(s: &str) -> Result<Vec<Sample>> {
    let mut lines = s.lines().peekable();
    let mut samples = Vec::new();

    while lines.peek().is_some() {
        let line1 = lines.next().unwrap();
        let line2 = lines.next().ok_or("no second line")?;
        let line3 = lines.next().ok_or("no third line")?;

        let before = parse_capture(line1)?;
        let instr = parse_instruction(line2)?;
        let after = parse_capture(line3)?;

        samples.push(Sample {
            before,
            instr,
            after,
        });

        lines.next();
    }

    Ok(samples)
}