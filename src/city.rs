use crate::coords::Coordinates;

// Polymorphic City trait
pub trait City {
    fn get_coordinates(&self) -> Result<Coordinates, String>;
}

