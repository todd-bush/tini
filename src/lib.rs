mod parser;

use std::path::Path;
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;

use parser::{parse_line, Parsed};

type IniParsed = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct Ini(IniParsed);

impl<'a> Ini {
    fn new() -> Ini {
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
        let file = File::open(path)
                       .ok()
                       .expect(&format!("Can't open `{}`!", path.as_ref().display()));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer)
                      .ok()
                      .expect(&format!("Can't read `{}`!", path.as_ref().display()));
        Ini::from_string(&buffer)
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }
    fn get_raw(&'a self, section: &str, key: &str, default: &str) -> String {
        let s = self.0.get(section);
        let result = match s {
            Some(hm) => {
                let k = hm.get(key);
                let value = match k {
                    Some(v) => v.to_owned(),
                    None => default.to_owned(),
                };
                value
            }
            None => default.to_owned(),
        };
        result
    }
    pub fn get<T: FromStr>(&'a self, section: &str, key: &str, default: &str) -> T {
        self.get_raw(section, key, default)
            .parse()
            .ok()
            .expect(&format!("cannot convert to given type"))
    }
    pub fn get_vec<T: FromStr>(&'a self, section: &str, key: &str, default: &str) -> Vec<T> {
        self.get_raw(section, key, default)
            .split(',')
            .map(|s| s.trim().parse().ok().expect(&format!("cannot convert {} to given type", s)))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_float() {
        let ini = Ini::from_string("[section]\nname=10.5");
        let name: f64 = ini.get("section", "name", "0");
        assert_eq!(name, 10.5);
    }

    #[test]
    fn test_float_vec() {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6");
        let name: Vec<f64> = ini.get_vec("section", "name", "0");
        assert_eq!(name, [1.2, 3.4, 5.6]);
    }
}
