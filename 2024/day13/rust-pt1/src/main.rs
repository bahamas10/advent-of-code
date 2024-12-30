use std::collections::HashSet;
use std::env;
use std::fs;

use anyhow::{Context, Result};
use gcd::Gcd;
use regex::Regex;
use rayon::prelude::*;

#[derive(Debug)]
struct ClawMachine {
    pub a_button: (i64, i64),
    pub b_button: (i64, i64),
    pub prize: (i64, i64),
}

impl ClawMachine {
    pub fn get_lowest_price(&self) -> Option<i64> {
        /*
        let max_ax = self.prize.0 / self.a_button.0;
        let max_bx = self.prize.0 / self.b_button.0;
        let max_ay = self.prize.1 / self.a_button.1;
        let max_by = self.prize.1 / self.b_button.1;
        */

        let x_gcd = (self.a_button.0 as u64).gcd(self.b_button.0 as u64) as i64;
        if self.prize.0 % x_gcd != 0 {
            return None;
        }
        let y_gcd = (self.a_button.1 as u64).gcd(self.b_button.1 as u64) as i64;
        if self.prize.1 % y_gcd != 0 {
            return None;
        }

        let mut valid = HashSet::new();
        // (prize - a button times) / b button times

        // (8400 - 0*94) / 22 = 381.xxxxx (has remainder is what we want)
        // (8400 - 1*94) / 22 = 377.xxxxx
        // (8400 - 2*94) / 22 = ...
        // this is how many times we need to try the a button

        // find the first valid guess
        let mut i = 0;
        let mut found: Option<i64> = None;
        loop {
            //            println!("{}/{}", i, max_ax);
            if self.prize.0 - (i * self.a_button.0) < 0 {
                break;
            }

            let res = (self.prize.0 - (i * self.a_button.0)) % self.b_button.0;
            if res == 0 {
                if found.is_none() {
                    found = Some(i);
                    break;
                }
            }

            i += 1;
        }

        // figure out how much to increment by
        println!("found = {:#?}", found);
        let x_gcd = (self.a_button.0 as u64).gcd(self.b_button.0 as u64);
        let incr = self.b_button.0 as u64 / x_gcd;
        let incr = incr as i64;

        let found = found?;
        println!("incr by {}", incr);

        // generate all valid solutions
        let mut i = found;
        println!("{}", self.prize.0);
        loop {
            let j = (self.prize.0 - i * self.a_button.0) / self.b_button.0;
            if j < 0 {
                break;
            }
            //println!("{},{}", i, j);

            // since are here, check to make sure this works in the y direction
            // too
            if self.prize.1 == i * self.a_button.1 + j * self.b_button.1 {
                valid.insert((i, j));
            }

            i += incr;
        }

        println!("found {} to test", valid.len());
        if valid.len() == 0 {
            return None;
        }

        let mut lowest_price = i64::MAX;
        for v in valid {
            let price = v.0 * 3 + v.1;
            if price < lowest_price {
                lowest_price = price;
            }
        }

        Some(lowest_price)
    }
}

fn main() -> Result<()> {
    let file = env::args().nth(1).context("need file as arg1")?;
    let input = fs::read_to_string(file)?;

    // ex: Button A: X+69, Y+23
    let button_re =
        Regex::new(r"^Button [AB]: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    // ex: Prize: X=8400, Y=5400
    let prize_re = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();

    let lines: Vec<_> = input.lines().collect();
    let mut i = 0;

    let mut machines = vec![];
    while i < lines.len() {
        let line1 = &lines[i];
        let line2 = &lines[i + 1];
        let line3 = &lines[i + 2];

        // button a
        let m = button_re.captures(line1).unwrap();
        let x: i64 = m[1].parse().unwrap();
        let y: i64 = m[2].parse().unwrap();
        let a_button = (x, y);

        // button b
        let m = button_re.captures(line2).unwrap();
        let x: i64 = m[1].parse().unwrap();
        let y: i64 = m[2].parse().unwrap();
        let b_button = (x, y);

        // prize
        let m = prize_re.captures(line3).unwrap();
        let x: i64 = m[1].parse().unwrap();
        let y: i64 = m[2].parse().unwrap();
        let prize = (x + 10000000000000, y + 10000000000000);
        //        let prize = (x, y);

        let machine = ClawMachine { a_button, b_button, prize };
        machines.push(machine);
        i += 4;
    }

    println!("found {} machines", machines.len());

    let final_price: i64 = machines
        .par_iter()
        .filter_map(|machine| machine.get_lowest_price())
        .sum();

    println!("final price = {}", final_price);

    Ok(())
}
