#![allow(unused, dead_code)]

macro_rules! tokens {
    ($(($token_name:ident, $raw:literal, $category:ident)),* $(,)?) => {
        #[allow(upper_case_acronyms, non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TokenKind {
            $($token_name,)*
            IDENT,
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TokenCategory {
            Delimiter,
            Operator,
            Keyword,
            Type,
            Comptime,
            Ident,
            Other,
        }

        impl TokenKind {
            pub fn from_str(s: &str) -> Self {
                match s {
                    $($raw => Self::$token_name, )*
                    _ => Self::IDENT
                }
            }

            pub fn can_continue(c: &str) -> bool {
                match c {
                    $(
                        c if $raw.chars().count() > 1 && $raw.starts_with(c) => true,
                    )*

                    _ => false
                }
            }

            pub fn category(&self) -> TokenCategory {
                match self {
                    $(
                        Self::$token_name => TokenCategory::$category,
                    )*
                    Self::IDENT => TokenCategory::Ident,
                    _ => TokenCategory::Other
                }
            }
        }
    };
}

impl TokenKind {
    pub fn is_delimiter(self) -> bool {
        matches!(self.category(), TokenCategory::Delimiter)
    }

    pub fn is_operator(self) -> bool {
        matches!(self.category(), TokenCategory::Operator)
    }

    pub fn is_keyword(self) -> bool {
        matches!(self.category(), TokenCategory::Keyword)
    }

    pub fn is_type(self) -> bool {
        matches!(self.category(), TokenCategory::Type)
    }

    pub fn is_comptime(self) -> bool {
        matches!(self.category(), TokenCategory::Comptime)
    }

    pub fn is_identifier(self) -> bool {
        matches!(self.category(), TokenCategory::Ident)
    }

    pub fn is_identifier_start(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    pub fn is_identifier_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}

tokens!(
    (BANG, "!", Operator),
    (PLUS, "+", Operator),
    (DIV, "/", Operator),
    (STAR, "*", Operator),
    (PLUSPLUS, "++", Operator),
    (MINUS, "-", Operator),
    (MINUSMINUS, "--", Operator),
    (EQUALS, "=", Operator),
    (NOTEQUALS, "!=", Operator),
    (EQUALSEQUALS, "==", Operator),
    (SINGLE_QUOTE, "'", Delimiter),
    (DOUBLE_QUOTE, "\"", Delimiter),
    (COLON, ":", Delimiter),
    (COLONCOLON, "::", Delimiter),
    (SEMICOLON, ";", Delimiter),
    (AMP, "&", Operator),
    (BAR, "|", Operator),
    (AMPAMP, "&&", Operator),
    (BARBAR, "||", Operator),
    (COMMENT, "//", Delimiter),
    (BACK_SLASH, "\\", Delimiter),
    (LEFT_PAREN, "(", Delimiter),
    (RIGHT_PAREN, ")", Delimiter),
    (LEFT_BRACKET, "[", Delimiter),
    (RIGHT_BRACKET, "]", Delimiter),
    (LEFT_CARET, "<", Delimiter),
    (RIGHT_CARET, ">", Delimiter),
    (LEFT_BRACE, "{", Delimiter),
    (RIGHT_BRACE, "}", Delimiter),
    (AND, "and", Keyword),
    (OR, "or", Keyword),
    (NOT, "not", Keyword),
    (IS, "is", Keyword),
    (IF, "if", Keyword),
    (IN, "in", Keyword),
    (ALIAS, "alias", Keyword),
    (TYPE, "type", Keyword),
    (STRUCT, "struct", Keyword),
    (ENUM, "enum", Keyword),
    (INTERFACE, "interface", Keyword),
    (AUTO, "auto", Type),
    (BOOL, "bool", Type),
    (CHAR, "char", Type),
    (INT, "int", Type),
    (UINT, "uint", Type),
    (FLOAT, "float", Type),
    (VOID, "void", Type),
    (INT8, "int8", Type),
    (INT16, "int16", Type),
    (INT32, "int32", Type),
    (INT64, "int64", Type),
    (INT128, "int128", Type),
    (UINT8, "uint8", Type),
    (UINT16, "uint16", Type),
    (UINT32, "uint32", Type),
    (UINT64, "uint64", Type),
    (UINT128, "uint128", Type),
    (FLOAT32, "float32", Type),
    (FLOAT64, "float64", Type),
    (FLOATLONG, "floatlong", Type),
    (PUBLIC, "public", Keyword),
    (PRIVATE, "private", Keyword),
    (CONST, "const", Keyword),
    (STATIC, "static", Keyword),
    (ASYNC, "async", Keyword),
    (FOR, "for", Keyword),
    (WHILE, "while", Keyword),
    (DO, "do", Keyword),
    (CONTINUE, "continue", Keyword),
    (TRIPLE_DOTS, "...", Keyword),
    (IMPL, "impl", Keyword),
    (TRUE, "true", Keyword),
    (FALSE, "false", Keyword),
    (MUT, "mut", Keyword),
    (USE, "use", Keyword),
    (COMPTIME, "comptime", Comptime),
    (DOLLAR, "$", Comptime),
);

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub raw: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(raw: String) -> Token {
        Token {
            kind: TokenKind::from_str(&raw),
            raw,
        }
    }
}
