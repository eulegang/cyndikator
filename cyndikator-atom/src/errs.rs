use quick_xml::de::DeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("malformed rss {0}")]
    Malformed(#[from] DeError),
}
