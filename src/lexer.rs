use std::thread::current;

#[derive(Debug)]
pub enum Token {
    ID(String),
    Char(char),
    Number(u8),
    Comma,
    
}

pub struct Lexer<'a> {
    source:  &'a String,
    current_char: char,
    current_index: usize,
    token_list: Vec<Token>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &String) -> Lexer {
        Lexer {
            source: source,
            current_char: source.chars().nth(0).unwrap_or_else(|| {
                println!("Empty String!");
                panic!();
            }),
            current_index: 0,
            token_list: Vec::new()
        }
    }

    pub fn advance(&mut self) {
        self.current_index += 1;
        self.current_char = self.source.chars().nth(self.current_index).unwrap_or('\0');
    }
}

