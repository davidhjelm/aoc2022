use std::{fs, io};

enum INST {
    ADDX(i32),
    NOOP,
}

struct Cpu {
    reg_x: i32,
    cycle: i32,
    row: Vec<char>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_x: 1,
            cycle: 0,
            row: vec![],
        }
    }

    fn run(&mut self, inst: INST) {
        match inst {
            INST::NOOP => self.cycle(),
            INST::ADDX(v) => {
                self.cycle();
                self.cycle();
                self.reg_x += v
            }
        }
    }

    fn cycle(&mut self) {
        let diff = self.reg_x - (self.cycle % 40);

        if diff >= -1 && diff <= 1 {
            self.row.push('#');
        } else {
            self.row.push('.');
        }

        self.cycle += 1;

        if self.cycle % 40 == 0 {
            println!("{}", String::from_iter(&self.row));
            self.row = vec![];
        }
    }

    fn parse(&mut self, input: &str) {
        match input {
            s if input.starts_with("addx") => {
                let amount = s.split_whitespace().last().unwrap().parse::<i32>().unwrap();
                self.run(INST::ADDX(amount));
            }
            "noop" => self.run(INST::NOOP),
            _ => panic!("should not get here"),
        }
    }
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d10").expect("Should open");

    let mut cpu = Cpu::new();

    for line in data.lines() {
        cpu.parse(line);
    }

    Ok(())
}

#[test]
fn tfunc() {
    let mut cpu = Cpu::new();

    let data = fs::read_to_string("./inputs/d10_test").expect("Should open");

    for line in data.lines() {
        cpu.parse(line);
    }
}
