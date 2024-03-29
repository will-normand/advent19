use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;

#[derive(Debug)]
enum Move {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

pub fn run() {
    let part = 2;
    let moves = read_file(String::from("data/day3.txt"));
    let wire0 = wire(&moves[0]);
    let wire1 = wire(&moves[1]);
    let answer: i32;
    if part == 1 {
        answer = min_distance(wire0, wire1);
    } else {
        answer = min_delay(wire0, wire1);
    };
    println!("The answer is {}", answer);
}

fn do_move((start_x, start_y): (i32, i32), current_move: &Move) -> Vec<(i32, i32)> {
    match current_move {
        Move::U(dy) => (start_y..start_y + dy).map(|y| (start_x, y + 1)).collect(),
        Move::D(dy) => (start_y - dy..start_y)
            .rev()
            .map(|y| (start_x, y))
            .collect(),
        Move::L(dx) => (start_x - dx..start_x)
            .rev()
            .map(|x| (x, start_y))
            .collect(),
        Move::R(dx) => (start_x..start_x + dx).map(|x| (x + 1, start_y)).collect(),
    }
}

fn wire(moves: &[Move]) -> Vec<(i32, i32)> {
    // moves.iter().flat_map(|m| do_move(m, current_move: &Move))
    let mut pos = (0, 0);
    let mut wire = Vec::<(i32, i32)>::new();

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
    let mut distances: Vec<i32> = common_points.map(|p| manhattan_distance(*p)).collect();
    distances.sort();
    println!("Distances are {:?}", distances);
    *distances.first().unwrap()
}

fn manhattan_distance((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
}

fn min_delay(wire1: Vec<(i32, i32)>, wire2: Vec<(i32, i32)>) -> i32 {
    let mut map1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map2: HashMap<(i32, i32), i32> = HashMap::new();
    let mut both: HashMap<(i32, i32), i32> = HashMap::new();

    // We've excluded (0,0) from the wires, so need to add 1 to get the correct length.
    for (i, pos) in wire1.iter().enumerate() {
        map1.entry(*pos)
            .or_insert_with(|| (i + 1).try_into().unwrap());
    }
    for (i, pos) in wire2.iter().enumerate() {
        map2.entry(*pos)
            .or_insert_with(|| (i + 1).try_into().unwrap());
    }
    for (k, v) in map1 {
        if let Some(v2) = map2.get(&k) {
            both.insert(k, v + v2);
        }
    }

    *both.values().min().unwrap()
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
    parse_moves(&contents)
}

fn parse_moves(moves: &str) -> Vec<Vec<Move>> {
    moves
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

    #[test]
    fn example_delays() {
        let moves = parse_moves(&String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        ));
        let wire0 = wire(&moves[0]);
        let wire1 = wire(&moves[1]);

        assert_eq!(610, min_delay(wire0, wire1));
    }
}
