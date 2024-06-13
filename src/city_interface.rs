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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three_point::*;

    #[test]
    #[should_panic]
    fn test_invalid_city() {
        let city = CityInterface::CSV(CityCSV::new(
            "INVALID CITY".to_owned(),
            CSV_FILE.to_string(),
        ));
        let coords = get_coordinates(&city).unwrap();
        dbg!(coords);
    }

    #[test]
    fn test_dup_city() {
        let city1 = CityInterface::CSV(CityCSV::new("Sydney".to_owned(), CSV_FILE.to_string()));
        let coords1 = get_coordinates(&city1).unwrap();
        let city2 = CityInterface::CSV(CityCSV::new("Sydney".to_owned(), CSV_FILE.to_string()));
        let coords2 = get_coordinates(&city2).unwrap();
        let city3 = CityInterface::CSV(CityCSV::new("Sydney".to_owned(), CSV_FILE.to_string()));
        let coords3 = get_coordinates(&city3).unwrap();

        assert_eq!(
            three_point_distance((&city1, &coords1), (&city2, &coords2), (&city3, &coords3)).2,
            0.0
        );
    }

    #[test]
    fn test_api() {
        let city1 = CityInterface::API(CityAPI::new("Lima".to_owned(), API_KEY.to_string()));
        let coords1 = get_coordinates(&city1).unwrap();
        let city2 = CityInterface::API(CityAPI::new("Huamanga".to_owned(), API_KEY.to_string()));
        let coords2 = get_coordinates(&city2).unwrap();
        let city3 = CityInterface::API(CityAPI::new("Iquitos".to_owned(), API_KEY.to_string()));
        let coords3 = get_coordinates(&city3).unwrap();

        assert_eq!(
            three_point_distance((&city1, &coords1), (&city2, &coords2), (&city3, &coords3))
                .2
                .round(),
            329.0
        );
    }

    #[test]
    fn test_csv() {
        let city1 = CityInterface::CSV(CityCSV::new("Lima".to_owned(), CSV_FILE.to_string()));
        let coords1 = get_coordinates(&city1).unwrap();
        let city2 = CityInterface::CSV(CityCSV::new("Ayacucho".to_owned(), CSV_FILE.to_string()));
        let coords2 = get_coordinates(&city2).unwrap();
        let city3 = CityInterface::CSV(CityCSV::new("Iquitos".to_owned(), CSV_FILE.to_string()));
        let coords3 = get_coordinates(&city3).unwrap();

        assert_eq!(
            three_point_distance((&city1, &coords1), (&city2, &coords2), (&city3, &coords3))
                .2
                .round(),
            329.0
        );
    }
}
