use std::io::Read;
use tauri::Emitter;

use crate::sftp::sftp_engine::SftpEngine;

#[tauri::command]
pub async fn sftp_download_file(
    window: tauri::Window,
    session_key: String,
    remote_path: String,
    local_path: String,
    state: tauri::State<'_, SftpEngine>,
) -> Result<(), String> {
    let registry = state.0.lock().unwrap();

    let instance = registry
        .get(&session_key)
        .ok_or_else(|| format!("SFTP Session {} tidak ditemukan", session_key))?;

    let path = std::path::Path::new(&remote_path);

    let stat = instance
        .sftp
        .stat(path)
        .map_err(|e| format!("Gagal stat file: {}", e))?;
    let total_size = stat.size.unwrap_or(0);

    let mut remote_file = instance
        .sftp
        .open(path)
        .map_err(|e| format!("Gagal membuka file remote: {}", e))?;

    drop(registry);

    let local = std::path::Path::new(&local_path);
    let mut local_file =
        std::fs::File::create(local).map_err(|e| format!("Gagal membuat file lokal: {}", e))?;

    let mut buffer = vec![0u8; 32 * 1024];
    let mut bytes_read: u64 = 0;

    loop {
        let n = remote_file
            .read(&mut buffer)
            .map_err(|e| format!("Gagal baca file remote: {}", e))?;

        if n == 0 {
            break;
        }

        std::io::Write::write_all(&mut local_file, &buffer[..n])
            .map_err(|e| format!("Gagal tulis file lokal: {}", e))?;

        bytes_read += n as u64;

        if total_size > 0 {
            let progress = (bytes_read as f64 / total_size as f64 * 100.0) as u8;
            window
                .emit(&format!("sftp-download-progress-{}", session_key), progress)
                .ok();
        }
    }

    window
        .emit(
            &format!("sftp-download-done-{}", session_key),
            local_path.clone(),
        )
        .ok();

    println!(
        "Download selesai: {} → {} ({} bytes)",
        remote_path, local_path, bytes_read
    );

    Ok(())
}
