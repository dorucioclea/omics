use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct AddToFavorites<'a> {
    event_pub: &'a dyn EventPublisher,

    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> AddToFavorites<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        AddToFavorites {
            event_pub,
            interaction_repo,
            publication_repo,
            reader_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<CommandResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(auth_id)?;
        let mut reader = self.reader_repo.find_by_id(&reader_id).await?;

        let mut favorite = reader.add_publication_to_favorites(&publication)?;

        self.interaction_repo
            .save_publication_favorite(&mut favorite)
            .await?;

        self.event_pub
            .publish_all(reader.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}