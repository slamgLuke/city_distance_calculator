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
            "Select Method to get coordinates\n0. Quit\n1. API\n2. CSV\n3. Mock",
        ));
        if let Ok(0) = input {
            std::process::exit(0);
        } else if let Ok(1) = input {
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

        // Read city names from user
        let city1_name = handle_input::<String>(Some("Enter first city name:")).unwrap();
        let city2_name = handle_input::<String>(Some("Enter second city name:")).unwrap();

        // Initialize city objects
        let city1 = city_from_type(&method, city1_name.as_str());
        let city2 = city_from_type(&method, city2_name.as_str());

        // Get coordinates for cities
        let coords1 = get_coordinates(&city1);
        let coords2 = get_coordinates(&city2);

        match (coords1, coords2) {
            (Ok(coords1), Ok(coords2)) => {
                println!(
                    "{} coordinates: {}, {}",
                    &city1_name, coords1.latitude, coords1.longitude
                );
                println!(
                    "{} coordinates: {}, {}",
                    &city2_name, coords2.latitude, coords2.longitude
                );
                println!(
                    "Distance between cities: {:.1} km",
                    distance_km(&coords1, &coords2)
                );
            }
            (Err(e), Ok(_)) => println!("Error getting coordinates for {}: {}", city1_name, e),
            (Ok(_), Err(e)) => println!("Error getting coordinates for {}: {}", city2_name, e),
            (Err(e1), Err(e2)) => println!("Error getting coordinates for both cities: {}, {}", e1, e2),
        }
    }
}
