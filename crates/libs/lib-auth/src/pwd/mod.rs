mod error;
mod scheme;

use std::str::FromStr;

pub use self::error::{Error, Result};
pub use scheme::SchemeStatus;
use self::scheme::{get_scheme, Scheme, DEFAULT_SCHEME};

use lazy_regex::regex_captures;
use uuid::Uuid;

pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    hash_for_scheme(DEFAULT_SCHEME, to_hash)
}

pub fn validate_pwd(to_hash: &ContentToHash, pwd_ref: &str) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse()?;

    validate_for_scheme(&scheme_name, to_hash, &hashed)?;

    if scheme_name == DEFAULT_SCHEME {
        Ok(SchemeStatus::Ok)
    } else {
        Ok(SchemeStatus::Outdated)
    }
}

fn hash_for_scheme(scheme_name: &str, to_hash: &ContentToHash) -> Result<String> {
    let pwd_hashed = get_scheme(scheme_name)?.hash(to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(
    scheme_name: &str,
    to_hash: &ContentToHash,
    pwd_ref: &str,
) -> Result<()> {
    get_scheme(scheme_name)?.validate(to_hash, pwd_ref)?;
    Ok(())
}

struct PwdParts {
    scheme_name: String,
    hashed: String,
}

impl FromStr for PwdParts {
    type Err = Error;

    fn from_str(pwd_with_scheme: &str) -> Result<Self> {
        regex_captures!(
            r#"^#(\w+)#(.*)"#,
            pwd_with_scheme
        )
        .map(|(_, scheme, hashed)| Self {
            scheme_name: scheme.to_string(),
            hashed: hashed.to_string(),
        })
        .ok_or(Error::PwdWithSchemeFailedParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_multi_scheme_ok() -> Result<()> {
        let fx_salt = Uuid::parse_str("d9029332-6a60-48ae-8c88-602e30b4e4c8")?;
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: fx_salt,
        };

        let pwd_hashed = hash_for_scheme("01", &fx_to_hash)?;
        let pwd_validate = validate_pwd(&fx_to_hash,&pwd_hashed)?;

        assert!(
            matches!(pwd_validate, SchemeStatus::Outdated),
            "status should be SchemeStatus::Outdated"
        );

        Ok(())
    }
}