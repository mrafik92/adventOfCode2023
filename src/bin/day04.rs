use std::collections::{BTreeMap, BTreeSet};

fn calculate_winning_numbers(line: &str) -> (u32, BTreeSet<u32>) {

    fn get_numbers_from_a_stupid_string(numbers: &str) -> BTreeSet<u32> {
        numbers.split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<BTreeSet<_>>()
    }

    let parts: Vec<_> = line.split(':').collect();
    let game_index = parts[0].chars().filter(|x| x.is_numeric()).collect::<String>().parse::<u32>().unwrap();

    let nums = parts[1].split('|').map(get_numbers_from_a_stupid_string).collect::<Vec<BTreeSet<u32>>>();
    let (winning_numbers, owned_numbers) = (nums[0].clone(), nums[1].clone());

    (game_index, winning_numbers
        .intersection(&owned_numbers).copied()
        .collect::<BTreeSet<_>>())
}

fn calculate_sum(i: u32) -> u32 {
    match i {
        0 => 0,
        _ => 2_i32.pow(i - 1) as u32
    }
}

fn accumulate_winning_numbers(games: &str) -> u32 {
    games.lines().map(calculate_winning_numbers).map(|x| calculate_sum(x.1.len() as u32)).sum::<u32>()
}

fn accumulate_winning_cards(games: &str) -> u32 {
    let mut initial = BTreeMap::new();
    for i in 0..games.lines().count() {
        initial.insert(i as u32 + 1, 1);
    }

    games.lines().fold(&mut initial, |acc, line| {
        let numbers = calculate_winning_numbers(line);
        for i in numbers.0 + 1..numbers.0 + numbers.1.len() as u32 + 1 {
            acc.insert(i, acc.get(&i).unwrap() + acc.get(&numbers.0).unwrap());
        }
        acc
    }).values().sum::<u32>()
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input04").unwrap();
    println!("Sum of all winning numbers: {}", accumulate_winning_numbers(&input));
    println!("Sum of all winning cards: {}", accumulate_winning_cards(&input));
}
