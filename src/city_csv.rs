use crate::coords::Coordinates;
use crate::city::City;

use csv::Reader;

pub struct CityCSV {
    pub file_path: String,
    pub name: String,
}

impl CityCSV {
    pub fn new(name: String, file_path: String) -> CityCSV {
        CityCSV { name, file_path }
    }
}

impl City for CityCSV {
    fn get_coordinates(&self) -> Result<Coordinates, String> {
        let reader = Reader::from_path(&self.file_path);
        if reader.is_err() {
            return Err("Error reading csv file".to_string());
        }
        let mut reader = reader.unwrap();
        for result in reader.records() {
            let record = result.unwrap();
            if record[0].to_lowercase() == self.name.to_lowercase() {
                return Ok(Coordinates {
                    latitude: record[2].parse::<f64>().unwrap(),
                    longitude: record[3].parse::<f64>().unwrap(),
                });
            }
        }
        Err("City not found".to_string())
    }
}
