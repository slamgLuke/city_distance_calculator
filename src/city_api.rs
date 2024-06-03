use crate::city::City;
use crate::coords::Coordinates;

use reqwest::Client;
use serde_json::Value;
use tokio::runtime::Runtime;

pub struct CityAPI {
    pub name: String,
    pub api_key: String,
}

impl CityAPI {
    pub fn new(name: String, api_key: String) -> CityAPI {
        CityAPI { name, api_key }
    }
}

impl City for CityAPI {
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
                return Err("Received invalid response".to_string());
            }
            let json: Result<Value, serde_json::Error> =
                serde_json::from_str(body.unwrap().as_str());
            if let Err(_) = json {
                return Err("Error parsing json".to_string());
            }
            // check every item in the json array until we find the coordinates
            for item in json.as_ref().unwrap().as_array().unwrap() {
                let (latitude_str, longitude_str) = (item["lat"].as_str(), item["lon"].as_str());
                if latitude_str.is_none() || longitude_str.is_none() {
                    continue;
                }
                let (latitude, longitude) = (
                    latitude_str.unwrap().parse::<f64>().ok(),
                    longitude_str.unwrap().parse::<f64>().ok(),
                );
                return Ok(Coordinates {
                    latitude: latitude.unwrap(),
                    longitude: longitude.unwrap(),
                });
            }
            Err("Coordinates not found in response".to_string())
        })
    }
}
