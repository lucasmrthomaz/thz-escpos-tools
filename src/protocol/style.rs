//! Tipos e modificadores de estilo de impressão ESC/POS.

/// Alinhamento horizontal do texto e elementos na linha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Alinhamento à esquerda (ESC a 0).
    #[default]
    Left,
    /// Alinhamento centralizado (ESC a 1).
    Center,
    /// Alinhamento à direita (ESC a 2).
    Right,
}

impl Alignment {
    /// Retorna os bytes de comando ESC a <n> para este alinhamento.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; 3] {
        match self {
            Self::Left => [0x1B, 0x61, 0x00],
            Self::Center => [0x1B, 0x61, 0x01],
            Self::Right => [0x1B, 0x61, 0x02],
        }
    }
}

/// Modo de corte de papel da guilhotina (quando suportado pela impressora).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutType {
    /// Corte total direto (GS V 0).
    Full,
    /// Corte parcial direto (GS V 1).
    Partial,
    /// Avança `n` linhas e realiza corte parcial (GS V 66 n) - mais seguro para não cortar texto.
    FeedAndCut(u8),
}

impl CutType {
    /// Retorna os bytes ESC/POS para acionar o corte.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Full => vec![0x1D, 0x56, 0x00],
            Self::Partial => vec![0x1D, 0x56, 0x01],
            Self::FeedAndCut(n) => vec![0x1D, 0x56, 0x42, *n],
        }
    }
}

/// Estilo de sublinhado de texto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Underline {
    #[default]
    None,
    Single,
    Double,
}

impl Underline {
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; 3] {
        match self {
            Self::None => [0x1B, 0x2D, 0x00],
            Self::Single => [0x1B, 0x2D, 0x01],
            Self::Double => [0x1B, 0x2D, 0x02],
        }
    }
}

/// Multiplicadores de escala de caracteres (GS ! n).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextScale {
    /// Multiplicador de largura (1x a 8x).
    pub width: u8,
    /// Multiplicador de altura (1x a 8x).
    pub height: u8,
}

impl TextScale {
    /// Cria uma nova escala de texto (valores de 1 a 8).
    #[must_use]
    pub const fn new(width: u8, height: u8) -> Self {
        let w = if width < 1 {
            1
        } else if width > 8 {
            8
        } else {
            width
        };
        let h = if height < 1 {
            1
        } else if height > 8 {
            8
        } else {
            height
        };
        Self {
            width: w,
            height: h,
        }
    }

    /// Retorna os bytes do comando ESC/POS GS ! n.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 3] {
        let w_bits = (self.width - 1) << 4;
        let h_bits = self.height - 1;
        [0x1D, 0x21, w_bits | h_bits]
    }
}
