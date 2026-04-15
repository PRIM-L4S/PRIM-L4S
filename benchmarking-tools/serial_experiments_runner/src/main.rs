use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

use chrono::Local;

use csv::WriterBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::path::Path;

use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

const TIME_BETWEEN_SCENARIOS: Duration = Duration::from_secs(5);
const RUN_TIME: Duration = Duration::from_secs(15);
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

fn spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.green} {msg}")
        .unwrap()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
}

fn wait_with_progress(duration: Duration, message: &str) {
    let wait_bar = ProgressBar::new(duration.as_secs());
    let wait_style =
        ProgressStyle::with_template(" [{wide_bar:.yellow/blue}] {msg} {pos}/{len}s elapsed")
            .unwrap()
            .progress_chars("=>-");

    wait_bar.set_style(wait_style);
    wait_bar.set_message(message.to_string());

    for _ in 0..duration.as_secs() {
        thread::sleep(Duration::from_secs(1));
        wait_bar.inc(1);
    }

    wait_bar.finish_and_clear();
}

fn run_scenario(
    scenario: &str,
    overall_progress: &ProgressBar,
) -> Result<ScenarioExecution, RunnerError> {
    for attempt in 1..=MAX_UP_RETRIES {
        let launch_spinner = ProgressBar::new_spinner();
        launch_spinner.set_style(spinner_style());
        launch_spinner.set_message(format!(
            "Launching scenario {} (attempt {}/{})",
            scenario, attempt, MAX_UP_RETRIES
        ));
        launch_spinner.enable_steady_tick(Duration::from_millis(120));

        let start_time = Local::now();

        let output = Command::new("make")
            .current_dir("../../docker-testbed")
            .args(["up", &format!("SCENARIO={}", scenario)])
            .output()?;

        if output.status.success() {
            launch_spinner.finish_with_message(format!("Scenario {} is up", scenario));

            let run_bar = ProgressBar::new(RUN_TIME.as_secs());
            let run_style = ProgressStyle::with_template(
                " [{wide_bar:.green/blue}] {msg} {pos}/{len}s ETA {eta_precise}",
            )
            .unwrap()
            .progress_chars("=>-");
            run_bar.set_style(run_style);
            run_bar.set_message(format!("Running scenario {}", scenario));

            for _ in 0..RUN_TIME.as_secs() {
                thread::sleep(Duration::from_secs(1));
                run_bar.inc(1);
            }
            run_bar.finish_and_clear();

            let cleanup_spinner = ProgressBar::new_spinner();
            cleanup_spinner.set_style(spinner_style());
            cleanup_spinner.set_message(format!("Cleaning up scenario {}", scenario));
            cleanup_spinner.enable_steady_tick(Duration::from_millis(120));
            clean_up()?;
            cleanup_spinner.finish_with_message(format!("Scenario {} cleaned", scenario));

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
            launch_spinner.abandon_with_message(format!(
                "Scenario {} failed to launch on attempt {}/{}",
                scenario, attempt, MAX_UP_RETRIES
            ));

            overall_progress.println(format!(
                "Scenario {}: make up failed (attempt {}/{}). Retrying in {}s...\nError details:\n{}",
                scenario,
                attempt,
                MAX_UP_RETRIES,
                UP_RETRY_WAIT.as_secs(),
                combined_output
            ));

            clean_up()?;
            wait_with_progress(
                UP_RETRY_WAIT,
                &format!("Waiting before retrying scenario {}", scenario),
            );
        } else {
            launch_spinner.abandon_with_message(format!(
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

    let mut wtr = WriterBuilder::new().from_writer(file);

    if !file_exists {
        wtr.write_record(["Scenario", "Launch time", "End time", "Description"])?;
    }

    let startup_cleanup_spinner = ProgressBar::new_spinner();
    startup_cleanup_spinner.set_style(spinner_style());
    startup_cleanup_spinner.set_message("Cleaning up before starting".to_string());
    startup_cleanup_spinner.enable_steady_tick(Duration::from_millis(120));
    clean_up()?;
    startup_cleanup_spinner.finish_with_message("Initial cleanup done");

    let scenarios = env::args().skip(1).collect::<Vec<String>>();

    let progress_bar = ProgressBar::new(scenarios.len() as u64);

    let sty = ProgressStyle::with_template(
        " [{wide_bar:.cyan/blue}] {msg} {pos}/{len} elapsed {elapsed_precise} ETA {eta_precise}",
    )
    .unwrap()
    .progress_chars("#>-");

    progress_bar.set_style(sty);
    progress_bar.set_message("Starting...");

    //running all scenarii
    for scenario in scenarios {
        progress_bar.set_message(format!("Running {}", scenario));

        let file = File::open(format!("../../docker-testbed/scenarii/{}.json", scenario))?;
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader)?;

        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let scenario_execution = run_scenario(&scenario, &progress_bar)?;

        wtr.write_record(&[
            scenario.clone(),
            scenario_execution.start_time.to_rfc3339(),
            scenario_execution.end_time.to_rfc3339(),
            desc,
        ])?;

        wtr.flush()?;

        progress_bar.inc(1);

        if progress_bar.position() < progress_bar.length().unwrap_or(0) {
            wait_with_progress(
                TIME_BETWEEN_SCENARIOS,
                &format!("Cooldown before next scenario after {}", scenario),
            );
        }
    }

    progress_bar.finish_with_message("All scenarios completed");

    wtr.flush()?;
    Ok(())
}
