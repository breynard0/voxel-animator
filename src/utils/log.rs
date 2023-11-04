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
        "{} \x1b[32m[{}:{}:{}] \x1b[{}m{}\x1b[0m",
        match level {
            LogLevel::INFO => "\x1b[34m[INFO]",
            LogLevel::WARNING => "\x1b[33m[WARNING]",
            LogLevel::ERROR => "\x1b[31m[ERROR]",
            LogLevel::FATAL => "\x1b[31;1m[FATAL]",
        },
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

pub fn log_seperated<S>(value: Vec<S>, level: LogLevel, truncate_len: usize)
where
    S: ToString,
{
    let mut output = String::new();
    for s in value {
        let s = s.to_string();
        match s.len() >= truncate_len {
            true => {
                output.push_str(&s.as_str()[0..truncate_len]);
                output.push(' ');
            }
            false => {
                output.push_str(&s);
                output.push_str(&" ".repeat(truncate_len - s.len() + 1));
            }
        }
    }
    log(output, level)
}
