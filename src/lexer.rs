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

        while let Some(nc) = chars.peek() {
            let current_kind = TokenKind::from_str(&tok);

            if current_kind.is_identifier() {
                if TokenKind::is_identifier_continue(*nc) {
                    tok.push(chars.next().unwrap());
                    continue;
                } else {
                    break;
                }
            }

            if TokenKind::can_continue(&tok) && !nc.is_whitespace() && !nc.is_ascii_alphanumeric() {
                tok.push(chars.next().unwrap());
                continue;
            }

            if current_kind.is_delimiter() {
                break;
            }

            break;
        }

        tokens.push(Token::new(tok));
    }

    Some(tokens)
}
