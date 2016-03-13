use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

type IniData = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct Ini(IniData, Vec<String>);

impl<'a> Ini {
    fn new() -> Ini {
        Ini(HashMap::new(), Vec::new())
    }
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Ini {
        let file = File::open(path)
                        .ok()
                        .expect(&format!("Can't open `{}` file!", path.as_ref().display()));
        let reader = BufReader::new(file);
        let mut result = Ini::new();
        result.1 = reader.lines()
                         .filter_map(|x| x.ok())
                         .collect();
        result
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        unimplemented!()
    }
    pub fn section<S: Into<String>>(&'a self, name: S) -> Option<&'a HashMap<String, String>> {
        let name = name.into();
        self.0.get(&name)
    }
}
