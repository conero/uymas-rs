use crate::CONFIG;
use cli::args;
use env_logger::filter::{Builder, Filter};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

const LOG_LEVEL: &str = "level";

/// 系统日志
pub struct Logger {
    filter: Filter,
}

impl Logger {
    pub fn new() -> Self {
        let mut builder = Builder::from_env(LOG_LEVEL);
        // 读取配置
        if !CONFIG.log.level.is_empty() {
            let filter = match CONFIG.log.level.to_lowercase().as_str() {
                "trace" => LevelFilter::Trace,
                "debug" => LevelFilter::Debug,
                "info" => LevelFilter::Info,
                "warn" => LevelFilter::Warn,
                "error" => LevelFilter::Error,
                "off" => LevelFilter::Off,
                _ => LevelFilter::Info,
            };
            builder.filter(None, filter);
        }

        Self {
            filter: builder.build(),
        }
    }

    // 初始化
    pub fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();
        log::set_max_level(logger.filter.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        // Check if the record is matched by the logger before logging
        // 通过`cfg!` 来判断是否开启改future
        let file_desc = if cfg!(feature = "dev") {
            if let Some(fl) = record.file() {
                format!(" {}@{}", fl, record.line().unwrap())
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        let now = chrono::Local::now();
        if self.filter.matches(record) {
            let log_msg = format!(
                "[{} {}{}] {}",
                now.format("%Y-%m-%d %H:%M:%S").to_string(),
                record.level(),
                file_desc,
                record.args()
            );

            // 保存日志到文件中
            let mut write_ok = false;
            if !CONFIG.log.file.is_empty() {
                let log_file_name = args::project_path(CONFIG.log.file.clone());
                let fl_rslt = cli::util_fs::append_file(log_file_name, log_msg.clone());
                match fl_rslt {
                    Ok(_) => {
                        write_ok = true;
                    }
                    Err(er) => {
                        println!("Error，{}日志写入失败，\n  {}", CONFIG.log.file.clone(), er);
                    }
                }
            }

            if !write_ok {
                println!("{}", log_msg);
            }
        }
    }

    fn flush(&self) {}
}
