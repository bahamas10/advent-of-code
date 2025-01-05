use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::env;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Block {
    Free,
    Obstacle,
}

impl Block {
    pub fn to_char(&self) -> char {
        match self {
            Self::Free => '.',
            Self::Obstacle => '#',
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Task {
    pub location: Point,
    pub direction: Direction,
    pub cost: usize,
    pub blocks: Vec<Point>,
}

fn print_grid(grid: &Vec<Vec<Block>>) {
    let width = grid[0].len();
    let height = grid.len();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x].to_char();
            print!("{}", c);
        }
        println!();
    }
}

fn enqueue_moves(
    queue: &mut VecDeque<Task>,
    task: Task,
) {
    // try move forward
    let mut next_loc = task.location.clone();
    match task.direction {
        Direction::North => next_loc.y -= 1,
        Direction::West => next_loc.x -= 1,
        Direction::South => next_loc.y += 1,
        Direction::East => next_loc.x += 1,
    }
    let mut b = task.blocks.clone();
    b.push(next_loc.clone());
    let t = Task {
        location: next_loc,
        direction: task.direction.clone(),
        cost: (task.cost + 1),
        blocks: b,
    };
    queue.push_front(t);

    // try move cw
    let t = Task {
        location: task.location.clone(),
        direction: task.direction.clockwise(),
        cost: (task.cost + 1000),
        blocks: task.blocks.clone(),
    };
    queue.push_back(t);

    // try move ccw
    let t = Task {
        location: task.location.clone(),
        direction: task.direction.counter_clockwise(),
        cost: (task.cost + 1000),
        blocks: task.blocks.clone(),
    };
    queue.push_back(t);
}

fn main() -> Result<()> {
    let file = env::args().nth(1).context("need file as arg1")?;
    let input = fs::read_to_string(file)?;

    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let block = match c {
                '.' => Block::Free,
                '#' => Block::Obstacle,
                'S' => {
                    start = Some(Point { x, y });
                    Block::Free
                }
                'E' => {
                    end = Some(Point { x, y });
                    Block::Free
                }
                _ => panic!(),
            };
            row.push(block);
        }
        grid.push(row);
    }

    let start = start.unwrap();
    let end = end.unwrap();

    print_grid(&grid);

    let mut queue = VecDeque::new();

    let task = Task {
        location: start.clone(),
        direction: Direction::East,
        cost: 0,
        blocks: vec![start.clone()],
    };
    enqueue_moves(&mut queue, task);

    let mut visited = HashMap::new();

    println!("end = {:#?}", end);

    let mut best_score = usize::MAX;

    let mut winners = vec![];

    loop {
        let task = match queue.pop_front() {
            Some(t) => t,
            None => break,
        };

        // check if we've been here before at a cheaper rate
        let key = (task.location.clone(), task.direction.clone());
        let cache = visited.get(&key).unwrap_or(&usize::MAX);
        if cache < &task.cost {
            continue;
        }
        visited.insert(key.clone(), task.cost);

        // check for a wall
        let block = &grid[task.location.y][task.location.x];
        match block {
            Block::Obstacle => continue,
            _ => (),
        }

        // check if we won
        if task.location == end {
            if task.cost > best_score {
                // we found all the best paths - should we break or just try
                // again?
                break;
            }
            if task.cost < best_score {
                winners.clear();
            }
            best_score = task.cost;
            winners.push(task.clone());

            // we win!
            continue;
        }

        // enque more moves
        enqueue_moves(&mut queue, task);
    }

    assert!(!winners.is_empty());
    println!("all best paths found for cost {}", winners[0].cost);

    let mut seats = HashSet::new();
    for winner in winners {
        println!("winner {} blocks", winner.blocks.len());
        for block in winner.blocks {
            seats.insert(block);
        }
    }

    println!("found {} seats", seats.len());

    Ok(())
}
