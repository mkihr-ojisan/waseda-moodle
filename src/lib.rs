pub mod course;
pub mod error;
pub mod session;

pub use self::error::Error;
pub use self::session::Session;

use self::error::ErrorKind;
use failure::ResultExt;
use html_extractor::{html_extractor, HtmlExtractor};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;
