static mut LOGS: Vec<(LogLevel, String)> = vec![];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

pub fn log<S>(value: S, level: LogLevel)
where
    S: ToString,
{
    unsafe {
        LOGS.push((level, value.to_string()));
    }

    let t = chrono::Local::now();
    println!(
        "\x1b[32m[{}:{}:{}] \x1b[{}m{}\x1b[0m",
        chrono::Timelike::hour(&t),
        chrono::Timelike::minute(&t),
        chrono::Timelike::second(&t),
        match level {
            LogLevel::INFO => "34",
            LogLevel::WARNING => "33",
            LogLevel::ERROR => "31",
            LogLevel::FATAL => "31;1",
        },
        value.to_string()
    );
}

pub fn get_logs() -> Vec<(LogLevel, String)> {
    unsafe { LOGS.clone() }
}
