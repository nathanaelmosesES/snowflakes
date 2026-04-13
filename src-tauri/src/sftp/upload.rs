use std::io::Write;
use tauri::Emitter;

use crate::sftp::sftp_engine::SftpEngine;

/// Upload file dari local path ke remote path.
/// Emit progress event: "sftp-upload-progress-{session_key}" dengan persentase 0–100.
#[tauri::command]
pub async fn sftp_upload_file(
    window: tauri::Window,
    session_key: String,
    local_path: String,
    remote_path: String,
    state: tauri::State<'_, SftpEngine>,
) -> Result<(), String> {
    let file_bytes =
        std::fs::read(&local_path).map_err(|e| format!("Gagal membaca file lokal: {}", e))?;
    let total_size = file_bytes.len() as u64;

    let registry = state.0.lock().unwrap();

    let instance = registry
        .get(&session_key)
        .ok_or_else(|| format!("SFTP Session {} tidak ditemukan", session_key))?;

    let remote = std::path::Path::new(&remote_path);

    let mut remote_file = instance
        .sftp
        .create(remote)
        .map_err(|e| format!("Gagal membuat file remote: {}", e))?;

    drop(registry);

    let chunk_size: usize = 32 * 1024;
    let mut bytes_written: u64 = 0;

    for chunk in file_bytes.chunks(chunk_size) {
        remote_file
            .write_all(chunk)
            .map_err(|e| format!("Gagal menulis ke file remote: {}", e))?;

        bytes_written += chunk.len() as u64;

        if total_size > 0 {
            let progress = (bytes_written as f64 / total_size as f64 * 100.0) as u8;
            window
                .emit(&format!("sftp-upload-progress-{}", session_key), progress)
                .ok();
        }
    }

    window
        .emit(
            &format!("sftp-upload-done-{}", session_key),
            remote_path.clone(),
        )
        .ok();

    println!(
        "Upload selesai: {} → {} ({} bytes)",
        local_path, remote_path, bytes_written
    );

    Ok(())
}
