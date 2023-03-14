pub mod client;
pub mod location;
pub mod weather;

pub type HttpResult<T> = Result<T, reqwest::Error>;
