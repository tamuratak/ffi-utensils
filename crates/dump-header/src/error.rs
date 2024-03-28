use clang::SourceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Source error")]
    Source {
        source: SourceError
    },
    #[error("IO error")]
    Io {
        source: std::io::Error
    }
}

#[derive(Error, Debug)]
#[cfg(feature = "dev")]
pub enum ErrorDev {
    #[error("IO error")]
    Io {
        source: std::io::Error
    },
    #[error("Toml deserialze error")]
    TomlDe {
        source: toml::de::Error
    },
    #[error("Toml serialize error")]
    TomlSer {
        source: toml::ser::Error
    },
}
