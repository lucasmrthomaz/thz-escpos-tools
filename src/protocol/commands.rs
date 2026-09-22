//! Constantes e funções de baixo nível para comandos ESC/POS.

/// Inicializa a impressora e reseta o buffer de impressão (ESC @).
pub const INIT: [u8; 2] = [0x1B, 0x40];

/// Quebra de linha (Line Feed - LF).
pub const LF: u8 = 0x0A;

/// Retorno de carro (Carriage Return - CR).
pub const CR: u8 = 0x0D;

/// Ativa modo negrito (ESC E 1).
pub const BOLD_ON: [u8; 3] = [0x1B, 0x45, 0x01];

/// Desativa modo negrito (ESC E 0).
pub const BOLD_OFF: [u8; 3] = [0x1B, 0x45, 0x00];

/// Ativa inversão preto/branco (GS B 1).
pub const INVERT_ON: [u8; 3] = [0x1D, 0x42, 0x01];

/// Desativa inversão preto/branco (GS B 0).
pub const INVERT_OFF: [u8; 3] = [0x1D, 0x42, 0x00];

/// Avança `lines` linhas (ESC d <n>).
#[must_use]
pub const fn feed_lines(lines: u8) -> [u8; 3] {
    [0x1B, 0x64, lines]
}

/// Pulso para abertura de gaveta de dinheiro (ESC p m t1 t2).
#[must_use]
pub const fn open_drawer(pin: u8) -> [u8; 5] {
    let m = if pin == 0 { 0 } else { 1 };
    [0x1B, 0x70, m, 50, 50]
}
