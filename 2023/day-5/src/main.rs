use core::panic;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct MappingDescription {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

fn extract_numbers(line: String) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect()
}

fn map_seed(mut seed: &i64, mut mappings: &Vec<Vec<i64>>) -> i64 {
    let mut new_seed = *seed;
    for mapping in mappings.iter() {
        if mapping[1] <= new_seed && new_seed < mapping[1] + mapping[2] {
            new_seed = mapping[0] + (new_seed - mapping[1]);
            return new_seed;
        }
    }
    new_seed
}

fn split_seeds<'a>(
    mut seed_descriptions: &Vec<Vec<i64>>,
    mut mappings: &Vec<Vec<i64>>,
) -> Vec<Vec<i64>> {
    // println!("Seed descriptions {:?}", seed_descriptions);
    // println!("Mappings {:?}", mappings);
    println!("{:?}", "-".repeat(40));
    println!("{:?}", "-".repeat(40));

    let mut new_seed_descriptions = vec![];
    let mut unchecked_seeds = seed_descriptions.clone();
    let mut checked_seeds: Vec<Vec<i64>> = vec![];
    let mut changed_seeds: Vec<Vec<i64>> = vec![];
    while !unchecked_seeds.is_empty() {
        let seeds = unchecked_seeds[0].clone();
        println!("{:?}", "*".repeat(20));
        println!("In seed {:?}", seeds);
        let mut changed = 0;
        for mapping in mappings {
            println!("{:?}", "-".repeat(10));
            println!("In mapping {:?}", mapping);

            let s_start_loc = seeds[0];
            let s_last_loc = seeds[0] + seeds[1] - 1;
            let s_dist = seeds[1];

            let dm_start_loc = mapping[0];
            let dm_last_loc = mapping[0] + mapping[2] - 1;

            let sm_start_loc = mapping[1];
            let sm_last_loc = mapping[1] + mapping[2] - 1;

            let m_dist = mapping[2];

            // println!("s_start_loc: {:?}",s_start_loc);
            println!("s_last_loc: {:?}",s_last_loc);
            // println!("s_dist: {:?}",s_dist);

            println!("dm_start_loc: {:?}",dm_start_loc);
            println!("dm_last_loc: {:?}",dm_last_loc);

            // println!("sm_start_loc: {:?}",sm_start_loc);
            println!("sm_last_loc: {:?}",sm_last_loc);

            println!("m_dist: {:?}",m_dist);

            if s_last_loc < sm_start_loc {
                // checked_seeds.push(vec![s_start_loc, s_dist]);
                println!("1");
                continue
            } else if sm_last_loc < s_start_loc {
                // checked_seeds.push(vec![s_start_loc, s_dist]);
                println!("2");
                continue
            } else if s_start_loc == sm_start_loc {
                if s_last_loc <= sm_last_loc {
                    changed_seeds.push(vec![dm_start_loc, s_dist]);
                    changed += 1;
                    println!("3");
                    break
                } else {
                    changed_seeds.push(vec![dm_start_loc, m_dist]);
                    unchecked_seeds.push(vec![sm_last_loc + 1, s_dist - m_dist]);
                    changed += 1;
                    println!("4");
                    break
                }
            } else if s_start_loc < sm_start_loc {
                unchecked_seeds.push(vec![s_start_loc, sm_start_loc - s_start_loc]);
                if s_last_loc <= sm_last_loc {
                    changed_seeds.push(vec![dm_start_loc, s_last_loc - sm_start_loc + 1]);
                    changed += 1;
                    println!("5");
                    break
                } else {
                    changed_seeds.push(vec![dm_start_loc, m_dist]);
                    unchecked_seeds.push(vec![sm_last_loc + 1, s_last_loc - sm_last_loc]);
                    changed += 1;
                    println!("6");
                    break
                }
            } else if sm_start_loc < s_start_loc {
                if s_last_loc <= sm_last_loc {
                    changed_seeds.push(vec![dm_start_loc + s_start_loc - sm_start_loc, s_dist]);
                    changed += 1;
                    println!("7");
                    break
                } else {
                    changed_seeds.push(vec![
                        dm_start_loc + s_start_loc - sm_start_loc,
                        sm_last_loc - s_start_loc + 1,
                    ]);
                    unchecked_seeds.push(vec![sm_last_loc + 1, s_last_loc - sm_last_loc]);
                    changed += 1;
                    println!("8");
                    break
                }
            } else {
                panic!("WHAT THE FUCK HAOPPOPPEND");
            }

        }
        if changed == 0 { checked_seeds.push(seeds) };
        unchecked_seeds.remove(0);
    }
    println!("old : {:?}", seed_descriptions);
    new_seed_descriptions.extend(checked_seeds);
    new_seed_descriptions.extend(changed_seeds);
    println!("new: {:?}", new_seed_descriptions);
    assert_eq!(
        seed_descriptions.iter().map(|v| v[1] as i64).sum::<i64>(),
        new_seed_descriptions
            .iter()
            .map(|v| v[1] as i64)
            .sum::<i64>()
    );
    return new_seed_descriptions;
}

fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(sample_input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let mut seeds = lines
        .next()
        .ok_or("No lines")?
        .map(extract_numbers)
        .unwrap();

    // let mut more_seeds: Vec<i64> = vec![];

    let mut more_seeds = seeds
        .chunks(2)
        .map(|c| c.iter().map(|s| *s).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut new_block = false;
    let mut mappings: Vec<Vec<i64>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            for seed in &mut seeds {
                *seed = map_seed(seed, &mappings)
            }
            // for seed in &mut more_seeds {
            //     *seed = map_seed(seed, &mappings)
            // }
            if !mappings.is_empty() {
                more_seeds = split_seeds(&more_seeds, &mappings);
            }
            mappings.clear();
            continue;
        }
        let numbers = extract_numbers(line);
        if !numbers.is_empty() {
            mappings.push(numbers);
        }
    }
    for seed in &mut seeds {
        *seed = map_seed(seed, &mappings)
    }
    // for seed in &mut more_seeds {
    //     *seed = map_seed(seed, &mappings)
    // }
    mappings.clear();

    p1 = *seeds.iter().min().unwrap();
    p2 = more_seeds
        .iter()
        .filter_map(|v| if v.is_empty() { None } else { Some(v[0]) })
        .min()
        .unwrap();

    println!("More seeds {:?}", more_seeds);

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
