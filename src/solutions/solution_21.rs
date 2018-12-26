use Solution;
use Result;
use util::file::data_path;
use util::file::load;
use std::str::FromStr;
use std::collections::BTreeSet;


#[derive(Default, Clone)]
pub struct Program {
    ops: Vec<Opcode>,
    ip_reg: Reg
}

#[derive(Default)]
pub struct Solution21 {
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
            _ => return Err("Invalid reg index".into())
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

        let reg = |i| Reg::from_index(i as usize);

        let op = match instr {
            "addr" => Addr { a: reg(a)?, b: reg(b)?, c: reg(c)? },
            "addi" => Addi { a: reg(a)?, b, c: reg(c)? },

            "mulr" => Mulr { a: reg(a)?, b: reg(b)?, c: reg(c)? },
            "muli" => Muli { a: reg(a)?, b, c: reg(c)? },

            "banr" => Banr { a: reg(a)?, b: reg(b)?, c: reg(c)? },
            "bani" => Bani { a: reg(a)?, b, c: reg(c)? },

            "borr" => Borr { a: reg(a)?, b: reg(b)?, c: reg(c)? },
            "bori" => Bori { a: reg(a)?, b, c: reg(c)? },

            "setr" => Setr { a: reg(a)?, c: reg(c)? },
            "seti" => Seti { a, c: reg(c)? },

            "gtir" => Gtir { a, b: reg(b)?, c: reg(c)? },
            "gtri" => Gtri { a: reg(a)?, b, c: reg(c)? },
            "gtrr" => Gtrr { a: reg(a)?, b: reg(b)?, c: reg(c)? },

            "eqir" => Eqir { a, b: reg(b)?, c: reg(c)? },
            "eqri" => Eqri { a: reg(a)?, b, c: reg(c)? },
            "eqrr" => Eqrr { a: reg(a)?, b: reg(b)?, c: reg(c)? },
            _ => return Err("Invalid opcode".into())
        };

        Ok(op)
    }
}

impl Program {
    fn run_once(&self, cpu: &mut Cpu) -> bool {
        let ix = cpu.get_reg(self.ip_reg) as usize;

        match self.ops.get(ix) {
            Some(instr) => {
                instr.exec(cpu).unwrap();

                let ix = cpu.get_reg(self.ip_reg);
                cpu.set_reg(self.ip_reg, ix + 1);
                true
            },
            None => false
        }
    }

    fn ip(&self, cpu: &Cpu) -> usize {
        cpu.get_reg(self.ip_reg) as usize
    }
}

fn part1(p: &Program) -> u64 {
    const EQ_RR_IX: usize = 28;

    let mut cpu = Cpu::new();

    while p.ip(&cpu) != EQ_RR_IX {
        p.run_once(&mut cpu);
    }

    return cpu.get_reg(Reg::Reg3);
}

fn part2(p: &Program) -> u64 {
    let eq_rr_ix = 28;
    let mut prev = BTreeSet::new();
    let mut last = 0;

    let mut cpu = Cpu::new();

    loop {
        if p.ip(&cpu) == eq_rr_ix {
            let val = cpu.get_reg(Reg::Reg3);
            if prev.contains(&val) {
                return last;
            }
            last = val;
            prev.insert(val);
        }

        p.run_once(&mut cpu);
    }
}

impl Solution for Solution21 {
    fn init(&mut self) -> Result<()> {
        let s = load(&data_path(21))?;
        self.data = s.parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = part1(&self.data);
        println!("result1: {}", result);
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = part2(&self.data);
        println!("result2: {}", result);
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