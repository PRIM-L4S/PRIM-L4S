use std::fmt;
use std::mem::size_of;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

use crate::socket_statistics::tcp_info::TcpInfo;

pub struct SocketStatistics {
    source_port: u16,
    destination_port: u16,
    result: TcpInfo,
    fd: Option<OwnedFd>,
    process_name: String,
}

#[derive(Debug)]
pub enum SockStatError {
    SocketNotEstablished(u8),
    NoMatchingSocket(String),
    Other(eyre::Error),
}

impl std::error::Error for SockStatError {}

impl fmt::Display for SockStatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SockStatError::NoMatchingSocket(info) => {
                write!(f, "No matching socket found: {}", info)
            }
            SockStatError::SocketNotEstablished(state) => {
                write!(
                    f,
                    "Socket is not in ESTABLISHED state, current state: {}",
                    state
                )
            }
            SockStatError::Other(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl SocketStatistics {
    pub fn new(source_port: u16, destination_port: u16, process_name: String) -> Self {
        SocketStatistics {
            source_port,
            destination_port,
            fd: None,
            result: Default::default(),
            process_name,
        }
    }

    async fn update(&mut self) -> Result<(), SockStatError> {
        let mut len: libc::socklen_t = size_of::<TcpInfo>() as libc::socklen_t;

        let fd = match &self.fd {
            Some(fd) => fd,
            None => {
                self.update_fd().await?;
                self.fd.as_ref().ok_or(SockStatError::Other(eyre::eyre!(
                    "The file descriptor of SocketStatistics was in an incorrect state"
                )))?
            }
        };

        let rc = unsafe {
            libc::getsockopt(
                fd.as_raw_fd(),
                libc::IPPROTO_TCP,
                libc::TCP_INFO,
                &mut self.result as *mut _ as *mut libc::c_void,
                &mut len as *mut libc::socklen_t,
            )
        };

        if rc < 0 {
            self.fd = None;
            return Err(SockStatError::Other(eyre::eyre!(
                "getsockopt TCP_INFO failed: {}",
                std::io::Error::last_os_error()
            )));
        }

        if self.result.tcpi_state == 7 || self.result.tcpi_state == 8 {
            self.fd = None;
            return Err(SockStatError::SocketNotEstablished(self.result.tcpi_state));
        } else if self.result.tcpi_state != 1 && self.result.tcpi_state != 2 {
            return Err(SockStatError::SocketNotEstablished(self.result.tcpi_state));
        }

        Ok(())
    }

    /// Searches /proc/net/tcp or /proc/net/tcp6 for a socket matching the source and destination ports
    ///
    /// Returns the inode number of the matching socket if found
    async fn find_inode_in_proc(&self, path: &str) -> Option<u64> {
        let content = tokio::fs::read_to_string(path).await.ok()?;
        for line in content.lines().skip(1) {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 10 {
                continue;
            }

            let local_ip_port = cols[1];
            let remote_ip_port = cols[2];
            let inode_str = cols[9];
            let inode = match inode_str.parse::<u64>() {
                Err(_) => continue,
                Ok(0) => continue,
                Ok(i) => i,
            };

            let parse_port = |s: &str| -> Option<u16> {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() != 2 {
                    return None;
                }
                u16::from_str_radix(parts[1], 16).ok()
            };

            let local_port = parse_port(local_ip_port)?;
            let remote_port = parse_port(remote_ip_port)?;

            if (local_port == self.source_port && remote_port == self.destination_port)
                || (local_port == self.destination_port && remote_port == self.source_port)
            {
                return Some(inode);
            }
        }

        None
    }

    /// Searches /proc for a process with the given name that has a socket with the given inode
    ///
    /// Returns the (PID, FD) tuple if found
    async fn find_pid_fd(&self, inode: u64) -> Result<(i32, i32), SockStatError> {
        let mut entries = tokio::fs::read_dir("/proc")
            .await
            .map_err(|e| SockStatError::Other(eyre::eyre!("Failed to read /proc: {}", e)))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let file_name = entry.file_name();
            let pid_str = file_name.to_string_lossy();

            if let Ok(pid) = pid_str.parse::<i32>() {
                // Check if process name matches
                let comm_path = format!("/proc/{}/comm", pid);
                if let Ok(comm) = tokio::fs::read_to_string(&comm_path).await {
                    if comm.trim() != self.process_name {
                        continue;
                    }
                } else {
                    continue;
                }

                // Scan FDs
                let fd_path = format!("/proc/{}/fd", pid);
                let mut fd_entries = match tokio::fs::read_dir(&fd_path).await {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                while let Ok(Some(fd_entry)) = fd_entries.next_entry().await {
                    if let Ok(target) = tokio::fs::read_link(fd_entry.path()).await {
                        let target_str = target.to_string_lossy();

                        if target_str == format!("socket:[{}]", inode) {
                            if let Ok(fd) = fd_entry.file_name().to_string_lossy().parse::<i32>() {
                                return Ok((pid, fd));
                            }
                        }
                    }
                }
            }
        }
        Err(SockStatError::NoMatchingSocket(format!(
            "No process named '{}' with socket inode {} found",
            self.process_name, inode
        )))
    }

    /// Updates the internal file descriptor to point to the target socket
    ///
    /// This function is automatically called by get_socket_infos() when needed
    /// But can also be called manually at any time
    /// The result is cached until the socket is closed
    pub async fn update_fd(&mut self) -> Result<(), SockStatError> {
        // Find inode first
        let inode = if let Some(i) = self.find_inode_in_proc("/proc/net/tcp").await {
            i
        } else if let Some(i) = self.find_inode_in_proc("/proc/net/tcp6").await {
            i
        } else {
            return Err(SockStatError::NoMatchingSocket(format!(
                "No matching socket inode found in /proc/net/tcp or /proc/net/tcp6"
            )));
        };

        let (pid, target_fd) = self.find_pid_fd(inode).await?;

        // Open a file descriptor referring to the process
        let pidfd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0) };
        if pidfd < 0 {
            return Err(SockStatError::Other(eyre::eyre!(
                "pidfd_open failed for PID {}: {}",
                pid,
                std::io::Error::last_os_error()
            )));
        }
        let pidfd = unsafe { OwnedFd::from_raw_fd(pidfd as i32) };

        // Duplicate the target FD into our process
        let stolen_fd =
            unsafe { libc::syscall(libc::SYS_pidfd_getfd, pidfd.as_raw_fd(), target_fd, 0) };
        if stolen_fd < 0 {
            return Err(SockStatError::Other(eyre::eyre!(
                "pidfd_getfd failed for PID {} FD {}: {}",
                pid,
                target_fd,
                std::io::Error::last_os_error()
            )));
        }
        let stolen_fd = unsafe { OwnedFd::from_raw_fd(stolen_fd as i32) };
        drop(pidfd);

        self.fd = Some(stolen_fd);

        Ok(())
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
