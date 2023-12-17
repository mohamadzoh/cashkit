use crate::constants::{PROVIDER_BASE_URL, PROVIDER_IP};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    pub name: &'static str,
    pub code: &'static str,
    pub symbol: &'static str,
    pub exponent: u8,
    pub numeric_code: &'static str,
    pub withdrawal_date: &'static str,
    pub next_update_date: &'static str,
    pub rates: Option<HashMap<String, f32>>,
}

impl Currency {
    pub fn new(
        name: &'static str,
        code: &'static str,
        symbol: &'static str,
        exponent: u8,
        numeric_code: &'static str,
        withdrawal_date: &'static str,
        rates: HashMap<String, f32>,
    ) -> Self {
        Self {
            name,
            code,
            symbol,
            exponent,
            numeric_code,
            withdrawal_date,
            next_update_date: "",
            rates: Some(rates),
        }
    }
    pub fn exchange_to(&self, amount: f64, to: &Currency) -> f64 {
        let mut result: f64 = 0.0;
        let binding: HashMap<String, f32> = self.exchange_rates();
        let rate: Option<&f32> = binding.get(to.code);
        if let Some(rate) = rate {
            result = amount * (*rate as f64);
        }

        result
    }

    pub fn exchange_into(&self, amount: f64, to: &str) -> f64 {
        let mut result: f64 = 0.0;
        let binding = self.exchange_rates();

        let rate = binding.get(to);
        if let Some(rate) = rate {
            result = amount * (*rate as f64);
        }

        result
    }


    fn exchange_rates(&self) -> HashMap<String, f32> {
        self.online_exchange_rate().unwrap_or(HashMap::new())
    }
    
    pub fn online_exchange_rate(&self) -> Option<HashMap<String, f32>> {
        // Create a TCP connection to the provider
        if let Ok(mut stream) = TcpStream::connect(PROVIDER_IP.to_owned() + ":80") {
            // Prepare the HTTP request
            let url: String = format!("{}{}", PROVIDER_BASE_URL, self.code);
            let request: String = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\n\r\n {} \r\n\r\n",
                url, PROVIDER_IP, "Connection: close"
            );
            // Send the request
            if let Err(_) = stream.write_all(request.as_bytes()) {
                return None;
            }

            // Read the response
            let mut response = String::new();
            if let Err(_) = stream.read_to_string(&mut response) {
                return None;
            }

            // Parse the response
            let response: String = response.trim().to_owned();
            Some(self.parse_response(response))
            // Some(response)
        } else {
            None
        }
    }

    fn parse_response(&self, response: String) -> HashMap<String, f32> {
        // Split the response into lines and get the second line (the first line is the HTTP header)
        // by splitting the response by ",\"rates\":{\""
        // and then get the first element of the second line by splitting the second line by "}"
        // and then remove all / and " from the line
        // and then split them by ,
        // and then create hashmap from the vector with form of code:rate
        // and then return the hashmap
        let mut rates_map: HashMap<String, f32> = HashMap::new();
        let lines: Vec<&str> = response.split(&",\"rates\":{\"".to_owned()).collect();
        if lines.len() < 2 {
            return rates_map;
        }
        let line: &str = lines[1].split(&"}".to_owned()).collect::<Vec<&str>>()[0];
        if line.len() < 1 {
            return rates_map;
        }
        // remove all / and "
        let line = line.replace("/", "").replace("\"", "");
        // split them by ,
        let rates_entries = line.split(",").collect::<Vec<&str>>();
        //create hashmap from the vector with form of code:rate

        rates_entries.iter().for_each(|rate_entry: &&str| {
            let rate_entry = rate_entry.split(":").collect::<Vec<&str>>();
            rates_map.insert(
                rate_entry[0].to_owned(),
                rate_entry[1].parse::<f32>().unwrap(),
            );
        });
        rates_map
    }
}
