use crate::calc::{ast::Node, token::Token, tokenizer::Tokenizer};
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}