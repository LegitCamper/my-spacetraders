use simple_logger::SimpleLogger;
use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

fn log() {
    INIT.call_once(|| {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .with_colors(true)
            .init()
            .unwrap();
    });
}

#[cfg(test)]
mod responses;
