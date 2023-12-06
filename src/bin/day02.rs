use std::collections::HashMap;

const MAX_VALUES: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn is_game_valid(game: &str) -> bool {
    game.split(';').all(|x| {
        x.split(',').all(|x| {
            let split: Vec<_> = x.split(' ').collect();
            match MAX_VALUES.iter().find(|&&y| y.0 == split[2]) {
                Some(&(_, max)) => split[1].parse::<i32>().unwrap() <= max,
                None => false,
            }
        })
    })
}

fn get_game_number(s: &str) -> i32 {
    s.split(' ').nth(1).unwrap().parse::<i32>().unwrap()
}

fn check_game_turn(s: &str) -> i32 {
    let splits: Vec<_> = s.split(':').collect();
    match is_game_valid(splits[1]) {
        true => get_game_number(splits[0]),
        false => 0,
    }
}

// input is "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn get_minimum_required_cubes(s: &str) -> i32 {
    // create a map between strings and numbers
    let mut minimums: HashMap<&str, i32> = HashMap::new();

    s.split(';').for_each(|x| {
        x.split(',')
            .map(|x| {
                let split = x.split(' ').collect::<Vec<&str>>();
                let (cube_number, cube_color) = (split[1].parse::<i32>().unwrap(), split[2]);
                minimums.insert(
                    cube_color,
                    *minimums.get(cube_color).unwrap_or(&0).max(&cube_number),
                );
            })
            .max()
            .unwrap()
    });

    minimums.iter().map(|(_, &v)| v).product::<i32>()
}

fn get_power_set(s: &str) -> i32 {
    get_minimum_required_cubes(s.split(':').nth(1).unwrap())
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input02").unwrap();
    let sum = input.lines().map(check_game_turn).sum::<i32>();
    println!("Sum of all game turns part 1: {}", sum);

    let sum_power_sets = input.lines().map(get_power_set).sum::<i32>();

    println!("Sum of all game turns part 2: {}", sum_power_sets);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_game_turn() {
        assert_eq!(
            check_game_turn("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            check_game_turn("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            check_game_turn(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            0
        );
        assert_eq!(
            check_game_turn(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            0
        );
        assert_eq!(
            check_game_turn("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            5
        );
    }

    #[test]
    fn test_get_minimum_required_cubes() {
        assert_eq!(
            get_power_set("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            get_power_set("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            get_power_set(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            1560
        );
        assert_eq!(
            get_power_set(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            630
        );
        assert_eq!(
            get_power_set("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }
}
