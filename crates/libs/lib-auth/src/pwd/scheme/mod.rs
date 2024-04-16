
mod error;
mod scheme_01;

pub use self::error::{Error, Result};

use crate::pwd::ContentToHash;

pub const DEFAULT_SCHEME: &str = "01";

pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;
    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()>;
}

#[derive(Debug)]
pub enum SchemeStatus {
    Ok,
    Outdated, 
}

pub fn get_scheme(scheme_name: &str) -> Result<Box<dyn Scheme>> {
    match scheme_name {
        "01" => Ok(Box::new(scheme_01::Scheme01)),
        _ => Err(Error::SchemeNotFound(scheme_name.to_string())),
    }
}