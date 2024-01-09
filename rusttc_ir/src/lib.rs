pub mod ir {
    use rusttc_ast::ast::Node;
    use rusttc_lexer::Token;

    // IR의 노드를 나타내는 열거형입니다.
    #[derive(Debug, PartialEq)]
    pub enum IrNode {
        Constant(i64),             // 상수
        Variable(String),          // 변수
        BinaryExpression(Box<BinaryExpression>), // 이항 표현식
    }

    // 이항 표현식을 나타내는 구조체입니다.
    #[derive(Debug, PartialEq)]
    pub struct BinaryExpression {
        pub left: IrNode,          // 왼쪽 피연산자
        pub operator: Token,       // 연산자
        pub right: IrNode,         // 오른쪽 피연산자
    }

    // AST를 IR로 변환하는 함수입니다.
    pub fn convert_to_ir(ast_nodes: Vec<Node>) -> Vec<IrNode> {
        ast_nodes.into_iter().map(|node| {
            match node {
                Node::Number(n) => IrNode::Constant(n),
                Node::Ident(id) => IrNode::Variable(id),
                Node::BinaryOp(binary_op) => {
                    let binary_op = *binary_op;
                    IrNode::BinaryExpression(Box::new(BinaryExpression {
                        left: convert_to_ir(vec![binary_op.left]).pop().unwrap(),
                        operator: binary_op.operator,
                        right: convert_to_ir(vec![binary_op.right]).pop().unwrap(),
                    }))
                },
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ir::*;
    use rusttc_ast::ast::{Node, BinaryOp};
    use rusttc_lexer::{Token, TokenKind};

    #[test]
    fn test_convert_constant() {
        let ast_nodes = vec![Node::Number(42)];
        let ir_nodes = convert_to_ir(ast_nodes);
        assert_eq!(ir_nodes, vec![IrNode::Constant(42)]);
    }

    #[test]
    fn test_convert_variable() {
        let ast_nodes = vec![Node::Ident("x".to_string())];
        let ir_nodes = convert_to_ir(ast_nodes);
        assert_eq!(ir_nodes, vec![IrNode::Variable("x".to_string())]);
    }

    #[test]
    fn test_convert_binary_expression() {
        let ast_nodes = vec![Node::BinaryOp(Box::new(BinaryOp {
            left: Node::Number(2),
            operator: Token::new(TokenKind::Plus, "+".to_string()),
            right: Node::Number(3),
        }))];
        let ir_nodes = convert_to_ir(ast_nodes);
        assert_eq!(ir_nodes, vec![
            IrNode::BinaryExpression(Box::new(BinaryExpression {
                left: IrNode::Constant(2),
                operator: Token::new(TokenKind::Plus, "+".to_string()),
                right: IrNode::Constant(3),
            }))
        ]);
    }
}
