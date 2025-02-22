//! Utilities for configuring logging
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::Once;

use colored::*;
use env_logger::Logger;
use log::LevelFilter;
use log::Log;
use log::Record;

static ONCE_INIT: Once = Once::new();

static TRACE_CONTEXT_LINES: usize = 20;

/// A special logger implementation that uses env_logger for configuring filters
/// and implements a custom logging format.
///
/// To reduce the number of trace log lines, this logger will log them into a ring buffer. When
/// When a higher log level record is logged, the previous trace logs are printed to provide
/// context.
struct SresLogger {
    /// Contains the last `TRACE_CONTEXT_LINES` of trace-level logs.
    trace_logs: Mutex<VecDeque<String>>,
    logger: Logger,
}

impl SresLogger {
    pub fn new(logger: Logger) -> Self {
        log::set_max_level(logger.filter());
        Self {
            trace_logs: Mutex::new(VecDeque::new()),
            logger,
        }
    }

    fn format_record(&self, record: &Record) -> String {
        match record.level() {
            log::Level::Error => {
                format!("{} {}", "E".red().bold(), record.args().to_string().red())
            }
            log::Level::Warn => format!(
                "{} {}",
                "W".yellow().bold(),
                record.args().to_string().yellow()
            ),
            log::Level::Info => format!(
                "{} {}",
                "I".blue().bold(),
                record.args().to_string().normal()
            ),
            log::Level::Debug => format!("{} {}", "D".blue(), record.args().to_string().normal()),
            log::Level::Trace => format!("{}", record.args().to_string().dimmed()),
        }
    }
}

impl Log for SresLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.logger.matches(record) {
            return;
        }
        let record_str = self.format_record(record);
        let mut trace_logs = self.trace_logs.lock().unwrap();
        if record.level() == LevelFilter::Trace {
            trace_logs.push_front(record_str);
            trace_logs.truncate(TRACE_CONTEXT_LINES);
        } else {
            if !trace_logs.is_empty() {
                if trace_logs.len() == TRACE_CONTEXT_LINES {
                    println!("{}", "...".dimmed());
                }
                for log in trace_logs.drain(0..).rev() {
                    println!("{}", log);
                }
            }
            println!("{}", record_str);
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    ONCE_INIT.call_once(|| {
        let filter_config = std::env::var("SRES_LOG").unwrap_or("error".to_string());
        let filter = env_logger::builder().parse_filters(&filter_config).build();
        log::set_boxed_logger(Box::new(SresLogger::new(filter))).unwrap();
    });
}

pub fn test_init(verbose: bool) {
    ONCE_INIT.call_once(|| {
        let filter_config = std::env::var("SRES_LOG").unwrap_or(
            if verbose {
                "info,cpu_state=trace"
            } else {
                "warn"
            }
            .to_string(),
        );
        let filter = env_logger::builder().parse_filters(&filter_config).build();
        log::set_boxed_logger(Box::new(SresLogger::new(filter))).unwrap();
    });
}
