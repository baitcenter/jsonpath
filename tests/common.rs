extern crate env_logger;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;

use std::io::Read;

use serde_json::Value;

use self::jsonpath::Selector;

#[allow(dead_code)]
pub fn setup() {
    let _ = env_logger::try_init();
}

#[allow(dead_code)]
pub fn read_json(path: &str) -> Value {
    let mut f = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

#[allow(dead_code)]
pub fn read_contents(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

#[allow(dead_code)]
pub fn select_and_then_compare<'a>(path: &str, json: Value, target: Value) {
    let mut s = Selector::new();
    let _ = s.str_path(path);
    let _ = s.value(&json);
    let result = serde_json::to_value(s.select().unwrap()).unwrap();
    assert_eq!(result, target, "{}", path);
}

#[allow(dead_code)]
pub fn compare_result<'a>(result: Vec<&Value>, target: Value) {
    let result = serde_json::to_value(result).unwrap();
    assert_eq!(result, target);
}