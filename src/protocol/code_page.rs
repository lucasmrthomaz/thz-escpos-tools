//! Mapeamento de tabelas de caracteres (Code Pages) para impressoras ESC/POS.

/// Tabelas de caracteres suportadas pelo motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodePage {
    /// Code Page 860 (Português de Portugal e Brasil - padrão do thermal-probe).
    #[default]
    Cp860,
    /// Code Page 437 (Padrão USA / Western European básico).
    Cp437,
    /// Code Page 850 (Multilingual Latin I).
    Cp850,
}

impl CodePage {
    /// Retorna o valor numérico para o comando ESC t <n>.
    #[must_use]
    pub const fn esc_pos_number(&self) -> u8 {
        match self {
            Self::Cp437 => 0,
            Self::Cp850 => 2,
            Self::Cp860 => 3,
        }
    }

    /// Retorna a sequência de bytes ESC/POS para ativar esta tabela de caracteres na impressora.
    #[must_use]
    pub const fn command_bytes(&self) -> [u8; 3] {
        [0x1B, 0x74, self.esc_pos_number()]
    }

    /// Codifica uma string UTF-8 na sequência de bytes equivalente para esta Code Page.
    #[must_use]
    pub fn encode(&self, text: &str) -> Vec<u8> {
        match self {
            Self::Cp860 => encode_cp860(text),
            Self::Cp437 => encode_cp437(text),
            Self::Cp850 => encode_cp850(text),
        }
    }
}

/// Codifica caracteres para a tabela CP860, validada no projeto de referência `thermal-probe`.
/// Caracteres sem suporte direto são convertidos para '?' (0x3F).
fn encode_cp860(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| {
            if c.is_ascii() {
                c as u8
            } else {
                match c {
                    // Minúsculas acentuadas
                    'á' => 0xA0,
                    'é' => 0x82,
                    'í' => 0xA1,
                    'ó' => 0xA2,
                    'ú' => 0xA3,
                    'à' => 0x85,
                    'â' => 0x83,
                    'ê' => 0x88,
                    'ô' => 0x93,
                    'ã' => 0x84,
                    'õ' => 0x94,
                    'ç' => 0x87,

                    // Maiúsculas acentuadas
                    'Á' => 0x86,
                    'É' => 0x90,
                    'Í' => 0x8B,
                    'Ó' => 0x9F,
                    'Ú' => 0x96,
                    'À' => 0x91,
                    'Â' => 0x8F,
                    'Ê' => 0x89,
                    'Ô' => 0x8C,
                    'Ã' => 0x8E,
                    'Õ' => 0x99,
                    'Ç' => 0x80,

                    // Símbolos comuns
                    'º' => 0xA7,
                    'ª' => 0xA6,
                    '§' => 0x15,
                    '¿' => 0xA8,
                    '¡' => 0xAD,

                    // Caractere desconhecido/não suportado
                    _ => b'?',
                }
            }
        })
        .collect()
}

/// Codificação básica CP437.
fn encode_cp437(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| {
            if c.is_ascii() {
                c as u8
            } else {
                match c {
                    'á' => 0xA0,
                    'é' => 0x82,
                    'í' => 0xA1,
                    'ó' => 0xA2,
                    'ú' => 0xA3,
                    'à' => 0x85,
                    'â' => 0x83,
                    'ê' => 0x88,
                    'ô' => 0x93,
                    'ç' => 0x87,
                    'Ç' => 0x80,
                    'É' => 0x90,
                    _ => b'?',
                }
            }
        })
        .collect()
}

/// Codificação básica CP850.
fn encode_cp850(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| {
            if c.is_ascii() {
                c as u8
            } else {
                match c {
                    'á' => 0xA0,
                    'é' => 0x82,
                    'í' => 0xA1,
                    'ó' => 0xA2,
                    'ú' => 0xA3,
                    'à' => 0x85,
                    'â' => 0x83,
                    'ê' => 0x88,
                    'ô' => 0x93,
                    'ã' => 0xC6,
                    'õ' => 0xE4,
                    'ç' => 0x87,
                    'Ç' => 0x80,
                    'Á' => 0xB5,
                    'É' => 0x90,
                    'Í' => 0xD6,
                    'Ó' => 0xE0,
                    'Ú' => 0xE9,
                    'Ã' => 0xC7,
                    'Õ' => 0xE5,
                    _ => b'?',
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp860_command_bytes() {
        assert_eq!(CodePage::Cp860.command_bytes(), [0x1B, 0x74, 0x03]);
        assert_eq!(CodePage::Cp437.command_bytes(), [0x1B, 0x74, 0x00]);
        assert_eq!(CodePage::Cp850.command_bytes(), [0x1B, 0x74, 0x02]);
    }

    #[test]
    fn test_cp860_portuguese_accents() {
        let text = "Atenção: café, pão & coração!";
        let encoded = CodePage::Cp860.encode(text);

        // 'A', 't', 'e', 'n', 'ç' (0x87), 'ã' (0x84), 'o'
        assert_eq!(encoded[4], 0x87); // ç
        assert_eq!(encoded[5], 0x84); // ã
        // 'c', 'a', 'f', 'é' (0x82)
        assert_eq!(encoded[12], 0x82); // é
    }

    #[test]
    fn test_cp860_unsupported_char() {
        let text = "Emoji 🚀 e japonês 日本";
        let encoded = CodePage::Cp860.encode(text);
        assert!(encoded.contains(&b'?'));
    }
}
