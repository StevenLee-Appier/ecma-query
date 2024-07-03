use query_parser::{lexer::Lexer, token::TokenKind};

fn main() {
    let code = "(type 'BinaryExpression')";
    let mut lexer = Lexer::new(code);
    loop {
        match lexer.get_token() {
            TokenKind::EOFToken => break,
            _ => {
                println!("{:?}", lexer.get_token())
            }
        }
        lexer.next_token();
    }
}

