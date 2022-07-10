use super::log;

pub struct LogFormat;

impl log::Format for LogFormat {
    fn parse_log(&self, s: &str) -> Option<Vec<log::KeyVal>> {
        if !s.contains('=') {
            return None;
        }

        let pairs = logfmt::parse(s);

        let mut out = Vec::with_capacity(pairs.len());
        for pair in pairs {
            out.push(log::KeyVal {
                key: pair.key,
                val: pair.val.unwrap_or_else(|| "".to_string()),
            });
        }

        Some(out)
    }
}
