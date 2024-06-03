use crate::city::HasCoords;
use crate::coords::Coordinates;

use rand::Rng;

pub struct CityMock {
    pub name: String,
}

impl CityMock {
    pub fn new(name: String) -> Self {
        CityMock { name }
    }
}

impl HasCoords for CityMock {
    fn get_coordinates(&self) -> Result<Coordinates, String> {
        Ok(Coordinates {
            latitude: rand::thread_rng().gen_range(-90.0..90.0),
            longitude: rand::thread_rng().gen_range(-180.0..180.0),
        })
    }
}
