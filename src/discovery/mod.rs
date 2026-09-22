//! Módulo de descoberta de impressoras térmicas conectadas (USB e Bluetooth/Serial).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrinterTransportType {
    UsbDirect,
    SerialCom,
    BluetoothSpp,
}

/// Representa uma impressora térmica localizada no sistema operacional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredPrinter {
    /// Nome amigável de exibição (ex: "TECH CLA58", "POS-58").
    pub name: String,
    /// Tipo de transporte identificado.
    pub transport_type: PrinterTransportType,
    /// Caminho do dispositivo (Win32 handle `\\?\usb#...` ou caminho Unix `/dev/usb/lp0`).
    pub path: String,
    /// Identificador do fabricante (Vendor ID), quando disponível.
    pub vid: Option<String>,
    /// Identificador do produto (Product ID), quando disponível.
    pub pid: Option<String>,
    /// Nome da porta serial (ex: "COM3", "/dev/ttyUSB0"), se aplicável.
    pub port: Option<String>,
}

#[cfg(windows)]
pub mod win32;

#[cfg(target_os = "linux")]
pub mod linux;

/// Descobre todas as impressoras térmicas (USB e Bluetooth/Serial) conectadas no sistema.
pub fn discover_printers() -> Vec<DiscoveredPrinter> {
    #[cfg(windows)]
    {
        win32::discover_printers_win32()
    }

    #[cfg(target_os = "linux")]
    {
        linux::discover_printers_linux()
    }

    #[cfg(not(any(windows, target_os = "linux")))]
    {
        Vec::new()
    }
}
