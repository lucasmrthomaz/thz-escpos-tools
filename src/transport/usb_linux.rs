//! Transporte direto para impressoras USB no Linux e WSL2 (via /dev/usb/lp*).
//! Permite impressão direta sem intermediação de CUPS ou filtros intermediários.

use super::Transport;
use crate::error::TransportError;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

/// Implementação de `Transport` para nós de impressora USB no Linux (ex: `/dev/usb/lp0`).
pub struct LinuxUsbTransport {
    file: File,
}

impl LinuxUsbTransport {
    /// Abre o nó de dispositivo de impressora no Linux.
    pub fn open<P: AsRef<Path>>(device_path: P) -> Result<Self, TransportError> {
        let path = device_path.as_ref();
        let file = OpenOptions::new().write(true).open(path).map_err(|e| {
            TransportError::AccessDenied {
                path: path.display().to_string(),
                source: e,
            }
        })?;
        Ok(Self { file })
    }
}

impl Transport for LinuxUsbTransport {
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError> {
        self.file.write_all(data).map_err(TransportError::Io)
    }

    fn flush(&mut self) -> Result<(), TransportError> {
        self.file.flush().map_err(TransportError::Io)
    }
}
