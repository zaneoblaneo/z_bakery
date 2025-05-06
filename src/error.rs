
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    InvalidUsage,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl From<std::io::Error> for Error {
    fn from(o: std::io::Error) -> Self {
        Self::IoError(o)
    }
}
