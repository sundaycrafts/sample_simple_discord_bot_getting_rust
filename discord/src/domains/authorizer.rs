use http::HeaderMap;
use std::error::Error;

pub trait Authorizer {
    fn authorize(&self, header: &HeaderMap, raw_bod: &str) -> Result<(), Box<dyn Error>>;
}
