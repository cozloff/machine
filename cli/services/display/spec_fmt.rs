use std::fmt;
use std::path::PathBuf;

pub struct SpecFmt;

impl SpecFmt {
    pub fn opt(value: &Option<String>) -> &str {
        value.as_deref().unwrap_or("unknown")
    }

    pub fn bool(value: Option<bool>) -> &'static str {
        match value {
            Some(true) => "yes",
            Some(false) => "no",
            None => "unknown",
        }
    }

    pub fn bytes_u32(value: Option<u32>) -> String {
        match value {
            Some(value) => format!("{value} bytes"),
            None => "unknown".to_string(),
        }
    }

    pub fn u64(value: Option<u64>) -> String {
        match value {
            Some(value) => value.to_string(),
            None => "unknown".to_string(),
        }
    }

    pub fn kib(value: Option<u64>) -> String {
        match value {
            Some(value) => format!("{value} KiB"),
            None => "unknown".to_string(),
        }
    }

    pub fn capacity(value: Option<u64>) -> String {
        match value {
            Some(bytes) => ByteSize(bytes).to_string(),
            None => "unknown".to_string(),
        }
    }

    pub fn path(value: &Option<PathBuf>) -> String {
        match value {
            Some(value) => value.display().to_string(),
            None => "unknown".to_string(),
        }
    }
}

struct ByteSize(u64);

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0;
        let gib = bytes as f64 / 1024_f64.powi(3);
        let gb = bytes as f64 / 1000_f64.powi(3);
        write!(f, "{bytes} bytes ({gib:.2} GiB / {gb:.2} GB)")
    }
}
