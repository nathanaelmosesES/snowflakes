use tauri::Emitter;

use crate::sftp::sftp_engine::SftpEngine;

#[tauri::command]
pub async fn get_active_sftp_session(
    state: tauri::State<'_, SftpEngine>,
) -> Result<Vec<String>, String> {
    let registry = state.0.lock().unwrap();
    let keys = registry.keys().cloned().collect::<Vec<String>>();
    Ok(keys)
}

#[tauri::command]
pub async fn disconnect_sftp(
    window: tauri::Window,
    session_key: String,
    state: tauri::State<'_, SftpEngine>,
) -> Result<(), String> {
    let mut registry = state.0.lock().unwrap();

    if let Some(session) = registry.remove(&session_key) {
        let _ = session.stop_tx.send(true);
        println!("SFTP Session {} disconnected", session_key);
        window
            .emit("sftp_session_updated", session_key)
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("SFTP Session {} tidak ditemukan", session_key))
    }
}
