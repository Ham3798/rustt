mod parser {
    use rusttc_lexer::{Token, TokenKind};
    use rusttc_ast::ast::{Node, BinaryOp};
    pub fn parse(tokens: &[Token]) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut iter = tokens.iter().peekable();

        while let Some(token) = iter.next() {
            match &token.kind {
                TokenKind::Number => {
                    if let Ok(value) = token.text.parse() {
                        nodes.push(Node::Number(value));
                    }
                },
                TokenKind::Ident => nodes.push(Node::Ident(token.text.clone())),
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    if let (Some(left), Some(right)) = (nodes.pop(), iter.next()) {
                        nodes.push(Node::BinaryOp(Box::new(BinaryOp {
                            left,
                            operator: token.clone(),
                            right: match right.kind {
                                TokenKind::Number => {
                                    Node::Number(right.text.parse().unwrap_or(0))
                                }
                                TokenKind::Ident => Node::Ident(right.text.clone()),
                                _ => Node::Number(0), // 간단한 예시를 위한 기본값 처리
                            },
                        })));
                    }
                },
                _ => {}
            }
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use rusttc_lexer::{Token, TokenKind};
    use rusttc_ast::ast::{Node, BinaryOp};

    use crate::parser::parse;
    
    #[test]
    fn test_parse_number() {
        let tokens = vec![
            Token::new(TokenKind::Number, "123".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        let result = parse(&tokens);
        assert_eq!(result, vec![Node::Number(123)]);
    }

    #[test]
    fn test_parse_ident() {
        let tokens = vec![
            Token::new(TokenKind::Ident, "x".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        let result = parse(&tokens);
        assert_eq!(result, vec![Node::Ident("x".to_string())]);
    }

    #[test]
    fn test_parse_binary_op() {
        let tokens = vec![
            Token::new(TokenKind::Number, "2".to_string()),
            Token::new(TokenKind::Plus, "+".to_string()),
            Token::new(TokenKind::Number, "3".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        let result = parse(&tokens);
        assert_eq!(
            result,
            vec![
                Node::BinaryOp(Box::new(BinaryOp {
                    left: Node::Number(2),
                    operator: Token::new(TokenKind::Plus, "+".to_string()),
                    right: Node::Number(3),
                }))
            ]
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        let tokens = vec![
            Token::new(TokenKind::Ident, "x".to_string()),
            Token::new(TokenKind::Star, "*".to_string()),
            Token::new(TokenKind::Number, "5".to_string()),
            Token::new(TokenKind::Minus, "-".to_string()),
            Token::new(TokenKind::Number, "3".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];
        // 예상되는 복잡한 표현식에 대한 결과를 구현합니다.
    }
}
