use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::io::Write;
use std::io::Read;
use ssh2::{Channel, Stream};
use tauri::{Window};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::ssh::ssh_instance::SshInstance;
use tauri::Emitter;

pub struct SshEngine(pub Arc<Mutex<HashMap<String, SshInstance>>>);

impl SshEngine {
    pub fn spawn_thread_write(rx: UnboundedReceiver<String>, channel : Channel){
        tauri::async_runtime::spawn_blocking(move || {
            let mut rx_mut = rx;
            let mut channel_mut = channel;
            while let Some(input) = rx_mut.blocking_recv() {
                if channel_mut.write_all(input.as_bytes()).is_err() {
                    break;
                }
                if channel_mut.flush().is_err() {
                    break;
                }
            }
            println!("Writer thread exited");
        });
    } 
    pub fn spawn_thread_read(hostname_for_read: String, reader: Stream, window_clone : Window) {
        tauri::async_runtime::spawn_blocking(move || {
        let mut buffer = [0u8; 4096];
        println!("Reader thread started for {}", hostname_for_read);
        let mut reader_mut = reader; 
        loop {
            match reader_mut.read(&mut buffer) {
                Ok(0) => {
                    println!("Stream closed");
                    break;
                } // Stream tertutup
                Ok(n) => {
                    let output = String::from_utf8_lossy(&buffer[..n]).to_string();
                    // Kirim output ke frontend via event
                    println!("Output: {}", output);
                    window_clone
                        .emit(
                            &format!("ssh-output-{}", hostname_for_read.replace(".", "-")),
                            output,
                        )
                        .ok();
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {

                        continue; // Coba baca lagi di iterasi berikutnya
                    } else {
                        println!("Reader error kritis: {:?}", e);
                        window_clone
                            .emit(
                                &format!(
                                    "ssh-error-output-{}",
                                    hostname_for_read.replace(".", "-")
                                ),
                                format!("Error: {}", e),
                            )
                            .ok();
                        break;
                    }
                }
            }
        }

        println!("Thread Exited, Cleaning up session");
    });
    }
}