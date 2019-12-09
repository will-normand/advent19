use super::utils;

pub fn run() {
    let contents = utils::read_file_to_string("data/day8.txt".to_string());
    let image_layers = load_image(&contents);

    let layers = image_layers
        .iter()
        .map(|layer: &&str| (count_char(&layer, '0'), layer));

    let (_, min) = layers.min_by_key(|(c, _)| *c).unwrap();
    let ones = count_char(&min, '1');
    let twos = count_char(&min, '2');
    let answer = ones * twos;
    println!("Ones {}, twos {}.  The answer is {}", ones, twos, answer);
}

fn count_char(s: &str, target: char) -> usize {
    s.chars().filter(|&c| -> bool { target == c }).count()
}

fn load_image(contents: &str) -> Vec<&str> {
    let mut slices: Vec<&str> = Vec::new();
    let mut prev_index = 0;

    for i in 1..contents.len() {
        if i % (25 * 6) == 0 {
            slices.push(&contents[prev_index..i]);
            prev_index = i;
        }
    }

    slices
}
