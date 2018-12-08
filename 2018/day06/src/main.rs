use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::FromStr;

fn main() {
    let input = read_input(Path::new("input.txt"));

    part1(&input);
    // part2(&guard_events);
}

fn read_input(path: &Path) -> Vec<Point> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line_res| line_res.unwrap().parse().unwrap())
        .collect()
}

fn part1(points: &[Point]) {
    let largest_point = points.iter().fold(Point::new(0, 0), |acc, point| {
        let x = if point.x > acc.x { point.x } else { acc.x };
        let y = if point.y > acc.y { point.y } else { acc.y };
        Point::new(x, y)
    });

    let smallest_point = points.iter().fold(
        Point::new(largest_point.x, largest_point.y),
        |acc, point| {
            let x = if point.x < acc.x { point.x } else { acc.x };
            let y = if point.y < acc.y { point.y } else { acc.y };
            Point::new(x, y)
        },
    );

    let width = largest_point.x - smallest_point.x + 1;
    let height = largest_point.y - smallest_point.y + 1;

    let mut grid = vec![0; (width * height) as usize];

    for point in smallest_point.iter_to(&largest_point) {
        let index = ((point.y - smallest_point.y) * width) + (point.x - smallest_point.x);

        // .0 = Point index
        // .1 = distance
        let the_point = points.iter().enumerate().fold((0, 0), |acc, p| {
            if acc.0 == -1 {
                return acc;
            }

            let distance = p.1.distance_from(&points[acc.0 as usize]);

            if distance == acc.1 {
                (-1, 0)
            } else if distance < acc.1 {
                (p.0 as i32, distance)
            } else {
                acc
            }
        });

        grid[index as usize] = the_point.0;
    }

    let mut point_dists: HashMap<usize, i32> = HashMap::new();

    for (idx, point) in points.iter().enumerate() {
        let area = grid.
    }

    println!(
        "Smallest point: {:?}, largest point: {:?}",
        smallest_point, largest_point
    );
}

type Result<T> = result::Result<T, String>;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn iter_to(&self, other: &Point) -> PointIter {
        PointIter {
            start: self.clone(),
            end: other.clone(),
            x: self.x - 1,
            y: self.y,
        }
    }

    fn distance_from(&self, other: &Point) -> i32 {
        (other.x - self.x + other.y - self.y).abs()
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Point> {
        let line_parts: Vec<&str> = s.split(", ").collect();
        Ok(Point {
            x: line_parts[0].parse().unwrap(),
            y: line_parts[1].parse().unwrap(),
        })
    }
}

struct PointIter {
    start: Point,
    end: Point,
    x: i32,
    y: i32,
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_x = self.x + 1;
        let mut next_y = self.y;

        if next_x > self.end.x {
            next_x = self.start.x;
            next_y += 1;
        }

        self.x = next_x;
        self.y = next_y;

        if next_y > self.end.y {
            None
        } else {
            Some(Point {
                x: next_x,
                y: next_y,
            })
        }
    }
}
