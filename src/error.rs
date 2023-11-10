use colored::*;
#[derive(Debug)]
pub enum ScrapError {
    ScannerError,
    ParserError,
    RuntimeError,
    EvaluatorError,
}

impl ScrapError {
    pub fn error(et: ScrapError, msg: &str, line: usize, filename: &str) {
        eprintln!(
            "{} {} {}",
            format!("[{et:?}]").red(),
            filename.to_string().blue().bold(),
            format!("at line {line}: {msg}").red()
        );
        match et {
            Self::ScannerError => (),
            Self::ParserError => std::process::exit(0),
            Self::RuntimeError => std::process::exit(0),
            Self::EvaluatorError => (),
        }
    }
}
