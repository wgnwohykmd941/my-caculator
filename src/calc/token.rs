use std::fmt::{Display, Formatter};
use rust_decimal::Decimal;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token  {
    Add,//加
    Sub,//减
    Mul,//乘
    Div,//除
    Caret,//幂
    Lparen,//左括号
    Rparen,//右括号
    Number(Decimal),//数字
    Eof//结束符
}//词性

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum OperatorPrecedence {
    Default,
    AorS,//加减优先级
    MorD,//乘除优先级
    Power,//幂优先级
    Negative,//负号优先级
}

impl Token {
       pub fn get_precedence(&self) -> OperatorPrecedence {
         use Token::*;
         use OperatorPrecedence::*;

        match self {
            Add | Token::Sub => AorS,
            Mul | Token::Div => MorD,
            Caret => Power,
            _ => Default,
        }
    }//获取字符优先级
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Caret => write!(f, "^"),
            Lparen => write!(f, "("),
            Rparen => write!(f, ")"),
            Number(n) => write!(f, "{}", n),
            Eof => write!(f, "EOF"),
        }
    }
}