pub type CalcResult<T> = Result<T, CalcError>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("未识别字符: {0}")]
    UnexpectedChar(char),//未识别字符
     #[error("无效操作符: {0}")]
    IndividOperator(String),//无效操作符
}