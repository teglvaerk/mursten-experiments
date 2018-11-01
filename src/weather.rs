use reqwest;
use reqwest::Url;


const API_URL: &str = "http://api.openweathermap.org/data/2.5/forecast";

enum City {
    BuenosAires,
}

impl City {
    pub fn country(&self) -> Country {
        match *self {
            City::BuenosAires => Country::Argentina,
        }
    }
    pub fn to_api_param(&self) -> &'static str {
        match *self {
            City::BuenosAires => "Buenos Aires,ar",
        }
    }
}

enum Country {
    Argentina,
}

struct WeatherRequest {
    city: City,
}

impl WeatherRequest {
    pub fn of(city: City) -> Self {
        WeatherRequest {
            city,
        }
    }
}

impl WeatherRequest {
    fn into_url(self) -> String {
        format!("{}?q={}&APPID={}",
                API_URL,
                self.city.to_api_param(),
                include_str!("weather.api_key"))
    }
}

trait WeatherRequestClient {
    fn get_forecast(&self, wr: WeatherRequest) -> reqwest::Result<reqwest::Response>;
}

impl WeatherRequestClient for reqwest::Client {
    fn get_forecast(&self, wr: WeatherRequest) -> reqwest::Result<reqwest::Response> {
        //format!("{}?q={}&APPID={}",
        self.get(API_URL)
            .query(&[("q", wr.city.to_api_param()),
                     ("APPID", include_str!("weather.api_key")),
                     ("unit", "metric")])
            .send()
    }
}


pub fn main() {
    let client = reqwest::Client::new();
    let request = WeatherRequest::of(City::BuenosAires);
    println!("Making request of: {}", request.city.to_api_param());
    let response = client.get_forecast(request).unwrap().text().unwrap();
    println!("Response: {}", response);
}
