use std::ops::{Index, IndexMut};
use std::io::stdin;

type PC = usize;

#[derive(Debug)]
pub struct Intcode {
    pc: PC,
    memory: Vec<i64>,
    original_memory: Vec<i64>,
}

enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

struct ModeIter {
    num: i64,
}

impl ModeIter {
    fn new(num: i64) -> Self {
        ModeIter{num}
    }

    fn next(&mut self) -> ParameterMode {
        if self.num == 0 {
            return ParameterMode::PositionMode;
        }
        let digit = self.num % 10;
        self.num /= 10;
        match digit {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            _ => panic!("Invalid ParameterMode"),
        }
    }
}

impl Intcode {
    pub fn new(memory: Vec<i64>) -> Self {
        Intcode {
            pc: 0,
            memory: memory.clone(),
            original_memory: memory
        }
    }

    pub fn compute(&mut self) {
        loop {
            let (opcode, mode_iter) = self.get_opcode();
            let inc = match opcode {
                1 => self.add(mode_iter),
                2 => self.mul(mode_iter),
                3 => self.input(mode_iter),
                4 => self.output(mode_iter),
                5 => self.jump_if_true(mode_iter),
                6 => self.jump_if_false(mode_iter),
                7 => self.less_than(mode_iter),
                8 => self.equals(mode_iter),
                99 => break,
                _ => panic!("Invalid opcode"),
            };
            // println!("{:?}", memory);
            self.pc += inc;
        }
    }

    pub fn result(&self) -> i64 {
        self.memory[0]
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.memory = self.original_memory.clone();
        self.pc = 0;
    }

    fn get_opcode(&self) -> (PC, ModeIter) {
        let value = self.memory[self.pc];
        let opcode = value % 100;
        let mode_digits = value / 100;
        (opcode as usize, ModeIter::new(mode_digits))
    }

    fn get_parameter_value(&self, parameter_address: usize, parameter_mode: &mut ModeIter) -> i64 {
        let parameter_value = self.memory[parameter_address];
        match parameter_mode.next() {
            ParameterMode::PositionMode => self.memory[parameter_value as usize],
            ParameterMode::ImmediateMode => parameter_value,
        }
    }
    
    fn jump_if_true(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let param1 = self.get_parameter_value(pc + 1, &mut mode_iter);
        let param2 = self.get_parameter_value(pc + 2, &mut mode_iter);
        if param1 != 0 {
            self.pc = param2 as usize;
            return 0;
        }
        3
    }

    fn jump_if_false(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let param1 = self.get_parameter_value(pc + 1, &mut mode_iter);
        let param2 = self.get_parameter_value(pc + 2, &mut mode_iter);
        if param1 == 0 {
            self.pc = param2 as usize;
            return 0;
        }
        3
    }
    
    fn less_than(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let param1 = self.get_parameter_value(pc + 1, &mut mode_iter);
        let param2 = self.get_parameter_value(pc + 2, &mut mode_iter);
        let out = self.memory[pc + 3];
        if param1 < param2 {
            self.memory[out as usize] = 1;
        } else {
            self.memory[out as usize] = 0;
        }
        4
    }

    fn equals(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let param1 = self.get_parameter_value(pc + 1, &mut mode_iter);
        let param2 = self.get_parameter_value(pc + 2, &mut mode_iter);
        let out = self.memory[pc + 3];
        if param1 == param2 {
            self.memory[out as usize] = 1;
        } else {
            self.memory[out as usize] = 0;
        }
        4
    }
    
    fn input(&mut self, _mode_iter: ModeIter) -> usize {
        let mut s = String::new();
        let v;
        let out = self.memory[self.pc + 1];
        loop {
            println!("Input number: ");
            stdin().read_line(&mut s).expect("Invalid Input!");
            let trimmed = s.trim();
            if let Ok(value) = trimmed.parse::<i64>() {
                v = value;
                break;
            } else {
                println!("Couldn't parse number: '{}'", s);
            }
        }
        self.memory[out as usize] = v;
        2
    }

    fn output(&mut self, mut mode_iter: ModeIter) -> usize {
        let param = self.get_parameter_value(self.pc + 1, &mut mode_iter);
        println!("Output: {}", param);
        2
    }

    fn add(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let a = self.get_parameter_value(pc + 1, &mut mode_iter);
        let b = self.get_parameter_value(pc + 2, &mut mode_iter);
        let res = self.memory[pc + 3];
        self.memory[res as usize] = a + b;
        4
    }

    fn mul(&mut self, mut mode_iter: ModeIter) -> usize {
        let pc = self.pc;
        let a = self.get_parameter_value(pc + 1, &mut mode_iter);
        let b = self.get_parameter_value(pc + 2, &mut mode_iter);
        let res = self.memory[pc + 3];
        self.memory[res as usize] = a * b;
        4
    }
}

impl Index<usize> for Intcode {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}
