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
    let buf_reader = BufReader::new(File::open(path).unwrap());
    let mut offsets = Vec::new();

    buf_reader.lines().for_each(|line_res| {
        offsets.push(line_res.unwrap().parse::<i32>().unwrap());
    });

    offsets
}

fn part1(offsets: &[i32]) {
    let frequency = offsets
        .iter()
        .fold(0, |acc: i32, offset: &i32| acc + offset);

    println!("Part 1: Frequency: {} (553)", frequency);
}

fn part2(offsets: &[i32]) {
    let mut freqs = HashMap::new();

    let mut frequency = 0;
    freqs.insert(0, true);

    loop {
        for offset in offsets {
            frequency += offset;

            if freqs.contains_key(&frequency) {
                println!("Part 2: Duplicate frequency: {} (78724)", frequency);
                return;
            }

            freqs.insert(frequency, true);
        }
    }
}
