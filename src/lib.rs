use std::path::Path;
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;

type Section = HashMap<String, String>;
type IniParsed = HashMap<String, Section>;

#[derive(Debug)]
pub struct Ini(IniParsed);

impl Ini {
    pub fn new() -> Ini {
        Ini(HashMap::new())
    }
    fn from_string(string: &str) -> Ini {
        let mut result = Ini::new();
        let mut section_name = String::new();
        let mut entry_list = HashMap::new();
        for (i, line) in string.lines().enumerate() {
            match parse_line(&line) {
                Parsed::Section(name) => {
                    if section_name.len() != 0 {
                        result.0.insert(section_name, entry_list.clone());
                        entry_list.clear();
                    }
                    section_name = name;
                }
                Parsed::Value(name, value) => {
                    entry_list.insert(name, value);
                }
                Parsed::Error(msg) => println!("line {}: error: {}", i, msg),
                _ => (),
            };
        }
        // add last section
        if section_name.len() != 0 {
            result.0.insert(section_name, entry_list.clone());
            entry_list.clear();
        }
        result
    }

    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Ini {
        let file = File::open(path).expect(&format!("Can't open `{}`!", path.as_ref().display()));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer)
                      .expect(&format!("Can't read `{}`!", path.as_ref().display()));
        Ini::from_string(&buffer)
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }
    fn get_raw(&self, section: &str, key: &str) -> Option<&String> {
        let s = self.0.get(section);
        match s {
            Some(hm) => hm.get(key),
            None => None,
        }
    }
    pub fn get<T: FromStr>(&self, section: &str, key: &str) -> Option<T> {
        let data = self.get_raw(section, key);
        match data {
            Some(x) => x.parse().ok(),
            None => None,
        }
    }
    pub fn get_def<T: FromStr>(&self, section: &str, key: &str, default: T) -> T {
        let s = self.get_raw(section, key);
        match s {
            Some(x) => x.parse().unwrap_or(default),
            None => default,
        }
    }
    pub fn get_vec<T>(&self, section: &str, key: &str, default: &[T]) -> Vec<T>
        where T: FromStr + Copy + Clone
    {
        let s = self.get_raw(section, key);
        match s {
            Some(x) => {
                x.split(',')
                 .zip(default)
                 .map(|(s, &d)| s.trim().parse().unwrap_or(d))
                 .collect()
            }
            None => default.to_vec(),
        }
    }
}

pub struct IniBuilder(IniParsed, Section, String);

impl IniBuilder {
    pub fn new() -> IniBuilder {
        IniBuilder(HashMap::new(), HashMap::new(), String::new())
    }
    pub fn section<S: Into<String>>(&mut self, name: S) -> &mut Self {
        if self.2.len() != 0 {
            self.0.insert(self.2.clone(), self.1.clone());
            self.1.clear()
        }
        self.2 = name.into();
        self
    }
    pub fn item<S: Into<String>>(&mut self, name: S, value: S) -> &mut Self {
        self.1.insert(name.into(), value.into());
        self
    }
    pub fn build(&mut self) -> Ini {
        let result = self.section("").0.clone();
        Ini(result)
    }
}

#[derive(Debug)]
enum Parsed {
    Error(String),
    Empty,
    Section(String),
    Value(String, String), /* Vector(String, Vec<String>), impossible, because HashMap field has type String, not Vec */
}

fn parse_line(line: &str) -> Parsed {
    let content = line.split(';').nth(0).unwrap().trim();
    if content.len() == 0 {
        return Parsed::Empty;
    }
    // add checks for content
    if content.starts_with('[') {
        if content.ends_with(']') {
            let section_name = content.trim_matches(|c| c == '[' || c == ']').to_owned();
            return Parsed::Section(section_name);
        } else {
            return Parsed::Error("incorrect section syntax".to_owned());
        }
    } else if content.contains('=') {
        let mut pair = content.split('=').map(|s| s.trim());
        let key = pair.next().unwrap().to_owned();
        let value = pair.next().unwrap().to_owned();
        return Parsed::Value(key, value);
    }
    Parsed::Error("incorrect syntax".to_owned())
}

#[test]
fn test_comment() {
    match parse_line(";------") {
        Parsed::Empty => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_entry() {
    match parse_line("name1 = 100 ; comment") {
        Parsed::Value(name, text) => {
            assert_eq!(name, String::from("name1"));
            assert_eq!(text, String::from("100"));
        }
        _ => assert!(false),
    }
}

#[test]
fn test_weird_name() {
    match parse_line("_.,:(){}-#@&*| = 100") {
        Parsed::Value(name, text) => {
            assert_eq!(name, String::from("_.,:(){}-#@&*|"));
            assert_eq!(text, String::from("100"));
        }
        _ => assert!(false),
    }
}

#[test]
fn test_text_entry() {
    match parse_line("text_name = hello world!") {
        Parsed::Value(name, text) => {
            assert_eq!(name, String::from("text_name"));
            assert_eq!(text, String::from("hello world!"));
        }
        _ => assert!(false),
    }
}

#[test]
fn test_incorrect_token() {
    match parse_line("[section = 1, 2 = value") {
        Parsed::Error(_) => assert!(true),
        _ => assert!(false),
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int() {
        let input: String = "[string]\nabc = 10".to_owned();
        let ini = Ini::from_buffer(input);
        let abc: Option<u32> = ini.get("string", "abc");
        assert_eq!(abc, Some(10));
    }

    #[test]
    fn test_float_def() {
        let ini = Ini::from_string("[section]\nname=10.5");
        let name: f64 = ini.get_def("section", "name", 0.0);
        assert_eq!(name, 10.5);
    }

    #[test]
    fn test_float_vec() {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6");
        let name: Vec<f64> = ini.get_vec("section", "name", &[0.0, 0.0, 0.0]);
        assert_eq!(name, [1.2, 3.4, 5.6]);
    }
}
