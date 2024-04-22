use memmap::MmapOptions;
use std::fs::File;
use std::fmt::Display;
use rustc_hash::FxHashMap as HashMap;
use bstr::{BStr, ByteSlice};
use rayon::prelude::*;

#[derive(Debug)]
struct WeatherData {
    min: f64,
    max: f64,
    count: u64,
    sum: f64,
}

impl Default for WeatherData {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            count: 0,
            sum: 0.0,
        }
    }
}

impl Display for WeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.sum / (self.count as f64);
        write!(f, "{:.1}/{avg:.1}/{:.1}", self.min, self.max)
    }
}

impl WeatherData {
    fn update(&mut self, v: f64) {
        self.min = self.min.min(v);
        self.max = self.max.max(v);
        self.count += 1;
        self.sum += v;
    }

    fn merge(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.count += other.count;
        self.sum += other.sum;
    }
}

fn make_map<'a>(i: impl Iterator<Item = &'a [u8]>) -> HashMap<&'a BStr, WeatherData> {
    let mut data: HashMap<&'a BStr, WeatherData> = Default::default();
    for line in i {
        let (name, value) = line.split_once_str(&[b';']).unwrap();
        let result = fast_float::parse(value);
        match result {
            Ok(value) => data.entry(name.into()).or_default().update(value),
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    data
}

fn solve_for_part((start, end): (usize, usize), mem: &[u8]) -> HashMap<&BStr, WeatherData> {
    make_map((&mem[start..end]).lines())
}

fn merge<'a>(a: &mut HashMap<&'a BStr, WeatherData>, b: &HashMap<&'a BStr, WeatherData>) {
    for (k, v) in b {
        a.entry(k).or_default().merge(v);
    }
}

pub fn cnk_solution(path: std::path::PathBuf) {
    let cores: usize = std::thread::available_parallelism().unwrap().into();
    let file = File::open(path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let chunk_size = mmap.len() / cores;
    let mut chunks: Vec<(usize, usize)> = vec![];
    let mut start = 0;
    for _ in 0..cores {
        let end = (start + chunk_size).min(mmap.len());
        let next_new_line = match memchr::memchr(b'\n', &mmap[end..]) {
            Some(v) => v,
            None => {
                assert_eq!(end, mmap.len());
                0
            }
        };
        let end = end + next_new_line;
        chunks.push((start, end));
        start = end + 1;
    }
    let parts: Vec<_> = chunks
        .par_iter()
        .map(|r| solve_for_part(*r, &mmap))
        .collect();

    let data: HashMap<&BStr, WeatherData> = parts.into_iter().fold(Default::default(), |mut a, b| {
        merge(&mut a, &b);
        a
    });
    
    let mut all: Vec<_> = data.into_iter().collect();
    all.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    print!("{{");
    for (i, (name, data)) in all.into_iter().enumerate() {
        if i == 0 {
            print!("{name}={data}");
        } else {
            print!(", {name}={data}");
        }
    }
    println!("}}");
}
