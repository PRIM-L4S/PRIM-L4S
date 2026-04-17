use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::constants::{RUN_TIME, SCENARIO_SETUP_TIME, TIME_BETWEEN_SCENARIOS};

pub struct ProgressUi {
    overall_bar: ProgressBar,
    current_task_bar: ProgressBar,
}

pub enum ProgressBarStyle {
    Overall,
    Running,
    Waiting,
    Spinner,
}

impl ProgressUi {
    pub fn new(total: u64, initial_message: impl Into<String>) -> Self {
        let multi = MultiProgress::new();
        multi.set_move_cursor(true);

        let overall = multi.add(ProgressBar::new(total));
        overall.set_style(get_progress_style(ProgressBarStyle::Overall));
        overall.set_position(0);
        overall.set_message(initial_message.into());

        let current_task_bar = multi.insert_after(&overall, ProgressBar::new(1));

        Self {
            overall_bar: overall,
            current_task_bar,
        }
    }

    pub fn overall(&self) -> &ProgressBar {
        &self.overall_bar
    }

    pub fn current_task(&self) -> &ProgressBar {
        &self.current_task_bar
    }

    pub fn spinner(&self, message: impl Into<String>) {
        self.current_task_bar
            .set_style(get_progress_style(ProgressBarStyle::Spinner));
        self.current_task_bar.set_message(message.into());
        self.current_task_bar
            .enable_steady_tick(Duration::from_millis(100));
    }

    pub fn sleep_with_progress(
        &self,
        duration: Duration,
        message: impl Into<String>,
        style: ProgressBarStyle,
    ) {
        // Disables the spinner, if it was active
        self.current_task_bar.disable_steady_tick();

        self.current_task_bar.set_position(0);

        self.current_task_bar.set_length(duration.as_secs());
        self.current_task_bar.set_style(get_progress_style(style));
        self.current_task_bar.set_message(message.into());

        for _ in 0..duration.as_secs() {
            thread::sleep(Duration::from_secs(1));
            self.current_task_bar.inc(1);
        }
    }

    pub fn finish(&self, message: impl Into<String>) {
        self.current_task_bar.finish_and_clear();
        self.overall_bar.finish_with_message(message.into());
    }
}

fn get_progress_style(style: ProgressBarStyle) -> ProgressStyle {
    match style {
        ProgressBarStyle::Running => ProgressStyle::with_template(
            " [{wide_bar:.green/blue}] {msg} {pos}/{len}s ETA {eta_precise}",
        )
        .unwrap()
        .progress_chars("=>-"),
        ProgressBarStyle::Waiting => {
            ProgressStyle::with_template(" [{wide_bar:.yellow/blue}] {msg} {pos}/{len}s elapsed")
                .unwrap()
                .progress_chars("=>-")
        }
        ProgressBarStyle::Overall => ProgressStyle::with_template(
            " [{wide_bar:.cyan/blue}] {msg} {pos}/{len} elapsed {elapsed_precise} ETA {eta_custom}",
        )
        .unwrap()
        .with_key("eta_custom", compute_eta_custom)
        .progress_chars("#>-"),
        ProgressBarStyle::Spinner => ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    }
}

fn format_duration_precise(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn compute_eta_custom(state: &indicatif::ProgressState, writer: &mut dyn std::fmt::Write) {
    let remaining_elements = state.len().unwrap_or(0).saturating_sub(state.pos());
    let remaining_time =
        (remaining_elements as u32) * (SCENARIO_SETUP_TIME + RUN_TIME + TIME_BETWEEN_SCENARIOS);

    let _ = writer.write_str(&format_duration_precise(remaining_time));
}
