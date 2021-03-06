use serde::Serialize;

use crate::domain::author::Author;
use crate::domain::category::Category;
use crate::domain::collection::Collection;
use crate::domain::interaction::Review;
use crate::domain::publication::{Image, Page, Publication, Statistics};
use crate::domain::reader::Reader;

#[derive(Serialize)]
pub struct StatisticsDto {
    pub views: u32,
    pub unique_views: u32,
    pub readings: u32,
    pub likes: u32,
    pub reviews: u32,
    pub stars: f32,
}

impl From<&Statistics> for StatisticsDto {
    fn from(statistics: &Statistics) -> Self {
        StatisticsDto {
            views: statistics.views(),
            unique_views: statistics.unique_views(),
            readings: statistics.readings(),
            likes: statistics.likes(),
            reviews: statistics.reviews(),
            stars: statistics.stars(),
        }
    }
}

#[derive(Serialize)]
pub struct AuthorDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub publications: Option<Vec<PublicationDto>>,
    pub publication_count: Option<usize>,
    pub collection_count: Option<usize>,
}

impl From<&Author> for AuthorDto {
    fn from(author: &Author) -> Self {
        AuthorDto {
            id: author.base().id().to_string(),
            username: author.username().to_string(),
            name: author.name().to_string(),
            lastname: author.lastname().to_string(),
            publications: None,
            publication_count: None,
            collection_count: None,
        }
    }
}

impl AuthorDto {
    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }

    pub fn publication_count(mut self, count: usize) -> Self {
        self.publication_count = Some(count);
        self
    }

    pub fn collection_count(mut self, count: usize) -> Self {
        self.collection_count = Some(count);
        self
    }
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub publications: Option<Vec<PublicationDto>>,
}

impl From<&Category> for CategoryDto {
    fn from(category: &Category) -> Self {
        CategoryDto {
            id: category.base().id().to_string(),
            name: category.name().to_string(),
            publications: None,
        }
    }
}

impl CategoryDto {
    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }
}

#[derive(Serialize)]
pub struct ImageDto {
    pub url: String,
}

impl From<&Image> for ImageDto {
    fn from(image: &Image) -> Self {
        ImageDto {
            url: image.url().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PageDto {
    pub number: u32,
    pub images: Vec<ImageDto>,
}

impl From<&Page> for PageDto {
    fn from(page: &Page) -> Self {
        PageDto {
            number: page.number(),
            images: page
                .images()
                .iter()
                .map(|image| ImageDto::from(image))
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    pub id: String,
    pub author_id: Option<String>,
    pub author: Option<AuthorDto>,
    pub name: String,
    pub synopsis: String,
    pub category_id: Option<String>,
    pub category: Option<CategoryDto>,
    pub tags: Vec<String>,
    pub statistics: StatisticsDto,
    pub pages: Option<Vec<PageDto>>,
    pub status: Option<String>,
}

impl From<&Publication> for PublicationDto {
    fn from(publication: &Publication) -> Self {
        PublicationDto {
            id: publication.base().id().to_string(),
            author_id: None,
            author: None,
            name: publication.header().name().to_string(),
            synopsis: publication.header().synopsis().to_string(),
            category_id: None,
            category: None,
            tags: publication
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
            statistics: StatisticsDto::from(publication.statistics()),
            pages: None,
            status: None,
        }
    }
}

impl PublicationDto {
    pub fn author_id(mut self, publication: &Publication) -> Self {
        self.author_id = Some(publication.author_id().to_string());
        self
    }

    pub fn author(mut self, author: AuthorDto) -> Self {
        self.author = Some(author);
        self
    }

    pub fn category_id(mut self, publication: &Publication) -> Self {
        self.category_id = Some(publication.header().category_id().to_string());
        self
    }

    pub fn category(mut self, category: CategoryDto) -> Self {
        self.category = Some(category);
        self
    }

    pub fn pages(mut self, publication: &Publication) -> Self {
        self.pages = Some(
            publication
                .pages()
                .iter()
                .map(|page| PageDto::from(page))
                .collect(),
        );
        self
    }

    pub fn status(mut self, publication: &Publication) -> Self {
        self.status = Some(publication.status_history().current().status().to_string());
        self
    }
}

#[derive(Serialize)]
pub struct CollectionDto {
    pub id: String,
    pub author_id: Option<String>,
    pub author: Option<AuthorDto>,
    pub name: String,
    pub synopsis: String,
    pub category_id: Option<String>,
    pub category: Option<CategoryDto>,
    pub tags: Vec<String>,
    pub publication_count: Option<usize>,
    pub publications: Option<Vec<PublicationDto>>,
}

impl From<&Collection> for CollectionDto {
    fn from(collection: &Collection) -> Self {
        CollectionDto {
            id: collection.base().id().to_string(),
            author_id: None,
            author: None,
            name: collection.header().name().to_string(),
            synopsis: collection.header().synopsis().to_string(),
            category_id: None,
            category: None,
            tags: collection
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
            publication_count: None,
            publications: None,
        }
    }
}

impl CollectionDto {
    pub fn author_id(mut self, collection: &Collection) -> Self {
        self.author_id = Some(collection.author_id().to_string());
        self
    }

    pub fn author(mut self, author: AuthorDto) -> Self {
        self.author = Some(author);
        self
    }

    pub fn category_id(mut self, collection: &Collection) -> Self {
        self.category_id = Some(collection.header().category_id().to_string());
        self
    }

    pub fn category(mut self, category: CategoryDto) -> Self {
        self.category = Some(category);
        self
    }

    pub fn publication_count(mut self, count: usize) -> Self {
        self.publication_count = Some(count);
        self
    }

    pub fn publications(mut self, publications: Vec<PublicationDto>) -> Self {
        self.publications = Some(publications);
        self
    }
}

#[derive(Serialize)]
pub struct ReviewDto {
    pub reader_id: Option<String>,
    pub reader: Option<ReaderDto>,
    pub publication_id: String,
    pub stars: u8,
    pub comment: String,
}

impl From<&Review> for ReviewDto {
    fn from(review: &Review) -> Self {
        ReviewDto {
            reader_id: None,
            reader: None,
            publication_id: review.base().publication_id().to_string(),
            stars: review.stars().value(),
            comment: review.comment().to_string(),
        }
    }
}

impl ReviewDto {
    pub fn reader_id(mut self, review: &Review) -> Self {
        self.reader_id = Some(review.base().reader_id().to_string());
        self
    }

    pub fn reader(mut self, review: ReaderDto) -> Self {
        self.reader = Some(review);
        self
    }
}

#[derive(Serialize)]
pub struct ReaderDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub subscribed: bool,
}

impl From<&Reader> for ReaderDto {
    fn from(reader: &Reader) -> Self {
        ReaderDto {
            id: reader.base().id().to_string(),
            username: reader.username().to_string(),
            name: reader.name().to_string(),
            lastname: reader.lastname().to_string(),
            subscribed: reader.is_subscribed(),
        }
    }
}
