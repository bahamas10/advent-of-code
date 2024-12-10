use std::collections::HashSet;
use std::env;
use std::fs;

use anyhow::{Context, Result};

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

    let mut guard_coords = guard_coords.expect("uh oh could not find guard");
    let height = grid.len();
    let width = grid[0].len();
    let mut direction = Direction::Up;

    println!("puzzle height = {}", height);
    println!("puzzle width = {}", width);
    println!("guard coords = {:?}", guard_coords);

    let mut seen = HashSet::new();
    seen.insert((guard_coords.1, guard_coords.0));

    loop {
        // try to move her and see what happens
        let new_coords = match get_new_coords(&guard_coords, &direction) {
            Some(coords) => coords,
            None => {
                println!("she walked away lol");
                break;
            }
        };
        if new_coords.0 >= width || new_coords.1 >= height {
            println!("she also walked away lol");
            break;
        }

        println!(
            "trying to move her from {:?} to {:?}",
            guard_coords, new_coords
        );

        let c = grid[new_coords.1][new_coords.0];

        match c {
            '.' => {
                grid[new_coords.1][new_coords.0] = '^';
                grid[guard_coords.1][guard_coords.0] = '.';
                // move her
                guard_coords = new_coords;
                seen.insert((guard_coords.1, guard_coords.0));
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

    println!("seen {} distinct places", seen.len());

    Ok(())
}
