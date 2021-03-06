mod authorization;
mod container;
mod development;
mod error;
mod handlers;
mod infrastructure;
mod response;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use common::config::Config;

use container::Container;
use handlers::{author, catalogue, category, collection, event, publication, role, user};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Omics")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::get();

    // Dependencies
    let container = web::Data::new(Container::new().await);
    if let Err(err) = container.subscribe().await {
        println!("Subscriptions: {}", err);
        return Ok(());
    }

    if config.env() == "development" {
        if let Err(err) = development::populate(&container).await {
            println!("{:?}", err);
        }
    }

    println!("Listening on {}", config.port());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().finish())
            .app_data(container.clone())
            .service(
                web::scope("/api")
                    .route("/dev", web::get().to(index))
                    .configure(author::routes)
                    .configure(catalogue::routes)
                    .configure(category::routes)
                    .configure(collection::routes)
                    .configure(event::routes)
                    .configure(publication::routes)
                    .configure(role::routes)
                    .configure(user::routes),
            )
    })
    .bind(format!("0.0.0.0:{}", config.port()))?
    .run()
    .await
}
