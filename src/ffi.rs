//! Camada de interface C-ABI (FFI) para exportação em `.dll` (Windows) e `.so` (Linux).
//! Permite consumo do motor por C, C++, C#, Python, Node.js/Electron, Go, Flutter, etc.

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::{Alignment, CodePage, CutType, EscPosBuilder, Transport, Underline, discover_printers};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    slice,
};

#[cfg(all(windows, feature = "transport-usb"))]
use crate::Win32UsbTransport;

#[cfg(all(target_os = "linux", feature = "transport-usb"))]
use crate::LinuxUsbTransport;

#[cfg(feature = "transport-serial")]
use crate::SerialTransport;

/// Cria um novo construtor `EscPosBuilder`.
/// Retorna um ponteiro opaco para a estrutura em heap.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_new(columns: u32) -> *mut EscPosBuilder {
    let builder = EscPosBuilder::new().with_columns(columns as usize);
    Box::into_raw(Box::new(builder))
}

/// Libera a memória alocada para o construtor `EscPosBuilder`.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_free(builder: *mut EscPosBuilder) {
    if !builder.is_null() {
        unsafe {
            drop(Box::from_raw(builder));
        }
    }
}

/// Insere o comando de inicialização da impressora (ESC @).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_init(builder: *mut EscPosBuilder) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let current = std::mem::take(b);
        *b = current.init();
    }
}

/// Define a tabela de caracteres ativa (0=Cp437, 2=Cp850, 3=Cp860 Português).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_code_page(builder: *mut EscPosBuilder, page: u8) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let cp = match page {
            0 => CodePage::Cp437,
            2 => CodePage::Cp850,
            _ => CodePage::Cp860,
        };
        let current = std::mem::take(b);
        *b = current.code_page(cp);
    }
}

/// Define o alinhamento horizontal (0=Esquerda, 1=Centro, 2=Direita).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_align(builder: *mut EscPosBuilder, align: u8) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let alignment = match align {
            1 => Alignment::Center,
            2 => Alignment::Right,
            _ => Alignment::Left,
        };
        let current = std::mem::take(b);
        *b = current.align(alignment);
    }
}

/// Ativa ou desativa o modo negrito.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_bold(builder: *mut EscPosBuilder, enable: bool) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let current = std::mem::take(b);
        *b = current.bold(enable);
    }
}

/// Define o estilo de sublinhado (0=Nenhum, 1=Simples, 2=Duplo).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_underline(builder: *mut EscPosBuilder, style: u8) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let u = match style {
            1 => Underline::Single,
            2 => Underline::Double,
            _ => Underline::None,
        };
        let current = std::mem::take(b);
        *b = current.underline(u);
    }
}

/// Avança `lines` linhas em branco.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_feed(builder: *mut EscPosBuilder, lines: u8) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let current = std::mem::take(b);
        *b = current.feed(lines);
    }
}

/// Adiciona texto UTF-8 codificado na Code Page ativa sem quebra de linha.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_text(builder: *mut EscPosBuilder, text: *const c_char) {
    if text.is_null() {
        return;
    }
    let (Some(b), Ok(s)) = (unsafe { builder.as_mut() }, unsafe {
        CStr::from_ptr(text).to_str()
    }) else {
        return;
    };
    let current = std::mem::take(b);
    *b = current.text(s);
}

/// Adiciona texto UTF-8 codificado na Code Page ativa seguido de quebra de linha (LF).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_text_ln(builder: *mut EscPosBuilder, text: *const c_char) {
    if text.is_null() {
        return;
    }
    let (Some(b), Ok(s)) = (unsafe { builder.as_mut() }, unsafe {
        CStr::from_ptr(text).to_str()
    }) else {
        return;
    };
    let current = std::mem::take(b);
    *b = current.text_ln(s);
}

/// Insere uma linha divisória repetindo o caractere `ch` até preencher a largura da bobina.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_separator(builder: *mut EscPosBuilder, ch: c_char) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let character = ch as u8 as char;
        let current = std::mem::take(b);
        *b = current.separator(character);
    }
}

/// Adiciona duas colunas alinhadas nos extremos da linha (ex: "Café" e "R$ 5,00").
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_two_columns(
    builder: *mut EscPosBuilder,
    left: *const c_char,
    right: *const c_char,
) {
    if left.is_null() || right.is_null() {
        return;
    }
    if let Some(b) = unsafe { builder.as_mut() } {
        let left_res = unsafe { CStr::from_ptr(left).to_str() };
        let right_res = unsafe { CStr::from_ptr(right).to_str() };
        if let (Ok(l), Ok(r)) = (left_res, right_res) {
            let current = std::mem::take(b);
            *b = current.two_columns(l, r);
        }
    }
}

/// Aciona o corte de papel (0=Total, 1=Parcial, 2=Avança `feed_lines` linhas e corta).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_cut(builder: *mut EscPosBuilder, cut_type: u8, feed_lines: u8) {
    if let Some(b) = unsafe { builder.as_mut() } {
        let cut = match cut_type {
            0 => CutType::Full,
            1 => CutType::Partial,
            _ => CutType::FeedAndCut(feed_lines),
        };
        let current = std::mem::take(b);
        *b = current.cut(cut);
    }
}

