use super::SocketStatistics;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tokio::net::TcpListener;

#[tokio::test]
async fn test_socket_statistics() {
    // Start a TCP listener on a random port to act as the server
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind listener");
    let local_addr = listener.local_addr().expect("Failed to get local addr");
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
         time.sleep(10)",
        server_port
    );

    let mut child = Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn python process");

    // Read the client port from python's stdout
    // This ensures the connection is established before we try to find it
    let child_stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(child_stdout);
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    let client_port: u16 = line.trim().parse().expect("Failed to parse client port");

    // Create SocketStatistics spying on "python3"
    let mut stats = SocketStatistics::new(client_port, server_port, "python3".to_string());
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
}
