use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::FromStr;

fn main() {
    let input = read_input(Path::new("input.txt"));

    part1(&input);
    part2(&input);
}

fn read_input(path: &Path) -> Vec<Claim> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line_res| line_res.unwrap().parse().unwrap())
        .collect()
}

fn part1(claims: &[Claim]) {
    // Create a grid of cells
    let mut arena = vec![0; 1000 * 1000];

    // Fill in the grid with the ids of each claim.
    // If multiple claims claim the same cell, its id is set to -1
    for claim in claims {
        // TODO: Implement an iterator for the points of a claim
        for h in 0..claim.height {
            for w in 0..claim.width {
                let index = (((h + claim.top) * 1000) + (w + claim.left)) as usize;
                if arena[index] == 0 {
                    arena[index] = claim.id;
                } else {
                    arena[index] = -1;
                }
            }
        }
    }

    // Count the number of cells claimed multiple times.
    let contested2 = arena
        .iter()
        .fold(0, |acc, val| if *val == -1 { acc + 1 } else { acc });

    println!("Part 1: Contested Area: {} (113966)", contested2);
}

fn part2(claims: &[Claim]) {
    for claim_a in claims {
        let mut overlap = false;

        for claim_b in claims {
            if claim_a.id != claim_b.id && claim_a.intersects(claim_b) {
                overlap = true;
                break;
            }
        }

        if !overlap {
            println!("Part 2: Non-overlapping Claim: {} (235)", claim_a.id);
            return;
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn intersects(&self, other: &Claim) -> bool {
        !(self.left + self.width < other.left
            || other.left + other.width < self.left
            || self.top + self.height < other.top
            || other.top + other.height < self.top)
    }
}

type Result<T> = result::Result<T, Box<Error>>;

// Inspired from BurntSushi's solution

// If a type implements FromStr, the str.parse<T>() method will pass the string
// to the from_str() method which can parse it and return a value. In this case,
// the string is being parsed into a Claim type.
impl FromStr for Claim {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Claim> {
        let line_parts: Vec<&str> = s.split(' ').collect();

        let id = line_parts[0].trim_start_matches('#');
        let pos: Vec<&str> = line_parts[2].split(',').collect();
        let size: Vec<&str> = line_parts[3].split('x').collect();

        Ok(Claim {
            id: id.parse().unwrap(),
            left: pos[0].parse().unwrap(),
            top: pos[1].trim_end_matches(':').parse().unwrap(),
            width: size[0].parse().unwrap(),
            height: size[1].parse().unwrap(),
        })
    }
}
