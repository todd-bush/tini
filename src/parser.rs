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
