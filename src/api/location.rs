use super::HttpResult;

pub trait LocationApi {
    fn get_user_location(&self) -> HttpResult<Location>;
    fn get_coordinates_for_zipcode(&self, zipcode: i16) -> HttpResult<Coordinates>;
}

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: String,
    pub longitude: String,
}

impl From<Location> for Coordinates {
    fn from(value: Location) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}
