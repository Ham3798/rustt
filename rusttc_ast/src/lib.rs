pub mod ast {
    use rusttc_lexer::Token;

    // AST의 노드를 나타내는 열거형입니다.
    #[derive(Debug, PartialEq)]
    pub enum Node {
        Number(i64),             // 숫자
        Ident(String),           // 식별자
        BinaryOp(Box<BinaryOp>), // 이항 연산자
    }

    // 이항 연산자 노드를 나타내는 구조체입니다.
    #[derive(Debug, PartialEq)]
    pub struct BinaryOp {
        pub left: Node,          // 왼쪽 피연산자
        pub operator: Token,     // 연산자
        pub right: Node,         // 오른쪽 피연산자
    }
}
