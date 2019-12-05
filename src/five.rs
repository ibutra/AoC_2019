use crate::intcode::Intcode;

pub fn run(input: String) {
    let input = input.trim();
    let input: Vec<&str> = input.split(",").collect();
    let mut memory = Vec::new();
    for value in input {
        let num : i64 = value.parse().expect("Failed to parse usize for Day 2");
        memory.push(num);
    }

    let mut computer = Intcode::new(memory);
    println!("Start compute");
    computer.compute();
    println!("Day 5 Task1: {}", computer.result());
}
