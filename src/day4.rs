use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Time {
    fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32) -> Time {
        Time {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Event {
    ShiftBegins(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Eq, Debug)]
pub struct LogEntry {
    time: Time,
    event: Event,
}

impl LogEntry {
    fn new(time: Time, event: Event) -> LogEntry {
        LogEntry {
            time: time,
            event: event,
        }
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.time == other.time
    }
}

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> Vec<LogEntry> {
    let mut output = Vec::new();
    for line in input.lines() {
        let date_str = &line[1..11];
        let year = date_str.split('-').collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let month = date_str.split('-').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let day = date_str.split('-').collect::<Vec<&str>>()[2]
            .parse::<u32>()
            .unwrap();

        let time_str = &line[12..17];
        let hour = time_str.split(':').collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let minute = time_str.split(':').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        let time = Time::new(year, month, day, hour, minute);

        let event_str = &line[19..];
        let mut event = Event::WakesUp;
        match event_str.chars().collect::<Vec<char>>()[0] {
            'G' => {
                let id = event_str.split(' ').collect::<Vec<&str>>()[1][1..]
                    .parse::<u32>()
                    .unwrap();
                event = Event::ShiftBegins(id);
            }
            'f' => event = Event::FallsAsleep,
            'w' => event = Event::WakesUp,
            _ => println!("ERROR"),
        }
        output.push(LogEntry::new(time, event));
    }
    output.sort();
    output
}

fn make_guard_map(logs: &Vec<LogEntry>) -> HashMap<u32, HashMap<u32, u32>> {
    let mut start_time: u32 = 0;
    let mut guards = HashMap::new();
    let mut current_id = 0;
    for log in logs {
        match log.event {
            Event::ShiftBegins(id) => {
                if !guards.contains_key(&id) {
                    let mut minmap: HashMap<u32, u32> = HashMap::new();
                    for min in 0..60 {
                        minmap.insert(min, 0);
                    }
                    guards.insert(id, minmap);
                }
                current_id = id;
            }
            Event::FallsAsleep => {
                start_time = log.time.minute;
            }
            Event::WakesUp => {
                if let Some(x) = guards.get_mut(&current_id) {
                    for min in start_time..log.time.minute {
                        if let Some(m) = x.get_mut(&min) {
                            *m += 1;
                        }
                    }
                }
            }
        }
    }
    guards
}

#[aoc(day4, part1)]
pub fn part1(logs: &Vec<LogEntry>) -> u32 {
    let mut guards = make_guard_map(logs);
    let mut total_sleep_guard = Vec::new();
    for (id, map) in guards.iter() {
        let mut total_sleep = 0;
        for (_, total) in map.iter() {
            total_sleep += total;
        }
        total_sleep_guard.push((id, total_sleep));
    }
    let mut max_id = 0;
    let mut max_min = 0;
    for pair in total_sleep_guard {
        if pair.1 > max_min {
            max_id = *pair.0;
            max_min = pair.1;
        }
    }

    let mut max_total = 0;
    let mut max_min_total = 0;
    if let Some(guard) = guards.get_mut(&max_id) {
        for (min, total) in guard {
            if total > &mut max_total {
                max_total = *total;
                max_min_total = *min;
            }
        }
    }

    // Return id of most sleepy guard times the minute he sleeps the most
    max_id * max_min_total
}

#[aoc(day4, part2)]
pub fn part2(logs: &Vec<LogEntry>) -> u32 {
    let guards = make_guard_map(logs);
    let mut max_id = 0;
    let mut max_min = 0;
    let mut max_min_total = 0;
    for (id, map) in guards.iter() {
        for (min, total) in map.iter() {
            if total > &max_min_total {
                max_id = *id;
                max_min = *min;
                max_min_total = *total;
            }
        }
    }
    max_id * max_min
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1example() {
        let input = load_input("inputs/04example.txt");
        assert_eq!(part1(&input), 240);
    }

    #[test]
    fn test_part1() {
        let input = load_input("inputs/04.txt");
        assert_eq!(part1(&input), 74743);
    }

    #[test]
    fn test_part2example() {
        let input = load_input("inputs/04example.txt");
        assert_eq!(part2(&input), 4455);
    }

    #[test]
    fn test_part2() {
        let input = load_input("inputs/04.txt");
        assert_eq!(part2(&input), 132484);
    }
}
