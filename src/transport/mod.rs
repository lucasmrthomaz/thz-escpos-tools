//! Camada de abstração e implementações de transporte de dados ESC/POS.

use crate::error::TransportError;

pub mod file;
pub mod mock;

#[cfg(feature = "transport-serial")]
pub mod serial;

#[cfg(all(windows, feature = "transport-usb"))]
pub mod usb_windows;

#[cfg(all(target_os = "linux", feature = "transport-usb"))]
pub mod usb_linux;

pub use file::FileTransport;
pub use mock::MockTransport;

#[cfg(feature = "transport-serial")]
pub use serial::SerialTransport;

#[cfg(all(windows, feature = "transport-usb"))]
pub use usb_windows::Win32UsbTransport;

#[cfg(all(target_os = "linux", feature = "transport-usb"))]
pub use usb_linux::LinuxUsbTransport;

/// Trait unificada de transporte para envio de bytes para impressoras térmicas.
pub trait Transport: Send {
    /// Transmite uma sequência de bytes brutos para o dispositivo de destino.
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError>;

    /// Garante que todos os buffers pendentes sejam descarregados fisicamente para o dispositivo.
    fn flush(&mut self) -> Result<(), TransportError>;
}
