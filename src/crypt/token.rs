use std::fmt::Display;

use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::b64u_encode;

pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token,salt,&config.TOKEN_KEY)?;
    Ok(())
} 

fn _generate_token(
    ident: &str,
    duration_sec: f64,
    salt: &str,
    key: &[u8],
) -> Result<Token> {
    todo!()
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: &str,
    key: &[u8], 
) -> Result<()> {
    todo!()
}

fn _token_sign_into_b64u(
    ident: &str,
    exp: &str,
    salt: &str,
    key: &[u8],
) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNC0wNS0wMVQwMDowMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2024-05-01T00:00:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        println!("->> {fx_token}");
        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }
}