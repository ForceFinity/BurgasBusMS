use actix_web::{
    dev::Response, get, http, post, web::{self, Json, Path}, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::CONFIG::BURGAS_BUS_API;

#[derive(Debug, Deserialize, Serialize)]
struct Res {
    short_buss_name: String,
    long_buss_name: String,
    color: String,
    text_color: String,
    primary_path: Vec<Stop>,
    secondary_path: Vec<Stop>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stop {
    lat: f64,
    long: f64,
    name: String,
}

#[get("/path/stops/{buss}")]
async fn get_stops(path: Path<String>) -> impl Responder {
    let buss = path.into_inner();
    let client = Client::new();

    // Fetch the routes
    let routes_response = client
        .get(format!("{}/transport/planner/routes", BURGAS_BUS_API))
        .send()
        .await;

    if routes_response.is_err() {
        return HttpResponse::InternalServerError().body("Failed to fetch routes from API");
    }

    // Fetch the stops
    let stops_response = client
        .get(format!("{}/transport/planner/stops", BURGAS_BUS_API))
        .send()
        .await;

    if stops_response.is_err() {
        return HttpResponse::InternalServerError().body("Failed to fetch stops from API");
    }

    match routes_response.unwrap().text().await {
        Ok(body) => {
            dbg!(&body);
            match serde_json::from_str::<Root>(&body) {
                Ok(stops) => {
                    let filtered_stops: Vec<Root2> = stops
                        .into_iter()
                        .filter(|stop| stop.short_name == buss)
                        .collect();

                    if filtered_stops.is_empty() {
                        HttpResponse::NotFound()
                            .body(format!("No stops found for short_name: {}", buss))
                    } else {
                        HttpResponse::Ok().json(filtered_stops)
                    }
                }
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Failed to parse API response: {}, body length: {}, body: {}",
                    err,
                    &body.len(),
                    &body
                )),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch API"),
    }
}

// Type alias for the main stops structure
pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub short_name: String,
    pub long_name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub patterns: Vec<Pattern>,
    pub inbound: Option<String>,
    pub outbound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub index: i64,
    pub route_id: i64,
    pub to_stop_id: Option<i64>,
    pub stops: Vec<i64>,
    pub geometry: String,
    pub direction: i64,
    pub pattern_hash: String,
    pub from_stop_id: Option<i64>,
    pub via_stop_id: Option<i64>,
}
