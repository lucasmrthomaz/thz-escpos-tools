//! Descoberta de impressoras térmicas no Linux e WSL2 via /sys e /dev.

#![cfg(target_os = "linux")]

use super::{DiscoveredPrinter, PrinterTransportType};
use std::{fs, path::Path};

/// Lê uma string limpa de um arquivo de atributo sysfs (ex: idVendor, idProduct).
fn read_sysfs_attr(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

/// Tenta encontrar VID e PID subindo a árvore de dispositivos do sysfs a partir de um nó lp.
fn find_usb_vid_pid(lp_sysfs_path: &Path) -> (Option<String>, Option<String>) {
    let mut current = lp_sysfs_path.to_path_buf();
    for _ in 0..5 {
        if let Ok(parent) = current.canonicalize() {
            let vid_path = parent.join("idVendor");
            let pid_path = parent.join("idProduct");
            if vid_path.exists() && pid_path.exists() {
                return (read_sysfs_attr(&vid_path), read_sysfs_attr(&pid_path));
            }
            if let Some(p) = parent.parent() {
                current = p.to_path_buf();
            } else {
                break;
            }
        } else {
            break;
        }
    }
    (None, None)
}

/// Varre o sistema Linux em busca de nós de impressoras USB (/dev/usb/lp*) e seriais.
pub fn discover_printers_linux() -> Vec<DiscoveredPrinter> {
    let mut printers = Vec::new();

    // 1. Impressoras USB (/dev/usb/lp0 .. lp9)
    for i in 0..10 {
        let dev_path = format!("/dev/usb/lp{i}");
        let path = Path::new(&dev_path);
        if path.exists() {
            let sysfs_path = format!("/sys/class/usbmisc/lp{i}/device");
            let (vid, pid) = find_usb_vid_pid(Path::new(&sysfs_path));
            printers.push(DiscoveredPrinter {
                name: format!("Impressora USB Linux (lp{i})"),
                transport_type: PrinterTransportType::UsbDirect,
                path: dev_path,
                vid,
                pid,
                port: None,
            });
        }
    }

    // 2. Dispositivos Seriais USB (/dev/ttyUSB*, /dev/ttyACM*)
    if let Ok(entries) = fs::read_dir("/dev") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let is_tty_usb = name.starts_with("ttyUSB") || name.starts_with("ttyACM");
            let is_rfcomm = name.starts_with("rfcomm");

            if is_tty_usb || is_rfcomm {
                let full_path = entry.path().to_string_lossy().to_string();
                let transport = if is_rfcomm {
                    PrinterTransportType::BluetoothSpp
                } else {
                    PrinterTransportType::SerialCom
                };

                printers.push(DiscoveredPrinter {
                    name: format!("Porta Serial ({name})"),
                    transport_type: transport,
                    path: full_path,
                    vid: None,
                    pid: None,
                    port: Some(name),
                });
            }
        }
    }

    printers
}
