use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::advanced::model::WeatherData;

pub fn template_solution(reader: BufReader<File>, station_data: &mut HashMap<String, WeatherData>) {
    // MAIN CODE: Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            // Split the line into station name and measurement
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 2 {
                let station_name = parts[0].to_string();
                let measurement: f64 = parts[1].parse().expect("Invalid measurement format");

                // Update weather data for the station
                let data = station_data
                    .entry(station_name)
                    .or_insert(WeatherData::new());
                data.add_measurement(measurement);
            }
        }
    }
}
