use crate::token::{Token, TokenKind};

pub fn lex(source: &str) -> Option<Vec<Token>> {
    let source = source.replace("\t", "").replace("\n", "");

    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    loop {
        let mut tok = if let Some(next) = chars.next() {
            if next.is_whitespace() {
                continue;
            }

            next.to_string()
        } else {
            break;
        };

        while let Some(nc) = chars.peek()
            && (TokenKind::can_continue(&tok)
                || (TokenKind::from_str(&tok).is_identifier() && nc.is_ascii_alphanumeric()))
            && !TokenKind::from_str(&nc.to_string()).is_delimiter()
            && !nc.is_whitespace()
        {
            tok.push(chars.next().unwrap());
        }

        tokens.push(Token::new(tok));
    }

    Some(tokens)
}
