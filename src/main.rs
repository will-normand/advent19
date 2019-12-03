fn main() {
    let day = 2;

    if day == 1 {
        let result = day1::day1();
        println!("Day 1 result is: {}", result)
    } else if day == 2 {
        day2::run();
    }
}

mod day1 {
    use std::fs;

    pub fn day1() -> i32 {
        println!("Running day 1");
        let masses = load_file("data/day1.txt".to_string());

        masses.iter().map(|mass| compound_fuel(*mass)).sum()
    }

    fn load_file(filename: String) -> Vec<i32> {
        let masses = fs::read_to_string(filename).expect("Failed to find file");
        masses
            .split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    fn calculate_fuel(mass: i32) -> i32 {
        (mass / 3) - 2
    }

    fn compound_fuel(mass: i32) -> i32 {
        let mut fuel = calculate_fuel(mass);
        let mut extra_fuel = fuel;

        while extra_fuel > 0 {
            extra_fuel = calculate_fuel(extra_fuel);
            fuel += if extra_fuel > 0 { extra_fuel } else { 0 }
        }

        fuel
    }
}

mod day2 {
    use std::fs;

    const TARGET: i32 = 19_690_720;

    enum Instruction {
        ADD(usize, usize, usize),
        MUL { a: usize, b: usize, result: usize },
        TERM,
    }

    pub fn run() {
        let input = load_file("data/day2.txt".to_string());

        for noun in 0..99 {
            for verb in 0..99 {
                let mut memory = input.clone();
                memory[1] = noun;
                memory[2] = verb;

                let result = process(memory);

                if result[0] == TARGET {
                    let answer = 100 * noun + verb;
                    println!("The answer is {}", answer);
                    return;
                }
            }
        }
    }

    fn load_file(filename: String) -> Vec<i32> {
        let integers = fs::read_to_string(filename).expect("Failed to find file");
        integers
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    fn get_instruction(memory: &[i32], instruction_pointer: usize) -> Instruction {
        let opcode = memory[instruction_pointer];

        if opcode == 99 {
            Instruction::TERM
        } else if opcode == 1 {
            Instruction::ADD(
                memory[instruction_pointer + 1] as usize,
                memory[instruction_pointer + 2] as usize,
                memory[instruction_pointer + 3] as usize,
            )
        } else if opcode == 2 {
            Instruction::MUL {
                a: memory[instruction_pointer + 1] as usize,
                b: memory[instruction_pointer + 2] as usize,
                result: memory[instruction_pointer + 3] as usize,
            }
        } else {
            panic!();
        }
    }

    fn process(mut memory: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        loop {
            let instruction: Instruction = get_instruction(&memory, i);

            match instruction {
                Instruction::ADD(a, b, result) => {
                    memory[result] = memory[a] + memory[b];
                }
                Instruction::MUL { a, b, result } => {
                    memory[result] = memory[a] * memory[b];
                }
                Instruction::TERM => {
                    break;
                }
            }

            i += 4;
        }

        memory
    }
}
