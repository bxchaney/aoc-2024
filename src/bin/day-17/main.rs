use aoc_2024::read_input;

#[derive(Debug)]
struct RegisterBlock {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
}

#[derive(Debug)]
enum Operation {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl TryFrom<i64> for Operation {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x == Operation::Adv as i64 => Ok(Operation::Adv),
            x if x == Operation::Bxl as i64 => Ok(Operation::Bxl),
            x if x == Operation::Bst as i64 => Ok(Operation::Bst),
            x if x == Operation::Jnz as i64 => Ok(Operation::Jnz),
            x if x == Operation::Bxc as i64 => Ok(Operation::Bxc),
            x if x == Operation::Out as i64 => Ok(Operation::Out),
            x if x == Operation::Bdv as i64 => Ok(Operation::Bdv),
            x if x == Operation::Cdv as i64 => Ok(Operation::Cdv),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operator: Operation,
    operand: i64,
}

impl Instruction {
    pub fn process(
        &self,
        mut registers: &mut RegisterBlock,
        mut out: &mut Vec<i64>,
        ip: i64,
    ) -> i64 {
        match self.operator {
            Operation::Adv => self.adv(&mut registers),
            Operation::Bxl => self.bxl(&mut registers),
            Operation::Bst => self.bst(&mut registers),
            Operation::Jnz => self.jnz(registers, ip),
            Operation::Bxc => self.bxc(registers),
            Operation::Out => self.out(registers, &mut out),
            Operation::Bdv => self.bdv(registers),
            Operation::Cdv => self.cdv(registers),
        }
    }

    fn combo(&self, registers: &RegisterBlock) -> i64 {
        match self.operand {
            0..=3 => self.operand,
            4 => registers.reg_a,
            5 => registers.reg_b,
            6 => registers.reg_c,
            _ => panic!("invalid combo operand"),
        }
    }

    fn adv(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_a = registers.reg_a >> (self.combo(&registers) as u32);
        1
    }

    fn bxl(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_b = registers.reg_b ^ self.operand;
        1
    }

    fn bst(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_b = self.combo(registers) & 0b111;
        1
    }

    fn jnz(&self, registers: &RegisterBlock, ip: i64) -> i64 {
        if registers.reg_a == 0 {
            1
        } else {
            (self.operand - ip * 2) / 2
        }
    }

    fn bxc(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_b = registers.reg_b ^ registers.reg_c;
        1
    }

    fn out(&self, registers: &RegisterBlock, out: &mut Vec<i64>) -> i64 {
        out.push(self.combo(registers) & 0b111);
        1
    }

    fn bdv(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_b = registers.reg_a >> (self.combo(&registers) as u32);
        1
    }

    fn cdv(&self, registers: &mut RegisterBlock) -> i64 {
        registers.reg_c = registers.reg_a >> (self.combo(&registers) as u32);
        1
    }
}

#[derive(Debug)]
struct System {
    registers: RegisterBlock,
    output: Vec<i64>,
    operations_pointer: i64,
    instructions: Vec<Instruction>,
}

impl System {
    fn run(&mut self) -> String {
        loop {
            if self.operations_pointer < 0
                || self.operations_pointer as usize >= self.instructions.len()
            {
                break;
            }
            let point_increment = self.instructions[self.operations_pointer as usize].process(
                &mut self.registers,
                &mut self.output,
                self.operations_pointer,
            );
            self.operations_pointer += point_increment;
        }

        self.output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn parse_instructions(instrution_str: String) -> Vec<Instruction> {
    let mut nums = instrution_str
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap());

    let mut instructions = vec![];
    while let Some(operator) = nums.next() {
        if let Some(operand) = nums.next() {
            instructions.push(Instruction {
                operator: Operation::try_from(operator).unwrap(),
                operand,
            })
        }
    }
    instructions
}

fn parse_input() -> System {
    let (reg_a, reg_b, reg_c, instructions);
    let mut input = read_input();
    if let Some(a) = input.next() {
        reg_a = a
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .parse::<i64>()
            .unwrap();
    } else {
        panic!();
    }
    if let Some(b) = input.next() {
        reg_b = b
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .parse::<i64>()
            .unwrap();
    } else {
        panic!();
    }
    if let Some(c) = input.next() {
        reg_c = c
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .parse::<i64>()
            .unwrap();
    } else {
        panic!();
    }
    input.next();
    if let Some(instruction_str) = input.next() {
        instructions = parse_instructions(instruction_str.unwrap());
    } else {
        panic!()
    }

    return System {
        registers: RegisterBlock {
            reg_a,
            reg_b,
            reg_c,
        },
        output: vec![],
        operations_pointer: 0,
        instructions,
    };
}

fn main() {
    let mut sys = parse_input();

    println!("{}", sys.run());
}
