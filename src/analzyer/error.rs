pub enum AnalyzerError {
    Generic(String),
    IO(std::io::Error),
}

impl From<std::io::Error> for AnalyzerError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<&str> for AnalyzerError {
    fn from(error: &str) -> Self {
        Self::Generic(error.to_owned())
    }
}
