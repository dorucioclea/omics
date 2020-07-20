use std::rc::Rc;

use crate::common::error::Error;
use crate::common::event::EventPublisher;
use crate::common::model::Entity;
use crate::identity::application::user::{
    ChangePasswordCommand, LoginCommand, RegisterCommand, UpdateCommand,
};
use crate::identity::domain::role::RoleRepository;
use crate::identity::domain::token::Token;
use crate::identity::domain::user::{
    AuthService, User, UserID, UserRegistered, UserRepository, UserUpdated,
};

pub struct UserService<TUserRepository, TEventPublisher, TAuthService, TRoleRepository> {
    user_repository: Rc<TUserRepository>,
    event_publisher: Rc<TEventPublisher>,
    auth_serv: Rc<TAuthService>,
    role_repository: Rc<TRoleRepository>,
}

impl<
        TUserRepository: UserRepository,
        TEventPublisher: EventPublisher,
        TAuthService: AuthService,
        TRoleRepository: RoleRepository,
    > UserService<TUserRepository, TEventPublisher, TAuthService, TRoleRepository>
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        event_publisher: Rc<TEventPublisher>,
        auth_serv: Rc<TAuthService>,
        role_repository: Rc<TRoleRepository>,
    ) -> Self {
        UserService {
            user_repository,
            event_publisher,
            auth_serv,
            role_repository,
        }
    }

    pub fn get_by_id(&self, user_id: UserID) -> Result<User, Error> {
        let user = self.user_repository.find_by_id(user_id)?;
        Ok(user)
    }

    pub fn register(&self, cmd: RegisterCommand) -> Result<(), Error> {
        cmd.validate()?;

        self.auth_serv.available(&cmd.username, &cmd.email)?;
        let hashed_password = self.auth_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repository.next_id()?,
            &cmd.username,
            &cmd.email,
            &hashed_password,
            &self.role_repository.get_by_code("user".to_owned())?,
        )?;

        self.user_repository.save(&mut user)?;

        let event = UserRegistered::new(
            user.id().value(),
            user.username().value(),
            user.email().value(),
        );
        self.event_publisher.publish("user.registered", event)?;

        Ok(())
    }

    pub fn login(&self, cmd: LoginCommand) -> Result<Token, Error> {
        self.auth_serv
            .authenticate(&cmd.username_or_email, &cmd.password)
    }

    pub fn update(&self, user_id: UserID, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repository.find_by_id(user_id)?;

        user.change_name(&cmd.name, &cmd.lastname)?;

        self.user_repository.save(&mut user)?;

        if let Some(person) = user.person() {
            let event = UserUpdated::new(user.id().value(), person.name(), person.lastname());
            self.event_publisher.publish("user.updated", event)?;
        }

        Ok(())
    }

    pub fn change_password(
        &self,
        user_id: UserID,
        cmd: ChangePasswordCommand,
    ) -> Result<(), Error> {
        cmd.validate()?;
        self.auth_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
    }
}
