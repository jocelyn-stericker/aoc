use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq)]
struct TimedEvent(Timestamp, Event);

impl PartialEq for TimedEvent {
    fn eq(&self, other: &TimedEvent) -> bool {
        self.0 == other.0
    }
}

impl Ord for TimedEvent {
    fn cmp(&self, other: &TimedEvent) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for TimedEvent {
    fn partial_cmp(&self, other: &TimedEvent) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

// Genereates map from id -> minute asleep -> num occurances
fn compute_minutes_asleep_per_guard(input: &str) -> HashMap<u32, HashMap<u32, u32>> {
    let line_re = Regex::new(
        r"(?x)
        ^
        \[
            # The timestamp
            (?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+)\s
            (?P<hour>\d+):(?P<minute>\d+)
        \]\s
        (
            (?P<begin_shift>Guard\s\#(?P<begin_shift_guard_id>\d+)\sbegins\sshift) |
            (?P<falls_asleep>falls\sasleep) |
            (?P<wakes_up>wakes\sup)
        )
        $",
    )
    .unwrap();

    let input = input.split('\n').filter(|line| line != &"").map(|line| {
        let line = line_re.captures(line).expect("Invalid line");
        TimedEvent(
            Timestamp {
                year: line.name("year").unwrap().as_str().parse::<u32>().unwrap(),
                month: line.name("month").unwrap().as_str().parse::<u32>().unwrap(),
                day: line.name("day").unwrap().as_str().parse::<u32>().unwrap(),
                hour: line.name("hour").unwrap().as_str().parse::<u32>().unwrap(),
                minute: line
                    .name("minute")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            },
            {
                if line.name("begin_shift").is_some() {
                    Event::BeginsShift(
                        line.name("begin_shift_guard_id")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap(),
                    )
                } else if line.name("falls_asleep").is_some() {
                    Event::FallsAsleep
                } else if line.name("wakes_up").is_some() {
                    Event::WakesUp
                } else {
                    unreachable!("Invalid event");
                }
            },
        )
    });

    let mut input: Vec<TimedEvent> = input.collect();
    input.sort();

    let mut curr_id: Option<u32> = None;
    let mut fell_asleep: Option<Timestamp> = None;

    // Map from id to minute asleep to num occurances
    let mut minutes_asleep: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for TimedEvent(timestamp, event) in &input {
        match event {
            Event::BeginsShift(id) => {
                curr_id = Some(*id);
            }
            Event::FallsAsleep => {
                fell_asleep = Some(*timestamp);
            }
            Event::WakesUp => {
                let id = curr_id.unwrap();
                let fell_asleep = fell_asleep.unwrap();
                let minutes_asleep = minutes_asleep.entry(id).or_insert(HashMap::new());

                // We know they're all in the same hour.
                for minute in fell_asleep.minute..timestamp.minute {
                    *minutes_asleep.entry(minute).or_insert(0) += 1;
                }
            }
        }
    }

    minutes_asleep
}

pub fn part_a(input: &str) -> u32 {
    let minutes_asleep = compute_minutes_asleep_per_guard(input);

    let best_id = minutes_asleep
        .iter()
        .fold((None, 0), |memo, (id, minutes)| {
            let minutes = minutes
                .iter()
                .fold(0, |memo, (_minute, count)| memo + count);

            if minutes > memo.1 {
                (Some(id), minutes)
            } else {
                memo
            }
        })
        .0
        .unwrap();

    let best_hour = minutes_asleep
        .get(best_id)
        .unwrap()
        .iter()
        .fold((None, 0), |memo, (minute, occurances)| {
            if occurances > &memo.1 {
                (Some(minute), *occurances)
            } else {
                memo
            }
        })
        .0
        .unwrap();

    best_hour * best_id
}

pub fn part_b(input: &str) -> u32 {
    let minutes_asleep = compute_minutes_asleep_per_guard(input);

    let best_id = minutes_asleep
        .iter()
        .fold((None, 0), |memo, (id, minutes)| {
            let max_count = minutes
                .iter()
                .fold(0, |memo, (_minute, count)| std::cmp::max(memo, *count));

            if max_count > memo.1 {
                (Some(id), max_count)
            } else {
                memo
            }
        })
        .0
        .unwrap();

    let best_hour = minutes_asleep
        .get(best_id)
        .unwrap()
        .iter()
        .fold((None, 0), |memo, (minute, occurances)| {
            if occurances > &memo.1 {
                (Some(minute), *occurances)
            } else {
                memo
            }
        })
        .0
        .unwrap();

    best_hour * best_id
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 240);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_a(include_str!("sample_unsorted.txt")), 240);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 151754);
    }

    #[test]
    fn example3() {
        assert_eq!(super::part_b(include_str!("sample_unsorted.txt")), 4455);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 19896);
    }
}
