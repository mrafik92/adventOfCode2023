use std::collections::{BTreeMap, BTreeSet};

struct Game {
    // indexed from 1
    index: u32,
    score: u32,
}

impl Game {
    fn new(index: u32, score: u32) -> Game {
        Game { index, score }
    }

    fn calculate_game_points(self) -> u32 {
        match self.score {
            0 => 0,
            _ => 2_i32.pow(self.score - 1) as u32,
        }
    }

    fn analyse_game(game: &str) -> Game {
        fn get_numbers_from_a_stupid_string(numbers: &str) -> BTreeSet<u32> {
            numbers
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<BTreeSet<_>>()
        }

        let parts: Vec<_> = game.split(':').collect();
        let game_index = parts[0]
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let nums = parts[1]
            .split('|')
            .map(get_numbers_from_a_stupid_string)
            .collect::<Vec<BTreeSet<u32>>>();
        let (winning_numbers, owned_numbers) = (nums[0].clone(), nums[1].clone());

        Game::new(
            game_index,
            winning_numbers
                .intersection(&owned_numbers)
                .collect::<BTreeSet<_>>()
                .len() as u32,
        )
    }
}

fn accumulate_winning_numbers(games: &str) -> u32 {
    games
        .lines()
        .map(Game::analyse_game)
        .map(Game::calculate_game_points)
        .sum::<u32>()
}

fn accumulate_winning_cards(games: &str) -> u32 {
    let mut initial = BTreeMap::new();
    for i in 0..games.lines().count() {
        initial.insert(i as u32 + 1, 1);
    }

    games
        .lines()
        .fold(&mut initial, |acc, card| {
            let game = Game::analyse_game(card);
            for i in game.index + 1..game.index + game.score + 1 {
                acc.insert(i, acc.get(&i).unwrap() + acc.get(&game.index).unwrap());
            }
            acc
        })
        .values()
        .sum::<u32>()
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input04").unwrap();
    println!(
        "Sum of all winning numbers: {}",
        accumulate_winning_numbers(&input)
    );
    println!(
        "Sum of all winning cards: {}",
        accumulate_winning_cards(&input)
    );
}
