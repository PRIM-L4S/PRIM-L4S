use std::env;
use std::process::Command;
use std::time::Duration;
use std::{thread, time};

use chrono::Local;

use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::path::Path;

use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

const RUN_TIME: Duration = Duration::from_secs(120);

#[derive(Debug, thiserror::Error)]
enum RunnerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Command \"{0}\" failed to execute: {1}")]
    ExecutionError(String, String),
}

fn run_scenario(scenario: &str, wait_time: Duration) -> Result<(), RunnerError> {
    //using make up SCENARIO=scenario
    let output = Command::new("make")
        .current_dir("../../docker-testbed")
        .args(["up", &format!("SCENARIO={}", scenario)])
        .output()?;
    if !output.status.success() {
        Err(RunnerError::ExecutionError(
            "make up".into(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))?
    }

    thread::sleep(wait_time);

    clean_up()?;

    Ok(())
}

fn clean_up() -> Result<(), RunnerError> {
    let output = Command::new("make")
        .current_dir("../../docker-testbed")
        .args(["down"])
        .output()?;
    if !output.status.success() {
        Err(RunnerError::ExecutionError(
            "make down".into(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))?
    }
    Ok(())
}

fn main() -> Result<(), RunnerError> {
    //opening results.csv for writing results
    let path = Path::new("results.csv");

    let file_exists = path.exists();

    let file = OpenOptions::new().create(true).append(true).open(path)?;

    let mut wtr = WriterBuilder::new().from_writer(file);

    if !file_exists {
        wtr.write_record(["Scenario", "Launch time", "End time", "Description"])?;
    }

    clean_up()?;

    //running all scenarii
    for scenario in env::args().skip(1) {
        println!("Running scenario {}...", scenario);

        let file = File::open(format!("../../docker-testbed/scenarii/{}.json", scenario))?;
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader)?;

        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Measure launch time
        let start_time = Local::now();

        run_scenario(&scenario, RUN_TIME)?;

        let end_time = Local::now();

        wtr.write_record(&[
            scenario,
            format!("{}", start_time.to_rfc3339()),
            format!("{}", end_time.to_rfc3339()),
            desc,
        ])?;

        wtr.flush()?;

        thread::sleep(time::Duration::from_secs(5));
    }

    wtr.flush()?;
    Ok(())
}
