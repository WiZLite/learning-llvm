pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterOrEqual,
    Equals,
    NotEqual,
}

// 不慣れで後で爆発しそうなので、
// とりあえず今はボックス化しまくる。
pub enum Expression {
    BinaryExpression(BinaryOp, Box<Expression>, Box<Expression>),
    I32Literal(i32),
    F64Literal(f64),
    Identifier(String),
    Assignment(String, Box<Expression>),
    BlockExpression(Vec<Box<Expression>>),
    WhileExpression(Box<Expression>, Box<Expression>),
    IfExpression(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    SymbolExpression(String),
    PrintlnExpression(Box<Expression>),
    FunctionCall(String, Vec<Box<Expression>>),
}

pub enum TopLevel {
    GlobalVarDef(Box<Expression>),
    FunctionCall(String, Vec<Box<Expression>>),
    FunctionDef(String, Vec<String>, Box<Expression>),
}

pub struct Program {
    top_levels: Vec<TopLevel>,
}

#[test]
pub fn expressionTest() {
    let main_func = TopLevel::FunctionDef(
        "main".to_string(),
        Vec::new(),
        Box::new(Expression::BlockExpression(vec![Box::new(
            Expression::FunctionCall("fact".to_string(), vec![]),
        )])),
    );

    let fact_func = TopLevel::FunctionDef(
        "fact".to_string(),
        vec!["n".to_string()],
        Box::new(Expression::BlockExpression(vec![Box::new(
            Expression::IfExpression(
                Box::new(Expression::BinaryExpression(
                    BinaryOp::LessThan,
                    Box::new(Expression::Identifier("n".to_string())),
                    Box::new(Expression::I32Literal(2)),
                )),
                Box::new(Expression::I32Literal(1)),
                Some(Box::new(Expression::BinaryExpression(
                    BinaryOp::Mul,
                    Box::new(Expression::Identifier("n".to_string())),
                    Box::new(Expression::FunctionCall(
                        "fact".to_string(),
                        vec![Box::new(Expression::BinaryExpression(
                            BinaryOp::Sub,
                            Box::new(Expression::Identifier("n".to_string())),
                            Box::new(Expression::I32Literal(1)),
                        ))],
                    )),
                ))),
            ),
        )])),
    );

    let program = Program {
        top_levels: vec![main_func, fact_func],
    };
    // todo
    // assert(コンパイルして、標準出力に120が帰ってくる)
}
