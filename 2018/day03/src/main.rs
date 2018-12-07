use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = read_input(Path::new("input.txt"));

    part1(&input);
    part2(&input);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_intersect() {
        let c1 = Claim {
            id: 1,
            left: 1,
            top: 3,
            width: 4,
            height: 4,
        };
        let c2 = Claim {
            id: 2,
            left: 3,
            top: 1,
            width: 4,
            height: 4,
        };

        assert!(c1.intersects(&c2));
    }
}

fn read_input(path: &Path) -> Vec<Claim> {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut input = Vec::new();

    for line_res in buf_reader.lines() {
        let line = line_res.unwrap();
        let line_parts: Vec<&str> = line.split(' ').collect();

        let id = line_parts[0].trim_start_matches('#');
        let pos: Vec<&str> = line_parts[2].split(',').collect();
        let size: Vec<&str> = line_parts[3].split('x').collect();

        input.push(Claim {
            id: id.parse().unwrap(),
            left: pos[0].parse().unwrap(),
            top: pos[1].trim_end_matches(':').parse().unwrap(),
            width: size[0].parse().unwrap(),
            height: size[1].parse().unwrap(),
        });
    }

    input
}

fn part1(claims: &[Claim]) {
    // Create a grid of cells
    let mut arena = vec![0; 1000 * 1000];

    // Fill in the grid with the ids of each claim.
    // If multiple claims claim the same cell, its id is set to -1
    for claim in claims {
        for h in 0..claim.height {
            for w in 0..claim.width {
                let index = ((h + claim.top) * 1000) + (w + claim.left);
                if arena[index as usize] == 0 {
                    arena[index as usize] = claim.id;
                } else {
                    arena[index as usize] = -1;
                }
            }
        }
    }

    // Count the number of cells claimed multiple times.
    let contested2 = arena.iter().fold(
        0,
        |acc: i32, val: &i32| {
            if *val == -1 {
                acc + 1
            } else {
                acc
            }
        },
    );

    println!("Part 1: Contested Area: {}", contested2);
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
            println!("Part 2: Non-overlapping Claim: {}", claim_a.id);
            return;
        }
    }
}
