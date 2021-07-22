use chrono::{DateTime, Local, TimeZone};
use colored::*;

pub trait Format {
    fn parse_log(&self, s: &str) -> Option<Vec<KeyVal>>;
}

pub struct KeyVal {
    pub key: String,
    pub val: String,
}

pub struct Handler {
    buf: String,
    formats: Vec<Box<dyn Format>>,
    max_field_len: usize,
}

impl Handler {
    pub fn new(formats: Vec<Box<dyn Format>>) -> Self {
        return Handler {
            buf: String::new(),
            formats,
            max_field_len: 42,
        };
    }

    pub fn handle(&mut self, s: &str) -> &String {
        self.buf.clear();

        match self.parse_kvs(s) {
            Some(kvs) => self.format_log(Log::new(kvs)),
            None => {
                self.buf.push_str(s);
                self.buf.push_str("\r\n");
            }
        };

        &self.buf
    }

    fn parse_kvs(&mut self, s: &str) -> Option<Vec<KeyVal>> {
        for format in self.formats.iter() {
            if let Some(kvs) = format.parse_log(s) {
                return Some(kvs);
            }
        }
        None
    }

    fn format_log(&mut self, log: Log) {
        if let Some(time) = log.time {
            self.buf.push_str(&time.blue().dimmed().to_string());
            self.buf.push_str(" ");
        }

        if let Some(level) = log.level {
            self.buf.push('[');
            self.buf.push_str(&format_level(&level));
            self.buf.push_str("] ");
        }

        if let Some(msg) = log.msg {
            self.buf.push_str(&msg);
            self.buf.push_str(" ");
        }

        let mut i: i32 = 0;
        self.buf.push_str(" {");
        for kv in log.kvs.iter() {
            if i > 0 {
                self.buf.push_str("  ");
            } else {
                self.buf.push(' ');
            }
            i += 1;

            self.buf.push_str(&kv.key.cyan().to_string());
            self.buf.push_str(": ");
            if kv.val.len() > self.max_field_len {
                self.buf
                    .push_str(&kv.val[..self.max_field_len].dimmed().to_string());
                self.buf.push_str("...");
            } else {
                self.buf.push_str(&kv.val.dimmed().to_string());
            }
        }
        self.buf.push_str(&" }\r\n".clear().to_string());
    }
}

struct Log {
    level: Option<String>,
    time: Option<String>,
    msg: Option<String>,
    kvs: Vec<KeyVal>,
}

impl Log {
    fn new(orig: Vec<KeyVal>) -> Self {
        let mut log = Log {
            level: None,
            time: None,
            msg: None,
            kvs: Vec::with_capacity(orig.len()),
        };

        for kv in orig {
            log.push(kv.key, kv.val);
        }

        log
    }

    fn push(&mut self, key: String, val: String) {
        if self.is_level(&key) {
            self.level = Some(val);
        } else if self.is_msg(&key) {
            self.msg = Some(val);
        } else if self.is_time(&key) {
            self.time = Some(format_time(&val));
        } else {
            self.kvs.push(KeyVal { key, val });
        }
    }

    fn is_level(&self, key: &str) -> bool {
        self.level.is_none() && LEVEL_FIELDS.contains(&key)
    }

    fn is_msg(&self, key: &str) -> bool {
        self.msg.is_none() && MESSAGE_FIELDS.contains(&key)
    }

    fn is_time(&self, key: &str) -> bool {
        self.time.is_none() && TIME_FIELDS.contains(&key)
    }
}

const LEVEL_FIELDS: &[&str] = &["level", "lvl", "loglvl"];
const MESSAGE_FIELDS: &[&str] = &["msg", "message"];
const TIME_FIELDS: &[&str] = &["time", "ts", "timestamp"];

fn format_level(level: &str) -> String {
    match level {
        "debug" => level.blue(),
        "info" => level.green(),
        "warn" | "warning" => level.yellow(),
        "error" => level.red(),
        "fatal" | "panic" => level.white().on_red(),
        _ => level.white(),
    }
    .bold()
    .to_string()
}

fn format_time(time: &str) -> String {
    if let Ok(n) = time.parse::<f64>() {
        let mut out = n as i64;
        if n < 1e12 {
            out *= 1e9 as i64;
        } else if n < 1e15 {
            out *= 1e6 as i64;
        } else if n < 1e18 {
            out *= 1e3 as i64;
        }
        let time = Local.timestamp(out / 1e9 as i64, (out % 1e9 as i64) as u32);
        return format_time_str(time);
    }

    if let Ok(n) = DateTime::parse_from_rfc3339(time) {
        return format_time_str(n.with_timezone(&Local));
    }
    if let Ok(n) = DateTime::parse_from_rfc2822(time) {
        return format_time_str(n.with_timezone(&Local));
    }

    time.to_string()
}

fn format_time_str(time: DateTime<Local>) -> String {
    time.format("%F %T").to_string()
}
