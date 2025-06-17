use crate::calc::{ast::Node, error::{CalcError, CalcResult}, token::{OperatorPrecedence, Token}, tokenizer::Tokenizer};
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}//解析器

impl<'a> Parser<'a> { 
    pub fn new(exp:&'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(exp);
        let current_token = tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(tokenizer.get_unexopected_char().unwrap())
        })?;
        Ok(Self { tokenizer, current_token})
    }

    pub fn parser(&mut self) -> CalcResult<Node> { 
        todo!()
    }
}//公有方法

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(self.tokenizer.get_unexopected_char().unwrap())
        })?;
        Ok(())
    }//获取下一个字符
    fn parser_exp(&mut self, precedence:OperatorPrecedence) -> CalcResult<Node> { 
        todo!()
    }

    fn parser_token(&mut self) -> CalcResult<Node> {
        match self.current_token {
            Token::Number(n) => {
                self.next_token()?;
                Ok(Node::Number(n))
            }
            Token::Sub => { 
                self.next_token()?;
                let expr = self.parser_exp(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Lparen => { 
                self.next_token()?;
                let expr = self.parser_exp(OperatorPrecedence::Default)?;
                if self.current_token != Token::Rparen { 
                    self.next_token()?;
                    if self.current_token == Token::Eof {
                        return Err(CalcError::IndividOperator("缺少右括号".into()));
                    }
                }   return Err(CalcError::IndividOperator(
                    format!("期望右括号，实际遇到{}", self.current_token)
                ));
            }
            _ => {
                return  0;
            }
        }   
    }
}