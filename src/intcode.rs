use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Intcode {
    pc: usize,
    memory: Vec<usize>,
    original_memory: Vec<usize>,
}

impl Intcode {
    pub fn new(memory: Vec<usize>) -> Self {
        Intcode {
            pc: 0,
            memory: memory.clone(),
            original_memory: memory
        }
    }

    pub fn compute(&mut self) {
        loop {
            let inc = match self.memory[self.pc] {
                1 => self.add(),
                2 => self.mul(),
                99 => break,
                _ => panic!("Invalid opcode"),
            };
            // println!("{:?}", memory);
            self.pc += inc;
        }
    }

    pub fn result(&self) -> usize {
        self.memory[0]
    }

    pub fn reset(&mut self) {
        self.memory = self.original_memory.clone();
        self.pc = 0;
    }

    fn add(&mut self) -> usize {
        let pc = self.pc;
        let a = self.memory[pc + 1];
        let b = self.memory[pc + 2];
        let res = self.memory[pc + 3];
        self.memory[res] = self.memory[a] + self.memory[b];
        4
    }

    fn mul(&mut self) -> usize{
        let pc = self.pc;
        let a = self.memory[pc + 1];
        let b = self.memory[pc + 2];
        let res = self.memory[pc + 3];
        self.memory[res] = self.memory[a] * self.memory[b];
        4
    }
}

impl Index<usize> for Intcode {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}
