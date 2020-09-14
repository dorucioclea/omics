use serde::Serialize;

use common::request::Include;
use common::result::Result;
use shared::domain::user::{UserId, UserRepository};

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetPublicationsResponse {
    publications: Vec<PublicationDto>,
}

pub struct GetPublications<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetPublications<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetPublications {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: Option<String>,
        collection_id: String,
        include: Include,
    ) -> Result<GetPublicationsResponse> {
        let collection = self
            .collection_repo
            .find_by_id(&CollectionId::new(collection_id)?)
            .await?;

        let can_view_unpublished_publications = if let Some(auth_id) = auth_id {
            let user = self.user_repo.find_by_id(&UserId::new(&auth_id)?).await?;
            collection.author_id().value() == auth_id || user.is_content_manager()
        } else {
            false
        };

        let mut publication_dtos = Vec::new();

        for item in collection.items() {
            let publication = self
                .publication_repo
                .find_by_id(item.publication_id())
                .await?;

            if !can_view_unpublished_publications && !publication.is_published() {
                continue;
            }

            let mut publication_dto = PublicationDto::from(&publication);

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

            publication_dtos.push(publication_dto)
        }

        Ok(GetPublicationsResponse {
            publications: publication_dtos,
        })
    }
}
