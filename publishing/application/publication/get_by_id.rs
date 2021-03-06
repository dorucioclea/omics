use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::interaction::InteractionService;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct GetById<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,

    interaction_serv: &'a InteractionService,
}

impl<'a> GetById<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        interaction_serv: &'a InteractionService,
    ) -> Self {
        GetById {
            event_pub,
            author_repo,
            category_repo,
            publication_repo,
            reader_repo,
            interaction_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<PublicationDto> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        let author = self.author_repo.find_by_id(publication.author_id()).await?;
        let category = self
            .category_repo
            .find_by_id(publication.header().category_id())
            .await?;

        let is_reader_author = publication.author_id() == &reader_id;

        if !is_reader_author {
            self.interaction_serv
                .add_view(&reader, &mut publication)
                .await?;

            self.publication_repo.save(&mut publication).await?;

            self.event_pub
                .publish_all(publication.base().events()?)
                .await?;
        }

        let mut publication_dto = PublicationDto::from(&publication)
            .author(AuthorDto::from(&author))
            .category(CategoryDto::from(&category))
            .pages(&publication);

        if is_reader_author {
            publication_dto = publication_dto.status(&publication);
        }

        Ok(publication_dto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn owner_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::author_as_reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                reader.base().id().to_string(),
                publication.base().id().to_string(),
            )
            .await
            .unwrap();
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, author.base().id().value());
        assert_eq!(res.name, publication.header().name().value());
        assert_eq!(
            res.category.unwrap().id,
            publication.header().category_id().value()
        );
        assert!(res.pages.unwrap().len() > 0);
        assert_eq!(res.statistics.views, 0);
        assert_eq!(res.statistics.unique_views, 0);
        assert_eq!(res.statistics.readings, 0);
        assert_eq!(res.status.unwrap(), "draft");

        assert_eq!(c.event_pub().events().await.len(), 0);
    }

    #[tokio::test]
    async fn reader_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                reader.base().id().to_string(),
                publication.base().id().to_string(),
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn reader_view_of_published() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                reader.base().id().to_string(),
                publication.base().id().to_string(),
            )
            .await
            .unwrap();
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, publication.author_id().value());
        assert_eq!(res.pages.unwrap().len(), 2);
        assert_eq!(res.statistics.views, 1);
        assert_eq!(res.statistics.unique_views, 1);
        assert!(res.status.is_none());

        assert!(c.event_pub().events().await.len() > 0);
    }

    #[tokio::test]
    async fn invalid_id() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(reader.base().id().to_string(), "#invalid".to_owned(),)
            .await
            .is_err());
    }
}
