use std::path::Path;
use std::collections::HashMap;

type IniData = HashMap<String, HashMap<String, String>>;

pub struct Ini(IniData);

impl<'a> Ini {
    fn new() -> Ini {
        Ini(HashMap::new())
    }
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Ini {
        unimplemented!()
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        unimplemented!()
    }
    pub fn section<S: Into<String>>(&'a self, name: S) -> Option<&'a HashMap<String, String>> {
        let name = name.into();
        self.0.get(&name)
    }
}
