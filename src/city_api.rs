use crate::city::HasCoords;
use crate::coords::Coordinates;

use reqwest::Client;
use serde_json::Value;
use tokio::runtime::Runtime;


pub const API_KEY: &'static str = "665bbdc3a5089409370770euy935171";

pub struct CityAPI {
    pub name: String,
    pub api_key: String,
}

impl CityAPI {
    pub fn new(name: String, api_key: String) -> CityAPI {
        CityAPI { name, api_key }
    }
}

impl HasCoords for CityAPI {
    fn get_coordinates(&self) -> Result<Coordinates, String> {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let url = format!(
                "https://geocode.maps.co/search?q={}&api_key={}",
                self.name, self.api_key
            );
            let client = Client::new();
            let response = client
                .get(url.as_str())
                .header("X-Api-Key", self.api_key.as_str())
                .send()
                .await;
            if let Err(_) = response {
                return Err("Error fetching response".to_string());
            }
            let body = response.unwrap().text().await;
            if let Err(_) = body {
                return Err("Error fetching response".to_string());
            }
            let json: Result<Value, serde_json::Error> =
                serde_json::from_str(body.unwrap().as_str());
            if let Err(_) = json {
                return Err("Error parsing json".to_string());
            }
            let json: Value = json.unwrap()[0].clone();
            // dbg!(&json);
            let (latitude_str, longitude_str) = (json["lat"].as_str(), json["lon"].as_str());
            if latitude_str.is_none() || longitude_str.is_none() {
                return Err("Error parsing json".to_string());
            }
            let (latitude, longitude) = (
                latitude_str.unwrap().parse::<f64>().ok(),
                longitude_str.unwrap().parse::<f64>().ok(),
            );
            Ok(Coordinates {
                latitude: latitude.unwrap(),
                longtude: longitude.unwrap(),
            })
        })
    }
}
