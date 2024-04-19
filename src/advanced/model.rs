pub struct WeatherData {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub count: usize,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: 0.0,
            count: 0,
        }
    }

    pub fn add_measurement(&mut self, measurement: f64) {
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);
        self.sum += measurement;
        self.count += 1;
    }

    pub fn calculate_mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}
