use std::collections::HashMap;

use actix_web::{
    dev::Response, get, http, post, web::{self, Json, Path}, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::{Stop, CLIENT, CONFIG::BURGAS_BUS_API, STOPS};

#[derive(Debug, Deserialize, Serialize)]
struct Res {
    id: i64,
    short_buss_name: String,
    long_buss_name: String,
    color: String,
    text_color: String,
    primary_path: Vec<Stop>,
    secondary_path: Vec<Stop>,
    pattern_hash_primary: String,
    pattern_hash_secondary: String,
    geometry_primary: String,
    geometry_secondary: String,
}

#[get("/path/stops/{buss}")]
async fn get_stops_filtered(path: Path<String>) -> impl Responder {
    let buss = path.into_inner();

    let routes_response = CLIENT
        .get(format!("{}/transport/planner/routes", BURGAS_BUS_API))
        .send()
        .await;

    if routes_response.is_err() {
        return HttpResponse::InternalServerError().body("Failed to fetch routes from API");
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
                        let empty_map: HashMap<i64, Stop> = HashMap::new();
                        let stops = STOPS.get().unwrap_or(&empty_map);

                        let mut primary_path: Vec<Stop> = Vec::new();
                        for stop in &filtered_stops[0].patterns[0].stops {
                            let stop_data = stops.get(&stop);
                            if let Some(stop_data) = stop_data {
                                primary_path.push(Stop {
                                    id: stop.clone(),
                                    name: stop_data.name.clone(),
                                    lat: stop_data.lat,
                                    long: stop_data.long,
                                    time: None
                                });
                            }
                        }

                        let mut secondary_path: Vec<Stop> = Vec::new();
                        for stop in &filtered_stops[0].patterns[1].stops {
                            let stop_data = stops.get(&stop);
                            if let Some(stop_data) = stop_data {
                                secondary_path.push(Stop {
                                    id: stop.clone(),
                                    name: stop_data.name.clone(),
                                    lat: stop_data.lat,
                                    long: stop_data.long,
                                    time: None
                                });
                            }
                        }
                        
                        // HttpResponse::Ok().json(filtered_stops)
                        HttpResponse::Ok().json(Res {
                            id: filtered_stops[0].id,
                            short_buss_name: filtered_stops[0].short_name.clone(),
                            long_buss_name: filtered_stops[0].long_name.clone(),
                            color: filtered_stops[0].color.clone().expect("Expected color to be Some value"),
                            text_color: filtered_stops[0].text_color.clone().expect("Expected color to be Some value"),
                            primary_path: primary_path,
                            secondary_path: secondary_path,
                            pattern_hash_primary: filtered_stops[0].patterns[0].pattern_hash.clone(),
                            pattern_hash_secondary: filtered_stops[0].patterns[1].pattern_hash.clone(),
                            geometry_primary: filtered_stops[0].patterns[0].geometry.clone(),
                            geometry_secondary: filtered_stops[0].patterns[1].geometry.clone(),
                        })
                        
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
