use std::{
    fs::File,
    io::{self, Read},
};

use backtrace::Backtrace;
use thiserror::Error;

use miette::{Diagnostic, NamedSource, Report, SourceOffset, SourceSpan};

#[derive(Debug, Error, Diagnostic)]
#[error("Test failure")]
#[diagnostic()]
/// A miette report for test case source code snippets
pub struct TestSourceSpan {
    filename: String,
    line: usize,
    col: usize,
    #[source_code]
    src: NamedSource,
    #[label("test failed here")]
    span: SourceSpan,
    #[related]
    related: Vec<Report>,
}

impl TestSourceSpan {
    /// Fetch miette source code and source span from given filename and line
    pub fn from_location(
        filename: String,
        line: usize,
        col: usize,
    ) -> std::io::Result<TestSourceSpan> {
        let mut file = File::open(&filename)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        let offset = SourceOffset::from_location(&source, line, col).offset();
        let span = (offset..offset + 1).into();
        Ok(TestSourceSpan {
            src: NamedSource::new(&filename, source),
            span,
            filename,
            line,
            col,
            related: Vec::new(),
        })
    }

    /// Add an error to the list of related errors
    pub fn add_related<E: Into<Report>>(&mut self, err: E) {
        self.related.push(err.into());
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }

    /// Tries to find source information from backtrace.
    pub fn from_backtrace() -> io::Result<Option<Self>> {
        // A substring of test source file paths
        const TEST_FILE_SUBSTRING: &str = "/cloudtruth-cli/tests/";
        // A substring of test harness source file paths
        const HARNESS_FILE_SUBSTRING: &str = "/cloudtruth-cli/tests/harness";
        for frame in Backtrace::new().frames().iter() {
            for symbol in frame.symbols().iter() {
                if let Some(filename) = symbol.filename().and_then(|f| f.to_str()) {
                    if filename.contains(TEST_FILE_SUBSTRING)
                        && !filename.contains(HARNESS_FILE_SUBSTRING)
                    {
                        if let (Some(line), Some(col)) = (symbol.lineno(), symbol.colno()) {
                            return Ok(Some(Self::from_location(
                                filename.into(),
                                line as usize,
                                col as usize,
                            )?));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}