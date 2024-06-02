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

fn main() {
    loop {
        let method: CityType;
        loop {
            let input = handle_input::<u32>(Some("Select Method to get coordinates\n1. API\n2. CSV\n3. Mock"));
            if let Ok(1) = input {
                method = CityType::API;
                break;
            } else if let Ok(2) = input {
                method = CityType::CSV;
                break;
            } else if let Ok(3) = input {
                method = CityType::Mock;
                break;
            } else {
                println!("Invalid input!\n");
            }
        }
        let city1_name = handle_input::<String>(Some("Enter first city name:")).unwrap();
        let city2_name = handle_input::<String>(Some("Enter second city name:")).unwrap();
        let (coords1, coords2) = (
            get_coordinates(&city_from_type(&method, city1_name.clone())),
            get_coordinates(&city_from_type(&method, city2_name.clone())),
        );
        
        if coords1.is_err() || coords2.is_err() {
            println!("Error fetching coordinates");
            continue;
        }
        let distance = distance_km(&coords1.unwrap(), &coords2.unwrap());
        println!("Distance between {} and {}: {} km", &city1_name, &city2_name, distance);
    }
}
