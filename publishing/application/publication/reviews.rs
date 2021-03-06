use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{ReaderDto, ReviewDto};
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderRepository;

#[derive(Serialize)]
pub struct ReviewsResponse {
    pub reviews: Vec<ReviewDto>,
}

pub struct Reviews<'a> {
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> Reviews<'a> {
    pub fn new(
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        Reviews {
            interaction_repo,
            reader_repo,
        }
    }

    pub async fn exec(&self, publication_id: String) -> Result<ReviewsResponse> {
        let reviews = self
            .interaction_repo
            .find_reviews(None, Some(&PublicationId::new(publication_id)?), None, None)
            .await?;

        let mut review_dtos = Vec::new();
        for review in reviews.iter() {
            let reader = self
                .reader_repo
                .find_by_id(review.base().reader_id())
                .await?;
            review_dtos.push(ReviewDto::from(review).reader(ReaderDto::from(&reader)));
        }

        Ok(ReviewsResponse {
            reviews: review_dtos,
        })
    }
}
