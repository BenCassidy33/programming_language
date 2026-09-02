#[cfg(test)]
mod token_tests {
    use programming_language::token::{Token, TokenKind};

    #[test]
    pub fn token_new_known() {
        assert!(
            Token::new("!")
                == Token {
                    raw: "!",
                    kind: TokenKind::BANG
                }
        );
    }

    #[test]
    pub fn token_new_unknown() {
        assert!(
            Token::new("x")
                == Token {
                    raw: "x",
                    kind: TokenKind::IDENT
                }
        );
    }
}
