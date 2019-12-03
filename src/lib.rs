pub fn run(day: u32) {
    if day == 1 {
        let result = day1::day1();
        println!("Day 1 result is: {}", result)
    } else if day == 2 {
        day2::run();
    } else if day == 3 {
        day3::run();
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

mod day3 {
    use std::collections::HashSet;
    use std::fs;

    #[derive(Debug)]
    enum Move {
        U(i32),
        D(i32),
        L(i32),
        R(i32),
    }

    pub fn run() {
        let moves = read_file(String::from("data/day3.txt"));
        let wire0 = wire(&moves[0]);
        let wire1 = wire(&moves[1]);
        let answer = min_distance(wire0, wire1);
        println!("The answer is {}", answer);
    }

    fn do_move((start_x, start_y): (i32, i32), current_move: &Move) -> Vec<(i32, i32)> {
        match current_move {
            Move::U(dy) => (start_y..start_y + dy).map(|y| (start_x, y + 1)).collect(),
            Move::D(dy) => {
                let mut moves: Vec<(i32, i32)> =
                    (start_y - dy..start_y).map(|y| (start_x, y)).collect();
                moves.reverse();
                moves
            }
            Move::L(dx) => {
                let mut moves: Vec<(i32, i32)> =
                    (start_x - dx..start_x).map(|x| (x, start_y)).collect();
                moves.reverse();
                moves
            }
            Move::R(dx) => (start_x..start_x + dx).map(|x| (x + 1, start_y)).collect(),
        }
    }

    fn wire(moves: &[Move]) -> Vec<(i32, i32)> {
        // moves.iter().flat_map(|m| do_move(m, current_move: &Move))
        let mut pos = (0, 0);
        let mut wire = Vec::<(i32, i32)>::new(); //vec![pos];

        for m in moves {
            let mut next_bit = do_move(pos, &m);
            pos = *next_bit.last().unwrap();
            wire.append(&mut next_bit);
        }
        wire
    }

    fn min_distance(wire1: Vec<(i32, i32)>, wire2: Vec<(i32, i32)>) -> i32 {
        let set1: HashSet<(i32, i32)> = wire1.into_iter().collect();
        let set2: HashSet<(i32, i32)> = wire2.into_iter().collect();
        let common_points = set1.intersection(&set2);
        println!("Common_points are {:?}", common_points);
        let mut distances: Vec<i32> = common_points.map(|p| manhattan_distance(*p)).collect();
        distances.sort();
        println!("Distances are {:?}", distances);
        *distances.first().unwrap()
    }

    fn manhattan_distance((x, y): (i32, i32)) -> i32 {
        x.abs() + y.abs()
    }

    fn move_from_sting(move_str: &str) -> Move {
        let m = move_str.as_bytes().first().unwrap();
        let dist = move_str[1..].parse::<i32>().unwrap();
        if m == &b'U' {
            Move::U(dist)
        } else if m == &b'D' {
            Move::D(dist)
        } else if m == &b'L' {
            Move::L(dist)
        } else if m == &b'R' {
            Move::R(dist)
        } else {
            panic!("Cannot parse move {}, dist {}", m, dist);
        }
    }

    fn read_file(filename: String) -> Vec<Vec<Move>> {
        let contents = fs::read_to_string(filename).expect("Failed to find file");
        contents
            .trim_end()
            .split('\n')
            .map(|wire_str| {
                wire_str
                    .split(',')
                    .map(|move_str| move_from_sting(move_str))
                    .collect()
            })
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn moving_up() {
            assert_eq!(vec![(1, 1), (1, 2), (1, 3)], do_move((1, 0), &Move::U(3)));
        }

        #[test]
        fn moving_left() {
            assert_eq!(vec![(-1, 0)], do_move((0, 0), &Move::L(1)));
        }

        #[test]
        fn wee_wire() {
            let moves = vec![Move::L(1), Move::D(2)];
            let expected_wire = vec![(-1, 0), (-1, -1), (-1, -2)];
            assert_eq!(wire(&moves), expected_wire);
        }

        #[test]
        fn manhattan() {
            assert_eq!(3, manhattan_distance((-2, 1)));
        }

        #[test]
        fn example_wire() {
            use Move::*;

            let m1 = vec![R(8), U(5), L(5), D(3)];
            let m2 = vec![U(7), R(6), D(4), L(4)];
            assert_eq!(6, min_distance(wire(&m1), wire(&m2)));
        }
    }
}
