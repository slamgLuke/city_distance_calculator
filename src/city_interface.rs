use crate::city::*;
use crate::city_api::*;
use crate::city_csv::*;
use crate::city_mock::*;
use crate::coords::Coordinates;

const API_KEY: &'static str = "665bbdc3a5089409370770euy935171";
const CSV_FILE: &'static str = "data/worldcities.csv";

// City Types Enum
#[derive(PartialEq)]
pub enum CityType {
    API,
    CSV,
    Mock,
}

// City Interface
pub enum CityInterface {
    API(CityAPI),
    CSV(CityCSV),
    Mock(CityMock),
}

pub fn get_coordinates(city: &CityInterface) -> Result<Coordinates, String> {
    match city {
        CityInterface::API(city) => city.get_coordinates(),
        CityInterface::CSV(city) => city.get_coordinates(),
        CityInterface::Mock(city) => city.get_coordinates(),
    }
}

pub fn city_from_type(city_type: &CityType, name: &str) -> CityInterface {
    match city_type {
        CityType::API => CityInterface::API(CityAPI::new(name.to_owned(), API_KEY.to_string())),
        CityType::CSV => CityInterface::CSV(CityCSV::new(name.to_owned(), CSV_FILE.to_string())),
        CityType::Mock => CityInterface::Mock(CityMock::new(name.to_owned())),
    }
}

pub fn get_name(city: &CityInterface) -> String {
    match city {
        CityInterface::API(city) => city.name.clone(),
        CityInterface::CSV(city) => city.name.clone(),
        CityInterface::Mock(city) => city.name.clone(),
    }
}
