use actix_web::{web, HttpRequest, HttpResponse, Responder};

use publishing::application::publication::{
    AddReview, AddReviewCommand, Approve, Create, CreateCommand, Delete, DeleteReview, GetById,
    Like, Publish, Read, Reject, Reviews, Search, SearchCommand, Unlike, Update, UpdateCommand,
    UpdatePages, UpdatePagesCommand,
};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

// POST /publications
async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications?q
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Search::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetById::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// PUT /publications/:id
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// PUT /publications/:id/pages
async fn update_pages(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdatePagesCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    UpdatePages::new(c.publishing.event_pub(), c.publishing.publication_repo())
        .exec(auth_id, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// DELETE /publications/:id
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Delete::new(c.publishing.event_pub(), c.publishing.publication_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// POST /publications/:id/publish
async fn publish(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Publish::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/approve
async fn approve(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Approve::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/reject
async fn reject(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Reject::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/read
async fn read(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Read::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/like
async fn like(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Like::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/unlike
async fn unlike(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Unlike::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/review
async fn review(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<AddReviewCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    AddReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// DELETE /publications/:id/review
async fn delete_review(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    DeleteReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id/reviews
async fn reviews(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    Reviews::new(c.publishing.interaction_repo(), c.publishing.reader_repo())
        .exec(path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/publications")
            .route("", web::post().to(create))
            .route("", web::get().to(search))
            .route("/{publicaton_id}", web::get().to(get_by_id))
            .route("/{publicaton_id}", web::put().to(update))
            .route("/{publicaton_id}/pages", web::put().to(update_pages))
            .route("/{publicaton_id}", web::delete().to(delete))
            .route("/{publicaton_id}/publish", web::post().to(publish))
            .route("/{publicaton_id}/approve", web::post().to(approve))
            .route("/{publicaton_id}/reject", web::post().to(reject))
            .route("/{publicaton_id}/read", web::post().to(read)) // TODO: should be GET with pages
            .route("/{publicaton_id}/like", web::post().to(like))
            .route("/{publicaton_id}/unlike", web::post().to(unlike))
            .route("/{publicaton_id}/review", web::post().to(review))
            .route("/{publicaton_id}/review", web::delete().to(delete_review))
            .route("/{publicaton_id}/reviews", web::get().to(reviews)),
    );
}
