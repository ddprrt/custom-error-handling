use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
enum ReadFromFileError {
    IoError(std::io::Error),
    ParseError(ParseIntError),
}

impl std::fmt::Display for ReadFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadFromFileError::IoError(err) => write!(f, "IO Error :-( {}", err),
            ReadFromFileError::ParseError(err) => write!(f, "NaN NaN NaN NaN Batman: {}", err),
        }
    }
}

impl std::error::Error for ReadFromFileError {}

impl From<ParseIntError> for ReadFromFileError {
    fn from(v: ParseIntError) -> Self {
        ReadFromFileError::ParseError(v)
    }
}

impl From<std::io::Error> for ReadFromFileError {
    fn from(v: std::io::Error) -> Self {
        ReadFromFileError::IoError(v)
    }
}

fn read_from_file(path: &str) -> Result<u32, ReadFromFileError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let num: u32 = buffer.trim().parse()?;
    Ok(num)
}

fn main() {
    match read_from_file("answer.txt") {
        Ok(result) => println!("The answer is {}", result),
        Err(ReadFromFileError::IoError(err)) => eprintln!("Oh no, IO! {:?}", err),
        Err(ReadFromFileError::ParseError(err)) => {
            eprintln!("Holy number parsing, Batman {}", err)
        }
    }
}
