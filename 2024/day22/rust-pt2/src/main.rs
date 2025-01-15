use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

use anyhow::Result;

fn evolve(mut secret: i64, iterations: usize) -> Vec<(i64, Option<i64>)>{
    let mut action = None;
    let mut last = None;

    let mut results = vec![];

    for _ in 0..iterations {
        // Calculate the result of multiplying the secret number by 64. Then, mix
        // this result into the secret number. Finally, prune the secret number.
        let result= secret * 64;
        secret ^= result;
        secret &= (1 << 24) -1;

        // Calculate the result of dividing the secret number by 32. Round the
        // result down to the nearest integer. Then, mix this result into the
        // secret number.  Finally, prune the secret number.
        let result = secret / 32;
        secret ^= result;
        secret &= (1 << 24) -1;

        // Calculate the result of multiplying the secret number by 2048. Then,
        // mix this result into the secret number. Finally, prune the secret
        // number.
        let result= secret * 2048;
        secret ^= result;
        secret &= (1 << 24) -1;

        // calculate price and action
        let price = secret % 10;
        if let Some(last) = last {
            action = Some(price - last);
        }
        last = Some(price);

        results.push((price, action));
    }
    results
}

fn main() -> Result<()> {
    let file = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&file)?;

    let mut solutions = HashMap::new();
    for line in contents.trim().lines() {
        let num: i64 = line.parse().unwrap();

        let results = evolve(num, 2000);

        let mut actions = VecDeque::new();
        let mut seen = HashSet::new();
        for result in results {
            let price = result.0;
            let Some(action) = result.1 else {
                continue;
            };

            actions.push_back(action);
            if actions.len() > 4 {
                actions.pop_front();
            }
            if actions.len() == 4 {
                if seen.contains(&actions) {
                    continue;
                }
                seen.insert(actions.clone());

                // do something
                let i = solutions.get(&actions).unwrap_or(&0);
                let mut i = i.clone();
                i += price;
                solutions.insert(actions.clone(), i);
            }
        }

    }

    println!("we found {} solutions", solutions.len());

    let mut max = 0;
    let mut f = None;
    for (m, n) in solutions {
        if n > max {
            max = n;
            f = Some(m.clone());
        }
    }
    println!("winner is {:?} at {} bananas", f.unwrap(), max);
    Ok(())
}

