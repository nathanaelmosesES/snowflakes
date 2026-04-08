// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::Serialize;
use ssh2::Session;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::System;
use tauri::Emitter;

#[derive(Serialize)]
struct SystemStats {
    cpu_usage: u32,
    ram_usage: u32,
}

#[derive(Serialize)]
struct SshSession {}

struct MetricsState(Mutex<System>);
#[tauri::command]
fn get_system_stats(state: tauri::State<'_, MetricsState>) -> SystemStats {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage() as u32;
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let ram_usage = ((used_mem as f64 / total_mem as f64) * 100.0) as u32;

    SystemStats {
        cpu_usage,
        ram_usage,
    }
}
use std::io::Read;
use tokio::sync::mpsc;

struct SshInputState(Mutex<Option<mpsc::Sender<String>>>);

pub struct SshInstance {
    pub session: Session,
    // pub channel: Channel,
    pub tx: mpsc::UnboundedSender<String>,
}
pub struct SshState(pub Arc<Mutex<HashMap<String, SshInstance>>>);

#[tauri::command]
async fn start_ssh_session(
    window: tauri::Window,
    state: tauri::State<'_, SshState>,
    hostname: String,
    bastion: String,
    initial_password: String,
    initial_username: String,
) -> Result<String, String> {
    // 1. Buat Koneksi TCP & Session Baru
    let tcp = std::net::TcpStream::connect(format!("{}:22", bastion))
        .map_err(|e| format!("Gagal koneksi ke server: {}", e))?;

    let mut sess = ssh2::Session::new().map_err(|e| e.to_string())?;
    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| format!("Handshake gagal: {}", e.message()))?;
    // 2. Lakukan Autentikasi
    sess.userauth_password(&initial_username, &initial_password)
        .map_err(|e| format!("Login Gagal: {}", e.message()))?;

    let mut channel = sess.channel_session().map_err(|e| e.to_string())?;
    channel
        .request_pty("xterm", None, None)
        .map_err(|e| e.to_string())?;

    channel.exec(&hostname).unwrap();

    // 5. Setup MPSC untuk Interactive Password (Sudo/Target Host)
    let mut registry = state.0.lock().unwrap();
    let hostname_clone = hostname.clone();
    let mut reader = channel.stream(0);
    // let mut writer = channel.clone();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    sess.set_blocking(false);
    registry.insert(
        hostname_clone.clone(),
        SshInstance {
            session: sess,
            // channel: channel,
            tx,
        },
    );

    tauri::async_runtime::spawn_blocking(move || {
        while let Some(input) = rx.blocking_recv() {
            if channel.write_all(input.as_bytes()).is_err() {
                break;
            }
            if channel.flush().is_err() {
                break;
            }
        }
        println!("Writer thread exited");
    });

    let window_clone = window.clone();
    let hostname_for_read = hostname.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut buffer = [0u8; 4096];
        println!("Reader thread started for {}", hostname_for_read);
        loop {
            match reader.read(&mut buffer) {
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
                        // Kasih jeda sedikit (misal 50ms) supaya CPU tidak 100%

                        std::thread::sleep(std::time::Duration::from_millis(50));
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

    Ok(hostname_clone)
}

#[tauri::command]
fn send_ssh_input(
    input: String,
    ip: String,
    state: tauri::State<'_, SshState>,
) -> Result<(), String> {
    let registry = state.0.lock().unwrap(); // Lock cuma sebentar (nanoseconds)

    if let Some(instance) = registry.get(&ip) {
        // Cuma "titip pesan" ke antrean mpsc
        // Tidak ada I/O jaringan di sini, jadi tidak akan freeze
        println!("Sending input: {}", input);
        instance
            .tx
            .send(format!("{}", input))
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Session not found".into())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn submit_ssh_password(
    input_state: tauri::State<'_, SshInputState>,
    password: String,
) -> Result<(), String> {
    let sender_lock = input_state.0.lock().unwrap();
    if let Some(tx) = sender_lock.as_ref() {
        tx.blocking_send(password).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("No active SSH session".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let metric_state = MetricsState(Mutex::new(System::new()));
    let ssh_state = SshState(Arc::new(Mutex::new(HashMap::new())));

    let ssh_input_state = SshInputState(Mutex::new(None));
    tauri::Builder::default()
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                use argon2::{self, hash_raw, Config, Variant, Version};

                let config = Config {
                    lanes: 4,
                    mem_cost: 10_000,
                    time_cost: 10,
                    variant: Variant::Argon2id,
                    version: Version::Version13,
                    ..Default::default()
                };
                let salt = "your-salt".as_bytes();
                let key =
                    hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .setup(|app| {
            // Mengambil handle untuk digunakan di dalam closure atau thread
            let app_handle = app.handle();

            // Contoh: Jika kamu ingin melakukan sesuatu saat app baru nyala
            // app_handle.emit_all("sys-status", "Backend Ready").unwrap();

            Ok(())
        })
        .manage(metric_state)
        .manage(ssh_state)
        .manage(ssh_input_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_stats,
            start_ssh_session,
            submit_ssh_password,
            send_ssh_input
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
