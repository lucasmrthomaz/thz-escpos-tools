//! Descoberta de impressoras no Windows via Windows SetupAPI (USBPRINT e COMPORT).
//! Baseado na implementação validada no projeto `thermal-probe`.

#![cfg(windows)]

use super::{DiscoveredPrinter, PrinterTransportType};
use std::{ffi::c_void, mem, ptr};

#[derive(Clone, Copy)]
#[repr(C)]
struct Guid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

const USBPRINT: Guid = Guid {
    a: 0x28d78fad,
    b: 0x5a12,
    c: 0x11d1,
    d: [0xae, 0x5b, 0, 0, 0xf8, 0x03, 0xa8, 0xc2],
};

const COMPORT: Guid = Guid {
    a: 0x86e0d1e0,
    b: 0x8089,
    c: 0x11d0,
    d: [0x9c, 0xe4, 0x08, 0, 0x3e, 0x30, 0x1f, 0x73],
};

const PRESENT_INTERFACE: u32 = 0x12;
const FRIENDLY_NAME: u32 = 12;
const HARDWARE_ID: u32 = 1;
const DEVICE_DESC: u32 = 0;

#[repr(C)]
struct InterfaceData {
    size: u32,
    class: Guid,
    flags: u32,
    reserved: usize,
}

#[repr(C)]
struct DeviceData {
    size: u32,
    class: Guid,
    instance: u32,
    reserved: usize,
}

#[link(name = "setupapi")]
unsafe extern "system" {
    fn SetupDiGetClassDevsW(
        class: *const Guid,
        enumerator: *const u16,
        parent: isize,
        flags: u32,
    ) -> isize;

    fn SetupDiEnumDeviceInterfaces(
        set: isize,
        dev: *const DeviceData,
        class: *const Guid,
        index: u32,
        data: *mut InterfaceData,
    ) -> i32;

    fn SetupDiGetDeviceInterfaceDetailW(
        set: isize,
        interface: *const InterfaceData,
        detail: *mut c_void,
        size: u32,
        required: *mut u32,
        device: *mut DeviceData,
    ) -> i32;

    fn SetupDiGetDeviceRegistryPropertyW(
        set: isize,
        device: *const DeviceData,
        property: u32,
        data_type: *mut u32,
        buffer: *mut u8,
        size: u32,
        required: *mut u32,
    ) -> i32;

    fn SetupDiDestroyDeviceInfoList(set: isize) -> i32;
}

struct RawWin32Device {
    name: String,
    hardware: String,
    path: String,
    port: Option<String>,
    is_bluetooth: bool,
}

fn wide_z(data: &[u16]) -> String {
    let len = data.iter().position(|&c| c == 0).unwrap_or(data.len());
    String::from_utf16_lossy(&data[..len])
}

fn com_from_name(name: &str) -> Option<String> {
    let start = name.rfind("(COM")?;
    let end = name[start..].find(')')?;
    Some(name[start + 1..start + end].to_string())
}

fn field(text: &str, tag: &str) -> Option<String> {
    let upper = text.to_ascii_uppercase();
    let idx = upper.find(tag)?;
    let slice = &upper[idx + tag.len()..];
    let token = slice
        .split(|c: char| !c.is_ascii_alphanumeric())
        .next()
        .filter(|s| !s.is_empty())?;
    Some(token.to_string())
}

fn vid_pid(hardware: &str) -> (Option<String>, Option<String>) {
    (field(hardware, "VID_"), field(hardware, "PID_"))
}

unsafe fn property(set: isize, dev: &DeviceData, prop: u32) -> String {
    let mut required = 0;
    unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            set,
            dev,
            prop,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut required,
        );
    }
    if required == 0 {
        return String::new();
    }
    let mut buf = vec![0u8; required as usize];
    let success = unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            set,
            dev,
            prop,
            ptr::null_mut(),
            buf.as_mut_ptr(),
            required,
            ptr::null_mut(),
        ) != 0
    };

    if success {
        let wide: Vec<u16> = buf
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| u16::from_le_bytes(*c))
            .collect();
        wide_z(&wide)
    } else {
        String::new()
    }
}

