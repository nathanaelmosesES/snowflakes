use ssh2::Channel;
use tokio::sync::mpsc;

pub struct SshInstance {
    pub tx: mpsc::UnboundedSender<String>,
}

impl SshInstance {
    pub fn bastion_session(    
    hostname: String,
    bastion: String,
    initial_password: String,
    initial_username: String ) -> Result<Channel, String> {
        let tcp = std::net::TcpStream::connect(format!("{}:22", bastion))
        .map_err(|e| format!("Gagal koneksi ke server: {}", e))?;

        let mut sess = ssh2::Session::new().map_err(|e| e.to_string())?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| format!("Handshake gagal: {}", e.message()))?;
        sess.userauth_password(&initial_username, &initial_password)
            .map_err(|e| format!("Login Gagal: {}", e.message()))?;

        let mut channel = sess.channel_session().map_err(|e| e.to_string())?;
        channel
            .request_pty("xterm", None, None)
            .map_err(|e| e.to_string())?;

        channel.exec(&hostname).unwrap();
        sess.set_blocking(false);
        Ok(channel)
    }
}