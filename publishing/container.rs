use std::sync::Arc;

use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::content_manager::ContentManagerRepository;
use crate::domain::interaction::{InteractionRepository, InteractionService};
use crate::domain::publication::{PublicationRepository, StatisticsService};
use crate::domain::reader::ReaderRepository;

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    author_repo: Arc<dyn AuthorRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
    content_manager_repo: Arc<dyn ContentManagerRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    reader_repo: Arc<dyn ReaderRepository>,

    statistics_serv: Arc<StatisticsService>,
    interaction_serv: Arc<InteractionService>,
}

impl<EPub> Container<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,
        author_repo: Arc<dyn AuthorRepository>,
        category_repo: Arc<dyn CategoryRepository>,
        collection_repo: Arc<dyn CollectionRepository>,
        content_manager_repo: Arc<dyn ContentManagerRepository>,
        interaction_repo: Arc<dyn InteractionRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        reader_repo: Arc<dyn ReaderRepository>,
    ) -> Self {
        let statistics_serv = Arc::new(StatisticsService::new(interaction_repo.clone()));
        let interaction_serv = Arc::new(InteractionService::new(interaction_repo.clone()));

        Container {
            event_pub,

            author_repo,
            category_repo,
            collection_repo,
            content_manager_repo,
            interaction_repo,
            publication_repo,
            reader_repo,

            statistics_serv,
            interaction_serv,
        }
    }

    pub async fn subscribe<ES>(&self, _event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber,
    {
        Ok(())
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn author_repo(&self) -> &dyn AuthorRepository {
        self.author_repo.as_ref()
    }

    pub fn category_repo(&self) -> &dyn CategoryRepository {
        self.category_repo.as_ref()
    }

    pub fn collection_repo(&self) -> &dyn CollectionRepository {
        self.collection_repo.as_ref()
    }

    pub fn content_manager_repo(&self) -> &dyn ContentManagerRepository {
        self.content_manager_repo.as_ref()
    }

    pub fn interaction_repo(&self) -> &dyn InteractionRepository {
        self.interaction_repo.as_ref()
    }

    pub fn publication_repo(&self) -> &dyn PublicationRepository {
        self.publication_repo.as_ref()
    }

    pub fn reader_repo(&self) -> &dyn ReaderRepository {
        self.reader_repo.as_ref()
    }

    // Service
    pub fn statistics_serv(&self) -> &StatisticsService {
        &self.statistics_serv
    }

    pub fn interaction_serv(&self) -> &InteractionService {
        &self.interaction_serv
    }
}
