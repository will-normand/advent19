pub fn run() {
    let answer = count_valid(147_981, 691_423);
    println!("The answer is {}", answer)
}

fn count_valid(from: u32, to: u32) -> u32 {
    let mut count = 0;
    for i in from..=to {
        if is_valid(i) {
            count += 1;
        }
    }

    count
}

fn is_valid(password: u32) -> bool {
    let digits: Vec<u32> = password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    valid_part_2(digits)
}

fn valid_part_2(digits: Vec<u32>) -> bool {
    let mut double = false;

    let mut i = 0;
    while i < digits.len() - 1 {
        let digit = digits.get(i);
        let next_digit = digits.get(i + 1);

        if next_digit < digit {
            return false;
        }

        if !double && (next_digit == digit) {
            // It's a double.  Need to check if that's part of a bigger group -
            // if it is we should skip it all.
            // let mut j = 2;
            double = true;
            while (i + 2 < digits.len()) && digits.get(i + 2) == next_digit {
                double = false;
                i += 1;
            }
        }
        i += 1;
    }

    double
}

#[allow(dead_code)]
fn valid_part_1(digits: Vec<u32>) -> bool {
    let mut double = false;
    let mut digits_iter = digits.iter().peekable();
    while let Some(digit) = digits_iter.next() {
        if let Some(&next_digit) = digits_iter.peek() {
            if next_digit < digit {
                return false;
            }
            if !double && (next_digit == digit) {
                double = true;
            }
        }
    }

    double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_valid_all_same() {
        assert!(!is_valid(111111));
    }

    #[test]
    fn is_not_valid_decreasing_pair() {
        assert!(!is_valid(223450));
    }

    #[test]
    fn is_not_valid_no_double() {
        assert!(!is_valid(123789));
    }

    #[test]
    fn is_valid_all_doubles() {
        assert!(is_valid(112233));
    }

    #[test]
    fn is_not_valid_no_double_but_triple() {
        assert!(!is_valid(123444));
    }

    #[test]
    fn is_valid_quad_but_double() {
        assert!(is_valid(111122));
    }
}
