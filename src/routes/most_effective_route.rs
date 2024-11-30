use actix_web::{get, http, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::{ structs::{Itinerary, Root}, CONFIG::BURGAS_BUS_API};

#[derive(Debug, Deserialize, Serialize)] // Serialize is needed for sending the request
struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Req {
    fromPlace: Coord,
    toPlace: Coord,
    maxWalkDistance: u16,
    walkingSpeed: u8,
}
#[derive(Debug, Deserialize, Serialize)]
struct stops {
    time: i64,
    bussName: String,
    walkingKM: f64,
    timeWalking: i64,
    timeWaiting: i64,
    timeInBus: i64,
    goto: Coord,
}
#[derive(Debug, Deserialize, Serialize)]
struct Res {
    stops: Vec<stops>
}

#[post("/path/plan")]
async fn plan(data: web::Json<Req>) -> impl Responder {
    let client = Client::new();

    // let client = reqwest::Client::builder()
    // .danger_accept_invalid_certs(true)
    // .max_response_body_size(10 * 1024 * 1024) // 10 MB
    // .build()?;

    // Serialize the data to JSON
    let json_body = match serde_json::to_string(&data.into_inner()) {
        Ok(body) => body,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to serialize request body"),
    };

    // Send the POST request
    match client.post(format!("{}/transport/planner/plan", BURGAS_BUS_API))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
    {
        Ok(response) => {
            // If the response is successful, return the response body
            match response.text().await {
                Ok(body) => 
                {
                    print!("real shit");
                    dbg!(&body);
                    match serde_json::from_str::<Root>(&body) {
                        Ok(itineraries) => {
                            // Initialize _res with an empty Vec<stops>
                            let mut _res = Res {
                                stops: Vec::new(),
                            };
                        
                            for value in &itineraries.plan.itineraries {
                                println!("{:?}", value);
                                _res.stops.push(stops {
                                    time: value.duration,
                                    bussName: value.legs[1].route.clone(),
                                    walkingKM: value.walk_distance,
                                    timeWalking: value.walk_time,
                                    timeWaiting: value.waiting_time,
                                    timeInBus: value.transit_time,
                                    goto: Coord {
                                        lat: value.legs[0].to.lat.clone(),
                                        lon: value.legs[0].to.lon.clone(),
                                    },
                                });
                            }
                        
                            // Return _res as a response
                            HttpResponse::Ok().json(_res)
                        },                        
                        Err(err) => HttpResponse::InternalServerError().body(format!(
                            "Failed to parse API response: {}, body length: {}, body: {}",
                            err,
                            &body.len(),
                            &body
                        )),
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Failed to fetch API"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to send request"),
    }
}
