//function to get first digit from a string
fn get_first_digit(s: &str) -> u32 {
    s.chars().skip_while(|p| !p.is_digit(10)).next().unwrap().to_digit(10).unwrap()
}

//function to get last digit from a string
fn get_last_digit(s: &str) -> u32 {
    s.chars().rev().skip_while(|p| !p.is_digit(10)).next().unwrap().to_digit(10).unwrap()
}

// function to get the concatenation of the first and last digit
fn get_concatenation(s: &str) -> u32 {
    let first_digit = get_first_digit(s);
    let last_digit = get_last_digit(s);
    let concatenation = format!("{}{}", first_digit, last_digit);
    concatenation.parse::<u32>().unwrap()
}

fn main() {
    // read file
    let input = std::fs::read_to_string("input").unwrap();
    let sum = input.lines().map(get_concatenation).sum::<u32>();
    println!("Sum of all numbers: {}", sum);
}

// create test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("abc123"), 1);
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("abc123"), 3);
    }
}