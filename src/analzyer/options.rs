use std::iter::once;
use std::path::{Path, PathBuf};

use regex::Regex;

#[derive(Clone)]
pub struct TextFileOptions {
    pub path: PathBuf,
    pub record_find_regex: Regex,
    pub delimiter: char,
    pub qualifier: Option<char>,
    // encoding:
    pub all_qualified_pattern: String,
    pub start_pattern: String,
    pub end_pattern: String,
    pub file_name: String,
    all_qualified_count: i32,
    is_all_qualified: bool,
}

impl TextFileOptions {
    pub fn new(
        path: &Path,
        record_find_regex: Regex,
        delimiter: char,
        qualifier: Option<char>,
    ) -> Result<Self, String> {
        Ok(Self {
            path: path.to_path_buf(),
            record_find_regex,
            delimiter,
            qualifier,
            all_qualified_pattern: format!(
                "{}{}{}",
                qualifier.unwrap_or_default(),
                delimiter,
                qualifier.unwrap_or_default()
            ),
            start_pattern: format!("{}{}", delimiter, qualifier.unwrap_or_default()),
            end_pattern: format!("{}{}", qualifier.unwrap_or_default(), delimiter),
            file_name: path
                .file_name()
                .ok_or("File name cannot be found in path")?
                .to_str()
                .ok_or("Path OsString cannot be converted to String")?
                .to_string(),
            all_qualified_count: 0,
            is_all_qualified: false,
        })
    }

    pub fn check_non_escaped_qualifiers(
        &mut self,
        record_num: usize,
        record: &String,
    ) -> Vec<String> {
        let qualifier = if let Some(qualifier) = &self.qualifier {
            qualifier
        } else {
            return Vec::new();
        };
        if !qualifier_in_record(qualifier, record) {
            return Vec::new();
        }

        if record_num < 10
            && check_all_qualified(
                &self.delimiter,
                qualifier,
                &self.all_qualified_pattern,
                &record,
            )
        {
            self.all_qualified_count += 1;
            if self.all_qualified_count >= 9 {
                self.is_all_qualified = true;
            }
        }

        let checks = if self.is_all_qualified {
            record[1..record.len() - 2]
                .split(&self.all_qualified_pattern)
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        } else {
            extract_qualified_values(qualifier, &self.delimiter, record)
        };
        let mut result = checks
            .into_iter()
            .filter(|check| {
                let chars: Vec<char> = check.chars().collect();
                let last_char = once(&'\0').chain(chars.iter().take(chars.len() - 1));
                let next_char = chars.iter().skip(1).chain(once(&'\0'));
                chars
                    .iter()
                    .zip(last_char)
                    .zip(next_char)
                    .any(|((cc, lc), nc)| cc == qualifier && (lc != qualifier && nc != qualifier))
            })
            .collect::<Vec<String>>();
        result.sort_by(|a, b| b.len().cmp(&a.len()));
        result
    }

    pub fn check_all_qualified(&self, record: &str) -> bool {
        let qualifier = if let Some(qualifier) = self.qualifier {
            qualifier
        } else {
            return false;
        };
        let check = if record
            .chars()
            .last()
            .map(|c| c != qualifier)
            .unwrap_or_default()
        {
            record
        } else {
            &record[0..record.len() - 1]
        };
        check.matches(self.delimiter).count() == check.matches(&self.all_qualified_pattern).count()
    }
}

pub fn qualifier_in_record(qualifier: &char, record: &str) -> bool {
    record.contains(*qualifier)
}

pub fn check_all_qualified(
    delimiter: &char,
    qualifier: &char,
    all_qualified_pattern: &str,
    record: &str,
) -> bool {
    let check = if record
        .chars()
        .last()
        .map(|c| &c != qualifier)
        .unwrap_or_default()
    {
        record
    } else {
        &record[0..record.len() - 1]
    };
    check.matches(|c| &c == delimiter).count() == check.matches(all_qualified_pattern).count()
}

pub fn extract_qualified_values(
    qualifier: &char,
    delimiter: &char,
    record: &String,
) -> Vec<String> {
    let mut result = Vec::new();
    let mut buffer = String::new();
    let mut temp_buffer = String::new();
    let record_length = record.len();
    let record_chars: Vec<char> = record.chars().collect();
    let next_chars = record_chars.iter().chain(once(delimiter));
    let chars = once(delimiter).chain(record_chars.iter());
    let mut in_qualified_value = false;
    for (i, (c1, c2)) in chars.zip(next_chars).enumerate() {
        if i == record_length {
            break;
        }
        if !in_qualified_value && c1 == qualifier && c2 == delimiter {
            buffer.push_str(&temp_buffer);
            temp_buffer.clear();
            continue;
        }
        if !in_qualified_value && c1 == delimiter && c2 == qualifier {
            in_qualified_value = true;
            if !buffer.is_empty() {
                result.push(buffer);
                buffer = String::new();
            }
            temp_buffer.clear();
            continue;
        }
        if in_qualified_value && c1 == qualifier && c2 == delimiter {
            in_qualified_value = false;
            temp_buffer.clear();
        }
        if in_qualified_value {
            if c1 == qualifier && buffer.is_empty() {
                continue;
            }
            buffer.push(*c1);
        } else {
            temp_buffer.push(*c1);
        }
    }
    if !buffer.is_empty() {
        result.push(buffer);
    }
    result
}
