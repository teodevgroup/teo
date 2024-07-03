pub mod matcher;
pub mod matcher_functions;
pub mod server;

use std::process::{Child, Command, Stdio};
use std::{env, thread};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use key_path::{KeyPath, path};
use serde_json::{Map, Number, Value};
use crate::lib::matcher::Matcher;
use whoami::Platform;

fn schema_from_file(file: &str) -> PathBuf {
    let file_path = Path::new(file);
    let parent = file_path.parent().unwrap();
    parent.join("schema.teo")
}

fn teo_exe_path_buf() -> PathBuf {
    let mut current_dir = env::current_dir().unwrap();
    while current_dir != PathBuf::from("/") {
        let exe_path = current_dir.join(if whoami::platform() == Platform::Windows {
            "target/debug/cargo-teo.exe"
        } else {
            "target/debug/cargo-teo"
        });
        if exe_path.is_file() {
            return exe_path;
        }
        current_dir = current_dir.parent().unwrap().to_owned()
    }
    panic!("Cannot find Teo executable file.")
}

fn teo_exe_path() -> String {
    teo_exe_path_buf().to_str().unwrap().to_string()
}

pub struct ExecutionHandle {
    child: Option<Child>
}

impl ExecutionHandle {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn execute(&mut self, file: &str, args: &str) {
        env::set_var("TEO_ENV", "test");
        self.child = Some(Command::new(teo_exe_path())
            .arg("-s")
            .arg(schema_from_file(file))
            .arg(args)
            .stdout(Stdio::null()).spawn().unwrap());
        thread::sleep(std::time::Duration::from_secs(3));
    }

    pub fn exit(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

unsafe impl Sync for ExecutionHandle { }

pub fn req<J: Borrow<Value>>(port: i32, action: &str, model: &str, data: J) -> Value {
    let url = format!("http://127.0.0.1:{}/{}/{}", port, model, action);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).json(data.borrow()).send().unwrap();
    res.json().unwrap()
}

pub fn purge_and_seed(port: i32) {
    println!("purge_and_seed start");
    let url = format!("http://127.0.0.1:{}/danger/purge_seed", port);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).send().unwrap();
    let text = res.text().unwrap();
    println!("{:?}" , text);
}

pub fn json_match<J: Borrow<Value>, M: Borrow<Matcher>>(value: J, matcher: M) -> Result<(), String> {
    json_match_internal(value.borrow(), matcher.borrow(), &path![])
}

fn json_match_internal(value: &Value, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    if matcher.is_ignore() {
        return Ok(());
    }
    match value {
        Value::Null => json_match_null(matcher, path)?,
        Value::String(string) => json_match_string(value, string, matcher, path)?,
        Value::Bool(bool) => json_match_bool(value, bool, matcher, path)?,
        Value::Number(number) => json_match_number(value, number, matcher, path)?,
        Value::Array(array) => json_match_array(value, array, matcher, path)?,
        Value::Object(object) => json_match_object(value, object, matcher, path)?,
    }
    Ok(())
}

fn json_match_null(matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    if matcher.is_null() {
        return Ok(());
    }
    json_match_error(&Value::Null, path)
}

fn json_match_string(value: &Value, string: &String, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::String(s) => json_match_error_if_not(s == string, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_bool(value: &Value, bool: &bool, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Bool(b) => json_match_error_if_not(b == bool, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_number(value: &Value, number: &Number, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Number(n) => json_match_error_if_not(n == number, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_array(value: &Value, array: &Vec<Value>, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Array(a) => {
            json_match_error_if_not(a.len() == array.len(), value, path)?;
            for (index, matcher) in a.iter().enumerate() {
                let array_value = array.get(index).unwrap();
                json_match_internal(array_value, matcher, &(path + index))?;
            }
            Ok(())
        },
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_object(value: &Value, map: &Map<String, Value>, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Object(m) => {
            // compare keys
            json_match_error_if_not(m.len() == map.len(), value, path)?;
            let m_keys: HashSet<&str> = m.keys().into_iter().map(|k| k.as_str()).collect();
            let map_keys: HashSet<&str> = map.keys().into_iter().map(|k| k.as_str()).collect();
            json_match_error_if_not(m_keys == map_keys, value, path)?;
            for (key, matcher) in m.iter() {
                let map_value = map.get(key).unwrap();
                json_match_internal(map_value, matcher, &(path + key))?;
            }
            Ok(())
        },
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_error(value: &Value, path: &KeyPath) -> Result<(), String> {
    if path.is_empty() {
        Err(format!("value `{}` does not match json matcher.", value.to_string()))
    } else {
        Err(format!("value `{}` at `{}` does not match json matcher.", value.to_string(), path.to_string()))
    }
}

fn json_match_error_if_not(result: bool, value: &Value, path: &KeyPath) -> Result<(), String> {
    if !result {
        json_match_error(value, path)
    } else {
        Ok(())
    }
}

#[macro_export]
macro_rules! assert_json {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                use std::borrow::Borrow;
                if let Err(string) = $crate::lib::json_match(left_val.borrow(), right_val.borrow()) {
                    panic!("{}", string)
                }
            }
        }
    };
}
