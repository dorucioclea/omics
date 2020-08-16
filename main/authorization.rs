use common::error::Error;
use common::result::Result;
use identity::domain::token::Token;

use crate::container::Container;

pub fn extract_token(authorization: &str) -> Result<Token> {
    if authorization.starts_with("Bearer ") {
        if let Some(token) = authorization.strip_prefix("Bearer ") {
            return Ok(Token::new(token));
        }
    }

    Err(Error::new("authorization", "invalid_header"))
}

pub async fn with_user(authorization_header: &str, c: &Container) -> Result<String> {
    let token = extract_token(authorization_header)?;
    let authorization_serv = c.identity.authorization_serv();
    let user = authorization_serv.authorize(&token).await?;

    Ok(user.base().id().value().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_token() {
        let token = extract_token("Bearer token#123").unwrap();
        assert_eq!(token.value(), "token#123");
    }

    #[test]
    fn invalid_token() {
        assert!(extract_token("token#123").is_err());
    }
}
