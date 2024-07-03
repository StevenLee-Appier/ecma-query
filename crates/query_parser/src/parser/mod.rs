use crate::{ast::{Identifier, RootNode, SExpressionNode}, lexer::Lexer, token::TokenKind};

struct Parser<'a> {
    lexer: Lexer<'a>,
}

type ParseResult<T> = Result<T, String>;

impl<'a> Parser<'a> {
    fn get_token(&mut self) -> TokenKind {
        self.lexer.get_token()
    }
    fn next_token(&mut self) {
        self.lexer.next_token();
    }
    fn expect_token(&mut self, token: TokenKind) {
       if self.get_token() == token  {
        self.next_token();
       }
    }
    fn match_token(&mut self, token: TokenKind ) -> bool {
        self.get_token() == token
    }
    fn parse_root(&mut self) -> ParseResult<RootNode<'a>> {

    }
    fn parse_node(&mut self) {
        self.expect_token(TokenKind::ParenthesesLeftPunctuator);
        // parse key 

        // parse children
        if self.match_token(TokenKind::ParenthesesLeftPunctuator) {

        }
        else {
            // array node
        }

    }
    fn parse_identifier(&mut self) -> ParseResult<Identifier<'a>> {
        
    }
}