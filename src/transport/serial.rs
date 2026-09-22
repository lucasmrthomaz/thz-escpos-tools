//! Transporte para portas seriais e Bluetooth SPP via `serialport`.
//! Suporta Windows (COMx) e Linux (/dev/ttyUSBx, /dev/rfcommx).

use super::Transport;
use crate::error::TransportError;
use serialport::SerialPort;
use std::{io::Write, time::Duration};

/// Implementação de `Transport` para comunicação serial (COM, USB-Serial, Bluetooth SPP).
pub struct SerialTransport {
    port: Box<dyn SerialPort>,
}

impl SerialTransport {
    /// Abre uma porta serial no caminho e baud rate especificados.
    pub fn open(port_name: &str, baud_rate: u32) -> Result<Self, TransportError> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_secs(10))
            .open()
            .map_err(|e| {
                TransportError::Serial(format!("Erro ao abrir porta '{port_name}': {e}"))
            })?;
        Ok(Self { port })
    }
}

impl Transport for SerialTransport {
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError> {
        self.port.write_all(data).map_err(TransportError::Io)
    }

    fn flush(&mut self) -> Result<(), TransportError> {
        self.port.flush().map_err(TransportError::Io)
    }
}
