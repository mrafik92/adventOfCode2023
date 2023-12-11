use std::collections::HashMap;
use regex::Regex;

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input08").unwrap();
    let directions = input.lines().next().unwrap().chars().map(|x| match x {
        'R' => 1,
        'L' => 0,
        _ => panic!("unknown direction")
    }).collect::<Vec<_>>();

    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();


    let map: Vec<_> = input.lines().skip(2).map(|x| {
        let mut results = vec![];

        for (_a, [x, xx, xxx]) in re.captures_iter(x).map(|c| c.extract::<3>()) {
            results.push(x);
            results.push(xx);
            results.push(xxx);
        }
        results
    }).collect();

    let mut initial_map = HashMap::new();
    map.iter().fold(&mut initial_map, |acc, entry| {
        acc.insert(entry[0], vec![entry[1], entry[2]]);
        acc
    });

    let mut current_location = initial_map.get(&"AAA");
    let mut counter : u128 = 0;
    let mut loc = &"AAA";
    while loc != &"ZZZ"    {
        let direction = directions[(counter % directions.len() as u128) as usize];
        counter += 1;
        loc = current_location.unwrap().get(direction as usize).unwrap();
        current_location = initial_map.get(loc);
    }

    println!("part1: {counter}");

    let mut start_entries : Vec<&str> = vec![];
    for entry in &initial_map {
        if entry.0.ends_with('A') {
            start_entries.push(entry.0);
        }
    }
    println!("entries: {:?}", start_entries);

    counter = 0;
    while !start_entries.iter().all(|loc| loc.ends_with('Z')) {
        let direction = directions[(counter % directions.len() as u128) as usize];
        counter += 1;
        for loc in start_entries.iter_mut() {
            *loc = initial_map.get(loc).unwrap().get(direction as usize).unwrap();
        }

        if start_entries.iter().any(|loc| loc.ends_with('Z')) {
            println!("found one; {:?}, {}", start_entries, counter);
        }
    }


    println!("part2: {counter}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let x = "AAA = (BBB, CCC)";
        let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();

        match re.captures(x) {
            None => {}
            Some(xx) => {
                println!("{:?}", xx);
                let mut results = vec![];

                xx.iter().skip(1).for_each(|x|
                    match x {
                        None => {}
                        Some(y) => { results.push(y.as_str()) }
                    });

                println!("{:?}", results)
            }
        }
    }

    #[test]
    fn test_vector_modifications() {
        let mut vector = vec![1, 2, 3, 4];

        for i in vector.iter_mut() {
            *i += 1;
        }

        println!("{:?}", vector)
    }
}
