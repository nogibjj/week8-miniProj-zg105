extern crate csv;
extern crate sys_info;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::time::Instant;
use std::process::Command;
use sys_info::mem_info;

pub fn calculate_median(values: &Vec<f64>) -> f64 {
    let mut sorted_values = values.clone();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted_values.len();
    if len % 2 == 1 {
        sorted_values[len / 2]
    } else {
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
    // Record the start time
    let start_time = Instant::now();
    // Load the CSV file
    let csv_file = "../data.csv"; // Update with your CSV file path
    let file = File::open(csv_file)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let mut values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let shape_leng: f64 = record[0].parse()?;
        values.push(shape_leng);
    }

    // Calculate and print the medians
    let median = calculate_median(&values);

    println!("Median: {}", median);

    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    println!("Memory Usage: {}%", mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0);
    println!("Time spent: {:?}", elapsed_time);
    // println!("Memory usage: {} bytes", std::mem::size_of::<f64>());

    Ok(())
}