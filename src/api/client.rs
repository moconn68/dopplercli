use super::{
    location::{Coordinates, Location, LocationApi},
    weather::{WeatherApi, WeatherInfo},
    HttpResult,
};

pub struct ApiClient {
    http_client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::blocking::Client::new(),
        }
    }
}

impl LocationApi for ApiClient {
    fn get_user_location(&self) -> HttpResult<Location> {
        let json: serde_json::Value = self
            .http_client
            .get("http://ipapi.co/json/")
            .send()?
            .json()?;
        let city = json["city"].as_str().unwrap().to_owned();
        let country = json["country"].as_str().unwrap().to_owned();
        let latitude = json["latitude"].to_string();
        let longitude = json["longitude"].to_string();

        Ok(Location {
            city,
            country,
            latitude,
            longitude,
        })
    }

    fn get_coordinates_for_zipcode(&self, zipcode: i16) -> HttpResult<Coordinates> {
        let json: serde_json::Value = self
            .http_client
            .get(format!("https://api.zippopotam.us/us/{zipcode}"))
            .send()?
            .json()?;

        let places = &json["places"];
        let latitude = places["latitude"].to_string();
        let longitude = places["longitude"].to_string();

        Ok(Coordinates {
            latitude,
            longitude,
        })
    }
}

impl WeatherApi for ApiClient {
    fn get_weather_for_coordinates(&self, coords: Coordinates) -> HttpResult<WeatherInfo> {
        let mut url = "https://api.open-meteo.com/v1/forecast?timezone=auto&".to_owned();
        url.push_str(&format!(
      "temperature_unit=fahrenheit&latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum",
      coords.latitude, coords.longitude
  ));

        let json: serde_json::Value = self.http_client.get(url).send()?.json()?;
        // Retrieve relevant fields from API call
        let json_daily = &json["daily"];
        let today_high_temp = json_daily["temperature_2m_max"][0].as_f64().unwrap();
        let today_low_temp = json_daily["temperature_2m_min"][0].as_f64().unwrap();
        let today_precipitation = json_daily["precipitation_sum"][0].as_f64().unwrap();

        Ok(WeatherInfo {
            temp_low: today_low_temp,
            temp_high: today_high_temp,
            precipitation_chance: today_precipitation,
        })
    }
}
