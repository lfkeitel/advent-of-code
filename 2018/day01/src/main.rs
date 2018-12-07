use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let offsets = read_offsets(Path::new("input.txt"));

    part1(&offsets);
    part2(&offsets);
}

fn read_offsets(path: &Path) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut offsets = Vec::new();

    for line_res in buf_reader.lines() {
        let offset: i32 = line_res.unwrap().parse().unwrap();
        offsets.push(offset);
    }

    offsets
}

fn part1(offsets: &[i32]) {
    let mut frequency = 0;

    for offset in offsets {
        frequency += offset
    }

    println!("Part 1: Frequency: {}", frequency);
}

fn part2(offsets: &[i32]) {
    let mut freqs = HashMap::new();

    let mut frequency = 0;
    freqs.insert(0, true);

    loop {
        for offset in offsets {
            frequency += offset;

            if freqs.contains_key(&frequency) {
                println!("Part 2: Duplicate frequency: {}", frequency);
                return;
            }

            freqs.insert(frequency, true);
        }
    }
}
