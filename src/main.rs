use core::{fmt, time};
use std::{
    collections::HashMap,
    io::{stdout, Write},
    thread::sleep,
};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::Rng;

const WIDTH: i32 = 79;
const HEIGHT: i32 = 20;
fn main() -> Result<(), std::io::Error> {
    let mut next_cells: HashMap<(i32, i32), CellState> = seed_cells().into_iter().collect();

    'game: loop {
        execute!(
            stdout(),
            Hide,
            Clear(Purge),
            Clear(All),
            DisableLineWrap,
            MoveTo(1, 1)
        )?;

        let cells = next_cells.clone();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", cells.get(&(x, y)).unwrap());

                let [left, right, above, below] = [
                    (x - 1) % WIDTH,
                    (x + 1) % WIDTH,
                    (y - 1) % HEIGHT,
                    (y + 1) % HEIGHT,
                ];
                let neighbors = [
                    &(left, above),
                    &(x, above),
                    &(right, above),
                    &(left, y),
                    &(right, y),
                    &(left, below),
                    &(x, below),
                    &(right, below),
                ];

                let mut neighbor_count: u8 = 0;

                for neighbor in neighbors {
                    if cells.get(neighbor) == Some(&CellState::ALIVE) {
                        neighbor_count += 1;
                    }
                }

                next_cells.insert(
                    (x, y),
                    next_state(cells.get(&(x, y)).unwrap(), neighbor_count),
                );
            }
            print!("\n");
            stdout().flush()?;
        }
        println!("Press Ctrl-C to quit.");
        sleep(time::Duration::from_millis(500));
    }

    Ok(())
}

fn next_state(current_state: &CellState, neighbor_count: u8) -> CellState {
    if (current_state == &CellState::ALIVE && (neighbor_count == 2 || neighbor_count == 3))
        || (current_state == &CellState::DEAD && neighbor_count == 3)
    {
        CellState::ALIVE
    } else {
        CellState::DEAD
    }
}

fn seed_cells() -> Vec<((i32, i32), CellState)> {
    let mut rng = rand::thread_rng();
    let mut next_cells = vec![];
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            next_cells.push((
                (x, y),
                if rng.gen_bool(0.5) {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                },
            ))
        }
    }
    next_cells
}

#[derive(Clone, Debug, PartialEq)]
enum CellState {
    ALIVE,
    DEAD,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::ALIVE => write!(f, "▉"),
            CellState::DEAD => write!(f, " "),
        }
    }
}
