use std::cell::Cell;

use crate::primitives::{Primitive, AST};

pub struct Parser {
    current: Cell<usize>,
    source: String,
    ast: AST,
}

impl Parser {
    pub fn new(glob: &str) -> Parser {
        Parser {
            source: glob.to_string(),
            current: Cell::new(0),
            ast: vec![],
        }
    }

    fn is_eol(&self) -> bool {
        self.current.get() >= self.source.len()
    }

    fn char(&self) -> char {
        self.source.chars().nth(self.current.get()).unwrap()
    }

    fn advance(&self) {
        self.current.set(self.current.get() + 1);
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current.get() + 1)
    }

    fn parse(&mut self) {
        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                '\\' => {
                    self.advance();
                    self.parse_literal();
                }
                '{' => self.parse_list(),
                '[' => self.parse_range(),
                '*' => {
                    if self.peek() == Some('*') {
                        self.advance();
                        self.ast.push(Primitive::Recursive);
                    } else {
                        self.ast.push(Primitive::Any);
                    }
                }
                '?' => {
                    self.ast.push(Primitive::Single);
                }
                _ => self.parse_literal(),
            }

            self.advance();
        }
    }

    pub fn to_regex(&mut self) -> String {
        // https://{meow,purr}.cat.com
        // (meow|purr)\.cat\.com - valid regex
        // let list_regex = Regex::new(r"\{(?<middle>.*)\}").unwrap();
        self.parse();
        self.regex_generator()
    }

    fn parse_literal(&mut self) {
        let c = self.char();
        // if the previous AST is a literal, then we can combine them
        if let Some(Primitive::Literal(literal)) = self.ast.last() {
            let new_ast = Primitive::Literal(format!("{}{}", literal, c));
            self.ast.pop();
            self.ast.push(new_ast);
        } else {
            // otherwise, we just add the literal
            self.ast.push(Primitive::Literal(c.to_string()));
        }
    }

    fn parse_range(&mut self) {
        self.advance(); // Move past the `[` character

        let mut range = String::new();
        let mut is_valid = false;

        while !self.is_eol() {
            if let ']' = self.char() {
                is_valid = true;
                break;
            } else {
                range.push(self.char());
            }
            self.advance();
        }

        if is_valid {
            self.ast.push(Primitive::Range(range));
        } else {
            panic!("Malformed range: missing closing `]`");
        }
    }

    fn parse_list(&mut self) {
        self.advance();

        let mut list: Vec<String> = vec![];
        let mut is_valid = false;
        let mut current_item = String::new();

        loop {
            if self.is_eol() {
                break;
            }

            match self.char() {
                ',' => {
                    if !current_item.is_empty() {
                        list.push(current_item);
                        current_item = String::new();
                    }
                }
                '}' => {
                    if !current_item.is_empty() {
                        list.push(current_item);
                    }
                    is_valid = true;
                    break;
                }
                c => current_item = format!("{}{}", current_item, c),
            }

            self.advance();
        }

        if is_valid {
            self.ast.push(Primitive::List(list));
        } else {
            panic!("Malformed range: missing closing `]`");
        }
    }

    fn regex_generator(&self) -> String {
        let mut regex_str = String::new();

        regex_str.push('^');
        for primitive in &self.ast {
            match primitive {
                Primitive::Single => {
                    regex_str.push('.');
                }
                Primitive::Any => {
                    regex_str.push_str(".*");
                }
                Primitive::Recursive => {
                    regex_str.push_str("(?:.*/)*[^/]*");
                }
                Primitive::Literal(str) => {
                    regex_str.push_str(&str);
                }
                Primitive::Range(range) => {
                    regex_str.push('[');
                    regex_str.push_str(range);
                    regex_str.push(']');
                }
                Primitive::List(list) => {
                    regex_str.push_str("(?:");
                    regex_str.push_str(&list.join("|"));
                    regex_str.push(')');
                }
            }
        }
        regex_str.push('$');

        regex_str
    }
}
