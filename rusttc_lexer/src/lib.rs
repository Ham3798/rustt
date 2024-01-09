mod cursor;

use cursor::Cursor;

#[derive(Clone, Debug)]
pub enum TokenKind {
    // Multi-char tokens:
    /// "// comment"
    LineComment { doc_style: Option<DocStyle> },

    /// `/* block comment */`
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment { doc_style: Option<DocStyle>, terminated: bool },

    /// Any whitespace character sequence.
    Whitespace,

    /// "ident" or "continue"
    ///
    /// At this step, keywords are also considered identifiers.
    Ident,

    /// Like the above, but containing invalid unicode codepoints.
    InvalidIdent,

    /// "r#ident"
    RawIdent,

    /// An unknown prefix, like `foo#`, `foo'`, `foo"`.
    ///
    /// Note that only the
    /// prefix (`foo`) is included in the token, not the separator (which is
    /// lexed as its own distinct token). In Rust 2021 and later, reserved
    /// prefixes are reported as errors; in earlier editions, they result in a
    /// (allowed by default) lint, and are treated as regular identifier
    /// tokens.
    UnknownPrefix,

    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    Literal { kind: LiteralKind, suffix_start: u32 },

    /// "'a"
    Lifetime { starts_with_number: bool },

    // One-char tokens:
    /// ";"
    Semi,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "@"
    At,
    /// "#"
    Pound,
    /// "~"
    Tilde,
    /// "?"
    Question,
    /// ":"
    Colon,
    /// "$"
    Dollar,
    /// "="
    Eq,
    /// "!"
    Bang,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "-"
    Minus,
    /// "&"
    And,
    /// "|"
    Or,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,
}

#[derive(Clone, Debug)]
pub enum LiteralKind {
    /// "12_u8", "0o100", "0b120i99", "1f32".
    Int { base: Base, empty_int: bool },
    /// "12.34f32", "1e3", but not "1f32".
    Float { base: Base, empty_exponent: bool },
    /// "'a'", "'\\'", "'''", "';"
    Char { terminated: bool },
    /// "b'a'", "b'\\'", "b'''", "b';"
    Byte { terminated: bool },
    /// ""abc"", ""abc"
    Str { terminated: bool },
    // /// "b"abc"", "b"abc"
    // ByteStr { terminated: bool },
    // /// `c"abc"`, `c"abc`
    // CStr { terminated: bool },
    // /// "r"abc"", "r#"abc"#", "r####"ab"###"c"####", "r#"a". `None` indicates
    // /// an invalid literal.
    // RawStr { n_hashes: Option<u8> },
    // /// "br"abc"", "br#"abc"#", "br####"ab"###"c"####", "br#"a". `None`
    // /// indicates an invalid literal.
    // RawByteStr { n_hashes: Option<u8> },
    // /// `cr"abc"`, "cr#"abc"#", `cr#"a`. `None` indicates an invalid literal.
    // RawCStr { n_hashes: Option<u8> },
}

#[derive(Clone, Debug)]
pub enum Base {
    /// Literal starts with "0b".
    Binary = 2,
    /// Literal starts with "0o".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}

#[derive(Clone, Debug)]
pub enum DocStyle {
    Outer,
    Inner,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: String) -> Self {
        Token { kind, text }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut cursor = Cursor::new(input);

    while !cursor.is_eof() {
        let ch = cursor.peek();
        let token = match ch {
            '+' => Token::new(TokenKind::Plus, ch.to_string()),
            '-' => Token::new(TokenKind::Minus, ch.to_string()),
            '*' => Token::new(TokenKind::Star, ch.to_string()),
            '/' => Token::new(TokenKind::Slash, ch.to_string()),
            '=' => Token::new(TokenKind::Equals, ch.to_string()),
            ';' => Token::new(TokenKind::Semicolon, ch.to_string()),
            '0'..='9' => {
                let num = consume_while(&mut cursor, |a| a.is_digit(10));
                Token::new(TokenKind::Number, num.to_string())
            }
            'a'..='z' | 'A'..='Z' => {
                let str = consume_while(&mut cursor, |a| a.is_alphabetic());
                Token::new(TokenKind::Ident, str.to_string())
            }
            _ => Token::new(TokenKind::Unknown, ch.to_string()),
        };
        tokens.push(token);
        cursor.bump();
    }

    tokens.push(Token::new(TokenKind::Eof, "".into()));
    tokens
}

fn consume_while<F>(cursor: &mut Cursor, mut condition: F) -> String
where
    F: FnMut(char) -> bool {
    let mut result = String::new();
    while !cursor.is_eof() && condition(cursor.peek()) {
        result.push(cursor.bump());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_string() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![Token::new(TokenKind::Eof, "".to_string())]);
    }

    #[test]
    fn test_tokenize_single_char_operators() {
        let input = "+-*/=;";
        let tokens = tokenize(input);
        let expected = vec![
            Token::new(TokenKind::Plus, "+".to_string()),
            Token::new(TokenKind::Minus, "-".to_string()),
            Token::new(TokenKind::Star, "*".to_string()),
            Token::new(TokenKind::Slash, "/".to_string()),
            Token::new(TokenKind::Equals, "=".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn test_tokenize_numbers() {
        let input = "123 456";
        let tokens = tokenize(input);
        let expected = vec![
            Token::new(TokenKind::Number, "123".to_string()),
            Token::new(TokenKind::Number, "456".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_identifiers() {
        let input = "foo bar";
        let tokens = tokenize(input);
        let expected = vec![
            Token::new(TokenKind::Ident, "foo".to_string()),
            Token::new(TokenKind::Ident, "bar".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_unknown_chars() {
        let input = "@";
        let tokens = tokenize(input);
        let expected = vec![
            Token::new(TokenKind::Unknown, "@".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        assert_eq!(tokens, expected);
    }
}
