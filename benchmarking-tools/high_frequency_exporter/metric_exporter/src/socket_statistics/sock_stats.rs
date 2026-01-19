use eyre::Result;
use std::io::Error;
use std::mem::{size_of, zeroed};

use crate::socket_statistics::tcp_info::TcpInfo;

pub struct SocketStatistics {
    result: TcpInfo,
    fd: Option<i32>,
    source_port: u16,
    destination_port: u16,
}

impl SocketStatistics {
    pub fn new(source_port: u16, destination_port: u16) -> Self {
        SocketStatistics {
            fd: None,
            result: unsafe { zeroed() },
            source_port,
            destination_port,
        }
    }

    fn update(&mut self) -> Result<()> {
        let mut len: libc::socklen_t = size_of::<TcpInfo>() as libc::socklen_t;

        let fd = match self.fd {
            Some(fd) => fd,
            None => {
                self.update_fd()?;
                self.fd.unwrap()
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
                return Err(Error::last_os_error().into());
            }
        }

        // This is not required but will avoid us mistakingly reading stats from a non-established socket
        if self.result.tcpi_state != 1 {
            return Err(eyre::eyre!(
                "Socket is not in ESTABLISHED state, found state {}",
                self.result.tcpi_state
            ));
        }

        Ok(())
    }

    pub fn update_fd(&mut self) -> Result<()> {
        // Find PID and FD of iperf3 connecting 4444 -> 5201 using lsof
        let lsof_command = format!(
            "lsof -P -n -i :{} | grep iperf3 | grep :{} | awk '{{print $2, $4}}' | head -n 1",
            self.source_port, self.destination_port
        );

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&lsof_command)
            .output()?;

        let s = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(eyre::eyre!(
                "Failed to find iperf3 process with ports {} -> {}",
                self.source_port,
                self.destination_port
            ));
        }

        let pid: i32 = parts[0].parse()?;
        let target_fd_str = parts[1].trim_matches(|c: char| !c.is_numeric());
        let target_fd: i32 = target_fd_str.parse()?;

        // Open a file descriptor referring to the process
        let pidfd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0) };
        if pidfd < 0 {
            return Err(eyre::eyre!(
                "pidfd_open failed for PID {}: {}",
                pid,
                Error::last_os_error()
            ));
        }

        // Duplicate the target FD into our process
        let stolen_fd = unsafe { libc::syscall(libc::SYS_pidfd_getfd, pidfd, target_fd, 0) };
        if stolen_fd < 0 {
            return Err(eyre::eyre!(
                "pidfd_getfd failed for PID {} FD {}: {}",
                pid,
                target_fd,
                Error::last_os_error()
            ));
        }

        unsafe { libc::close(pidfd as i32) };
        println!(
            "Stealing FD {} from PID {} -> Local FD {}",
            target_fd, pid, stolen_fd
        );

        self.fd = Some(stolen_fd.try_into()?);

        Ok(())
    }

    pub fn get_socket_infos(&mut self) -> Result<&TcpInfo> {
        self.update()?;
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
