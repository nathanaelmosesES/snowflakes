use crate::sftp::sftp_instance::SftpInstance;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct SftpEngine(pub Arc<Mutex<HashMap<String, SftpInstance>>>);
