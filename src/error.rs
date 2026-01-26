//! Custom error types for the SDIFF application.
//!
//! This module defines all error types used throughout the application using the
//! `thiserror` crate for ergonomic error handling. Each error variant provides
//! context about what went wrong and includes the source error when applicable.

/// Errors that can occur during file parsing.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// The specified file does not exist.
    ///
    /// This error occurs when attempting to parse a file that cannot be found
    /// at the given path.
    #[error("File not found: {path}")]
    FileNotFound {
        /// The path to the file that could not be found
        path: String,
    },

    /// Failed to read the file contents.
    ///
    /// This error occurs when the file exists but cannot be read due to
    /// permissions, I/O errors, or other system-level issues.
    #[error("Failed to read file {path}: {source}")]
    ReadError {
        /// The path to the file that could not be read
        path: String,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// The file contains invalid JSON syntax.
    ///
    /// This error occurs when the file appears to be JSON but has syntax errors
    /// that prevent parsing.
    #[error("Invalid JSON in {path}: {source}")]
    JsonError {
        /// The path to the file with invalid JSON
        path: String,
        /// The underlying JSON parsing error
        #[source]
        source: serde_json::Error,
    },

    /// The file contains invalid YAML syntax.
    ///
    /// This error occurs when the file appears to be YAML but has syntax errors
    /// that prevent parsing.
    #[error("Invalid YAML in {path}: {source}")]
    YamlError {
        /// The path to the file with invalid YAML
        path: String,
        /// The underlying YAML parsing error
        #[source]
        source: serde_yaml::Error,
    },

    /// The file contains invalid TOML syntax.
    ///
    /// This error occurs when the file appears to be TOML but has syntax errors
    /// that prevent parsing.
    #[error("Invalid TOML in {path}: {source}")]
    TomlError {
        /// The path to the file with invalid TOML
        path: String,
        /// The underlying TOML parsing error
        #[source]
        source: toml::de::Error,
    },

    /// Could not determine the file format.
    ///
    /// This error occurs when the file extension is unknown and attempts to parse
    /// as JSON and YAML both fail.
    #[error("Could not detect file format for {path}")]
    UnknownFormat {
        /// The path to the file with unknown format
        path: String,
    },
}

/// Errors that can occur during output formatting.
#[derive(Debug, thiserror::Error)]
pub enum OutputError {
    /// The requested output format is not recognized.
    ///
    /// This error occurs when an invalid format string is provided.
    #[error("Unknown output format: {format}")]
    UnknownFormat {
        /// The format string that was not recognized
        format: String,
    },

    /// Failed to serialize the diff to JSON.
    ///
    /// This error occurs when the diff structure cannot be converted to JSON,
    /// which should be rare since we control the data structures.
    #[error("Failed to serialize to JSON: {source}")]
    JsonSerializationError {
        /// The underlying JSON serialization error
        #[source]
        source: serde_json::Error,
    },
}

/// Top-level error type for the SDIFF application.
///
/// This enum combines all possible errors that can occur during the diff process.
/// It uses `#[from]` attributes to enable automatic conversion from specific error
/// types using the `?` operator.
#[derive(Debug, thiserror::Error)]
pub enum SdiffError {
    /// A parsing error occurred.
    #[error(transparent)]
    Parse(#[from] ParseError),

    /// An output formatting error occurred.
    #[error(transparent)]
    Output(#[from] OutputError),

    /// An invalid configuration was provided.
    ///
    /// This error occurs when configuration parameters are invalid or conflicting.
    #[error("Invalid configuration: {message}")]
    ConfigError {
        /// Description of what is wrong with the configuration
        message: String,
    },
}

impl ParseError {
    /// Creates a `FileNotFound` error from a path.
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound { path: path.into() }
    }

    /// Creates a `ReadError` from a path and I/O error.
    pub fn read_error(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::ReadError {
            path: path.into(),
            source,
        }
    }

    /// Creates a `JsonError` from a path and JSON parsing error.
    pub fn json_error(path: impl Into<String>, source: serde_json::Error) -> Self {
        Self::JsonError {
            path: path.into(),
            source,
        }
    }

    /// Creates a `YamlError` from a path and YAML parsing error.
    pub fn yaml_error(path: impl Into<String>, source: serde_yaml::Error) -> Self {
        Self::YamlError {
            path: path.into(),
            source,
        }
    }

    /// Creates a `TomlError` from a path and TOML parsing error.
    pub fn toml_error(path: impl Into<String>, source: toml::de::Error) -> Self {
        Self::TomlError {
            path: path.into(),
            source,
        }
    }

    /// Creates an `UnknownFormat` error from a path.
    pub fn unknown_format(path: impl Into<String>) -> Self {
        Self::UnknownFormat { path: path.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::file_not_found("test.json");
        assert_eq!(err.to_string(), "File not found: test.json");
    }

    #[test]
    fn test_unknown_format_error() {
        let err = ParseError::unknown_format("/path/to/file.txt");
        assert!(err.to_string().contains("Could not detect file format"));
        assert!(err.to_string().contains("/path/to/file.txt"));
    }

    #[test]
    fn test_output_error_display() {
        let err = OutputError::UnknownFormat {
            format: "xml".to_string(),
        };
        assert_eq!(err.to_string(), "Unknown output format: xml");
    }

    #[test]
    fn test_sdiff_error_from_parse_error() {
        let parse_err = ParseError::file_not_found("test.json");
        let sdiff_err: SdiffError = parse_err.into();
        assert!(matches!(sdiff_err, SdiffError::Parse(_)));
    }

    #[test]
    fn test_config_error() {
        let err = SdiffError::ConfigError {
            message: "Invalid option".to_string(),
        };
        assert!(err.to_string().contains("Invalid configuration"));
    }
}
