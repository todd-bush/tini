const SEPARATOR: char = ',';
const ESCAPE: char = '\\';

pub struct Tokenizer<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.string.chars().skip(self.index);
        let mut index = self.index;
        let mut result = None;
        while let Some(curr) = iterator.next() {
            match curr {
                SEPARATOR => {
                    result = Some(self.string[self.index..index].trim());
                    self.index = index + 1;
                    break;
                }
                ESCAPE => {
                    if iterator.next().is_some() {
                        index += 1;
                    }
                }
                _ => (),
            }
            index += 1;
        }
        let max_len = self.string.len();
        if self.index < max_len && result == None {
            result = Some(self.string[self.index..].trim());
            self.index = max_len;
        }
        result
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Tokenizer {
        Tokenizer { string, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn items() {
        let s = "1,2,3,4,5";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn escaped() {
        let s = "1,2,3\\,4,5,6";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "2", "3\\,4", "5", "6"]);
    }

    #[test]
    fn last_quoted() {
        let s = "1,2,3\\,";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "2", "3\\,"]);
    }

    #[test]
    fn one_escaped_item() {
        let s = "\\,\\,\\,";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, [s]);
    }

    #[test]
    fn empty_one() {
        let s = "1,,2,3";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "", "2", "3"]);
    }

    #[test]
    fn empty_and_escape() {
        let s = "1,\\,,2,3";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "\\,", "2", "3"]);
    }

    #[test]
    fn empty_last() {
        let s = "1,2,";
        let items: Vec<_> = Tokenizer::new(s).collect();
        assert_eq!(items, ["1", "2"]);
    }
}
