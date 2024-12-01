use std::{collections::HashMap, time::Duration};

use actix_web::{
    dev::Response, get, post, web::{self, Json, Path}, App, Error, HttpResponse, HttpServer, Responder
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::time::sleep;

use crate::{Stop, CLIENT, CONFIG::BURGAS_BUS_API, STOPS};

use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct train_location {
    from_lat: f64,
    from_long: f64,
    from_id: i64,
    to_lat: f64,
    to_long: f64,
    to_id: i64
}

#[derive(Debug, Deserialize, Serialize)]
struct Res {
    id: i64,
    short_buss_name: String,
    long_buss_name: String,
    color: String,
    text_color: String,
    train_locations_primary: Vec<train_location>,
    primary_path: Vec<Stop>,
    train_locations_secondary: Vec<train_location>,
    secondary_path: Vec<Stop>,
    pattern_hash_primary: String,
    pattern_hash_secondary: String,
    geometry_primary: String,
    geometry_secondary: String,
}

async fn get_arrival_time(stop_id: i64, bus_id: i64) -> Result<StopTime, Error> {
    let routes_response = CLIENT
        .get(format!("{}/transport/planner/stops/{}/times", BURGAS_BUS_API, stop_id))
        .send()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to fetch routes from API"));

    let body = routes_response
        .unwrap()
        .text()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to fetch routes from API"));

    let stop_times: StopTime = serde_json::from_str(&body.unwrap())
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to serialize")).unwrap();

    let filtered_stop_times: StopTime = stop_times
        .into_iter()
        .filter(|stop_time| stop_time.route.route_id == bus_id)
        .collect();

    println!("{:?}", filtered_stop_times);

    sleep(Duration::from_secs(1)).await;

    Ok(filtered_stop_times)
}



#[get("/path/busses/{buss}")]
async fn get_busses_networked(path: Path<String>) -> impl Responder {
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

                        let mut train_locations_primary: Vec<train_location> = Vec::new();

                        let mut primary_path: Vec<Stop> = Vec::new();

                        let mut preveous_date_primary:String = "2000-12-01T04:39:00Z".to_string();
                        let mut preveous_lat_primary: f64 = 0.0;
                        let mut preveous_long_primary: f64 = 0.0;
                        let mut preveous_stop_id_primary: i64 = 0;
                        
                        for stop in &filtered_stops[0].patterns[0].stops {
                            let stop_data = stops.get(&stop);
                            if let Some(stop_data) = stop_data {

                                let arrival_data = get_arrival_time(stop.clone(), filtered_stops[0].id)
                                    .await
                                    .map_err(|_| return HttpResponse::InternalServerError().body("Failed to fetch routes from API"));
                                let Data = arrival_data.unwrap();

                                let time1 = DateTime::parse_from_rfc3339(&preveous_date_primary).expect("Invalid datetime format");
                                let time2 = DateTime::parse_from_rfc3339(&Data[0].times[0].scheduled_arrival.clone()).expect("Invalid datetime format");
                                let delta_time = time2 - time1;

                                if delta_time.num_milliseconds() < 0 {
                                    train_locations_primary.push(train_location {
                                        from_lat: preveous_lat_primary,
                                        from_long: preveous_long_primary,
                                        from_id: preveous_stop_id_primary,
                                        to_lat: stop_data.lat,
                                        to_long: stop_data.long,
                                        to_id: stop_data.id
                                    });
                                }

                                primary_path.push(Stop {
                                    id: stop.clone(),
                                    name: stop_data.name.clone(),
                                    lat: stop_data.lat,
                                    long: stop_data.long,
                                    time: Some(Data[0].times[0].scheduled_arrival.clone())
                                });
                                preveous_date_primary = Data[0].times[0].scheduled_arrival.clone();
                                preveous_lat_primary = stop_data.lat;
                                preveous_long_primary = stop_data.long;
                                preveous_stop_id_primary = stop_data.id;
                            }
                        }
                        let mut train_locations_secondary: Vec<train_location> = Vec::new();

                        let mut preveous_date_secondary: String = "2000-12-01T04:39:00Z".to_string();
                        let mut preveous_lat_secondary: f64 = 0.0;
                        let mut preveous_long_secondary: f64 = 0.0;
                        let mut preveous_stop_id_secondary: i64 = 0;

                        let mut secondary_path: Vec<Stop> = Vec::new();
                        for stop in &filtered_stops[0].patterns[1].stops {

                            let stop_data = stops.get(&stop);
                            if let Some(stop_data) = stop_data {

                                let arrival_data = get_arrival_time(stop.clone(), filtered_stops[0].id)
                                    .await
                                    .map_err(|_| return HttpResponse::InternalServerError().body("Failed to fetch routes from API"));
                                let Data = arrival_data.unwrap();

                                let time1 = DateTime::parse_from_rfc3339(&preveous_date_secondary).expect("Invalid datetime format");
                                let time2 = DateTime::parse_from_rfc3339(&Data[0].times[0].scheduled_arrival.clone()).expect("Invalid datetime format");
                                let delta_time = time2 - time1;

                                if delta_time.num_milliseconds() < 0 {
                                    train_locations_secondary.push(train_location {
                                        from_lat: preveous_lat_secondary,
                                        from_long: preveous_long_secondary,
                                        from_id: preveous_stop_id_secondary,
                                        to_lat: stop_data.lat,
                                        to_long: stop_data.long,
                                        to_id: stop_data.id
                                    });
                                }

                                secondary_path.push(Stop {
                                    id: stop.clone(),
                                    name: stop_data.name.clone(),
                                    lat: stop_data.lat,
                                    long: stop_data.long,
                                    time: Some(Data[0].times[0].scheduled_arrival.clone())
                                });
                                preveous_date_secondary = Data[0].times[0].scheduled_arrival.clone();
                                preveous_lat_secondary = stop_data.lat;
                                preveous_long_secondary = stop_data.long;
                                preveous_stop_id_secondary = stop_data.id;
                            }

                        }
                        
                        // HttpResponse::Ok().json(filtered_stops)
                        HttpResponse::Ok().json(Res {
                            id: filtered_stops[0].id,
                            short_buss_name: filtered_stops[0].short_name.clone(),
                            long_buss_name: filtered_stops[0].long_name.clone(),
                            color: filtered_stops[0].color.clone().expect("Expected color to be Some value"),
                            text_color: filtered_stops[0].text_color.clone().expect("Expected color to be Some value"),
                            train_locations_primary: train_locations_primary,
                            primary_path: primary_path,
                            train_locations_secondary: train_locations_secondary,
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






pub type StopTime = Vec<StopTime2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTime2 {
    pub route: Route,
    pub times: Vec<Time>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub route_id: i64,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub stop_id: i64,
    pub scheduled_arrival: String,
    pub scheduled_departure: String,
    pub arrival_delay: i64,
    pub departure_delay: i64,
    pub realtime: bool,
    pub trip_id: String,
    pub headsign: String,
}
