use serde::{Deserialize, Serialize};

use crate::sftp::sftp_engine::SftpEngine;

#[derive(Debug, Serialize, Deserialize)]
pub struct SftpEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u64, // Unix timestamp
}

#[tauri::command]
pub async fn sftp_list_dir(
    session_key: String,
    remote_path: String,
    state: tauri::State<'_, SftpEngine>,
) -> Result<Vec<SftpEntry>, String> {
    let registry = state.0.lock().unwrap();

    let instance = registry
        .get(&session_key)
        .ok_or_else(|| format!("SFTP Session {} tidak ditemukan", session_key))?;

    let path = std::path::Path::new(&remote_path);

    let entries = instance
        .sftp
        .readdir(path)
        .map_err(|e| format!("Gagal membaca direktori: {}", e))?;

    let result = entries
        .into_iter()
        .map(|(pathbuf, stat)| {
            let name = pathbuf
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let full_path = pathbuf.to_string_lossy().to_string();
            let is_dir = stat.is_dir();
            let size = stat.size.unwrap_or(0);
            let modified = stat.mtime.unwrap_or(0);

            SftpEntry {
                name,
                path: full_path,
                is_dir,
                size,
                modified,
            }
        })
        .collect::<Vec<SftpEntry>>();

    Ok(result)
}
