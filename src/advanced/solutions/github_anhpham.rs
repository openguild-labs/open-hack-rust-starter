use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use rayon::prelude::*; // Import Rayon

use crate::advanced::model::WeatherData;

pub fn github_anhpham_solution(reader: BufReader<File>, station_data: &mut HashMap<String, WeatherData>) {
    // Collect lines into a vector
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // Parallel processing with Rayon
    let new_station_data: HashMap<String, WeatherData> = lines.par_iter().map(|line| {
        // Split the line into station name and measurement
        let parts: Vec<&str> = line.split(';').collect();
        let mut result_data = HashMap::new();
        if parts.len() == 2 {
            let station_name = parts[0].to_string();
            let measurement: f64 = parts[1].parse().expect("Invalid measurement format");

            // Update weather data for the station
            let data = result_data
                .entry(station_name.clone())
                .or_insert(WeatherData::new());
            data.add_measurement(measurement);
        }
        result_data
    }).reduce(HashMap::new, |mut acc, data| {
        // Merge intermediate results into a single HashMap
        for (key, value) in data {
            if let Some(existing_data) = acc.get_mut(&key) {
                existing_data.add_measurement(value.sum);
                existing_data.count += value.count;
                existing_data.min = existing_data.min.min(value.min);
                existing_data.max = existing_data.max.max(value.max);
            } else {
                acc.insert(key, value);
            }
        }
        acc
    });

    // Update the original station_data with the new data
    for (key, value) in new_station_data {
        station_data.entry(key).or_insert(value);
    }
}
