use serde::Serialize;

use common::event::EventPublisher;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto, ReaderInteractionDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::interaction::InteractionService;
use crate::domain::publication::{PublicationId, PublicationRepository, StatisticsService};
use crate::domain::reader::{ReaderId, ReaderRepository};
use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    pub publication: PublicationDto,
    pub reader: Option<ReaderInteractionDto>,
}

pub struct GetById<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
    user_repo: &'a dyn UserRepository,

    interaction_serv: &'a InteractionService,
    statistics_serv: &'a StatisticsService,
}

impl<'a> GetById<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
        interaction_serv: &'a InteractionService,
        statistics_serv: &'a StatisticsService,
    ) -> Self {
        GetById {
            event_pub,
            author_repo,
            category_repo,
            publication_repo,
            reader_repo,
            user_repo,
            interaction_serv,
            statistics_serv,
        }
    }

    pub async fn exec(
        &self,
        auth_id: Option<String>,
        publication_id: String,
        include: Include,
    ) -> Result<GetByIdResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let is_content_manager = if let Some(auth_id) = &auth_id {
            let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            user.is_content_manager()
        } else {
            false
        };

        let (mut publication_dto, reader_interaction_dto) = if let Some(auth_id) = auth_id {
            let is_reader_author = publication.author_id().value() == auth_id;

            if is_reader_author {
                (
                    PublicationDto::from(&publication)
                        .status(&publication)
                        .pages(&publication),
                    None,
                )
            } else if is_content_manager {
                (
                    PublicationDto::from(&publication).status(&publication),
                    None,
                )
            } else {
                let reader_id = ReaderId::new(auth_id)?;
                let reader = self.reader_repo.find_by_id(&reader_id).await?;

                self.interaction_serv
                    .add_view(&reader, &mut publication)
                    .await?;

                self.publication_repo.save(&mut publication).await?;

                self.event_pub
                    .publish_all(publication.base().events()?)
                    .await?;

                let reader_statistics = self
                    .statistics_serv
                    .get_history(Some(&reader_id), Some(&publication_id), None, None)
                    .await?;

                (
                    PublicationDto::from(&publication),
                    Some(ReaderInteractionDto::new(
                        reader_statistics.views() > 0,
                        reader_statistics.readings() > 0,
                        reader_statistics.likes() > 0,
                        reader_statistics.reviews() > 0,
                    )),
                )
            }
        } else {
            (PublicationDto::from(&publication), None)
        };

        if include.has("author") {
            let user = self.user_repo.find_by_id(publication.author_id()).await?;
            let author = self.author_repo.find_by_id(publication.author_id()).await?;
            publication_dto = publication_dto.author(AuthorDto::from(&user, &author));
        }

        if include.has("category") {
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;
            publication_dto = publication_dto.category(CategoryDto::from(&category));
        }

        Ok(GetByIdResponse {
            publication: publication_dto,
            reader: reader_interaction_dto,
        })
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
            c.user_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        let res = uc
            .exec(
                Some(reader1.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add("author").add("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, author1.base().id().value());
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
            c.user_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default(),
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
            c.user_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        let res = uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add("author").add("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, publication.author_id().value());
        assert!(res.pages.is_none());
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
            c.user_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                Some(reader1.base().id().to_string()),
                "#invalid".to_owned(),
                Include::default(),
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn reader_interaction() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        c.interaction_serv()
            .add_like(&reader2, &mut publication)
            .await
            .unwrap();

        let res = uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default(),
            )
            .await
            .unwrap();
        let res = res.reader.unwrap();
        assert!(res.viewed);
        assert!(res.liked);
        assert!(!res.read);
        assert!(!res.reviewed);
    }
}
