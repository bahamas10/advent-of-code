use std::collections::HashSet;
use std::env;
use std::fs;

use anyhow::{Context, Result};

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_new_coords(
    coords: &(usize, usize),
    direction: &Direction,
) -> Option<(usize, usize)> {
    Some(match direction {
        Direction::Up => (coords.0, coords.1.checked_sub(1)?),
        Direction::Down => (coords.0, coords.1.checked_add(1)?),
        Direction::Left => (coords.0.checked_sub(1)?, coords.1),
        Direction::Right => (coords.0.checked_add(1)?, coords.1),
    })
}

fn map_has_loop(
    mut grid: Vec<Vec<char>>,
    mut guard_coords: (usize, usize),
) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let mut direction = Direction::Up;

    let mut seen = HashSet::new();
    seen.insert((guard_coords.1, guard_coords.0, direction.clone()));

    loop {
        // try to move her and see what happens
        let new_coords = match get_new_coords(&guard_coords, &direction) {
            Some(coords) => coords,
            None => {
                break;
            }
        };
        if new_coords.0 >= width || new_coords.1 >= height {
            break;
        }

        let c = grid[new_coords.1][new_coords.0];

        match c {
            '.' => {
                grid[new_coords.1][new_coords.0] = '^';
                grid[guard_coords.1][guard_coords.0] = '.';
                // move her
                guard_coords = new_coords;
                let tup = (guard_coords.1, guard_coords.0, direction.clone());
                if seen.contains(&tup) {
                    // loop detected
                    return true;
                }

                seen.insert(tup);
            }
            '#' => {
                // rotate her
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            c => panic!("wtf is {}", c),
        }
    }

    false
}

fn main() -> Result<()> {
    let file = env::args().nth(1).context("need file as arg1")?;
    let contents = fs::read_to_string(file)?;

    let mut grid = vec![];

    let mut guard_coords = None;

    for (y, line) in contents.trim().lines().enumerate() {
        let mut row = vec![];

        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == '^' {
                assert!(guard_coords.is_none());
                guard_coords = Some((x, y))
            }
        }

        grid.push(row);
    }

    let guard_coords = guard_coords.expect("uh oh could not find guard");

    let height = grid.len();
    let width = grid[0].len();
    let mut total_loops = 0;
    for y in 0..height {
        for x in 0..width {
            let mut grid = grid.clone();
            if grid[x][y] == '.' {
                grid[x][y] = '#';
            }
            //print!("testing {},{} as obstacle: ", x, y);
            let has_loop = map_has_loop(grid.clone(), guard_coords.clone());

            if has_loop {
                total_loops += 1;
                //println!("loop");
            } else {
                //println!("no loop");
            }
        }
    }

    //println!("{} have loops", total_loops);

    Ok(())
}
