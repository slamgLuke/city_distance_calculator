use crate::city_interface::CityInterface;
use crate::coords::Coordinates;
use crate::distance::distance_km;

pub fn three_point_distance<'a>(
    first: (&'a CityInterface, &'a Coordinates),
    second: (&'a CityInterface, &'a Coordinates),
    third: (&'a CityInterface, &'a Coordinates),
) -> (&'a CityInterface, &'a CityInterface, f64) {
    let mut min_distance = f64::MAX;
    let mut min_cities = (first.0, second.0);

    let cities = vec![first, second, third];

    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            let distance = distance_km(cities[i].1, cities[j].1);
            if distance < min_distance {
                min_distance = distance;
                min_cities = (cities[i].0, cities[j].0);
            }
        }
    }

    (min_cities.0, min_cities.1, min_distance)
}
