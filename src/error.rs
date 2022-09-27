#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HttpError: {}", _0)]
    HttpError(#[from] reqwest::Error),

    #[error("IoError: {}", _0)]
    IoError(#[from] std::io::Error),

    #[error("ZipError: {}", _0)]
    ZipError(#[from] zip::result::ZipError),

    #[error("SerdeJsonError: {}", _0)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("FTDError: {}", _0)]
    FTDError(#[from] ftd::p1::Error),

    #[error("IgnoreError: {}", _0)]
    IgnoreError(#[from] ignore::Error),

    #[error("FromPathBufError: {}", _0)]
    FromPathBufError(#[from] camino::FromPathBufError),

    #[error("StripPrefixError: {}", _0)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("SitemapParseError: {}", _0)]
    SitemapParseError(#[from] fpm::sitemap::ParseError),

    #[error("URLParseError: {}", _0)]
    UrlParseError(#[from] url::ParseError),

    #[error("UTF8Error: {}", _0)]
    UTF8Error(#[from] std::string::FromUtf8Error),

    #[error("ParseIntError: {}", _0)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("APIResponseError: {}", _0)]
    APIResponseError(String),

    #[error("PackageError: {message}")]
    PackageError { message: String },

    #[error("UsageError: {message}")]
    UsageError { message: String },

    #[error("GenericError: {}", _0)]
    GenericError(String),

    #[error("GroupNotFound: id: {id}, {message}")]
    GroupNotFound { id: String, message: String },

    #[error("CRAboutNotFound CR#{cr_number}: {message}")]
    CRAboutNotFound { message: String, cr_number: usize },
}
