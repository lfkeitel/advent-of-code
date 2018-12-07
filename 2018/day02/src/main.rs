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
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut input = Vec::new();

    for line_res in buf_reader.lines() {
        input.push(line_res.unwrap());
    }

    input
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
        for cnt in letters.values() {
            if *cnt == 2 {
                labels_with_two += 1;
                break;
            }
        }

        // Find any letters with a count of exactly 3
        for cnt in letters.values() {
            if *cnt == 3 {
                labels_with_three += 1;
                break;
            }
        }
    }

    println!("Part 1: Checksum: {}", labels_with_two * labels_with_three);
}

fn part2(labels: &[String]) {
    for (i, label_a) in labels.iter().enumerate() {
        for label_b in labels.iter().skip(i + 1) {
            if labels_similar(label_a, label_b) {
                println!("Part 2: Same Letters: {}", same_letters(label_a, label_b));
            }
        }
    }
}

fn labels_similar(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut diff = false;

    for i in 0..a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            if diff {
                return false;
            } else {
                diff = true
            }
        }
    }

    diff
}

fn same_letters(a: &str, b: &str) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut same_bytes = Vec::with_capacity(a_bytes.len() - 1);

    for i in 0..a_bytes.len() {
        if a_bytes[i] == b_bytes[i] {
            same_bytes.push(a_bytes[i]);
        }
    }

    String::from_utf8(same_bytes).unwrap()
}
