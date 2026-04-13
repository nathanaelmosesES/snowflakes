import { invoke } from "@tauri-apps/api/core";
import type { SessionInfo } from "../types/settings";
import { loadSessionPass } from "./vault";

/**
 * Memulai sesi SFTP baru atau menghubungkan kembali jika diperlukan.
 */
export async function connectToSftpSession(
    session: SessionInfo,
    onStatus: (status: string) => void
): Promise<string> {
    try {
        onStatus("Fetching credentials...");

        // Ambil password dari vault jika tidak ada di objek session
        const password = session.password || await loadSessionPass(session.sessionKey);

        onStatus("Initializing SFTP session...");
        const res = await invoke("start_sftp_session", {
            bastion: session.bastionIp,
            hostname: session.targetIp,
            initialPassword: password,
            initialUsername: session.username,
        });

        const key = res as string;
        onStatus("SFTP Ready.");
        return key;
    } catch (e) {
        console.error("[SFTP Controller] Connection failed:", e);
        throw e;
    }
}

/**
 * Mengambil daftar file dan folder di direktori tertentu.
 */
export async function listDirectory(sessionKey: string, path: string): Promise<any[]> {
    try {
        return await invoke("sftp_list_dir", {
            sessionKey: sessionKey,
            path: path,
        });
    } catch (e) {
        console.error("[SFTP Controller] List directory failed:", e);
        throw e;
    }
}

/**
 * Mengunduh file dari remote server ke lokal.
 */
export async function downloadFile(
    sessionKey: string,
    remotePath: string,
    localPath: string
): Promise<void> {
    try {
        await invoke("sftp_download_file", {
            sessionKey: sessionKey,
            remotePath: remotePath,
            localPath: localPath,
        });
    } catch (e) {
        console.error("[SFTP Controller] Download failed:", e);
        throw e;
    }
}

/**
 * Mengunggah file dari lokal ke remote server.
 */
export async function uploadFile(
    sessionKey: string,
    localPath: string,
    remotePath: string
): Promise<void> {
    try {
        await invoke("sftp_upload_file", {
            sessionKey: sessionKey,
            localPath: localPath,
            remotePath: remotePath,
        });
    } catch (e) {
        console.error("[SFTP Controller] Upload failed:", e);
        throw e;
    }
}

/**
 * Mendapatkan informasi sesi SFTP yang sedang aktif berdasarkan key.
 */
export async function getActiveSftpSession(sessionKey: string): Promise<any> {
    try {
        return await invoke("get_active_sftp_session", {
            sessionKey: sessionKey,
        });
    } catch (e) {
        console.error("[SFTP Controller] Failed to get session info:", e);
        throw e;
    }
}

/**
 * Memutuskan koneksi SFTP.
 */
export async function disconnectSftp(sessionKey: string): Promise<void> {
    try {
        await invoke("disconnect_sftp", {
            sessionKey: sessionKey,
        });
    } catch (e) {
        console.error("[SFTP Controller] Disconnect failed:", e);
        throw e;
    }
}