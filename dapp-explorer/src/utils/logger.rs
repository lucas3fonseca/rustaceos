use log::{Level, Log, Metadata, Record, SetLoggerError};
use wasm_bindgen::JsValue;
use web_sys::console;

struct WebLogger;

static LOGGER: WebLogger = WebLogger;

pub struct Config {
    pub level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: Level::Trace,
        }
    }
}

impl Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let msg = format!(
                "{}:{} -- {}",
                record.level(),
                record.target(),
                record.args()
            );
            let msg = &JsValue::from_str(&msg);
            match metadata.level() {
                Level::Info => console::info_1(&msg),
                Level::Warn => console::warn_1(&msg),
                Level::Error => console::error_1(&msg),
                _ => console::debug_1(&msg),
            }
        }
    }

    fn flush(&self) {}
}

pub fn try_init(config: Config) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    let level = config.level;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() {
    try_init(Config::default())
        .expect("logger::init should not be called after logger initialized");
}

pub fn custom_init(config: Config) {
    try_init(config).expect("logger::custom_init should not be called after logger initialized");
}
