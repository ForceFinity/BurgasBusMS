use actix_cors::Cors;
use actix_web::{http, get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use routes::get_stops::get_stops;
use routes::most_effective_route::plan;
mod CONFIG;
mod structs;
mod routes {
    pub mod most_effective_route;
    pub mod get_stops;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
            // .wrap(
            //     Cors::default()
            //         .allowed_origin(&format!("{}:{}", config::API_URL_FRONTEND, config::PORT_FRONTEND))
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            //         .allowed_headers(vec![http::header::CONTENT_TYPE])
            //         .max_age(3600),
            // )
            .wrap(Cors::permissive())
            .service(plan)
            .service(get_stops)
            .service(health)
    })
    .bind(format!("127.0.0.1:{}", CONFIG::PORT))?
    .run()
    .await
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("im Healthy")
}