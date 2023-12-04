fn get_first_last_number(s: &str) -> u32 {
    let v: Vec<&str> = s.matches(char::is_numeric).collect();

    format!("{}{}", v[0], v[v.len() - 1])
        .parse::<u32>()
        .unwrap()
}

fn get_first_last_spelled_number(s: &str) -> u32 {
    let s = s
        .replace("one", "on1e")
        .replace("two", "tw2o")
        .replace("three", "thre3e")
        .replace("four", "fou4r")
        .replace("five", "fiv5e")
        .replace("six", "si6x")
        .replace("seven", "seve7n")
        .replace("eight", "eigh8t")
        .replace("nine", "nin9e");

    get_first_last_number(&s)
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input01").unwrap();
    let sum = input.lines().map(get_first_last_number).sum::<u32>();
    println!("Sum of all numbers part 1: {}", sum);

    let sum = input
        .lines()
        .map(get_first_last_spelled_number)
        .sum::<u32>();
    println!("Sum of all numbers part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_last_number() {
        assert_eq!(get_first_last_number("abc123abbb3"), 13);
        assert_eq!(get_first_last_number("abc1abbb"), 11);
    }

    #[test]
    fn test_get_first_last_spelled_number() {
        assert_eq!(get_first_last_spelled_number("two1nine"), 29);
        assert_eq!(get_first_last_spelled_number("eightwothree"), 83);
        assert_eq!(get_first_last_spelled_number("abcone2threexyz"), 13);
        assert_eq!(get_first_last_spelled_number("xtwone3four"), 24);
        assert_eq!(get_first_last_spelled_number("4nineeightseven2"), 42);
        assert_eq!(get_first_last_spelled_number("zoneight234"), 14);
        assert_eq!(get_first_last_spelled_number("7pqrstsixteen"), 76);
    }
}
