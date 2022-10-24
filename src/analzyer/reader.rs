use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use super::{options::TextFileOptions, results::AnalyzerResult};

pub struct RecordsReader {
    options: TextFileOptions,
    lines: Lines<BufReader<File>>,
    temp_record: Option<String>,
    is_done: bool,
    pub header_line: String,
    pub header_delimiter_count: usize,
}

impl RecordsReader {
    pub fn new(options: &TextFileOptions) -> AnalyzerResult<Self> {
        let file = File::open(&options.path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let header_line = match lines.next() {
            Some(header_line) => header_line?,
            None => return Err("Could not find a header line for the file".into()),
        };
        let header_delimiter_count = header_line.matches(options.delimiter).count();
        Ok(Self {
            options: options.clone(),
            lines,
            temp_record: None,
            is_done: false,
            header_line,
            header_delimiter_count,
        })
    }
}

impl Iterator for RecordsReader {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        loop {
            let line_result = match self.lines.next() {
                Some(line_result) => line_result,
                None => {
                    return if let Some(record) = self.temp_record.take() {
                        self.is_done = true;
                        Some(Ok(record))
                    } else {
                        None
                    }
                }
            };
            let line = match line_result {
                Ok(line) => line,
                Err(error) => return Some(Err(error)),
            };
            if self.temp_record.is_none() {
                self.temp_record = Some(line);
                continue;
            }
            if self.options.record_find_regex.is_match(&line) {
                if let Some(record) = self.temp_record.take() {
                    self.temp_record = Some(line);
                    return Some(Ok(record));
                }
                self.temp_record = Some(line);
                continue;
            }
            if line.is_empty() {
                if let Some(record) = &mut self.temp_record {
                    record.push('\n');
                    record.push_str(&line);
                }
                continue;
            }
            if let Some(qualifier) = &self.options.qualifier {
                let record = self.temp_record.as_mut().unwrap();
                let last_start = record
                    .rfind(&self.options.start_pattern)
                    .map(|i| i as i32)
                    .unwrap_or(-1);
                let last_end = record
                    .rfind(&self.options.end_pattern)
                    .map(|i| i as i32)
                    .unwrap_or(-1);
                let last_record_char = &record.chars().last().unwrap();
                if last_record_char != qualifier && last_start != -1 && last_start > last_end {
                    record.push('\r');
                    record.push_str(&line);
                    continue;
                }
            }
            if line.matches(self.options.delimiter).count() != self.header_delimiter_count {
                let record = self.temp_record.as_mut().unwrap();
                record.push('\r');
                record.push_str(&line);
                continue;
            }
            let record = self.temp_record.take().unwrap();
            self.temp_record = Some(line);
            return Some(Ok(record));
        }
    }
}
