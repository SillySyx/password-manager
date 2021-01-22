use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde_json::{json, Value};

use event_sourcing::{EventLog, Event};

pub fn load_eventlog(key: &[u8]) -> Result<EventLog, Box<dyn Error>> {
    let filepath = PathBuf::from("./eventlog");
    let bytes = read_file_bytes(filepath)?;
    if bytes.is_empty() {
        return Ok(EventLog::new());
    }

    let iv = crypto::generate_iv_from_seed("silly goose")?;
    let bytes = crypto::decrypt(&bytes, key, &iv)?;

    let json = std::str::from_utf8(&bytes)?;
    if json.is_empty() {
        return Ok(EventLog::new());
    }

    let eventlog = parse_json_to_eventlog(json.to_string())?;

    Ok(eventlog)
}

pub fn save_eventlog(eventlog: EventLog, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let json = parse_eventlog_to_json(&eventlog)?;
    let bytes = json.as_bytes();

    let iv = crypto::generate_iv_from_seed("silly goose")?;
    let bytes = crypto::encrypt(&bytes, key, &iv)?;

    let filepath = PathBuf::from("./eventlog");
    save_file_bytes(filepath, &bytes)?;

    Ok(())
}

pub fn append_event_to_eventlog(event: Event, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut eventlog = load_eventlog(key).unwrap_or(EventLog::new());

    eventlog.events.push(event);

    save_eventlog(eventlog, key)
}

fn read_file_bytes(path: PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn save_file_bytes(path: PathBuf, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write(bytes)?;

    Ok(())
}

fn parse_json_to_eventlog(data: String) -> Result<EventLog, Box<dyn Error>> {
    let json: Value = serde_json::from_str(&data)?;

    let entries = match json.as_array() {
        Some(value) => value,
        None => return Err(Box::from("failed to read json array")),
    };

    let eventlog = entries
        .iter()
        .fold(EventLog::new(), |mut eventlog, event| {
            let data: &Vec<Value> = match event["data"].as_array() {
                Some(value) => value,
                None => return eventlog,
            };

            let data = data
                .iter()
                .fold(vec![], |mut list, value| {
                    let value = value.as_u64().unwrap();
                    list.push(value as u8);
                    list
                });

            let event = Event {
                event_type: event["event_type"].as_str().unwrap().to_string(),
                data,
            };

            eventlog.events.push(event);
            eventlog
        });

    Ok(eventlog)
}


fn parse_eventlog_to_json(eventlog: &EventLog) -> Result<String, Box<dyn Error>> {
    let events = eventlog
        .events
        .iter()
        .fold(vec![], |mut list, event| {
            let value: Value = json!({
                "event_type": event.event_type,
                "data": event.data,
            });
            list.push(value);
            list
        });

    let value = Value::from(events);

    let value = value.to_string();

    Ok(value)
}