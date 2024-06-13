use crate::coords::Coordinates;

// Calculate the distance between two coordinates in kilometers
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = Coordinates {
            latitude: 0.0,
            longitude: 0.0,
        };
        let b = Coordinates {
            latitude: 1.0,
            longitude: 1.0,
        };
        assert_eq!(distance_km(&a, &b).round(), 157.0);
    }

    #[test]
    fn test_2() {
        let a = Coordinates {
            latitude: 34.0522,
            longitude: -118.2437,
        };
        let b = Coordinates {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        assert_eq!(distance_km(&a, &b).round(), 3936.0);
    }

    #[test]
    fn test_3() {
        let a = Coordinates {
            latitude: -33.8688,
            longitude: 151.2093,
        };
        let b = Coordinates {
            latitude: 35.6895,
            longitude: 139.6917,
        };
        assert_eq!(distance_km(&a, &b).round(), 7827.0);
    }

    #[test]
    fn test_4() {
        let a = Coordinates {
            latitude: -1.0,
            longitude: -1.0,
        };
        let b = Coordinates {
            latitude: 2.0,
            longitude: 2.0,
        };
        assert_eq!(distance_km(&a, &b).round(), 472.0);
    }

    #[test]
    fn test_5() {
        let a = Coordinates {
            latitude: 51.5074,
            longitude: -0.1278,
        };
        let b = Coordinates {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        assert_eq!(distance_km(&a, &b).round(), 344.0);
    }

    #[test]
    fn test_6() {
        let a = Coordinates {
            latitude: -37.8136,
            longitude: 144.9631,
        };
        let b = Coordinates {
            latitude: -33.8688,
            longitude: 151.2093,
        };
        assert_eq!(distance_km(&a, &b).round(), 713.0);
    }

    #[test]
    fn test_7() {
        let a = Coordinates {
            latitude: 35.6895,
            longitude: 139.6917,
        };
        let b = Coordinates {
            latitude: 55.7558,
            longitude: 37.6173,
        };
        assert_eq!(distance_km(&a, &b).round(), 7478.0);
    }
}
