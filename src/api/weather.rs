use super::location::Coordinates;
use super::HttpResult;

pub trait WeatherApi {
    fn get_weather_for_coordinates(&self, coords: Coordinates) -> HttpResult<WeatherInfo>;
}
#[derive(Debug)]
pub struct WeatherInfo {
    pub temp_low: f64,
    pub temp_high: f64,
    pub precipitation_chance: f64,
}

impl std::fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There is a {}% chance of precipitation today, with a low temperature of {} degrees and a high of {} degrees.", self.precipitation_chance, self.temp_low, self.temp_high)
    }
}
