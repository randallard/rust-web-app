use crate::config;
use crate::crypt::{Error, Result};

pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
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