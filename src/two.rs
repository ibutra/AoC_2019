use crate::intcode::Intcode;

pub fn run(input: String) {
    let input = input.trim();
    let input: Vec<&str> = input.split(",").collect();
    let mut memory = Vec::new();
    for value in input {
        let num : usize = value.parse().expect("Failed to parse usize for Day 2");
        memory.push(num);
    }

    let mut computer = Intcode::new(memory);

    //Part 1
    computer[1] = 12;
    computer[2] = 2;
    computer.compute();
    println!("Day2 Part 1: {:?}", computer.result());
    
    //Part 2
    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer[1] = noun;
            computer[2] = verb;
            computer.compute();
            if computer.result() == 19690720 {
                println!("Day2 Part2: Noun: {} Verb: {} Result: {}", noun, verb, 100*noun + verb);
                return;
            }
        }
    }
    println!("Day2 Part1: No Verb and noun found");
            
}

