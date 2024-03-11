use thiserror::Error;

pub type MarketResult<T> = std::result::Result<T, MarketError>;

/// The error enum for MarketData
#[derive(Debug, Error)]
pub enum MarketError {
    #[error("Error parsing the Url: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[cfg(feature = "use-async")]
    #[error("Unable to retrieve data: {0}")]
    RetriveDataError(#[from] reqwest::Error),

    #[cfg(feature = "use-sync")]
    #[error("Unable to retrieve data: {0}")]
    RetriveDataError(#[from] std::io::Error),

    #[error("Unable to deserialize: {0}")]
    UnableToDeserialize(#[from] serde_json::error::Error),

    #[error("Http error: {0}")]
    HttpError(String),

    #[error("Problem with downloaded data: {0}")]
    DownloadedData(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Unable to write to: {0}")]
    ToWriter(String),

    #[error("Unsuported Interval for selected publisher: {0}")]
    UnsuportedInterval(String),
}