/// Finaliza a montagem do recibo, consome o builder e retorna um buffer de bytes.
/// O tamanho em bytes do buffer é gravado no ponteiro `out_len`.
/// A memória retornada DEVE ser liberada posteriormente com `escpos_bytes_free`.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_builder_build(
    builder: *mut EscPosBuilder,
    out_len: *mut usize,
) -> *mut u8 {
    if builder.is_null() || out_len.is_null() {
        return std::ptr::null_mut();
    }
    let b = unsafe { Box::from_raw(builder) };
    let mut bytes = b.build();
    bytes.shrink_to_fit();
    let len = bytes.len();
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    unsafe {
        *out_len = len;
    }
    ptr
}

/// Libera a memória de um buffer de bytes alocado por `escpos_builder_build`.
#[unsafe(no_mangle)]
pub extern "C" fn escpos_bytes_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            drop(Vec::from_raw_parts(ptr, len, len));
        }
    }
}

/// Libera uma string alocada pelo motor (ex: retornada por `escpos_discover_printers_json`).
#[unsafe(no_mangle)]
pub extern "C" fn escpos_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

/// Retorna a lista de impressoras detectadas em formato JSON serializado (UTF-8).
/// O ponteiro retornado deve ser liberado com `escpos_string_free`.
#[cfg(feature = "discovery")]
#[unsafe(no_mangle)]
pub extern "C" fn escpos_discover_printers_json() -> *mut c_char {
    let printers = discover_printers();
    let mut json = String::from("[");
    for (i, p) in printers.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        let vid_str = p.vid.as_deref().unwrap_or("");
        let pid_str = p.pid.as_deref().unwrap_or("");
        let port_str = p.port.as_deref().unwrap_or("");
        json.push_str(&format!(
            r#"{{"name":"{}","type":"{:?}","path":"{}","vid":"{}","pid":"{}","port":"{}"}}"#,
            p.name.replace('\\', "\\\\").replace('"', "\\\""),
            p.transport_type,
            p.path.replace('\\', "\\\\").replace('"', "\\\""),
            vid_str,
            pid_str,
            port_str
        ));
    }
    json.push(']');

    CString::new(json)
        .map(CString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

/// Envia bytes brutos para impressora USB no Windows via handle direto de dispositivo.
/// Retorna 0 em caso de sucesso ou código de erro negativo (-1: caminho inválido, -2: erro de E/S).
#[cfg(all(windows, feature = "transport-usb"))]
#[unsafe(no_mangle)]
pub extern "C" fn escpos_send_usb_windows(
    device_path: *const c_char,
    data: *const u8,
    len: usize,
) -> i32 {
    if device_path.is_null() || data.is_null() || len == 0 {
        return -1;
    }
    let path_str = match unsafe { CStr::from_ptr(device_path).to_str() } {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let slice = unsafe { slice::from_raw_parts(data, len) };

    match Win32UsbTransport::open(path_str) {
        Ok(mut transport) => {
            if transport.write_all(slice).is_err() || transport.flush().is_err() {
                -2
            } else {
                0
            }
        }
        Err(_) => -2,
    }
}

/// Envia bytes brutos para nó de impressora USB no Linux (/dev/usb/lp*).
/// Retorna 0 em caso de sucesso ou código de erro negativo.
#[cfg(all(target_os = "linux", feature = "transport-usb"))]
#[unsafe(no_mangle)]
pub extern "C" fn escpos_send_usb_linux(
    device_path: *const c_char,
    data: *const u8,
    len: usize,
) -> i32 {
    if device_path.is_null() || data.is_null() || len == 0 {
        return -1;
    }
    let path_str = match unsafe { CStr::from_ptr(device_path).to_str() } {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let slice = unsafe { slice::from_raw_parts(data, len) };

    match LinuxUsbTransport::open(path_str) {
        Ok(mut transport) => {
            if transport.write_all(slice).is_err() || transport.flush().is_err() {
                -2
            } else {
                0
            }
        }
        Err(_) => -2,
    }
}

/// Envia bytes brutos para porta serial / Bluetooth SPP.
/// Retorna 0 em caso de sucesso ou código de erro negativo.
#[cfg(feature = "transport-serial")]
#[unsafe(no_mangle)]
pub extern "C" fn escpos_send_serial(
    port_name: *const c_char,
    baud_rate: u32,
    data: *const u8,
    len: usize,
) -> i32 {
    if port_name.is_null() || data.is_null() || len == 0 {
        return -1;
    }
    let port_str = match unsafe { CStr::from_ptr(port_name).to_str() } {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let slice = unsafe { slice::from_raw_parts(data, len) };

    match SerialTransport::open(port_str, baud_rate) {
        Ok(mut transport) => {
            if transport.write_all(slice).is_err() || transport.flush().is_err() {
                -2
            } else {
                0
            }
        }
        Err(_) => -2,
    }
}
