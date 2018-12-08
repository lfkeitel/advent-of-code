use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::FromStr;

use chrono::{NaiveDateTime, NaiveTime, Timelike};

fn main() {
    let input = read_input(Path::new("input.txt"));

    let mut guard_events: HashMap<i32, Vec<Event>> = HashMap::new();

    let mut current_guard = 0;
    input.iter().for_each(|event| match event {
        Event::ShiftBegins(_, id) => current_guard = *id,
        _ => {
            let guard_events = guard_events.entry(current_guard).or_default();
            guard_events.push(event.clone());
        }
    });

    part1(&guard_events);
    part2(&guard_events);
}

fn read_input(path: &Path) -> Vec<Event> {
    let mut lines: Vec<Event> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line_res| line_res.unwrap().parse().unwrap())
        .collect();

    lines.sort_unstable_by(|a, b| a.timestamp().cmp(b.timestamp()));

    lines
}

fn part1(events: &HashMap<i32, Vec<Event>>) {
    // .0 = guard id
    // .1 = minute
    // .2 = minutes asleep
    let guard = events.iter().fold((0, 0, 0), |acc, guard| {
        let minutes = build_minute_vec(guard.1);
        let sleepiest_minute = find_sleepiest_minute(&minutes);
        let total = minutes.iter().sum();

        if total > acc.2 {
            (*guard.0, sleepiest_minute as i32, total)
        } else {
            acc
        }
    });

    println!("Part 1: {} (8950)", guard.0 * guard.1);
}

fn part2(events: &HashMap<i32, Vec<Event>>) {
    // .0 = guard id
    // .1 = minute
    // .2 = times asleep that minute
    let guard = events.iter().fold((0, 0, 0), |acc, guard| {
        let minutes = build_minute_vec(guard.1);
        let sleepiest_minute = find_sleepiest_minute(&minutes);
        let times = minutes[sleepiest_minute];

        if times > acc.2 {
            (*guard.0, sleepiest_minute as i32, times)
        } else {
            acc
        }
    });

    println!("Part 2: {} (78452)", guard.0 * guard.1);
}

fn build_minute_vec(events: &[Event]) -> Vec<i32> {
    let mut minutes = vec![0; 60];
    let mut last_asleep_time = NaiveTime::from_hms(0, 0, 0);

    for e in events {
        match e {
            Event::FallsAsleep(t) => last_asleep_time = t.time(),
            Event::WakesUp(t) => {
                let sleep_time = (t.time() - last_asleep_time).num_minutes();
                let start_min = i64::from(last_asleep_time.minute());

                for m in start_min..start_min + sleep_time {
                    minutes[m as usize] += 1;
                }
            }
            _ => {}
        }
    }

    minutes
}

fn find_sleepiest_minute(minutes: &[i32]) -> usize {
    minutes.iter().enumerate().fold(
        0,
        |acc, item| {
            if *item.1 > minutes[acc] {
                item.0
            } else {
                acc
            }
        },
    )
}

type Result<T> = result::Result<T, String>;

#[derive(Debug)]
enum Event {
    ShiftBegins(NaiveDateTime, i32),
    FallsAsleep(NaiveDateTime),
    WakesUp(NaiveDateTime),
}

impl Event {
    fn timestamp(&self) -> &NaiveDateTime {
        match self {
            Event::ShiftBegins(t, _) => t,
            Event::FallsAsleep(t) => t,
            Event::WakesUp(t) => t,
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        match self {
            Event::ShiftBegins(t, id) => Event::ShiftBegins(*t, *id),
            Event::FallsAsleep(t) => Event::FallsAsleep(*t),
            Event::WakesUp(t) => Event::WakesUp(*t),
        }
    }
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Event> {
        // [1518-06-02 23:58] Guard #179 begins shift
        // [1518-09-18 00:43] wakes up
        // [1518-06-06 00:10] falls asleep

        let line_parts: Vec<&str> = s.split(' ').collect();
        let timestamp_str = line_parts[0..2]
            .join(" ")
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_owned();

        let timestamp = NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M").unwrap();

        match line_parts[2] {
            "wakes" => Ok(Event::WakesUp(timestamp)),
            "falls" => Ok(Event::FallsAsleep(timestamp)),
            _ => {
                let id_str = line_parts[3].trim_start_matches('#');
                Ok(Event::ShiftBegins(timestamp, id_str.parse().unwrap()))
            }
        }
    }
}
