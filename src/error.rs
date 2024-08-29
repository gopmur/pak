pub mod pak {
    use std::io;
    use std::path::Path;
    use std::{fmt, result};
    pub enum Error {
        IsADirectory(String),
        NotADirectory(String),
        NotFound(String),
        PermissionDenied(String),
        DirectoryNotEmpty(String),
        Other,
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::IsADirectory(path) => write!(f, "\"{path}\" is a directory"),
                Error::NotADirectory(path) => write!(f, "\"{path}\" is not a directory"),
                Error::NotFound(path) => write!(f, "\"{path}\" not found"),
                Error::PermissionDenied(path) => {
                    write!(f, "\"{path}\" permission denied while processing")
                }
                Error::DirectoryNotEmpty(path) => write!(f, "\"{path}\" is not empty"),
                Error::Other => write!(f, "an unexpected error occurred"),
            }
        }
    }

    impl Error {
        pub fn from_io_error(io_error: io::Error, path: &Path) -> Self {
            match io_error.kind() {
                io::ErrorKind::IsADirectory => {
                    Error::IsADirectory(path.to_string_lossy().to_string())
                }
                io::ErrorKind::NotADirectory => {
                    Error::NotADirectory(path.to_string_lossy().to_string())
                }
                io::ErrorKind::NotFound => Error::NotFound(path.to_string_lossy().to_string()),
                io::ErrorKind::PermissionDenied => {
                    Error::PermissionDenied(path.to_string_lossy().to_string())
                }
                io::ErrorKind::DirectoryNotEmpty => {
                    Error::DirectoryNotEmpty(path.to_string_lossy().to_string())
                }
                _ => Error::Other,
            }
        }
    }
    pub type Result<T> = result::Result<T, Error>;
}