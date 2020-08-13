use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{AuthenticationService, PasswordHasher, UserRepository};

#[derive(Deserialize)]
pub struct LoginCommand {
    username_or_email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    auth_token: String,
}

pub struct Login<'a, EPub, URepo, PHasher, TRepo, TEnc> {
    event_pub: &'a EPub,

    authentication_serv: AuthenticationService<'a, URepo, PHasher, TRepo, TEnc>,
}

impl<'a, EPub, URepo, PHasher, TRepo, TEnc> Login<'a, EPub, URepo, PHasher, TRepo, TEnc>
where
    EPub: EventPublisher,
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        event_pub: &'a EPub,
        authentication_serv: AuthenticationService<'a, URepo, PHasher, TRepo, TEnc>,
    ) -> Self {
        Login {
            event_pub,
            authentication_serv,
        }
    }

    pub async fn exec(&self, cmd: LoginCommand) -> Result<LoginResponse> {
        match self
            .authentication_serv
            .authenticate(&cmd.username_or_email, &cmd.password)
            .await
        {
            Ok((user, token)) => {
                self.event_pub.publish_all(user.base().events()?).await?;

                Ok(LoginResponse {
                    auth_token: token.value().to_owned(),
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn not_validated_user() {
        let c = mocks::container();
        let uc = Login::new(c.event_pub(), c.authentication_serv());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(LoginCommand {
                username_or_email: user.identity().username().value().to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());
    }

    #[tokio::test]
    async fn validated_user() {
        let c = mocks::container();
        let uc = Login::new(c.event_pub(), c.authentication_serv());

        let mut user = mocks::validated_user1();
        c.user_repo().save(&mut user).await.unwrap();

        let res = uc
            .exec(LoginCommand {
                username_or_email: user.identity().username().value().to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .unwrap();
        assert!(!res.auth_token.is_empty());
        assert_eq!(c.token_repo().cache().len().await, 1);
        assert_eq!(c.event_pub().events().await.len(), 1);

        assert!(uc
            .exec(LoginCommand {
                username_or_email: "non-existing".to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());

        assert!(uc
            .exec(LoginCommand {
                username_or_email: user.identity().username().value().to_owned(),
                password: "invalid".to_owned(),
            })
            .await
            .is_err());
    }
}
