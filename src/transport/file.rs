//! Transporte para gravação de saídas ESC/POS em arquivos no disco (.bin ou .prn).

use super::Transport;
use crate::error::TransportError;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

/// Implementação de `Transport` que grava o payload em um arquivo.
pub struct FileTransport {
    file: File,
}

impl FileTransport {
    /// Cria ou sobrescreve um arquivo para receber o fluxo de bytes ESC/POS.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, TransportError> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.as_ref())
            .map_err(|e| TransportError::AccessDenied {
                path: path.as_ref().display().to_string(),
                source: e,
            })?;
        Ok(Self { file })
    }
}

impl Transport for FileTransport {
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError> {
        self.file.write_all(data).map_err(TransportError::Io)
    }

    fn flush(&mut self) -> Result<(), TransportError> {
        self.file.flush().map_err(TransportError::Io)
    }
}
