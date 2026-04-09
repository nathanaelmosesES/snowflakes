use tokio::sync::mpsc;
use uuid::Uuid;

use crate::ssh::ssh_instance;
use crate::ssh::{ssh_engine::SshEngine, ssh_instance::SshInstance};




#[tauri::command]
pub async fn start_ssh_session(
    window: tauri::Window,
    state: tauri::State<'_, SshEngine>,
    hostname: String,
    bastion: String,
    initial_password: String,
    initial_username: String,
) -> Result<String, String> {
    let channel = ssh_instance::SshInstance::bastion_session(hostname.clone(), bastion, initial_password, initial_username)?;
    let mut registry = state.0.lock().unwrap();
    let hostname_clone = hostname.clone();
    let reader = channel.stream(0);
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    
    registry.insert(
        format!("{}_{}",hostname_clone.clone(), Uuid::new_v4().to_string()),
        SshInstance {
            tx,
        },
    );
    let window_clone = window.clone();
    let hostname_for_read = hostname.clone();
    SshEngine::spawn_thread_write(rx, channel);
    SshEngine::spawn_thread_read(hostname_for_read, reader, window_clone);

    Ok(hostname_clone)
}