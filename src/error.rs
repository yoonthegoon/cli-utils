#[derive(Debug)]
pub enum Error {
    ConfigDoesNotExist,
    Io(std::io::Error),
    OptionNone,
    TomlSer(toml::ser::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::TomlSer(value)
    }
}
