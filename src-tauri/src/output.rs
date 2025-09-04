use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub(crate) enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Serialize)]
pub(crate) struct LogEntry {
    message: String,
    level: LogLevel,
}

static LOGS: Lazy<Mutex<Vec<LogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub(crate) fn get_logs() -> Vec<LogEntry> {
    let held_logs = LOGS.lock().unwrap().clone();

    LOGS.lock().unwrap().clear();

    held_logs
}

pub(crate) fn add_log(log: String, level: LogLevel) {
    LOGS.lock().unwrap().push(LogEntry { message: log, level });
}
