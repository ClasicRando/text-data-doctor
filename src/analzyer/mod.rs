mod error;
mod options;
mod reader;
mod results;

pub use options::{TextFileOptions, extract_qualified_values};

use self::{
    reader::RecordsReader,
    results::{AnalyzeResult, AnalyzerResult},
};

struct Analyzer {
    options: TextFileOptions,
}

impl Analyzer {
    fn run(&mut self) -> AnalyzerResult<AnalyzeResult> {
        let mut reader = RecordsReader::new(&self.options)?;
        let mut all_qual_count = 0;
        let mut is_all_qual = false;
        let mut too_few_delimiters = false;
        let mut bad_delim_records: Vec<String> = Vec::new();
        let mut non_escaped_records: Vec<String> = Vec::new();
        for (i, record) in reader.enumerate() {
            let record = record?;
            let non_escaped_qualifiers = self.options.check_non_escaped_qualifiers(i, &record);
        }
        Ok(AnalyzeResult::Good)
    }
}
