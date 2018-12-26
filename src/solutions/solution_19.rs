use Solution;
use Result;
use util::file::data_path;
use std::str::FromStr;
use util::file::load;
use util::file::sample_path;


#[derive(Default, Clone)]
pub struct Program {
    ops: Vec<Opcode>,
    ip_reg: Reg
}

#[derive(Default)]
pub struct Solution19 {
    sample: Program,
    data: Program,
}

pub struct Cpu {
    register: [u64; 6]
}

#[derive(Copy, Clone, Debug)]
enum Reg {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
}

#[derive(Debug, Clone)]
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

impl Default for Reg {
    fn default() -> Self {
        Reg::Reg0
    }
}

impl Reg {
    fn to_index(&self) -> usize {
        use self::Reg::*;
        match self {
            Reg0 => 0,
            Reg1 => 1,
            Reg2 => 2,
            Reg3 => 3,
            Reg4 => 4,
            Reg5 => 5
        }
    }

    fn from_index(ix: usize) -> Result<Reg> {
        use self::Reg::*;
        let reg = match ix {
            0 => Reg0,
            1 => Reg1,
            2 => Reg2,
            3 => Reg3,
            4 => Reg4,
            5 => Reg5,
            _ => return Err("Invalid index".into())
        };

        Ok(reg)
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { register: [0; 6] }
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

    fn from_instr_str(instr: &str, a: u64, b: u64, c: u64) -> Result<Opcode> {
        use self::Opcode::*;
        let reg_a = Reg::from_index(a as usize);
        let reg_b = Reg::from_index(b as usize);
        let reg_c = Reg::from_index(c as usize);

        let op = match instr {
            "addr" => Addr { a: reg_a?, b: reg_b?, c: reg_c? },
            "addi" => Addi { a: reg_a?, b, c: reg_c? },

            "mulr" => Mulr { a: reg_a?, b: reg_b?, c: reg_c? },
            "muli" => Muli { a: reg_a?, b, c: reg_c? },

            "banr" => Banr { a: reg_a?, b: reg_b?, c: reg_c? },
            "bani" => Bani { a: reg_a?, b, c: reg_c? },

            "borr" => Borr { a: reg_a?, b: reg_b?, c: reg_c? },
            "bori" => Bori { a: reg_a?, b, c: reg_c? },

            "setr" => Setr { a: reg_a?, c: reg_c? },
            "seti" => Seti { a, c: reg_c? },

            "gtir" => Gtir { a, b: reg_b?, c: reg_c? },
            "gtri" => Gtri { a: reg_a?, b, c: reg_c? },
            "gtrr" => Gtrr { a: reg_a?, b: reg_b?, c: reg_c? },

            "eqir" => Eqir { a, b: reg_b?, c: reg_c? },
            "eqri" => Eqri { a: reg_a?, b, c: reg_c? },
            "eqrr" => Eqrr { a: reg_a?, b: reg_b?, c: reg_c? },
            _ => return Err("Invalid opcode".into())
        };

        Ok(op)
    }
}

/*
prog 19:
2: reg5 = 1;
3: reg2 = 1;
4: reg1 = reg5 * reg2;
5,6,7,8 if (reg1 == reg4):  reg0 = reg5 + reg0;
9: reg2 = reg2 + 1;
10,11,12: if (reg2 <= reg4): GOTO 4;
13: reg5 = reg5 + 1
14, 15, 16: if (reg5 <= reg4): GOTO 3;
17: reg3 = reg3 * reg3
*/
#[allow(dead_code)]
fn sum_fact19(n: usize) -> usize {
    let mut counter = 0;
    for r5 in 1..=n {
        for r2 in 1..=n {
            if r5 * r2 == n {
                counter += r5;
            }
        }
    }

    counter
}

fn sum_fact(n: u64) -> u64 {
    let sum: u64 = (2..=n/2)
        .filter(|i| n % i == 0)
        .sum();

    sum + n + 1
}

impl Program {
    fn run_with_cpu(&self, cpu: &mut Cpu) {
        loop {
            let ix = cpu.get_reg(self.ip_reg) as usize;

            match self.ops.get(ix) {
                Some(instr) => {
                    instr.exec(cpu).unwrap();

                    let ix = cpu.get_reg(self.ip_reg);
                    cpu.set_reg(self.ip_reg, ix + 1);
                },
                None => return
            }
        }
    }

    fn run1(&self) -> u64 {
        let mut cpu = Cpu::new();
        self.run_with_cpu(&mut cpu);
        cpu.get_reg(Reg::Reg0)
    }

    fn run2(&self) -> u64 {
        let mut modded: Program = self.clone();
        //Run cpu until we reach the second line and extract reg 4
        modded.ops[1] = Opcode::Seti {a: 1000, c: self.ip_reg};

        let mut cpu = Cpu::new();
        cpu.set_reg(Reg::Reg0, 1);
        modded.run_with_cpu(&mut cpu);

        cpu.get_reg(Reg::Reg4)
    }
}

fn part1(p: &Program) -> Result<u64> {
    let n = p.run1();
    Ok(n)
}

fn part2(p: &Program) -> Result<u64> {
    let n = p.run2();
    Ok(n)
}


impl Solution for Solution19 {
    fn init(&mut self) -> Result<()> {
        let s = load(&sample_path(19))?;
        self.sample = s.parse()?;

        let s = load(&data_path(19))?;
        self.data = s.parse()?;

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.sample)?;
        println!("sample1: {}", result);

        let result = part1(&self.data)?;
        println!("data1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()>  {
        let result = part2(&self.data)?;
        println!("result2: {}", sum_fact(result));
        Ok(())
    }
}

impl FromStr for Opcode {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let instr = parts.next().ok_or("no instruction")?;
        let a = parts.next()
            .ok_or("no a")?
            .parse()?;
        let b = parts.next()
            .ok_or("no b")?
            .parse()?;
        let c = parts.next().ok_or("no c")?
            .parse()?;

        Opcode::from_instr_str(instr, a, b, c)
    }
}

impl FromStr for Program {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut l = s.lines();

        let ip_line = l.next()
            .ok_or("no ip line")?;
        let mut s = ip_line.split_whitespace();
        s.next().ok_or("no ip first")?;

        let n = s.next()
            .ok_or("no ip index")?
            .parse()?;

        let ip_reg = Reg::from_index(n)?;

        let ops = l
            .map(|l| l.parse())
            .collect::<Result<Vec<Opcode>>>()?;

        Ok(Program{
            ip_reg,
            ops
        })
    }
}