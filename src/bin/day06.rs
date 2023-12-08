fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input06").unwrap();
    let nums: Vec<Vec<u64>> = input
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter_map(|x| x.trim().parse::<u64>().ok())
                .collect()
        })
        .collect();

    let nums_2: Vec<u64> = input
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .chars()
                .filter(|x| x != &' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    println!(
        "part 1: {}",
        nums[0]
            .iter()
            .zip(&nums[1])
            .map(|x| calculate_possible_times(*x.0, *x.1))
            .product::<u64>()
    );

    println!("part 2: {}", calculate_possible_times(nums_2[0], nums_2[1]));
}

fn calculate_possible_times(time: u64, distance: u64) -> u64 {
    let mut counter = 0;

    for i in 0..time {
        if i * (time - i) > distance {
            counter += 1;
        }
    }

    counter
}
