use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::role::{Role, RoleId};

use crate::domain::user::{
    Email, Identity, Password, Provider, User, UserRepository, UserService, Username,
};

#[derive(Deserialize)]
pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub validation_code: String, // TODO: remove, only for testing
}

pub struct Register<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,

    user_serv: &'a UserService,
}

impl<'a> Register<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        user_repo: &'a dyn UserRepository,
        user_serv: &'a UserService,
    ) -> Self {
        Register {
            event_pub,
            user_repo,
            user_serv,
        }
    }

    pub async fn exec(&self, cmd: RegisterCommand) -> Result<RegisterResponse> {
        self.user_serv.available(&cmd.username, &cmd.email).await?;

        let hashed_password = self.user_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repo.next_id().await?,
            Identity::new(
                Provider::Local,
                Username::new(cmd.username)?,
                Email::new(cmd.email)?,
                Some(Password::new(hashed_password)?),
            )?,
            Role::new(RoleId::new("user")?, "User")?,
        )?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(RegisterResponse {
            id: user.base().id().to_string(),
            validation_code: user.validation().unwrap().code().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::user::UserId;
    use crate::mocks;

    #[tokio::test]
    async fn new_user() {
        let c = mocks::container();
        let uc = Register::new(c.event_pub(), c.user_repo(), c.user_serv());

        let cmd = RegisterCommand {
            username: "new-user".to_owned(),
            email: "new@user.com".to_owned(),
            password: "P@asswd!".to_owned(),
        };

        let res = uc.exec(cmd).await.unwrap();
        let saved_user = c
            .user_repo()
            .find_by_id(&UserId::new(&res.id).unwrap())
            .await
            .unwrap();
        assert_eq!(res.id, saved_user.base().id().value());
        assert_eq!(saved_user.identity().username().value(), "new-user");
        assert_eq!(saved_user.identity().email().value(), "new@user.com");
        assert_ne!(
            saved_user.identity().password().unwrap().value(),
            "P@asswd!"
        );

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn invalid_data() {
        let c = mocks::container();
        let uc = Register::new(c.event_pub(), c.user_repo(), c.user_serv());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(RegisterCommand {
                username: "us".to_owned(),
                email: "new@user.com".to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());

        assert!(uc
            .exec(RegisterCommand {
                username: "new-user".to_owned(),
                email: "invalid-email".to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());

        assert!(uc
            .exec(RegisterCommand {
                username: "new-user".to_owned(),
                email: "new@user.com".to_owned(),
                password: "1234".to_owned(),
            })
            .await
            .is_err());
    }

    #[tokio::test]
    async fn existing_user() {
        let c = mocks::container();
        let uc = Register::new(c.event_pub(), c.user_repo(), c.user_serv());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(RegisterCommand {
                username: user.identity().username().to_string(),
                email: user.identity().email().to_string(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());

        assert!(uc
            .exec(RegisterCommand {
                username: "other".to_owned(),
                email: user.identity().email().to_string(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());

        assert!(uc
            .exec(RegisterCommand {
                username: user.identity().username().to_string(),
                email: "other@other.com".to_owned(),
                password: "P@asswd!".to_owned(),
            })
            .await
            .is_err());
    }
}
