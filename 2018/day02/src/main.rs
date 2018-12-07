use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = read_input(Path::new("input.txt"));

    part1(&input);
    part2(&input);
}

fn read_input(path: &Path) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect()
}

fn part1(labels: &[String]) {
    let mut labels_with_two = 0;
    let mut labels_with_three = 0;

    for label in labels {
        let mut letters = HashMap::new();

        for letter in label.as_bytes() {
            let letter_cnt = letters.entry(letter).or_insert(0);
            *letter_cnt += 1;
        }

        // Find any letters with a count of exactly 2
        if letters.values().any(|&cnt| cnt == 2) {
            labels_with_two += 1;
        }

        // Find any letters with a count of exactly 3
        if letters.values().any(|&cnt| cnt == 3) {
            labels_with_three += 1;
        }
    }

    println!(
        "Part 1: Checksum: {} (6474)",
        labels_with_two * labels_with_three
    );
}

fn part2(labels: &[String]) {
    for (i, label_a) in labels.iter().enumerate() {
        for label_b in labels.iter().skip(i + 1) {
            if labels_similar(label_a, label_b) {
                println!(
                    "Part 2: Same Letters: {} (mxhwoglxgeauywfkztndcvjqr)",
                    same_letters(label_a, label_b)
                );
                return;
            }
        }
    }
}

fn labels_similar(a: &str, b: &str) -> bool {
    a.as_bytes().iter().zip(b.as_bytes()).fold(
        0,
        |acc, pair| {
            if *pair.0 != *pair.1 {
                acc + 1
            } else {
                acc
            }
        },
    ) == 1
}

fn same_letters(a: &str, b: &str) -> String {
    let same_bytes = a
        .as_bytes()
        .iter()
        .zip(b.as_bytes())
        .filter_map(|pair| {
            if *pair.0 == *pair.1 {
                Some(*pair.0)
            } else {
                None
            }
        })
        .collect();

    String::from_utf8(same_bytes).unwrap()
}
