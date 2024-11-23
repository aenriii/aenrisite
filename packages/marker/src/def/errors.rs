use std::io;


pub enum TranspilationError {
    IoError(io::Error),
    ParserError(String),
}
