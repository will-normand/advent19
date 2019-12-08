
use super::utils;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
struct Orbit {
    object: String,
    orbits: Option<Rc<Orbit>>,
}

pub fn run() {
    let orbits = load_orbits(utils::read_file_to_string("data/day6.txt".to_string()));
    let data_map = make_map(orbits);
    let orbit_count = count_orbits(&data_map);
    let you = orbits_as_list("YOU".to_string(), &data_map);
    let san = orbits_as_list("SAN".to_string(), &data_map);
    let transfers = intersection_distance(&you, &san);

    println!(
        "The orbit count is {}, transfers is {}",
        orbit_count, transfers
    );
}

fn load_orbits(orbits_string: String) -> Vec<(String, String)> {
    orbits_string
        .split('\n')
        .map(|s| {
            let split: Vec<&str> = s.split(')').collect();
            (split[0].to_string(), split[1].to_string())
        })
        .collect()
}

fn make_map(raw_orbits: Vec<(String, String)>) -> HashMap<String, Rc<Orbit>> {
    let mut orbits: HashMap<String, Rc<Orbit>> = HashMap::new();

    orbits.insert(
        "COM".to_string(),
        Rc::new(Orbit {
            object: "COM".to_string(),
            orbits: None,
        }),
    );

    let mut missing_orbits: Vec<(String, String)> = Vec::new();
    let mut attempting_orbits: Vec<(String, String)> = raw_orbits;

    while !attempting_orbits.is_empty() {
        for (orbit_name, name) in &attempting_orbits {
            match orbits.get(orbit_name) {
                Some(existing_orbit) => {
                    let new_orbit = Orbit {
                        object: name.to_string(),
                        orbits: Some(Rc::clone(&existing_orbit)),
                    };
                    orbits.insert(name.to_string(), Rc::new(new_orbit));
                }
                None => {
                    missing_orbits.push((orbit_name.to_string(), name.to_string()));
                }
            }
        }

        attempting_orbits = missing_orbits.clone();
        missing_orbits.clear();
    }
    orbits
}

fn count_indirect_orbits(object_name: String, map_data: &HashMap<String, Rc<Orbit>>) -> u32 {
    let object: &Rc<Orbit> = &map_data[&object_name];
    let mut count = 0;

    let mut orbit = object.orbits.clone();
    while orbit.is_some() {
        count += 1;
        orbit = orbit.unwrap().orbits.clone();
    }

    count
}

fn count_orbits(map_data: &HashMap<String, Rc<Orbit>>) -> u32 {
    map_data
        .keys()
        .map(|k| count_indirect_orbits(k.to_string(), &map_data))
        .sum()
}

fn orbits_as_list(object_name: String, map_data: &HashMap<String, Rc<Orbit>>) -> Vec<String> {
    let mut orbit_list = Vec::new();
    let object: &Rc<Orbit> = &map_data[&object_name];

    let mut orbit = object.orbits.clone();

    while orbit.is_some() {
        let current = orbit.unwrap();
        let name: String = current.object.clone().to_string();
        orbit_list.push(name);
        orbit = current.orbits.clone();
    }

    orbit_list
}

fn intersection_distance(you: &[String], san: &[String]) -> u32 {
    let mut min = u32::max_value();

    for (yi, y) in you.iter().enumerate() {
        if let Some(si) = san.iter().position(|s| s == y) {
            let distance = (si + yi) as u32;
            if distance < min {
                min = distance;
            }
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_orbits_parses_string() {
        let expected: Vec<(String, String)> = vec![
            ("COM".to_string(), "A".to_string()),
            ("A".to_string(), "B".to_string()),
        ];
        let actual = load_orbits("COM)A\nA)B".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn making_orbits() {
        let mut orbits_map: HashMap<String, Rc<Orbit>> = HashMap::new();

        let orbit_com = Rc::new(Orbit {
            object: "COM".to_string(),
            orbits: None,
        });

        orbits_map.insert("COM".to_string(), orbit_com);

        let orbit_a = Rc::new(Orbit {
            object: "A".to_string(),
            orbits: Some(Rc::clone(&orbits_map[&"COM".to_string()])),
        });

        orbits_map.insert("A".to_string(), orbit_a);

        assert_eq!(
            Some(Rc::new(Orbit {
                object: "COM".to_string(),
                orbits: None,
            })),
            orbits_map[&"A".to_string()].orbits
        )
    }

    #[test]
    fn making_maps() {
        let input: Vec<(String, String)> = vec![
            ("COM".to_string(), "A".to_string()),
            ("A".to_string(), "B".to_string()),
        ];

        let result = make_map(input);
        println!("RESULT IS {:?}", result);
    }

    #[test]
    fn count_individual_orbits() {
        let input: Vec<(String, String)> = vec![
            ("COM".to_string(), "A".to_string()),
            ("A".to_string(), "B".to_string()),
        ];
        let data_map = make_map(input);

        assert_eq!(0, count_indirect_orbits("COM".to_string(), &data_map));
        assert_eq!(1, count_indirect_orbits("A".to_string(), &data_map));
        assert_eq!(2, count_indirect_orbits("B".to_string(), &data_map));
    }

    #[test]
    fn count_all_orbits() {
        let input: Vec<(String, String)> = vec![
            ("COM".to_string(), "A".to_string()),
            ("A".to_string(), "B".to_string()),
        ];
        let data_map = make_map(input);

        assert_eq!(3, count_orbits(&data_map));
    }

    #[test]
    fn count_orbits_example() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        let actual = count_orbits(&make_map(load_orbits(input.to_string())));
        assert_eq!(42, actual);
    }

    #[test]
    fn distance_example() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

        let data_map = make_map(load_orbits(input.to_string()));
        let you = orbits_as_list("YOU".to_string(), &data_map);
        let san = orbits_as_list("SAN".to_string(), &data_map);
        let actual = intersection_distance(&you, &san);
        println!("{:?}", actual);
        assert_eq!(4, actual);
    }
}
