use serde_json::{Map, Value};
use std::fs::File;
use std::io::Write;

pub fn export_json(vec: Vec<String>) {
    let mut map = Map::new();

    for s in vec {
        let key = s.split_whitespace().next().unwrap_or("").to_lowercase();
        map.insert(key, Value::String(s));
    }
    let json_value = Value::Object(map);
    let json_string = serde_json::to_string_pretty(&json_value).expect("Failed to serialize JSON");

    let mut file = File::create("output.json").expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
}
