

pub fn load_test_json(name: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("test_json/{}.json", name))
}