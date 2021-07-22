use logfmt;

use super::log;

pub struct LogFormat;

impl LogFormat {
    pub fn new() -> Box<dyn log::Format + 'static> {
        return Box::new(&LogFormat);
    }
}

impl log::Format for &LogFormat {
    fn parse_log(&self, s: &str) -> Option<Vec<log::KeyVal>> {
        if !s.contains("=") {
            return None;
        }

        let pairs = logfmt::parse(s);

        let mut out = Vec::new();
        for pair in pairs {
            out.push(log::KeyVal {
                key: pair.key,
                val: pair.val.unwrap_or("".to_string()),
            });
        }

        Some(out)
    }
}
