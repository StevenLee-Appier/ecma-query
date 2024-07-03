mod ds;
mod literal;
use ds::{LexerCahce, LexerCursor};
use crate::token::TokenKind;

pub struct Lexer<'a> {
    cursor: LexerCursor<'a>,
    cache: LexerCahce,
}

impl <'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut iter = source.char_indices();

        match iter.next() {
            Some((index, ch)) => {
                let mut lexer = Lexer {
                    cursor: LexerCursor::new(source, Some(ch), iter, index),
                    cache: LexerCahce::new(TokenKind::Start),
                };
                lexer.next_token();
                lexer
            }
            None => {
                Lexer {
                    cursor: LexerCursor::new(source, None, iter, 0),
                    cache: LexerCahce::new(TokenKind::EOFToken)
                }
            }
        }
    }
    fn get_char(&mut self) -> Option<char> {
        self.cursor.cur_char.clone()
    }
    fn eat_char(&mut self) {
        match self.cursor.iter.next() {
            Some((index, char)) => {
                self.cursor.offset = index;
                self.cursor.cur_char = Some(char);
            }
            None => {
                self.cursor.cur_char = None;
            }
        }
    }
    fn eat_change_line_char(&mut self) {
        self.eat_char();
        self.cursor.cur_line += 1;
        self.cursor.cur_line_start = self.cursor.offset;
    }
    pub fn get_token(&self) -> TokenKind {
        self.cache.token.clone()
    }
    pub fn next_token(&mut self) {
        self.cache.token = self.scan();
    }
    fn skip_change_line_and_space(&mut self) -> bool {
        let mut flag = false;
        loop {
            match self.get_char() {
                Some(ch) => {
                    match ch {
                        ' ' | '\t' => {
                            flag = true;
                            self.eat_char(); 
                            continue; 
                        }
                        '\n' => {
                            self.eat_change_line_char();
                            continue;
                        }
                        _ => break
                    }
                }
                None => break
            }
        }
        flag
    }
    fn scan(&mut self) -> TokenKind {
        let is_space = self.skip_change_line_and_space();
        if is_space {
            return TokenKind::Space;
        }
        match self.get_char() {
            None => {
               TokenKind::EOFToken
            }
            Some(ch) => {
                match ch {
                    '(' => {
                        self.eat_char();
                        TokenKind::ParenthesesLeftPunctuator
                    }
                    ')' => {
                        self.eat_char();
                        TokenKind::ParenthesesRightPunctuator
                    }
                    '0' | '1' | '2' | '3' | '4' | 
                    '5' | '6' | '7' | '8' | '9' => {
                        todo!()
                    }
                    '\'' | '\"' => {
                       self.read_string(ch)
                    }
                    _ => {
                        self.read_idenfier()
                    }
                }
            }
        }
    }
}
