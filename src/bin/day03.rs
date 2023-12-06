use std::cmp::{max, min};

fn get_numbers_from_a_stupid_string(s: &str) -> Vec<(i32, u32)> {
    let mut result = Vec::new();
    let mut current_number = String::new();
    let mut found_index = -1;

    for (index, curr_char) in s.char_indices() {
        if curr_char.is_numeric() {
            if found_index == -1 {
                found_index = index as i32;
            }
            current_number.push(curr_char);
        } else if !current_number.is_empty() {
            result.push((found_index, current_number.parse::<u32>().unwrap()));
            current_number.clear();
            found_index = -1;
        }
    }

    if !current_number.is_empty() {
        result.push((found_index, current_number.parse::<u32>().unwrap()));
    }

    result
}

fn check_surrounding_symbols(lines: &[&str], x: i32, y: i32) -> bool {
    lines.iter().any(|line| {
        line[max(x - 1, 0) as usize..min(x + y + 1, line.len() as i32) as usize]
            .chars()
            .any(|c| c != '.' && !c.is_numeric())
    })
}

fn count_valid_numbers_on_line(lines: &[&str], i: usize) -> u32 {
    let mut count: u32 = 0;
    let found_numbers = get_numbers_from_a_stupid_string(lines[i]);
    for (size, number) in found_numbers {
        if check_surrounding_symbols(lines, size, number.to_string().len() as i32) {
            count += number;
        }
    }
    count
}

fn get_valid_numbers(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    lines
        .windows(3)
        .map(|lines| count_valid_numbers_on_line(lines, 1))
        .sum::<u32>()
        + count_valid_numbers_on_line(&lines[0..2], 0)
        + count_valid_numbers_on_line(&lines[lines.len() - 2..lines.len()], 1)
}

fn get_adjacent_numbers(numbers: Vec<(i32, u32)>, gear_index: usize) -> Vec<(i32, u32)> {
    numbers
        .iter()
        .filter(|(size, number)| {
            let range = max(size - 1, 0)..number.to_string().len() as i32 + size + 1;
            range.contains(&(gear_index as i32))
        })
        .map(|(size, number)| (*size, *number))
        .collect::<Vec<_>>()
}

fn count_valid_gears(lines: &[&str]) -> u32 {
    let mut count: u32 = 0;

    lines[1].match_indices('*').for_each(|(gear_index, _)| {
        let adjacent_from_first_line =
            get_adjacent_numbers(get_numbers_from_a_stupid_string(lines[0]), gear_index);

        let adjacent_from_second_line =
            get_adjacent_numbers(get_numbers_from_a_stupid_string(lines[2]), gear_index);

        let adjacent_from_same_line =
            get_adjacent_numbers(get_numbers_from_a_stupid_string(lines[1]), gear_index);

        if adjacent_from_first_line.len() == 1
            && adjacent_from_second_line.len() == 1
            && adjacent_from_same_line.is_empty()
        {
            count += adjacent_from_first_line[0].1 * adjacent_from_second_line[0].1;
        }

        if adjacent_from_first_line.is_empty()
            && adjacent_from_second_line.is_empty()
            && adjacent_from_same_line.len() == 2
        {
            count += adjacent_from_same_line[0].1 * adjacent_from_same_line[1].1;
        }

        if adjacent_from_first_line.len() == 2
            && adjacent_from_second_line.is_empty()
            && adjacent_from_same_line.is_empty()
        {
            count += adjacent_from_first_line[0].1 * adjacent_from_first_line[1].1;
        }

        if adjacent_from_first_line.is_empty()
            && adjacent_from_second_line.len() == 2
            && adjacent_from_same_line.is_empty()
        {
            count += adjacent_from_second_line[0].1 * adjacent_from_second_line[1].1;
        }

        if adjacent_from_same_line.len() == 1
            && adjacent_from_first_line.len() == 1
            && adjacent_from_second_line.is_empty()
        {
            count += adjacent_from_same_line[0].1 * adjacent_from_first_line[0].1;
        }

        if adjacent_from_same_line.len() == 1
            && adjacent_from_second_line.len() == 1
            && adjacent_from_first_line.is_empty()
        {
            count += adjacent_from_same_line[0].1 * adjacent_from_second_line[0].1;
        }
    });

    count
}

fn get_valid_gears(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    lines.windows(3).map(count_valid_gears).sum::<u32>()
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input03").unwrap();
    println!("Sum of all numbers part 1: {}", get_valid_numbers(&input));

    println!("Sum of all numbers part 2: {}", get_valid_gears(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_string() {
        assert_eq!(
            get_numbers_from_a_stupid_string("1..11"),
            vec![(0, 1), (3, 11)]
        );
    }

    #[test]
    fn test_get_valid_numbers() {
        assert_eq!(get_valid_numbers("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n"), 4361);
    }

    #[test]
    fn test_get_valid_gears() {
        assert_eq!(get_valid_gears("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n"), 467835);
    }
}
