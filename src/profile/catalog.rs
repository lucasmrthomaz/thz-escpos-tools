//! Catálogo de perfis pré-definidos para modelos comuns de impressoras térmicas.

use super::model::{PrinterProfile, TransportProfile};

/// Perfil validado para impressora TECH CLA58 (58mm, 384 dots, USB direct).
#[must_use]
pub fn tech_cla58() -> PrinterProfile {
    PrinterProfile {
        id: "tech-cla58".to_string(),
        name: "TECH CLA58 (58mm)".to_string(),
        transport: TransportProfile {
            transport_type: "usb-printer-class".to_string(),
            vid: Some("6868".to_string()),
            pid: Some("0200".to_string()),
            port: None,
        },
        protocol: "escpos".to_string(),
        raster_mode: "gs-v-0".to_string(),
        printable_width_dots: 384,
        baud: 9600,
        code_page: 3,
    }
}

/// Perfil genérico para impressoras de 58 mm (POS-58, 384 dots).
#[must_use]
pub fn generic_58mm() -> PrinterProfile {
    PrinterProfile {
        id: "generic-58mm".to_string(),
        name: "Impressora Genérica 58mm".to_string(),
        transport: TransportProfile {
            transport_type: "usb-printer-class".to_string(),
            vid: None,
            pid: None,
            port: None,
        },
        protocol: "escpos".to_string(),
        raster_mode: "gs-v-0".to_string(),
        printable_width_dots: 384,
        baud: 9600,
        code_page: 3,
    }
}

/// Perfil genérico para impressoras de 80 mm (POS-80, 576 dots).
#[must_use]
pub fn generic_80mm() -> PrinterProfile {
    PrinterProfile {
        id: "generic-80mm".to_string(),
        name: "Impressora Genérica 80mm".to_string(),
        transport: TransportProfile {
            transport_type: "usb-printer-class".to_string(),
            vid: None,
            pid: None,
            port: None,
        },
        protocol: "escpos".to_string(),
        raster_mode: "gs-v-0".to_string(),
        printable_width_dots: 576,
        baud: 9600,
        code_page: 3,
    }
}
