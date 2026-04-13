use ssh2::Sftp;
use tokio::sync::watch;

pub struct SftpInstance {
    pub sftp: Sftp,
    pub stop_tx: watch::Sender<bool>,
}

impl SftpInstance {
    pub fn bastion_session(
        hostname: String,
        bastion: String,
        initial_password: String,
        initial_username: String,
    ) -> Result<Sftp, String> {
        let tcp = std::net::TcpStream::connect(format!("{}:22", bastion))
            .map_err(|e| format!("Gagal koneksi ke bastion: {}", e))?;

        let mut sess = ssh2::Session::new().map_err(|e| e.to_string())?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| format!("Handshake gagal: {}", e.message()))?;
        sess.userauth_password(&initial_username, &initial_password)
            .map_err(|e| format!("Login gagal: {}", e.message()))?;

        let mut channel = sess
            .channel_session()
            .map_err(|e| format!("Gagal membuka channel: {}", e))?;

        channel
            .exec(&hostname)
            .map_err(|e| format!("Gagal exec ke hostname: {}", e))?;

        sess.set_blocking(true);
        let sftp = sess
            .sftp()
            .map_err(|e| format!("Gagal membuka subsystem SFTP: {}", e))?;

        Ok(sftp)
    }
}
