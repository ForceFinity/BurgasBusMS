use std::collections::HashMap;
use std::{clone, io};
use std::sync::OnceLock;

use actix_cors::Cors;
use actix_web::{http, get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use reqwest::Client;
use routes::get_busses_in_a_network::get_busses_networked;
use routes::get_stops;
use routes::get_stops_filtered::get_stops_filtered;
use routes::most_effective_route::plan;
use once_cell::sync::Lazy;
mod CONFIG;
mod structs;
mod routes {
    pub mod most_effective_route;
    pub mod get_stops_filtered;
    pub mod get_stops;
    pub mod get_busses_in_a_network;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Stop {
    id: i64,
    name: String,
    lat: f64,
    long: f64,
    time: Option<String>
}

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);
pub static STOPS_RAW: OnceLock<Root> = OnceLock::new();
pub static STOPS: OnceLock<HashMap<i64, Stop>> = OnceLock::new();

async fn initialize_routes() -> Result<(), std::io::Error> {
    let response = CLIENT
        .get(format!("{}/transport/planner/stops", CONFIG::BURGAS_BUS_API))
        .send()
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Fetch error"))?;

    let body = response
        .text()
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "ROUTES fetch error"))?;

    let response_json: Root = serde_json::from_str(&body)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to deserialize routes"))?;

    dbg!(&response_json);

   
    let stops = STOPS.get_or_init(|| {
        let mut map = HashMap::new();
        for stop in &response_json {
            map.insert(
                stop.id,
                Stop {
                    id: stop.id,
                    name: stop.name.clone(),
                    lat: stop.latitude,
                    long: stop.longitude,
                    time: None
                }
            );
        }
        map
    });

    dbg!(&stops);

    STOPS_RAW.set(response_json)
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "ROUTES already initialized"))?;
    STOPS.set(stops.clone())
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "FAILED TO REFINE STOPS"))?;
    
    Ok(())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize routes before starting the server
    match initialize_routes().await {
        Ok(response) => {
            println!("Successfully fetched routes");
        }
        Err(e) => {
            eprintln!("Failed to initialize routes: {:?}", e);
        }
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
            .wrap(Cors::permissive())
            .service(plan)
            .service(get_stops_filtered)
            .service(get_stops::get_stops)
            .service(get_busses_networked)
            .service(health)
            
    })
    .bind(format!("0.0.0.0:{}", CONFIG::PORT))?
    .run()
    .await
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("im Healthy")
}


use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub patterns: Vec<Pattern>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub index: i64,
    pub route_id: i64,
    pub to_stop_id: Option<i64>,
    pub direction: i64,
    pub from_stop_id: Option<i64>,
    pub via_stop_id: Option<i64>,
}