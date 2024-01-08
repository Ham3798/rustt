mod cursor;

use cursor::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Semicolon,
    Unknown,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
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
