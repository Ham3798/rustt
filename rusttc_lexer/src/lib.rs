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

    // /// Like the above, but containing invalid unicode codepoints.
    // InvalidIdent,

    // /// "r#ident"
    // RawIdent,

    // /// An unknown prefix, like `foo#`, `foo'`, `foo"`.
    // ///
    // /// Note that only the
    // /// prefix (`foo`) is included in the token, not the separator (which is
    // /// lexed as its own distinct token). In Rust 2021 and later, reserved
    // /// prefixes are reported as errors; in earlier editions, they result in a
    // /// (allowed by default) lint, and are treated as regular identifier
    // /// tokens.
    // UnknownPrefix,

    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    // Literal { kind: LiteralKind, suffix_start: u32 },
    Literal,

    /// "'a"
    Lifetime { starts_with_number: bool },

    // One-firstar tokens:
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

    /// 나중에 삭제 예정
    Error,
    EOF
}

// #[derive(Clone, Debug)]
// pub enum LiteralKind {
//     /// "12_u8", "0o100", "0b120i99", "1f32".
//     Int { base: Base, empty_int: bool },
//     /// "12.34f32", "1e3", but not "1f32".
//     Float { base: Base, empty_exponent: bool },
//     /// "'a'", "'\\'", "'''", "';"
//     firstar { terminated: bool },
//     /// "b'a'", "b'\\'", "b'''", "b';"
//     Byte { terminated: bool },
//     /// ""abc"", ""abc"
//     Str { terminated: bool },
//     /// "b"abc"", "b"abc"
//     ByteStr { terminated: bool },
//     /// `c"abc"`, `c"abc`
//     CStr { terminated: bool },
//     /// "r"abc"", "r#"abc"#", "r####"ab"###"c"####", "r#"a". `None` indicates
//     /// an invalid literal.
//     RawStr { n_hashes: Option<u8> },
//     /// "br"abc"", "br#"abc"#", "br####"ab"###"c"####", "br#"a". `None`
//     /// indicates an invalid literal.
//     RawByteStr { n_hashes: Option<u8> },
//     /// `cr"abc"`, "cr#"abc"#", `cr#"a`. `None` indicates an invalid literal.
//     RawCStr { n_hashes: Option<u8> },
// }

// #[derive(Clone, Debug)]
// pub enum Base {
//     /// Literal starts with "0b".
//     Binary = 2,
//     /// Literal starts with "0o".
//     Octal = 8,
//     /// Literal doesn't contain a prefix.
//     Decimal = 10,
//     /// Literal starts with "0x".
//     Hexadecimal = 16,
// }

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
        let cur = cursor.bump();
        let token = match cur {
            // Handling whitespace
            ' ' | '\t' | '\r' | '\n' => {
                let whitespace = consume_while(&mut cursor, |c| c.is_whitespace());
                Token::new(TokenKind::Whitespace, whitespace)
            },

            ';' => Token::new(TokenKind::Semi, cur.to_string()),
            ',' => Token::new(TokenKind::Comma, cur.to_string()),
            '.' => Token::new(TokenKind::Dot, cur.to_string()),
            '(' => Token::new(TokenKind::OpenParen, cur.to_string()),
            ')' => Token::new(TokenKind::CloseParen, cur.to_string()),
            '{' => Token::new(TokenKind::OpenBrace, cur.to_string()),
            '}' => Token::new(TokenKind::CloseBrace, cur.to_string()),
            '[' => Token::new(TokenKind::OpenBracket, cur.to_string()),
            ']' => Token::new(TokenKind::CloseBracket, cur.to_string()),
            '@' => Token::new(TokenKind::At, cur.to_string()),
            '#' => Token::new(TokenKind::Pound, cur.to_string()),
            '~' => Token::new(TokenKind::Tilde, cur.to_string()),
            '?' => Token::new(TokenKind::Question, cur.to_string()),
            ':' => Token::new(TokenKind::Colon, cur.to_string()),
            '$' => Token::new(TokenKind::Dollar, cur.to_string()),
            '=' => Token::new(TokenKind::Eq, cur.to_string()),
            '!' => Token::new(TokenKind::Bang, cur.to_string()),
            '<' => Token::new(TokenKind::Lt, cur.to_string()),
            '>' => Token::new(TokenKind::Gt, cur.to_string()),
            '-' => Token::new(TokenKind::Minus, cur.to_string()),
            '&' => Token::new(TokenKind::And, cur.to_string()),
            '|' => Token::new(TokenKind::Or, cur.to_string()),
            '+' => Token::new(TokenKind::Plus, cur.to_string()),
            '*' => Token::new(TokenKind::Star, cur.to_string()),
            
            '/' => {
                match cursor.first() {
                    '/' => {
                        cursor.bump(); // Consume the second '/'
                        let doc_style = match cursor.first() {
                            '!' => {
                                cursor.bump(); // Consume '!'
                                Some(DocStyle::Inner)
                            },
                            '/' => {
                                cursor.bump(); // Consume the third '/'
                                Some(DocStyle::Outer)
                            },
                            _ => None,
                        };
            
                        let comment = consume_while(&mut cursor, |c| c != '\n');
                        let comment_text = match doc_style {
                            Some(DocStyle::Inner) => "//!".to_string() + &comment,
                            Some(DocStyle::Outer) => "///".to_string() + &comment,
                            None => "//".to_string() + &comment,
                        };
            
                        Token::new(TokenKind::LineComment { doc_style }, comment_text)
                    },
                    '*' => {
                        cursor.bump(); // Consume '*'
                        let mut depth = 1;
                        let mut comment_text = "/*".to_string();
                        
                        while depth > 0 && !cursor.is_eof() {
                            match (cursor.first(), cursor.second()) {
                                ('*', '/') => {
                                    cursor.bump(); // Consume '*'
                                    cursor.bump(); // Consume '/'
                                    depth -= 1;
                                    comment_text.push_str("*/");
                                },
                                ('/', '*') => {
                                    cursor.bump(); // Consume '/'
                                    cursor.bump(); // Consume '*'
                                    depth += 1;
                                    comment_text.push_str("/*");
                                },
                                _ => {
                                    comment_text.push(cursor.bump());
                                },
                            }
                        }
            
                        let terminated = depth == 0;
                        let doc_style = if comment_text.starts_with("/*!") {
                            Some(DocStyle::Inner)
                        } else if comment_text.starts_with("/**") {
                            Some(DocStyle::Outer)
                        } else {
                            None
                        };
            
                        Token::new(TokenKind::BlockComment { doc_style, terminated }, comment_text)
                    },
                    _ => {
                        Token::new(TokenKind::Slash, "/".to_string())
                    },
                }
            },

            '^' => Token::new(TokenKind::Caret, cur.to_string()),
            '%' => Token::new(TokenKind::Percent, cur.to_string()),
            ('0'..='9') => {
                let num = consume_while(&mut cursor, |a| a.is_digit(10));
                Token::new(TokenKind::Literal , num.to_string())
            },
            'a'..='z' | 'A'..='Z' => {
                let str = consume_while(&mut cursor, |a| a.is_alphabetic());
                Token::new(TokenKind::Ident, str.to_string())
            },
            '\0' => Token::new(TokenKind::EOF, cur.to_string()),
            _ => Token::new(TokenKind::Error, "Err".to_string())
        };
        tokens.push(token);
    }
    tokens
}

fn consume_while<F>(cursor: &mut Cursor, mut condition: F) -> String
where
    F: FnMut(char) -> bool {
    let mut result = String::new();
    while !cursor.is_eof() && condition(cursor.first()) {
        result.push(cursor.bump());
    }
    result
}
