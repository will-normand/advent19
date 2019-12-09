mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod day8;

pub fn run(day: u32) {
    if day == 1 {
        let result = day1::day1();
        println!("Day 1 result is: {}", result)
    } else if day == 2 {
        day2::run();
    } else if day == 3 {
        day3::run();
    } else if day == 4 {
        day4::run();
    } else if day == 6 {
        day6::run();
    } else if day == 8 {
        day8::run();
    }
}

mod utils {
    use std::fs;

    pub fn read_file_to_string(filename: String) -> String {
        fs::read_to_string(filename)
            .expect("Failed to find file")
            .trim_end()
            .to_string()
    }
}
