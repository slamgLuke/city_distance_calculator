use crate::coords::Coordinates;

use csv::Reader;
use rand::Rng;
use reqwest::Client;
use serde_json::Value;
use tokio::runtime::Runtime;

const API_KEY: &'static str = "665bbdc3a5089409370770euy935171";
const CSV_FILE: &'static str = "data/worldcities.csv";

pub enum CityType {
    API,
    CSV,
    Mock,
}
pub enum City {
    API(CityAPI),
    CSV(CityCSV),
    Mock(CityMock),
}

pub fn city_from_type(city_type: &CityType, name: String) -> City {
    match city_type {
        CityType::API => City::API(CityAPI {
            name,
            api_key: API_KEY.to_string(),
        }),
        CityType::CSV => City::CSV(CityCSV {
            file_path: CSV_FILE.to_string(),
            name,
        }),
        CityType::Mock => City::Mock(CityMock { name }),
    }
}

pub trait HasCoords {
    fn get_coordinates(&self) -> Result<Coordinates, String>;
}

pub fn get_coordinates(city: &City) -> Result<Coordinates, String> {
    match city {
        City::API(city) => city.get_coordinates(),
        City::CSV(city) => city.get_coordinates(),
        City::Mock(city) => city.get_coordinates(),
    }
}

pub struct CityAPI {
    pub name: String,
    pub api_key: String,
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

pub struct CityCSV {
    pub file_path: String,
    pub name: String,
}
impl HasCoords for CityCSV {
    fn get_coordinates(&self) -> Result<Coordinates, String> {
        let reader = Reader::from_path(&self.file_path);
        if reader.is_err() {
            return Err("Error reading csv".to_string());
        }
        let mut reader = reader.unwrap();
        for result in reader.records() {
            let record = result.unwrap();
            if record[0].to_lowercase() == self.name.to_lowercase() {
                return Ok(Coordinates {
                    latitude: record[2].parse::<f64>().unwrap(),
                    longtude: record[3].parse::<f64>().unwrap(),
                });
            }
        }
        Err("City not found".to_string())
    }
}

pub struct CityMock {
    pub name: String,
}
impl HasCoords for CityMock {
    fn get_coordinates(&self) -> Result<Coordinates, String> {
        Ok(Coordinates {
            latitude: rand::thread_rng().gen_range(-90.0..90.0),
            longtude: rand::thread_rng().gen_range(-180.0..180.0),
        })
    }
}
