use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::{Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::WarpError;

pub struct PtyManager {
    processes: Vec<Arc<Mutex<PtyProcess>>>,
    active_process: Option<usize>,
}

pub struct PtyProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    pid: u32,
    command: String,
}

impl PtyManager {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            processes: Vec::new(),
            active_process: None,
        })
    }

    pub async fn spawn_shell(&mut self, shell_command: &str) -> Result<usize, WarpError> {
        let mut child = Command::new(shell_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let pid = child.id().unwrap_or(0);

        let process = PtyProcess {
            child,
            stdin,
            stdout,
            pid,
            command: shell_command.to_string(),
        };

        let process_id = self.processes.len();
        self.processes.push(Arc::new(Mutex::new(process)));
        self.active_process = Some(process_id);

        Ok(process_id)
    }

    pub async fn write_input(&mut self, input: &str) -> Result<(), WarpError> {
        if let Some(active_id) = self.active_process {
            if let Some(process_arc) = self.processes.get(active_id) {
                let mut process = process_arc.lock().await;
                if let Some(ref mut stdin) = process.stdin {
                    stdin.write_all(input.as_bytes()).await?;
                    stdin.flush().await?;
                }
            }
        }
        Ok(())
    }

    pub async fn read_output(&mut self) -> Result<String, WarpError> {
        if let Some(active_id) = self.active_process {
            if let Some(process_arc) = self.processes.get(active_id) {
                let mut process = process_arc.lock().await;
                if let Some(ref mut stdout) = process.stdout {
                    let mut buffer = [0; 4096];
                    match stdout.try_read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            return Ok(String::from_utf8_lossy(&buffer[..n]).to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(String::new())
    }

    pub async fn kill_process(&mut self, process_id: usize) -> Result<(), WarpError> {
        if let Some(process_arc) = self.processes.get(process_id) {
            let mut process = process_arc.lock().await;
            process.child.kill().await?;
        }
        Ok(())
    }

    pub fn get_active_process_id(&self) -> Option<usize> {
        self.active_process
    }

    pub async fn switch_to_process(&mut self, process_id: usize) -> Result<(), WarpError> {
        if process_id < self.processes.len() {
            self.active_process = Some(process_id);
        }
        Ok(())
    }
}
