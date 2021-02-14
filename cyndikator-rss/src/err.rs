use quick_xml::de::DeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("malformed rss {0}")]
    Malformed(#[from] DeError),

    #[cfg(feature = "fetch")]
    #[error("unable to fetch http content {0}")]
    FailedHttpFetch(#[from] reqwest::Error),

    #[cfg(feature = "fetch")]
    #[error("invalid scheme '{0}'")]
    InvalidScheme(String),
}
