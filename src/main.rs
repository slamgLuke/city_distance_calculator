mod city;
mod city_api;
mod city_csv;
mod city_mock;
mod coords;

use city::*;
use coords::*;
use std::io;
use std::str::FromStr;

fn handle_input<T: FromStr>(prompt: Option<&str>) -> Result<T, T::Err> {
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>()
}

fn get_method() -> CityType {
    loop {
        let input = handle_input::<u32>(Some(
            "Select Method to get coordinates\n1. API\n2. CSV\n3. Mock",
        ));
        if let Ok(1) = input {
            return CityType::API;
        } else if let Ok(2) = input {
            return CityType::CSV;
        } else if let Ok(3) = input {
            return CityType::Mock;
        } else {
            println!("Invalid input!\n");
        }
    }
}

fn main() {
    loop {
        let method: CityType = get_method();

        let city1_name = handle_input::<String>(Some("Enter first city name:")).unwrap();
        let city2_name = handle_input::<String>(Some("Enter second city name:")).unwrap();
        let (coords1, coords2) = (
            get_coordinates(&city_from_type(&method, city1_name.as_str())),
            get_coordinates(&city_from_type(&method, city2_name.as_str())),
        );

        if let (Ok(coords1), Ok(coords2)) = (coords1, coords2) {
            println!("City 1 coordinates: {:?}", coords1);
            println!("City 2 coordinates: {:?}", coords2);
            println!("Distance between cities: {}", distance_km(&coords1, &coords2));
        } else {
            println!("Error getting coordinates!\n");
        }
    }
}
