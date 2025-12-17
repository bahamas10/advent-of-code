use std::env;
use std::fs;
use std::collections::HashMap;

use anyhow::Result;
use log::debug;

type Cache = HashMap<Vec<i64>, Option<i64>>;

#[derive(Debug)]
struct Machine {
    joltages: Vec<i64>,
    buttons: Vec<i64>,
}

impl Machine {
    fn is_solved(&self) -> bool {
        for joltage in self.joltages.iter() {
            if *joltage != 0 {
                return false;
            }
        }
        true
    }

    fn is_valid(&self) -> bool {
        for joltage in self.joltages.iter() {
            if *joltage < 0 {
                return false;
            }
        }
        true
    }

    fn solve(&self, cache: &mut Cache) -> Option<i64> {
        if self.is_solved() {
            debug!("solved");
            return Some(0);
        }
        if !self.is_valid() {
            return None;
        }
        if let Some(v) = cache.get(&self.joltages) {
            return *v;
        }

        // figure out light form (mod 2) of the joltage to figure out what to do
        // next
        let lights = to_lights(&self.joltages);
        debug!("lights = {:?}", lights);

        let has_lights_on = lights.iter().any(|l| *l);

        if !has_lights_on {
            debug!("no lights on, halfing");
            // all lights are off! let's try to divide by 2 for funsies
            let half: Vec<_> = self.joltages.iter().map(|d| d / 2).collect();

            let machine =
                Machine { joltages: half, buttons: self.buttons.clone() };

            if let Some(n) = machine.solve(cache).map(|n| n * 2) {
                cache.insert(self.joltages.clone(), Some(n));
                cache.insert(machine.joltages.clone(), Some(n / 2));
                return Some(n);
            }

            debug!("dividing in half didnt work lol");
            //return None;

            // try pressing every button randomly? this sucks lol
            let mut best = None;
            for button in &self.buttons {
                let mut joltages = self.joltages.clone();
                for i in 0..lights.len() {
                    let base: i64 = 2;
                    let idx = base.pow((lights.len() - i - 1) as u32);
                    if idx & button > 0 {
                        joltages[i] -= 1
                    }
                }
                let machine = Machine {
                    joltages,
                    buttons: self.buttons.clone(),
                };

                let o = machine.solve(cache);
                cache.insert(machine.joltages.clone(), o);
                if let Some(solved) = o {
                    let n = solved + 1;
                    match best {
                        Some(thing) => {
                            if n < thing {
                                best = Some(n);
                            }
                        }
                        None => {
                            best = Some(n);
                        }
                    }
                };
            }
            best
        } else {
            debug!("some lights on");
            // let's try to get to all lights off state
            // find fastest paths
            let paths = find_paths_to_lights_off(&lights, &self.buttons);

            debug!("found {} paths to lights off", paths.len());

            let mut best = None;

            // simulate all paths and take the best one
            for path in paths {
                debug!("testing path: {:?}", path);

                // copy joltages to mutate them
                let mut joltages = self.joltages.clone();
                for button in path.iter() {
                    for i in 0..lights.len() {
                        let base: i64 = 2;
                        let idx = base.pow((lights.len() - i - 1) as u32);
                        if idx & button > 0 {
                            joltages[i] -= 1
                        }
                    }
                }

                let machine =
                    Machine { joltages, buttons: self.buttons.clone() };

                let o = machine.solve(cache);
                cache.insert(machine.joltages.clone(), o);
                if let Some(solved) = o {
                    let n = solved + path.len() as i64;
                    match best {
                        Some(thing) => {
                            if n < thing {
                                best = Some(n);
                            }
                        }
                        None => {
                            best = Some(n);
                        }
                    }
                };
            }
            if best.is_none() {
                debug!("no best path found");
            }
            best
        }
    }
}

fn find_paths_to_lights_off(lights: &[bool], buttons: &[i64]) -> Vec<Vec<i64>> {
    let want = lights_to_number(lights);

    let base: i64 = 2;
    let max = base.pow(buttons.len() as u32);

    let mut paths = vec![];
    for i in 0..max {
        //        println!("pressing combo {}", i);
        // simulate all the button presses of this combo
        let mut num = 0;
        let mut pressed = vec![];
        for (j, button) in buttons.iter().enumerate() {
            let k = 1 << j;
            if i & k > 0 {
                pressed.push(*button);
                num ^= button;
                //                println!("pressing button {}: {}", j, button);
            }
        }

        if num == want {
            paths.push(pressed);
        }
    }

    paths.sort_by(|a, b| a.len().cmp(&b.len()));
    paths
}

fn lights_to_number(lights: &[bool]) -> i64 {
    let len = lights.len();
    let mut num = 0;
    let base: i64 = 2;
    for (i, light) in lights.iter().enumerate() {
        if *light {
            num += base.pow((len - i - 1) as u32);
        }
    }
    num
}

fn to_lights(joltages: &[i64]) -> Vec<bool> {
    joltages.iter().map(|d| d % 2 == 1).collect()
}

fn text_button_to_number(button: &str, len: usize) -> i64 {
    let elements: Vec<usize> =
        button.split(',').map(|x| x.parse().unwrap()).collect();

    let base: i64 = 2;
    let mut num = 0;
    for element in elements {
        num += base.pow((len - element - 1) as u32);
    }
    num
}

fn parse_input_file(fname: &str) -> Result<Vec<Machine>> {
    let input = fs::read_to_string(fname)?;

    let mut out = vec![];
    for line in input.lines() {
        let line = line.replace(['{', '}', '(', ')'], "");
        let mut spl: Vec<_> = line.split_whitespace().skip(1).collect();

        let joltages = spl.pop().unwrap();
        let buttons = spl;

        let joltages: Vec<i64> =
            joltages.split(',').map(|x| x.parse().unwrap()).collect();
        let len = joltages.len();

        let buttons: Vec<i64> = buttons
            .into_iter()
            .map(|x| text_button_to_number(x, len))
            .collect();

        let machine = Machine { joltages, buttons };
        out.push(machine);
    }

    Ok(out)
}

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<_> = env::args().collect();
    let fname = &args[1];

    let machines = parse_input_file(fname)?;

    let mut total = 0;
    for machine in machines {
        println!("testing machine: {:?}", machine.joltages);
        let mut cache = HashMap::new();
        let num = match machine.solve(&mut cache) {
            Some(n) => n,
            None => {
                println!("couldnt solve");
                continue;
            }
        };
        total += num;
    }

    println!("took {} total", total);

    Ok(())
}
