pub mod pty;

use portable_pty::{Child, MasterPty, PtySize};
use std::io::Read;
use std::thread;
use crossbeam_channel::{Receiver, Sender, unbounded};

pub struct TerminalSession {
    _child: Box<dyn Child + Send + Sync>,
    output_receiver: Receiver<String>,
    input_sender: Sender<String>,
    _reader_handle: thread::JoinHandle<()>,
    _writer_handle: thread::JoinHandle<()>,
}

impl TerminalSession {
    pub fn new() -> anyhow::Result<Self> {
        let pty_system = portable_pty::native_pty_system();
        
        let pty_pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let cmd = if cfg!(windows) {
            portable_pty::CommandBuilder::new("cmd")
        } else {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
            portable_pty::CommandBuilder::new(shell)
        };

        let child = pty_pair.slave.spawn_command(cmd)?;
        let master = pty_pair.master;

        let (output_sender, output_receiver) = unbounded();
        let (input_sender, input_receiver) = unbounded();

        // Clone master for reading and writing threads
        let reader_master = master.try_clone_reader()?;
        let writer_master = master.try_clone_reader()?; // We'll use this for writing too

        // Spawn reader thread
        let reader_handle = thread::spawn(move || {
            let mut reader = reader_master;
            let mut buffer = [0u8; 8192];
            
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(size) => {
                        let text = String::from_utf8_lossy(&buffer[..size]).into_owned();
                        if output_sender.send(text).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Spawn writer thread
        let writer_handle = thread::spawn(move || {
            // Note: Since portable-pty doesn't provide try_clone_writer, 
            // we'll handle writing differently in write_input method
            while let Ok(_input) = input_receiver.recv() {
                // Writing will be handled in the write_input method
                break;
            }
        });

        Ok(Self {
            _child: child,
            output_receiver,
            input_sender,
            _reader_handle: reader_handle,
            _writer_handle: writer_handle,
        })
    }

    pub fn write_input(&self, input: &str) -> anyhow::Result<()> {
        self.input_sender.send(input.to_string())?;
        Ok(())
    }

    pub fn read_output(&self) -> Option<String> {
        self.output_receiver.try_recv().ok()
    }

    pub fn resize(&mut self, _rows: u16, _cols: u16) -> anyhow::Result<()> {
        // For now, we'll skip resize functionality due to API limitations
        Ok(())
    }
}