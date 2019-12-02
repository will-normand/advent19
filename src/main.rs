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

    pub fn run() {
        let mut input = load_file("data/day2.txt".to_string());
        input[1] = 12;
        input[2] = 2;

        process(input);
    }

    fn load_file(filename: String) -> Vec<i32> {
        let opcodes = fs::read_to_string(filename).expect("Failed to find file");
        opcodes
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    fn process(mut tape: Vec<i32>) {
        let mut i = 0;
        loop {
            let opcode = tape[i];

            if opcode == 99 {
                break;
            }

            let a: usize = tape[i + 1] as usize;
            let b: usize = tape[i + 2] as usize;
            let c: usize = tape[i + 3] as usize;

            println!("opcode {}, a {}, b {}, c {}", opcode, a, b, c);

            if opcode == 1 {
                tape[c] = tape[a] + tape[b];
            } else if opcode == 2 {
                tape[c] = tape[a] * tape[b];
            }

            i += 4;
        }
        for i in tape {
            print!("{},", i);
        }
    }
}
