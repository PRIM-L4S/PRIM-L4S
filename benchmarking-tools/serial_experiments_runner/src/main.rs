use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{env, io};

use chrono::Local;

use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::path::Path;

use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};

const TIME_BETWEEN_SCENARIOS: Duration = Duration::from_secs(5);
const RUN_TIME: Duration = Duration::from_secs(120);
const MAX_UP_RETRIES: usize = 10;
const UP_RETRY_WAIT: Duration = Duration::from_secs(10);

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

struct ScenarioExecution {
    start_time: chrono::DateTime<Local>,
    end_time: chrono::DateTime<Local>,
}

fn run_scenario(scenario: &str) -> Result<ScenarioExecution, RunnerError> {
    print!("Scenario {}: Starting... ", scenario);
    io::stdout().flush()?;

    for attempt in 1..=MAX_UP_RETRIES {
        let start_time = Local::now();

        let output = Command::new("make")
            .current_dir("../../docker-testbed")
            .args(["up", &format!("SCENARIO={}", scenario)])
            .output()?;

        if output.status.success() {
            print!("Running... ");
            io::stdout().flush()?;
            thread::sleep(RUN_TIME);
            clean_up()?;
            println!("Finished and cleaned.");

            let end_time = Local::now();

            return Ok(ScenarioExecution {
                start_time,
                end_time,
            });
        }

        let combined_output = format!(
            "stdout:\n{}\n\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        if attempt < MAX_UP_RETRIES {
            eprintln!(
                "\n\nScenario {}: make up failed (attempt {}/{}). Retrying in {}s...\nError details:\n{}",
                scenario,
                attempt,
                MAX_UP_RETRIES,
                UP_RETRY_WAIT.as_secs(),
                combined_output
            );

            clean_up()?;
            thread::sleep(UP_RETRY_WAIT);
        }
    }

    Err(RunnerError::ExecutionError(
        "make up".into(),
        "Exhausted retry attempts for make up".into(),
    ))
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

    print!("Cleaning up before starting... ");
    io::stdout().flush()?;

    clean_up()?;

    println!("Done.");

    //running all scenarii
    for scenario in env::args().skip(1) {
        let file = File::open(format!("../../docker-testbed/scenarii/{}.json", scenario))?;
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader)?;

        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let scenario_execution = run_scenario(&scenario)?;

        wtr.write_record(&[
            scenario,
            format!("{}", scenario_execution.start_time.to_rfc3339()),
            format!("{}", scenario_execution.end_time.to_rfc3339()),
            desc,
        ])?;

        wtr.flush()?;

        thread::sleep(TIME_BETWEEN_SCENARIOS);
    }

    wtr.flush()?;
    Ok(())
}
