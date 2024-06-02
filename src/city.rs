use crate::city_api::{CityAPI, API_KEY};
use crate::city_csv::{CityCSV, CSV_FILE};
use crate::city_mock::CityMock;
use crate::coords::Coordinates;

pub trait HasCoords {
    fn get_coordinates(&self) -> Result<Coordinates, String>;
}

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

pub fn get_coordinates(city: &City) -> Result<Coordinates, String> {
    match city {
        City::API(city) => city.get_coordinates(),
        City::CSV(city) => city.get_coordinates(),
        City::Mock(city) => city.get_coordinates(),
    }
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
