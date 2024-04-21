use std::time::SystemTime;

pub fn timeit<F: Fn() -> T, T>(name: &'static str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("> {name} | Solution took {} seconds", duration.as_secs());
    result
}

pub fn get_time<F: Fn() -> T, T>(name: &'static str, f: F) -> u64 {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("> {name} | Solution took {} seconds", duration.as_secs());
    duration.as_secs()
}
