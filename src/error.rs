use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),

    #[error("Could not find BluOS controller")]
    NoBluOSError,

    #[error("Error performing GET request to {}: {:#}", .url, .source)]
    RequestError { source: reqwest::Error, url: String },

    #[error("Error fetching data from {}: {:#}", .url, .source)]
    RequestFetchError { source: reqwest::Error, url: String },

    #[error("Error parsing XML {} from {}: {:#}", .xml, .url, .source)]
    XMLError {
        source: serde_xml_rs::Error,
        xml: String,
        url: String,
    },

    #[error(transparent)]
    CancelError(#[from] std::sync::mpsc::SendError<bool>),

    #[error("Already discovering using zeroconf")]
    AlreadyDiscovering,

    #[error("IDK BRO")]
    Unknown,
}
