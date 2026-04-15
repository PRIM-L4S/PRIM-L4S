use std::env;
use std::process::Command;
use std::time::Duration;

use chrono::Local;

use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::path::Path;

use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

mod progress;
use progress::ProgressUi;

use crate::progress::ProgressBarStyle;

const TIME_BETWEEN_SCENARIOS: Duration = Duration::from_secs(5);
const RUN_TIME: Duration = Duration::from_mins(4);
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

fn run_scenario(
    scenario: &str,
    progress_ui: &ProgressUi,
) -> Result<ScenarioExecution, RunnerError> {
    for attempt in 1..=MAX_UP_RETRIES {
        progress_ui.spinner(format!(
            "Launching scenario {} (attempt {}/{})",
            scenario, attempt, MAX_UP_RETRIES
        ));

        let start_time = Local::now();

        let output = Command::new("make")
            .current_dir("../../docker-testbed")
            .args(["up", &format!("SCENARIO={}", scenario)])
            .output()?;

        if output.status.success() {
            progress_ui.sleep_with_progress(RUN_TIME, "Running", ProgressBarStyle::Running);

            progress_ui.spinner(format!("Cleaning up scenario {}", scenario));
            clean_up()?;

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
            progress_ui.current_task().abandon_with_message(format!(
                "Scenario {} failed to launch on attempt {}/{}",
                scenario, attempt, MAX_UP_RETRIES
            ));

            progress_ui.overall().println(format!(
                "Scenario {}: make up failed (attempt {}/{}). Retrying in {}s...\nError details:\n{}",
                scenario,
                attempt,
                MAX_UP_RETRIES,
                UP_RETRY_WAIT.as_secs(),
                combined_output
            ));

            clean_up()?;
            progress_ui.sleep_with_progress(
                UP_RETRY_WAIT,
                format!("Waiting before retrying scenario {}", scenario),
                ProgressBarStyle::Waiting,
            );
        } else {
            progress_ui.current_task().abandon_with_message(format!(
                "Scenario {} failed to launch after {} attempts",
                scenario, MAX_UP_RETRIES
            ));
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

    let mut result_file = WriterBuilder::new().from_writer(file);

    if !file_exists {
        result_file.write_record(["Scenario", "Launch time", "End time", "Description"])?;
    }

    let scenarios = env::args().skip(1).collect::<Vec<String>>();
    let progress_ui = ProgressUi::new(scenarios.len() as u64, "Starting...");

    progress_ui.spinner("Cleaning up before starting");
    clean_up()?;

    //running all scenarii
    for scenario in scenarios {
        progress_ui
            .overall()
            .set_message(format!("Scenario {}", scenario));

        let file = File::open(format!("../../docker-testbed/scenarii/{}.json", scenario))?;
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader)?;

        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let scenario_execution = run_scenario(&scenario, &progress_ui)?;

        result_file.write_record(&[
            scenario.clone(),
            scenario_execution.start_time.to_rfc3339(),
            scenario_execution.end_time.to_rfc3339(),
            desc,
        ])?;

        result_file.flush()?;

        progress_ui.overall().inc(1);

        if progress_ui.overall().position() < progress_ui.overall().length().unwrap_or(0) {
            progress_ui.sleep_with_progress(
                TIME_BETWEEN_SCENARIOS,
                "Cooldown before next scenario",
                ProgressBarStyle::Waiting,
            );
        }
    }

    result_file.flush()?;

    progress_ui.finish("All scenarios completed");
    Ok(())
}
