use std::{char, iter::Peekable, str::Chars};
use crate::calc::token::Token;

pub struct  Tokenizer<'a> {
    expression:Peekable <Chars<'a>>,
    reached_eof:bool,
    unexpected_char:Option<char>
}
//实现分词迭代器
//将输入字符串迭代为Token

impl <'a> Tokenizer<'a> {
    pub fn new(exp: &'a str) -> Self {
        Self {
            expression: exp.chars().peekable(),
            reached_eof: false,
            unexpected_char: None,
        }
    }

    pub fn get_unexopected_char(&self) -> Option<char> {
        self.unexpected_char
    }
}

impl <'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_eof {
            return None;
        }//表达式空出口
        let next_char = self.expression.next();
        match next_char {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);
                while let Some(next) = self.expression
                .next_if(|c| c.is_numeric()) {
                    number.push(next);
                }
                Some(Token::Number(number.parse().unwrap()))
            }//数字
            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expression
                .next_if(|c| c.is_whitespace()) {}
                self.next()
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::Lparen),
            Some(')') => Some(Token::Rparen),
            None => {
                self.reached_eof = true;
                Some(Token::Eof)//表达式结束
            }
            Some(chr) => {
                self.unexpected_char = Some(chr);
                Some(Token::Eof)//未识别字符
            }
        }        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let exp = "1    +2*3";
        let mut tokenizer = Tokenizer::new(exp);
        assert_eq!(tokenizer.next(), Some(Token::Number(1.into())));
        assert_eq!(tokenizer.next(), Some(Token::Add));
    }
    #[test]
    fn test_collect() {
        let exp = "1 +2*3";
        let tokens: Vec<_> = Tokenizer::new(exp).collect();
        assert_eq!(tokens, vec![
            Token::Number(1.into()),
            Token::Add,
            Token::Number(2.into()),
            Token::Mul,
            Token::Number(3.into()),
            Token::Eof,
        ]);
    }
}
