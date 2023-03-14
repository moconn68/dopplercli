use crate::api::{client::ApiClient, location::LocationApi, weather::WeatherApi};

mod api;

fn main() {
    let api_client = ApiClient::new();
    let user_location = api_client.get_user_location().unwrap();
    println!(
        "Fetching weather data for {}, {}...",
        user_location.city, user_location.country
    );
    match api_client.get_weather_for_coordinates(user_location.into()) {
        Ok(weather_info) => {
            println!("{weather_info}");
        }
        Err(e) => {
            println!("It didn't work! Error: {e:?}");
        }
    }
}
