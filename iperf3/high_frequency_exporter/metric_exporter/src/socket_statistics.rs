use std::{collections::HashMap, fmt, num::ParseIntError};

use tokio::process::Command;

#[derive(Debug)]
pub struct SocketStatistics {
    // Could easily be added, but not needed for now

    // pub sender_address: String,
    // pub destination_address: String,
    // pub state: String,
    pub recv_q: u64,
    pub send_q: u64,
    pub statistics: HashMap<String, String>,
}

impl SocketStatistics {
    pub fn get_u64_statistic(&self, key: &str) -> Result<u64, SockStatError> {
        match self.statistics.get(key) {
            Some(value_str) => value_str
                .parse::<u64>()
                .map_err(|op: ParseIntError| SockStatError::Other(eyre::eyre!(op))),
            None => Err(SockStatError::ParsingError(format!(
                "Statistic '{}' not found",
                key
            ))),
        }
    }
}

pub enum SockStatError {
    NoMatchingSocket,
    TooManyMatchingSockets,
    ParsingError(String),
    Other(eyre::Error),
}

impl fmt::Display for SockStatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SockStatError::NoMatchingSocket => write!(f, "No matching socket found"),
            SockStatError::TooManyMatchingSockets => {
                write!(f, "Multiple matching sockets found")
            }
            SockStatError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
            SockStatError::Other(err) => write!(f, "Other error: {}", err),
        }
    }
}

/// Runs the ss command and parses its output to gather socket statistics.
pub async fn get_socket_statistics(
    sender_port: u16,
    destination_port: u16,
) -> Result<SocketStatistics, SockStatError> {
    let output = Command::new("ss")
        .arg("--info") // Show internal TCP information
        .arg("--tcp") // TCP sockets only (might need to change this)
        .arg("--numeric") // Do not try to resolve service names. Show exact bandwidth values, instead of human-readable.
        .arg("--no-header") // Do not print the header line
        .arg("--oneline") // One line per socket
        .arg(format!(
            "sport = :{} and dport = :{}",
            sender_port, destination_port
        )) // Filter for iperf3 default port
        .output()
        .await
        .map_err(|err| SockStatError::Other(err.into()))?;

    let output = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if output.is_empty() {
        return Err(SockStatError::NoMatchingSocket);
    }

    if output.contains('\n') {
        return Err(SockStatError::TooManyMatchingSockets);
    }

    let parts: Vec<&str> = output.split_whitespace().collect();

    if parts.len() < 5 {
        return Err(SockStatError::ParsingError(
            "Unexpected output format from ss command".to_string(),
        ));
    }

    let mut statistics: HashMap<String, String> = HashMap::new();

    for part in &parts[5..] {
        // By doing so, we loose some data such as the pacing_rate
        // which for some reason isn't separated by a ':' but by a space.
        if let Some((key, value)) = part.split_once(':') {
            statistics.insert(key.to_string(), value.to_string());
        }
    }

    Ok(SocketStatistics {
        // sender_address: parts[3].to_string(),
        // destination_address: parts[4].to_string(),
        // state: parts[0].to_string(),
        recv_q: parts[1]
            .parse()
            .map_err(|op: ParseIntError| SockStatError::Other(eyre::eyre!(op)))?,
        send_q: parts[2]
            .parse()
            .map_err(|op: ParseIntError| SockStatError::Other(eyre::eyre!(op)))?,
        statistics,
    })
}
