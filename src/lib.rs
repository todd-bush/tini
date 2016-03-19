//! _**tini** is a **t**iny **ini**-file reader and writer_
//!
//! This small library provides basic functions to operate with ini-files
//! # Examples
//! ## Read from buffer and get string values
//! ````
//! use tini::Ini;
//!
//! let conf = Ini::from_buffer(["[search]",
//!                              "g = google.com",
//!                              "dd = duckduckgo.com"].join("\n"));
//!
//! let g: String = conf.get("search", "g").unwrap();
//! let dd: String = conf.get("search", "dd").unwrap();
//!
//! assert_eq!(g, "google.com");
//! assert_eq!(dd, "duckduckgo.com");
//! ````
//! ## Construct in program and get vectors
//! ````
//! use tini::Ini;
//!
//! let conf = Ini::new().section("floats")
//!                      .item("consts", "3.1416, 2.7183")
//!                      .section("integers")
//!                      .item("lost", "4,8,15,16,23,42")
//!                      .end();
//! let consts: Vec<f64> = conf.get_vec("floats", "consts").unwrap();
//! let lost: Vec<i32> = conf.get_vec("integers", "lost").unwrap();
//!
//! assert_eq!(consts, [3.1416, 2.7183]);
//! assert_eq!(lost, [4, 8, 15, 16, 23, 42]);
//! ````
use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufReader, Read, BufWriter, Write};
use std::fs::File;
use std::str::FromStr;
use parser::{parse_line, Parsed};
use std::fmt;


#[macro_export]
/// Get record with the default value.
///
/// If the item contains a mistake, you can use the default value for the correct replacement.
///
/// ```
/// #[macro_use]
/// extern crate tini;
/// use tini::Ini;
///
/// fn main() {
///     let conf = Ini::new().section("test")
///                          .item("pi", "~3.14")
///                          .end();
///     let result: f32 = get_or!(conf, "test", "pi", std::f32::consts::PI);
///     assert_eq!(result, std::f32::consts::PI);
/// }
/// ```
macro_rules! get_or {
    ($o:ident, $s:expr, $k:expr, $d:expr) => {
        $o.get($s, $k).unwrap_or($d)
    }
}

#[macro_export]
/// Get vector record with the default value.
///
/// If the item contains a mistake, you can use the default value for the correct replacement.
///
/// ```
/// #[macro_use]
/// extern crate tini;
/// use tini::Ini;
///
/// fn main() {
///     let conf = Ini::new().section("test")
///                          .item("list", "1, 2, --, 4")
///                          .end();
///     let result = get_vec_or!(conf, "test", "list", vec![1, 2, 3, 4]);
///     assert_eq!(result, vec![1, 2, 3, 4]);
/// }
/// ```
macro_rules! get_vec_or {
    ($o:ident, $s:expr, $k:expr, $d:expr) => {
        $o.get_vec($s, $k).unwrap_or($d)
    }
}

type Section = HashMap<String, String>;
type IniParsed = HashMap<String, Section>;

/// Structure for INI-file data
#[derive(Debug)]
pub struct Ini {
    #[doc(hidden)]
    data: IniParsed,

    last_section: Section,
    last_section_name: String,
}

impl Ini {
    /// Create an empty Ini
    pub fn new() -> Ini {
        Ini {
            data: IniParsed::new(),
            last_section: Section::new(),
            last_section_name: String::new(),
        }
    }
    fn from_string(string: &str) -> Ini {
        let mut result = Ini::new();
        for (i, line) in string.lines().enumerate() {
            match parse_line(&line) {
                Parsed::Section(name) => result = result.section(name),
                Parsed::Value(name, value) => result = result.item(name, value),
                Parsed::Error(msg) => println!("line {}: error: {}", i, msg),
                _ => (),
            };
        }
        result.end()
    }
    /// Construct Ini from file
    ///
    /// # Examples
    /// You may use Path
    ///
    /// ```
    /// use std::path::Path;
    /// use tini::Ini;
    ///
    /// let path = Path::new("./example.ini");
    /// let conf = Ini::from_file(path);
    /// assert!(conf.ok().is_some());
    /// ```
    ///
    /// or `&str`
    ///
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_file("./example.ini");
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Ini, io::Error> {
        let file = try!(File::open(path));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        try!(reader.read_to_string(&mut buffer));
        Ok(Ini::from_string(&buffer))
    }
    /// Construct Ini from buffer
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }
    /// Add new section to Ini. This function also appends previous section to Ini
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::new().section("empty").end();
    /// assert_eq!(conf.to_buffer(), "[empty]".to_owned());
    /// ```
    pub fn section<S: Into<String>>(mut self, name: S) -> Self {
        if self.last_section_name.len() != 0 {
            self = self.end()
        }
        self.last_section_name = name.into();
        self
    }
    /// Add key-value pair to last section
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::new().section("test")
    ///                      .item("value", "10")
    ///                      .end();
    /// let value: Option<u8> = conf.get("test", "value");
    /// assert_eq!(value, Some(10));
    /// ```
    pub fn item<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.last_section.insert(name.into(), value.into());
        self
    }
    /// Append last created section to Ini
    pub fn end(mut self) -> Ini {
        self.data.insert(self.last_section_name.clone(), self.last_section.clone());
        self.last_section.clear();
        self
    }
    /// Write Ini to file. This function is similar to `from_file` in use.
    pub fn to_file<S: AsRef<Path> + ?Sized>(&self, path: &S) -> Result<(), io::Error> {
        let file = try!(File::create(path));
        let mut writer = BufWriter::new(file);
        let result: String = format!("{}", self);
        try!(writer.write_all(result.as_bytes()));
        Ok(())
    }
    /// Write Ini to buffer
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// // you may use `conf.to_buffer()`
    /// let value: String = conf.to_buffer();
    /// // or format!("{}", conf);
    /// // let value: String = format!("{}", conf);
    /// // but the result will be the same
    /// assert_eq!(value, "[section]\none = 1".to_owned());
    /// ```
    pub fn to_buffer(&self) -> String {
        let buffer = format!("{}", self);
        buffer
    }

    fn get_raw(&self, section: &str, key: &str) -> Option<&String> {
        let s = self.data.get(section);
        match s {
            Some(hm) => hm.get(key),
            None => None,
        }
    }
    /// Get scalar value of key in section
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn get<T: FromStr>(&self, section: &str, key: &str) -> Option<T> {
        let data = self.get_raw(section, key);
        match data {
            Some(x) => x.parse().ok(),
            None => None,
        }
    }
    /// Get vector value of key in section
    ///
    /// The function returns None if one of the elements can not be parsed.
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\nlist = 1, 2, 3, 4");
    /// let value: Option<Vec<u8>> = conf.get_vec("section", "list");
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec<T>(&self, section: &str, key: &str) -> Option<Vec<T>>
        where T: FromStr + Copy
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
        for section in &self.data {
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
