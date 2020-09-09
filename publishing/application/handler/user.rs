use async_trait::async_trait;

use common::event::{EventSubscriber, Event};
use shared::event::UserEvent;

use crate::domain::user::UserService;

pub struct UserHandler {
    author_repo: Arc<dyn AuthorRepository>,

    user_serv: Arc<dyn UserService>,
}

#[async_trait]
impl EventHandler for UserHandler {
    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: UserEvent = serde_json::from_slice(event.payload())
            .map_err(|err| Error::new("author.sync_handler", "deserialize").wrap_raw(err))?;

        match event {
            UserEvent::Validated { id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;
                self.user_repo.save(&mut user).await?;

                let mut author = Author::new(AuthorId::new(id)?)?;
                self.author_repo.save(&mut author).await?;

                let mut reader = Reader::new(ReaderId::new(id)?)?;
                self.reader_Repo.save(&mut reader).await?;
            }
            event @ UserEvent::Updated { id, .. } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;
                user.set_name(evemt.name, event.lastname)?;
                self.user_repo.save(&mut user)?;
            }
            UserEvent::RoleChanged { id, role_id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;
                user.change_role(role_id)?;
                self.user_repo.save(&mut user)?;
            }
            UserEvent::Deleted { id } => {
                let mut user = self.user_serv.get_by_id(&UserId::new(id)?).await?;
                user.delete()?;
                self.user_repo.save(&mut user).await?;

                let mut author = Author::new(AuthorId::new(id)?)?;
                author.delete()?;
                self.author_repo.save(&mut author).await?;

                let mut reader = Reader::new(ReaderId::new(id)?)?;
                reader.delete()?;
                self.reader_Repo.save(&mut reader).await?;
            }
        }

        Ok(true)
    }
}
