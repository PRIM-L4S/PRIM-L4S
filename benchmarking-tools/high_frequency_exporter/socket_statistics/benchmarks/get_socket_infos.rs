use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Instant;
use tokio::net::TcpListener;

use eyre::Result;
use socket_statistics::SocketStatistics;

const NUMBER_OF_ITERATIONS: u32 = 1000;

#[tokio::main]
async fn main() -> Result<()> {
    // Start a TCP listener on a random port to act as the server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    let server_port = local_addr.port();

    // Start a python process that connects to this port
    // sends 42 bytes, then sleeps for a while
    let python_code = format!(
        "import socket, time, sys; \
         s = socket.socket(socket.AF_INET, socket.SOCK_STREAM); \
         s.connect(('127.0.0.1', {})); \
         print(s.getsockname()[1]); \
         sys.stdout.flush(); \
         s.send(b'x' * 42); \
         time.sleep(1000)",
        server_port
    );

    let mut child = Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .stdout(Stdio::piped())
        .spawn()?;

    // Read the client port from python's stdout
    // This ensures the connection is established before we try to find it
    let child_stdout = child
        .stdout
        .take()
        .ok_or(eyre::eyre!("Failed to open stdout"))?;
    let mut reader = BufReader::new(child_stdout);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let client_port: u16 = line.trim().parse()?;

    // Create SocketStatistics spying on "python3"
    let mut stats = SocketStatistics::new(client_port, server_port, "python3".to_string());

    // ================ Benchmarked Code ================

    stats.update_fd().await?;

    let start = Instant::now();

    for _ in 0..NUMBER_OF_ITERATIONS {
        let _ = stats.get_socket_infos().await;
    }

    let duration = start.elapsed();

    println!(
        "Average time per get_socket_infos(): {:?}",
        duration / NUMBER_OF_ITERATIONS
    );

    // ===================================================

    let info = stats.get_socket_infos().await;

    // Clean up child process
    child.kill().ok();

    match info {
        Err(err) => panic!("Failed to get socket infos: {}", err),
        Ok(info) => {
            assert_eq!(info.tcpi_state, 1, "Socket should be in ESTABLISHED state");
            assert!(
                info.tcpi_bytes_sent == 42,
                "Bytes sent should be equal to 42"
            );
        }
    }

    Ok(())
}
