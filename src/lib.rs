use std::fs::File;
use std::io::{self, BufReader, BufRead};
use regex::Regex;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    static ref DATE_REGEX: regex::Regex 
        = Regex::new(r"^(\d{2}\.\d{2}\.\d{4}|\d{4}-\d{2}-\d{2})$").unwrap();
    static ref LABEL_REGEX: regex::Regex
        = Regex::new(r"(\w{2,5}-\d{1,6}|--)").unwrap();
    static ref TIME_REGEX: regex::Regex
        = Regex::new(r"(\d{1,6})(h|m)").unwrap();
    static ref LOG_REGEX: regex::Regex 
        = Regex::new(r"^((?:\s*(?:\d{1,6}(?:h|m)))+)\s+([^\s].*)$").unwrap();
    static ref COMM_REGEX: regex::Regex
        = Regex::new(r"^#.+$").unwrap();
}



pub enum TimesheetParseError {
    IOError(io::Error),
    LineError(usize, String),
    DateNotPresent(usize)
}


#[allow(dead_code)]
#[derive(Serialize)]
pub struct Record {
    date: String,
    time: String,
    hours: f64,
    label: String,
    description: String
}

pub fn get_records_from_file(filename: Option<&str>) -> Result<Vec<Record>, TimesheetParseError> {

    let stdin = io::stdin();
    let handle = stdin.lock();

    let buffered: Box<dyn BufRead> = match filename {
        Some(name) => Box::new(BufReader::new(File::open(name)
            .map_err(TimesheetParseError::IOError)?)),
        None => Box::new(handle)
    };

    let mut records = Vec::new();
    let mut current_date: Option<String> = None;
    let mut current_label: Option<String> = None;

    for (i, res_line) in buffered.lines().enumerate() {
        let line = res_line.map_err(TimesheetParseError::IOError)?;
        match parse_line(&line) {
            ParsedLine::Date (date) => { current_date = Some(date.to_owned()) },
            ParsedLine::Label (label) => { current_label = Some(label.to_owned()) },
            ParsedLine::Empty => {},
            ParsedLine::Unknown => { return Err(TimesheetParseError::LineError(i, line.to_owned())); },
            ParsedLine::Log { time, text } => {
                let date = current_date.clone().ok_or(TimesheetParseError::DateNotPresent(i))?;
                let label = current_label.clone().unwrap_or_else(||"--".to_owned());
                let notation = TimeNotation::from_str(&time);
                records.push(Record {
                    date, label, description: text.to_owned(),
                    time: notation.as_string(),
                    hours: notation.as_hours()
                });
            }
        }
    }
    Ok(records)
}

enum ParsedLine<'a> {
    Date (&'a str),
    Label (&'a str),
    Log { time: &'a str, text: &'a str },
    Empty,
    Unknown
}


fn parse_line(line: &str) -> ParsedLine {
    if line.trim().is_empty() {
        return ParsedLine::Empty;
    }
    if let Some(_) = COMM_REGEX.captures(line) {
        return ParsedLine::Empty;
    }
    if let Some(capture) = DATE_REGEX.captures(line) {
        return ParsedLine::Date(capture.get(0).unwrap().as_str());
    }
    if let Some(capture) = LABEL_REGEX.captures(line) {
        return ParsedLine::Label(capture.get(0).unwrap().as_str());
    }
    if let Some(capture) = LOG_REGEX.captures(line) {
        return ParsedLine::Log {
            time: capture.get(1).unwrap().as_str(),
            text: capture.get(2).unwrap().as_str(),
        };
    }
    return ParsedLine::Unknown;
}

struct TimeNotation {
    minutes: u32
}

impl TimeNotation {
    fn from_str(notation: &str) -> Self {
        let mut minutes = 0;
        for capture in TIME_REGEX.captures_iter(notation) {
            let amount = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let unit   = capture.get(2).unwrap().as_str();
            match unit {
                "h" => { minutes += amount * 60; }
                "m" => { minutes += amount; }
                _   => { panic!("TIME_REGEX must make sure to onlu have 'h' or 'm' as time unit"); }
            }
        }
        return Self { minutes };
    }

    fn as_string(&self) -> String {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        if hours > 0 && minutes > 0 {
            return format!("{}h {}m", hours, minutes);
        } else if hours > 0 {
            return format!("{}h", hours);
        } else if minutes > 0 {
            return format!("{}m", minutes);
        } else {
            return "0m".to_owned();
        }
    }

    fn as_hours(&self) -> f64 {
        return self.minutes as f64 / 60.0;
    }
}
