#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn distance_km(a: &Coordinates, b: &Coordinates) -> f64 {
    // Haversine formula
    let r = 6371.0; // Earth radius in km
    let d_lat = (b.latitude - a.latitude).to_radians();
    let d_lon = (b.longitude - a.longitude).to_radians();
    let a_lat = a.latitude.to_radians();
    let b_lat = b.latitude.to_radians();
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + (d_lon / 2.0).sin() * (d_lon / 2.0).sin() * a_lat.cos() * b_lat.cos();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    return r * c;
}
