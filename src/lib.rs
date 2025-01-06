use std::cell::Cell;

use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_match(haystack: &str, glob: &str) {}

enum Primitive {
    Literal(String),
    Any,
    Single,
    List(Vec<String>),
    Range(String),
}

struct Span {
    pub start: u32,
    pub end: u32,
}

struct Parser {
    start: Cell<usize>,
    current: Cell<usize>,
    source: String,
}

impl Parser {
    pub fn to_regex(&self, pattern: &str) {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
        let mut ast: Vec<Primitive> = vec![];

        loop {}
    }

    fn is_EOL(&self) -> bool {
        self.current.get() >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
