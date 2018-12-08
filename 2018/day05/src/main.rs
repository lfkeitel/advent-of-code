use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    let polymer = read_input(Path::new("input.txt"));

    part1(&polymer);
    part2(&polymer);
}

fn read_input(path: &Path) -> String {
    let mut input = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input.trim().to_owned()
}

fn part1(polymer: &str) {
    println!("Part 1: {} units", collapse_polymer(polymer).len());
}

fn part2(polymer: &str) {
    let shortest_chain = (b'A'..=b'Z').fold(polymer.len(), |acc, letter| {
        println!("Removing {}", letter as char);

        let new_polymer = String::from_utf8(
            polymer
                .as_bytes()
                .iter()
                .filter(|&l| *l != letter && *l != letter + 32)
                .cloned()
                .collect(),
        )
        .unwrap();

        let units = collapse_polymer(&new_polymer).len();
        if units < acc {
            units
        } else {
            acc
        }
    });

    println!("Part 2: {} units", shortest_chain);
}

// TODO: Refactor this to be more efficient
fn collapse_polymer(polymer: &str) -> String {
    // I really don't like this, but it's simple and easy
    lazy_static! {
        static ref re: Regex = Regex::new(r"aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ|Aa|Bb|Cc|Dd|Ee|Ff|Gg|Hh|Ii|Jj|Kk|Ll|Mm|Nn|Oo|Pp|Qq|Rr|Ss|Tt|Uu|Vv|Ww|Xx|Yy|Zz").unwrap();
    }

    let mut result = polymer.to_owned();

    loop {
        let new_result = re.replace_all(&result, "");
        if new_result == result {
            break;
        }
        result = new_result.to_owned().to_string();
    }

    result
}
