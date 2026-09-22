//! # thz-escpos-tools
//!
//! Motor e suíte de ferramentas de baixo nível em Rust para manipulação, geração,
//! renderização e envio direto de comandos ESC/POS para impressoras térmicas (58mm e 80mm).
//!
//! Compatível com **Windows** e **Linux / WSL2**, sem dependência do spooler do sistema operacional.

pub mod error;
pub mod protocol;

#[cfg(feature = "graphics")]
pub mod graphics;

#[cfg(feature = "profile")]
pub mod profile;

pub mod transport;

#[cfg(feature = "discovery")]
pub mod discovery;

pub mod ffi;

// Reexportações de alto nível
pub use error::{EscPosError, ProtocolError, TransportError};
pub use protocol::{Alignment, CodePage, CutType, EscPosBuilder, TextScale, Underline};
pub use transport::{FileTransport, MockTransport, Transport};

#[cfg(feature = "transport-serial")]
pub use transport::SerialTransport;

#[cfg(all(windows, feature = "transport-usb"))]
pub use transport::Win32UsbTransport;

#[cfg(all(target_os = "linux", feature = "transport-usb"))]
pub use transport::LinuxUsbTransport;

#[cfg(feature = "graphics")]
pub use error::GraphicsError;
#[cfg(feature = "graphics")]
pub use graphics::{ThresholdMethod, image_to_gs_v0, raw_gray_to_gs_v0};

#[cfg(feature = "profile")]
pub use error::ProfileError;
#[cfg(feature = "profile")]
pub use profile::{PrinterProfile, TransportProfile, generic_58mm, generic_80mm, tech_cla58};

#[cfg(feature = "discovery")]
pub use discovery::{DiscoveredPrinter, PrinterTransportType, discover_printers};
