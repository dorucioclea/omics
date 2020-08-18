use common::event::EventPublisher;
use common::result::Result;

use crate::domain::content_manager::{ContentManagerId, ContentManagerRepository};
use crate::domain::publication::{PublicationId, PublicationRepository};

// TODO: add comment
pub struct Approve<'a, EPub, CMRepo, PRepo> {
    event_pub: &'a EPub,

    content_manager_repo: &'a CMRepo,
    publication_repo: &'a PRepo,
}

impl<'a, EPub, CMRepo, PRepo> Approve<'a, EPub, CMRepo, PRepo>
where
    EPub: EventPublisher,
    CMRepo: ContentManagerRepository,
    PRepo: PublicationRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        content_manager_repo: &'a CMRepo,
        publication_repo: &'a PRepo,
    ) -> Self {
        Approve {
            event_pub,
            content_manager_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, content_manager_id: String, publication_id: String) -> Result<()> {
        let content_manager_id = ContentManagerId::new(content_manager_id)?;
        let content_manager = self
            .content_manager_repo
            .find_by_id(&content_manager_id)
            .await?;

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        publication.approve(&content_manager)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn approve() {
        let c = mocks::container();
        let uc = Approve::new(
            c.event_pub(),
            c.content_manager_repo(),
            c.publication_repo(),
        );

        let mut cm = mocks::content_manager1();
        c.content_manager_repo().save(&mut cm).await.unwrap();
        let mut publication = mocks::publication1();
        publication.publish().unwrap();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            cm.base().id().value().to_owned(),
            publication.base().id().value().to_owned(),
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(
            publication.status_history().current().status().to_string(),
            "published"
        );

        if let Status::Published { admin_id } = publication.status_history().current().status() {
            assert_eq!(admin_id, &cm.base().id());
        }
    }
}