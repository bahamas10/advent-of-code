use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&fname)?;

    let mut connections = HashMap::new();
    for line in contents.trim().lines() {
        let spl: Vec<_> = line.split('-').collect();
        let a = spl[0];
        let b = spl[1];
        connections.entry(a).or_insert(HashSet::new()).insert(b);
        connections.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut answer: Option<Vec<_>> = None;
    let mut min = -1;
    for (root, neighbors) in &connections {
        let mut seen = HashMap::new();
        seen.insert(root, 1);

        for neighbor in neighbors {
            seen.insert(neighbor, 1);
        }

        for neighbor in neighbors {
            let grandneighbors = &connections[neighbor];
            for grandneighbor in grandneighbors {
                if seen.contains_key(grandneighbor) {
                    *seen.entry(grandneighbor).or_insert(0) += 1;
                }
            }
        }

//        println!("{} => {:#?}", root, seen);

        let mut counts = HashSet::new();
        for value in seen.values() {
            counts.insert(value.clone());
        }
        let mut sorted = vec![];
        for count in counts {
            sorted.push(count);
        }
        sorted.sort();
        sorted.reverse();

        // reverse the hashmap
        let mut venn = HashMap::new();
        for (key, value) in seen {
            venn.entry(value.clone()).or_insert(vec![]).push(key);
        }

//        println!("{:#?}", venn);
//        println!("{:#?}", sorted);

        let mut friends = vec![];
        for num in sorted {
            let subset = &venn[&num];
//            println!("{:#?}", subset);
            let mut clone = subset.clone();
            friends.append(&mut clone);

            if friends.len() >= num {
                // we found the biggest for this single computer
                let num = num as i64;
                if num > min {
                    min = num;
                    answer = Some(friends.clone());
                }

                break;
            }
        }

    }

    let mut answer = answer.unwrap();
    answer.sort();
    let s = answer.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",");
    println!("this won: {}", s);

    Ok(())
}
