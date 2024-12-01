use std::collections::HashMap;

use actix_web::{
    dev::Response, get, http, post, web::{self, Json, Path}, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::{Stop, CONFIG::BURGAS_BUS_API, STOPS};

#[get("/path/stops")]
async fn get_stops() -> impl Responder {
    let empty_map: HashMap<i64, Stop> = HashMap::new();
    let stops = STOPS.get().unwrap_or(&empty_map);
    HttpResponse::Ok().json(stops)
}
