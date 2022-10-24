use super::error::AnalyzerError;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

pub enum AnalyzeResult {
    FileNotFound,
    BadRegex,
    BadEncoding,
    NonEscapedQualifiers,
    DelimiterCount {
        too_many: bool,
        qualifier: bool,
    },
    Both,
    NewLineNoQualifier,
    Good,
}

pub enum PeekResult {
    FileNotFound,
    Failed,
    BadEncoding,
    Success,
}
