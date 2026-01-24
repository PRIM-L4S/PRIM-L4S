use std::fmt;
use std::io::Error;
use std::mem::size_of;
use std::os::fd::{FromRawFd, OwnedFd};
use tokio::process::Command;

use crate::socket_statistics::tcp_info::TcpInfo;

pub struct SocketStatistics {
    source_port: u16,
    destination_port: u16,
    result: TcpInfo,
    fd: Option<i32>,
}

pub enum SockStatError {
    SocketNotEstablished(u8),
    NoMatchingSocket,
    TooManyMatchingSockets,
    Other(eyre::Error),
}

impl fmt::Display for SockStatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SockStatError::NoMatchingSocket => write!(f, "No matching socket found"),
            SockStatError::SocketNotEstablished(state) => {
                write!(
                    f,
                    "Socket is not in ESTABLISHED state, current state: {}",
                    state
                )
            }
            SockStatError::TooManyMatchingSockets => {
                write!(f, "Multiple matching sockets found")
            }
            SockStatError::Other(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl SocketStatistics {
    pub fn new(source_port: u16, destination_port: u16) -> Self {
        SocketStatistics {
            source_port,
            destination_port,
            fd: None,
            result: Default::default(),
        }
    }

    async fn update(&mut self) -> Result<(), SockStatError> {
        let mut len: libc::socklen_t = size_of::<TcpInfo>() as libc::socklen_t;

        let fd = match self.fd {
            Some(fd) => fd,
            None => {
                self.update_fd().await?;
                self.fd.ok_or(SockStatError::Other(eyre::eyre!(
                    "The file descriptor of SocketStatistics was in an incorrect state"
                )))?
            }
        };

        unsafe {
            let rc = libc::getsockopt(
                fd,
                libc::IPPROTO_TCP,
                libc::TCP_INFO,
                &mut self.result as *mut _ as *mut libc::c_void,
                &mut len as *mut libc::socklen_t,
            );

            if rc < 0 {
                self.drop_fd();
                return Err(SockStatError::Other(eyre::eyre!(
                    "getsockopt TCP_INFO failed: {}",
                    Error::last_os_error()
                )));
            }
        }

        if self.result.tcpi_state == 7 || self.result.tcpi_state == 8 {
            self.drop_fd();
            return Err(SockStatError::SocketNotEstablished(self.result.tcpi_state));
        } else if self.result.tcpi_state != 1 && self.result.tcpi_state != 2 {
            return Err(SockStatError::SocketNotEstablished(self.result.tcpi_state));
        }

        Ok(())
    }

    pub async fn update_fd(&mut self) -> Result<(), SockStatError> {
        // Find PID and FD of iperf3 connecting source_port -> destination_port using lsof
        // TODO: Replace this with a more efficient method
        let lsof_command = format!(
            "lsof -P -n -i :{} | grep iperf3 | grep :{} | awk '{{print $2, $4}}' | head -n 1",
            self.source_port, self.destination_port
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&lsof_command)
            .output()
            .await
            .map_err(|e| SockStatError::Other(eyre::eyre!("Failed to execute lsof: {}", e)))?;

        let s = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(SockStatError::NoMatchingSocket);
        }
        if parts.len() > 2 {
            return Err(SockStatError::TooManyMatchingSockets);
        }

        let pid: i32 = parts[0].parse().map_err(|e| {
            SockStatError::Other(eyre::eyre!("Failed to parse PID from lsof output: {}", e))
        })?;
        let target_fd_str = parts[1].trim_matches(|c: char| !c.is_numeric());
        let target_fd: i32 = target_fd_str.parse().map_err(|e| {
            SockStatError::Other(eyre::eyre!("Failed to parse FD from lsof output: {}", e))
        })?;

        // Open a file descriptor referring to the process
        let pidfd = unsafe { OwnedFd::from_raw_fd(libc::syscall(libc::SYS_pidfd_open, pid, 0) as i32) };
        if pidfd.into() < 0 {
            return Err(SockStatError::Other(eyre::eyre!(
                "pidfd_open failed for PID {}: {}",
                pid,
                Error::last_os_error()
            )));
        }

        // Duplicate the target FD into our process
        let stolen_fd = unsafe { libc::syscall(libc::SYS_pidfd_getfd, pidfd.into(), target_fd, 0) };
        if stolen_fd < 0 {
            return Err(SockStatError::Other(eyre::eyre!(
                "pidfd_getfd failed for PID {} FD {}: {}",
                pid,
                target_fd,
                Error::last_os_error()
            )));
        }
        drop(pidfd);

        self.fd = Some(stolen_fd.try_into().map_err(|e| {
            SockStatError::Other(eyre::eyre!("Failed to convert stolen FD to i32: {}", e))
        })?);

        Ok(())
    }

    fn drop_fd(&mut self) {
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd) };
            self.fd = None;
        }
    }

    /// Retrieves the latest socket statistics
    ///
    /// This function should return quickly most of the time
    /// But takes longer the first time the socket is accessed
    ///
    /// The asynchronousness is only used when needing to update the FD
    pub async fn get_socket_infos(&mut self) -> Result<&TcpInfo, SockStatError> {
        self.update().await?;
        Ok(&self.result)
    }
}

impl Drop for SocketStatistics {
    fn drop(&mut self) {
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd) };
        }
    }
}
