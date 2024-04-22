use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

use crate::advanced::model::WeatherData;

pub fn github_lamdanghoang_solution(
    reader: BufReader<File>,
    station_data: &mut HashMap<String, WeatherData>,
) {
    // MAIN CODE
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let length = lines.len();
    let mut reader_1: Vec<String> = Vec::new();
    let mut reader_2: Vec<String> = Vec::new();
    for (i, val) in lines.into_iter().enumerate() {
        if i < length / 2 {
            reader_1.push(val);
        } else {
            reader_2.push(val);
        }
    }

    // Thread

    let mut share_new_station_data = Arc::new(Mutex::new(HashMap::new()));
    let clone_station_data2 = Arc::clone(&mut share_new_station_data);
    let clone_station_data1 = Arc::clone(&mut share_new_station_data);
    let thread1 = thread::spawn(move || {
        for line in reader_1 {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 2 {
                let station_name = parts[0].to_string();
                let measurement: f64 = parts[1].parse().expect("Invalid measurement format");

                let mut share_new_station_data_lock = clone_station_data2.lock().unwrap();
                let data = share_new_station_data_lock
                    .entry(station_name)
                    .or_insert(WeatherData::new());
                data.add_measurement(measurement);
            }
        }
    });

    let thread2 = thread::spawn(move || {
        for line in reader_2 {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 2 {
                let station_name = parts[0].to_string();
                let measurement: f64 = parts[1].parse().expect("Invalid measurement format");

                let mut share_new_station_data_lock = clone_station_data1.lock().unwrap();
                let data = share_new_station_data_lock
                    .entry(station_name)
                    .or_insert(WeatherData::new());
                data.add_measurement(measurement);
            }
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let data = share_new_station_data.lock().unwrap();

    for (key, value) in data.deref() {
        station_data.entry(key.clone()).or_insert(value.clone());
    }
}
