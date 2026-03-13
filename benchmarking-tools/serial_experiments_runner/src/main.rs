use std::env;
use std::process::Command;
use std::{thread, time};

use chrono::Local;

use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::path::Path;

use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //opening results.csv for writing results
    let path = Path::new("results.csv");
    let file_exists = path.exists();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("results.csv")?;

    let mut wtr = WriterBuilder::new().from_writer(file);

    if !file_exists {
        wtr.write_record(["Scenario", "Description", "Launch time"])?;
    }

    //running all scenarii
    let scenarii: Vec<String> = env::args().skip(1).collect();

    for scenario in scenarii.iter() {
        println!("Running scenario {}...", scenario);

        if let Err(e) = (|| -> Result<(), Box<dyn std::error::Error>> {
            let file = File::open(format!("../../docker-testbed/scenarii/{}.json", scenario))?;
            let reader = BufReader::new(file);
            let json: Value = serde_json::from_reader(reader)?;

            let desc = json["description"].as_str().unwrap_or("").to_string();

            // Measure launch time
            let start_time = Local::now();

            //using make up SCENARIO=scenario
            let output = Command::new("make")
                .current_dir("../../docker-testbed")
                .args(["up", &format!("SCENARIO={}", scenario)])
                .output()?;
            if !output.status.success() {
                eprintln!(
                    "Error running scenario {}: {}",
                    scenario,
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            wtr.write_record(&[
                scenario.to_string(),
                desc,
                format!("{}", start_time.format("%d/%m/%Y %H:%M")),
            ])?;

            thread::sleep(time::Duration::from_secs(120));

            let output = Command::new("make")
                .current_dir("../../docker-testbed")
                .args(["down"])
                .output()?;
            if !output.status.success() {
                eprintln!(
                    "Error stopping scenario {}: {}",
                    scenario,
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            wtr.flush()?;
            Ok(())
        })() {
            eprintln!("Error with scenario {}: {}", scenario, e);
        }
    }
    Ok(())
}
