use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub plan: Plan,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub date: i64,
    pub itineraries: Vec<Itinerary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Itinerary {
    pub duration: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub walk_time: i64,
    pub transit_time: i64,
    pub waiting_time: i64,
    pub walk_distance: f64,
    pub walk_limit_exceeded: bool,
    pub elevation_lost: f64,
    pub elevation_gained: f64,
    pub transfers: f64,
    pub legs: Vec<Leg>,
    pub too_sloped: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    pub mode: String,
    pub route: String,
    pub route_type: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub departure_delay: i64,
    pub arrival_delay: i64,
    pub real_time: bool,
    pub pathway: bool,
    pub is_non_exact_frequency: bool,
    pub interline_with_previous_leg: bool,
    pub from: From,
    pub to: To,
    pub rented_bike: bool,
    pub duration: f64,
    pub transit_leg: bool,
    pub leg_geometry: LegGeometry,
    pub steps: Vec<Step>,
    pub trip_id: Option<String>,
    pub route_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    pub name: String,
    pub departure: i64,
    pub lat: f64,
    pub lon: f64,
    pub vertex_type: String,
    pub arrival: Option<i64>,
    pub stop_id: Option<i64>,
    pub stop_index: Option<i64>,
    pub stop_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct To {
    pub name: String,
    pub arrival: i64,
    pub departure: Option<i64>,
    pub stop_id: i64,
    pub stop_index: i64,
    pub lat: f64,
    pub lon: f64,
    pub stop_code: String,
    pub vertex_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegGeometry {
    pub points: String,
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub distance: f64,
    pub relative_direction: String,
    pub street_name: String,
    pub absolute_direction: String,
    pub stay_on: bool,
    pub area: bool,
    pub bogus_name: bool,
    pub coordinate: Coordinate,
    pub lon: f64,
    pub lat: f64,
    pub elevation: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}
