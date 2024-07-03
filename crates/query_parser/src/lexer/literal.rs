use crate::token::TokenKind;

use super::Lexer;

impl <'a> Lexer<'a> {
    pub (super) fn read_idenfier(&mut self) -> TokenKind {
        loop {
            match self.get_char() {
                None => break,
                Some(ch) => {
                    match ch {
                        'A'..='Z' | 'a'..='z' | '-' | '_' => {
                            self.eat_char();
                            continue;
                        }
                        ' ' | '\t' | '\n' => break,
                        '(' | ')' => break,
                        _ => {
                            panic!("[ERROR]: Unexpect char {:?}", ch);
                        }
                    }
                }
            }
        }
        TokenKind::Identifier
    }
    pub (super) fn read_string(&mut self, end_char: char) -> TokenKind {
        self.eat_char();
        loop {
            match self.get_char() {
                None => panic!(),
                Some(ch) => {
                    if ch == end_char {
                        self.eat_char();
                        return TokenKind::StringLiteral;
                    }
                    self.eat_char();
                    continue;
                }
            }
        }
    }
}