fn enumerate_devices(class: &Guid) -> Vec<RawWin32Device> {
    let mut found = Vec::new();
    unsafe {
        let set = SetupDiGetClassDevsW(class, ptr::null(), 0, PRESENT_INTERFACE);
        if set == -1 {
            return found;
        }

        let mut index = 0;
        loop {
            let mut iface = InterfaceData {
                size: mem::size_of::<InterfaceData>() as u32,
                class: *class,
                flags: 0,
                reserved: 0,
            };
            if SetupDiEnumDeviceInterfaces(set, ptr::null(), class, index, &mut iface) == 0 {
                break;
            }
            index += 1;

            let mut required = 0;
            SetupDiGetDeviceInterfaceDetailW(
                set,
                &iface,
                ptr::null_mut(),
                0,
                &mut required,
                ptr::null_mut(),
            );
            if !(6..=65536).contains(&required) {
                continue;
            }

            let mut detail = vec![0u64; (required as usize).div_ceil(8)];
            let raw = detail.as_mut_ptr() as *mut u8;
            *(raw as *mut u32) = if cfg!(target_pointer_width = "64") {
                8
            } else {
                6
            };

            let mut dev = DeviceData {
                size: mem::size_of::<DeviceData>() as u32,
                class: *class,
                instance: 0,
                reserved: 0,
            };

            if SetupDiGetDeviceInterfaceDetailW(
                set,
                &iface,
                raw as *mut c_void,
                required,
                ptr::null_mut(),
                &mut dev,
            ) != 0
            {
                let path_offset = if cfg!(target_pointer_width = "64") {
                    4
                } else {
                    2
                };
                let path_wide = std::slice::from_raw_parts(
                    (raw as *const u16).add(path_offset),
                    ((required as usize) - (path_offset * 2)) / 2,
                );
                let path = wide_z(path_wide);

                let mut name = property(set, &dev, FRIENDLY_NAME);
                if name.is_empty() {
                    name = property(set, &dev, DEVICE_DESC);
                }
                let hardware = property(set, &dev, HARDWARE_ID);
                let port = com_from_name(&name);
                let is_bluetooth = hardware.contains("BTHENUM") || hardware.contains("00001101");

                found.push(RawWin32Device {
                    name,
                    hardware,
                    path,
                    port,
                    is_bluetooth,
                });
            }
        }

        SetupDiDestroyDeviceInfoList(set);
    }
    found
}

/// Executa a varredura nativa do Windows SetupAPI.
pub fn discover_printers_win32() -> Vec<DiscoveredPrinter> {
    let mut printers = Vec::new();

    // 1. USBPRINT
    let usb_devices = enumerate_devices(&USBPRINT);
    for d in usb_devices {
        let (vid, pid) = vid_pid(&d.hardware);
        printers.push(DiscoveredPrinter {
            name: if d.name.is_empty() {
                "Impressora USB".to_string()
            } else {
                d.name
            },
            transport_type: PrinterTransportType::UsbDirect,
            path: d.path,
            vid,
            pid,
            port: None,
        });
    }

    // 2. COMPORT / Bluetooth
    let com_devices = enumerate_devices(&COMPORT);
    for d in com_devices {
        let name_upper = d.name.to_ascii_uppercase();
        let hw_upper = d.hardware.to_ascii_uppercase();
        let path_upper = d.path.to_ascii_uppercase();

        let is_printer = name_upper.contains("MPT")
            || name_upper.contains("CLA58")
            || name_upper.contains("POS")
            || name_upper.contains("PRINTER")
            || path_upper.contains("DC0D51597B0C")
            || hw_upper.contains("DC0D51597B0C");

        if let Some(port) = d.port {
            let (vid, pid) = vid_pid(&d.hardware);
            if d.is_bluetooth && is_printer {
                printers.push(DiscoveredPrinter {
                    name: d.name,
                    transport_type: PrinterTransportType::BluetoothSpp,
                    path: format!("\\\\.\\{port}"),
                    vid,
                    pid,
                    port: Some(port),
                });
            } else if !d.is_bluetooth && is_printer {
                printers.push(DiscoveredPrinter {
                    name: d.name,
                    transport_type: PrinterTransportType::SerialCom,
                    path: format!("\\\\.\\{port}"),
                    vid,
                    pid,
                    port: Some(port),
                });
            }
        }
    }

    printers
}
