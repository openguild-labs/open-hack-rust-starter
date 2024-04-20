use crate::advanced::model::WeatherData;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::solutions;
use super::utils::timeit;

#[allow(dead_code)]
fn main_template(
    f: fn(reader: BufReader<File>, station_data: &mut HashMap<String, WeatherData>) -> (),
) {
    use std::env;

    // Get the current working director
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the relative path to the file
    let relative_path = "src/advanced/small_data.txt";

    // Combine current directory with relative path
    let file_path = current_dir.join(relative_path);

    // Open the file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Initialize a HashMap to store weather station data
    let mut station_data: HashMap<String, WeatherData> = HashMap::new();

    f(reader, &mut station_data);

    // END CODE: Output the results
    let mut output = String::new();
    output.push('{');
    for (station, data) in station_data.iter() {
        let mean = data.calculate_mean();
        output.push_str(&format!(
            "{}={:.1}/{:.1}/{:.1}, ",
            station, data.min, mean, data.max
        ));
    }
    // Remove the trailing comma and space
    output.pop();
    output.pop();
    output.push('}');

    println!("{}", output);
}

#[test]
fn benchmark_template() {
    timeit("template_code", || {
        main_template(solutions::template::template_solution)
    });
}

#[test]
fn benchmark_github_xxxxxx() {
    timeit("github_xxxxxx", || {
        main_template(solutions::github_xxxxxxx::github_xxxxxx_solution)
    });
}
