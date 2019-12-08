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
