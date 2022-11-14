use std::sync::Once;
use simple_logger::SimpleLogger;

pub fn load_test_json(name: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("test_json/{}.json", name))
}

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap();
    });

}
