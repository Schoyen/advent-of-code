use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::NaiveDateTime;
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

    for (i, event) in events.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if let EventState::Asleep = event.state {}
    }

    minutes
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents: String = fs::read_to_string(filename).unwrap();

    let events = get_sorted_events(&contents);
    let guards = get_guards(&events);

    println!("{:#?}", guards);
}
