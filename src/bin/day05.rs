use std::ops::Range;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

type FarmerRange = Range<i64>;

#[derive(Debug)]
struct FarmerMap {
    ranges: Vec<(FarmerRange, FarmerRange)>,
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input05").unwrap();
    let x = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = get_seeds(x[0]);

    let all_maps: Arc<Vec<FarmerMap>> = Arc::new(
        x.iter()
            .skip(1)
            .map(|x| calculate_map(x))
            .collect::<Vec<FarmerMap>>(),
    );

    println!(
        "{}",
        seeds
            .iter()
            .map(|x| calculate_min_destination(all_maps.clone(), *x))
            .min()
            .unwrap()
    );

    let mut join_handles: Vec<JoinHandle<i64>> = vec![];
    let seeds_ranged = get_seeds_ranged(x[0]);
    for seed_range in seeds_ranged.iter() {
        let all_maps_cloned = all_maps.clone();
        let seed_range = seed_range.clone();
        join_handles.push(thread::spawn(move || {
            let mut local_minimum = i64::MAX;
            for seed in seed_range.start..seed_range.end {
                let distance = calculate_min_destination(all_maps_cloned.clone(), seed);
                local_minimum = local_minimum.min(distance);
            }
            println!("{} -> min_distance = {}", seed_range.start, local_minimum,);
            local_minimum
        }));
    }

    println!("{} threads spawned", join_handles.len());
    let num_threads = join_handles.len() as i64;

    let mut min_distance: i64 = i64::MAX;
    while let Some(cur_thread) = join_handles.pop() {
        min_distance = min_distance.min(cur_thread.join().unwrap());
        println!(
            "{} threads finished, {} threads remaining",
            num_threads - join_handles.len() as i64, join_handles.len()
        );
    }

    println!("most_minimum_distance{}", min_distance);
}

fn get_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn get_seeds_ranged(seeds: &str) -> Vec<FarmerRange> {
    let mut ranged_seeds: Vec<FarmerRange> = vec![];
    let chunked = seeds.split(' ').skip(1).collect::<Vec<&str>>();

    for i in chunked.chunks(2) {
        ranged_seeds.push(FarmerRange {
            start: i[0].parse::<i64>().unwrap(),
            end: i[0].parse::<i64>().unwrap() + i[1].parse::<i64>().unwrap(),
        });
    }

    ranged_seeds
}

fn calculate_map(map: &str) -> FarmerMap {
    let splits: Vec<_> = map.split('\n').collect();

    let mut ranges: Vec<(FarmerRange, FarmerRange)> = vec![];

    splits.iter().skip(1).for_each(|x| {
        let nums = x
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        ranges.push((
            FarmerRange {
                start: nums[1],
                end: nums[1] + nums[2],
            },
            FarmerRange {
                start: nums[0],
                end: nums[0] + nums[2],
            },
        ));
    });

    FarmerMap { ranges }
}

fn calculate_min_destination(farmer_maps: Arc<Vec<FarmerMap>>, seed: i64) -> i64 {
    let mut next_location = seed;
    for farmer_map in farmer_maps.iter() {
        for range in farmer_map.ranges.iter() {
            if range.0.contains(&next_location) {
                next_location = range.1.start + (next_location - range.0.start);
                break;
            }
        }
    }
    next_location
}
