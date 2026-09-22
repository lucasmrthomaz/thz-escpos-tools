//! Transporte direto via handle Win32 de dispositivo USB (usbprint.sys).
//! Ignora o spooler do Windows e envia sequências de bytes puras diretamente à impressora.

use super::Transport;
use crate::error::TransportError;
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

/// Implementação de `Transport` para Windows via caminho de interface USBPRINT (ex: `\\?\usb#...`).
pub struct Win32UsbTransport {
    file: File,
}

impl Win32UsbTransport {
    /// Abre o handle direto para o caminho Win32 do dispositivo USBPRINT.
    pub fn open(device_path: &str) -> Result<Self, TransportError> {
        let file = OpenOptions::new()
            .write(true)
            .open(device_path)
            .map_err(|e| TransportError::AccessDenied {
                path: device_path.to_string(),
                source: e,
            })?;
        Ok(Self { file })
    }
}

impl Transport for Win32UsbTransport {
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError> {
        self.file.write_all(data).map_err(TransportError::Io)
    }

    fn flush(&mut self) -> Result<(), TransportError> {
        self.file.flush().map_err(TransportError::Io)
    }
}
