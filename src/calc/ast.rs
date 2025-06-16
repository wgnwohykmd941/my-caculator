use rust_decimal::Decimal;
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),//后可能嵌套各种种类节点
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    
}