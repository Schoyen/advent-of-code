use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

#[derive(Debug)]
enum EventState {
    Awake,
    Asleep,
}

#[derive(Debug)]
struct Event {
    start: NaiveDateTime,
    state: EventState,
}

fn create_event(event: &str, re_datetime: &Regex, re_desc: &Regex) -> Event {
    let str_dt = re_datetime
        .captures(event)
        .unwrap()
        .get(0)
        .map_or("", |m| m.as_str());

    let str_desc = re_desc
        .captures(event)
        .unwrap()
        .get(1)
        .map_or("", |m| m.as_str());

    let dt = NaiveDateTime::parse_from_str(&str_dt, "%F %H:%M").unwrap();
    let state = {
        if str_desc.contains("asleep") {
            EventState::Asleep
        } else {
            EventState::Awake
        }
    };

    Event {
        start: dt,
        state: state,
    }
}

fn get_id(event: &str, re_id: &Regex) -> Option<u32> {
    let mut id: Option<u32> = None;

    if let Some(capture) = re_id.captures(event) {
        if let Some(mat) = capture.get(1) {
            id = Some(mat.as_str().parse::<u32>().unwrap());
        }
    }

    id
}

fn get_guards(events: &Vec<&str>) -> HashMap<u32, Vec<Event>> {
    let mut guards: HashMap<u32, Vec<Event>> = HashMap::new();
    let mut id = 0;
    let re_datetime = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();
    let re_desc = Regex::new(r"] (.*)").unwrap();
    let re_id = Regex::new(r"#(\d+)").unwrap();

    for event in events {
        let id_opt = get_id(event, &re_id);
        let event_s = create_event(event, &re_datetime, &re_desc);

        if let Some(i) = id_opt {
            id = i;
        }

        if let None = guards.get(&id) {
            let list: Vec<Event> = Vec::new();
            guards.insert(id, list);
        }

        guards.entry(id).and_modify(|e| e.push(event_s));
    }

    guards
}

fn get_sorted_events(contents: &str) -> Vec<&str> {
    let mut events: Vec<&str> = Vec::new();

    for event in contents.lines() {
        events.push(event);
    }

    events.sort();
    events
}

fn compute_minutes_asleep(events: &Vec<Event>) -> u32 {
    let mut minutes = 0;

    for i in 1..events.len() - 1 {
        if let EventState::Awake = events[i].state {
            continue;
        }

        let start_min = events[i].start.minute();
        let end_min = events[i + 1].start.minute();

        minutes += end_min - start_min;
    }

    minutes
}

fn find_most_common_sleeping_time(events: &Vec<Event>) -> (u32, u32) {
    let mut pop_minute = 0;

    let mut minutes: Vec<u32> = Vec::new();
    minutes.resize(60, 0);

    for i in 1..events.len() - 1 {
        if let EventState::Awake = events[i].state {
            continue;
        }

        let start_min = events[i].start.minute() as usize;
        let end_min = events[i + 1].start.minute() as usize;

        for min in start_min..end_min {
            minutes[min] += 1;
        }
    }

    let mut most_occurences = 0;

    for (i, val) in minutes.iter().enumerate() {
        if *val > most_occurences {
            most_occurences = *val;
            pop_minute = i;
        }
    }

    (pop_minute as u32, most_occurences)
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents: String = fs::read_to_string(filename).unwrap();

    let events = get_sorted_events(&contents);
    let guards = get_guards(&events);

    let mut max_time = 0;
    let mut max_id = 0;

    for (id, guard_events) in &guards {
        let minutes_asleep = compute_minutes_asleep(guard_events);

        if minutes_asleep > max_time {
            max_time = minutes_asleep;
            max_id = *id;
        }
    }

    let mut pop_minute = 0;

    if let Some(e) = guards.get(&max_id) {
        let res = find_most_common_sleeping_time(e);
        pop_minute = res.0;
    }

    println!(
        "Guard id * most common sleeping minute = {0} * {1} = {2}",
        max_id,
        pop_minute,
        pop_minute * max_id
    );

    pop_minute = 0;
    let mut most_occurences = 0;
    max_id = 0;

    for (&id, guard_events) in &guards {
        let res = find_most_common_sleeping_time(guard_events);

        if res.1 > most_occurences {
            pop_minute = res.0;
            most_occurences = res.1;
            max_id = id;
        }
    }

    println!(
        "Guard id * most common sleeping minute = {0} * {1} = {2}",
        max_id,
        pop_minute,
        pop_minute * max_id
    );
}
