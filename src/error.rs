#[derive(Debug)]
pub enum Error {
    ConfigDoesNotExist,
    Io(std::io::Error),
    OptionNone,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
