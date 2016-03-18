use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufReader, Read, BufWriter, Write};
use std::fs::File;
use std::str::FromStr;
use parser::{parse_line, Parsed};
use std::fmt;

type Section = HashMap<String, String>;
type IniParsed = HashMap<String, Section>;

#[derive(Debug)]
pub struct Ini(IniParsed);

#[macro_export]
macro_rules! get_or {
    ($o:ident, $s:expr, $k:expr, $d:expr) => {
        $o.get($s, $k).unwrap_or($d)
    }
}

#[macro_export]
macro_rules! get_vec_or {
    ($o:ident, $s:expr, $k:expr, $d:expr) => {
        $o.get_vec($s, $k).unwrap_or($d)
    }
}

impl Ini {
    pub fn new() -> Ini {
        Ini(IniParsed::new())
    }
    fn from_string(string: &str) -> Ini {
        let mut result = Ini::new();
        let mut section_name = String::new();
        let mut entry_list = Section::new();
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

    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Ini, io::Error> {
        let file = try!(File::open(path));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        try!(reader.read_to_string(&mut buffer));
        Ok(Ini::from_string(&buffer))
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }

    pub fn to_file<S: AsRef<Path> + ?Sized>(&self, path: &S) -> Result<(), io::Error> {
        let file = try!(File::create(path));
        let mut writer = BufWriter::new(file);
        let result: String = format!("{}", self);
        try!(writer.write_all(result.as_bytes()));
        Ok(())
    }
    pub fn to_buffer(&self) -> String {
        let buffer = format!("{}", self);
        buffer
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
    pub fn get_vec<T>(&self, section: &str, key: &str) -> Option<Vec<T>>
        where T: FromStr + Copy + Clone
    {
        let s = self.get_raw(section, key);
        match s {
            Some(x) => {
                let parsed: Vec<Option<T>> = x.split(',').map(|s| s.trim().parse().ok()).collect();
                if parsed.iter().any(|e| e.is_none()) {
                    return None;
                }
                Some(parsed.iter().map(|s| s.unwrap()).collect())
            }
            None => None,
        }
    }
}

impl fmt::Display for Ini {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for section in &self.0 {
            buffer.push_str(&format!("[{}]\n", section.0));
            for (key, value) in section.1.iter() {
                buffer.push_str(&format!("{} = {}\n", key, value));
            }
        }
        // remove last '\n'
        buffer.pop();
        write!(f, "{}", buffer)
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

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn test_bool() {
        let ini = Ini::from_buffer("[string]\nabc = true");
        let abc: Option<bool> = ini.get("string", "abc");
        assert_eq!(abc, Some(true));
    }

    #[test]
    fn test_float() {
        let ini = Ini::from_string("[section]\nname=10.5");
        let name: Option<f64> = ini.get("section", "name");
        assert_eq!(name, Some(10.5));
    }

    #[test]
    fn test_float_vec() {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6");
        let name: Option<Vec<f64>> = ini.get_vec("section", "name");
        assert_eq!(name, Some(vec![1.2, 3.4, 5.6]));
    }

    #[test]
    fn test_parse_error() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let name: Option<Vec<u8>> = ini.get_vec("section", "list");
        assert_eq!(name, None);
    }

    #[test]
    fn test_get_or_macro() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let with_value: Vec<u8> = get_vec_or!(ini, "section", "list", vec![1, 2, 3, 4]);
        assert_eq!(with_value, vec![1, 2, 3, 4]);
    }
}

mod parser {
    #[derive(Debug)]
    pub enum Parsed {
        Error(String),
        Empty,
        Section(String),
        Value(String, String), /* Vector(String, Vec<String>), impossible, because HashMap field has type String, not Vec */
    }

    pub fn parse_line(line: &str) -> Parsed {
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

    #[cfg(test)]
    mod test {
        use super::*;

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
    }
}
