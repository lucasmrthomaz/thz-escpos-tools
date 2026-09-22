//! Construtor fluente de alto nível para montagem de documentos e recibos ESC/POS.

use super::{
    code_page::CodePage,
    commands::{BOLD_OFF, BOLD_ON, INIT, INVERT_OFF, INVERT_ON, LF, feed_lines, open_drawer},
    style::{Alignment, CutType, TextScale, Underline},
};

/// Construtor de payloads ESC/POS com suporte a codificação de caracteres e formatação.
#[derive(Debug, Clone)]
pub struct EscPosBuilder {
    buffer: Vec<u8>,
    current_code_page: CodePage,
    columns: usize,
}

impl Default for EscPosBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EscPosBuilder {
    /// Cria uma nova instância com configurações padrão (CP860, 32 colunas para 58mm).
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(256),
            current_code_page: CodePage::Cp860,
            columns: 32,
        }
    }

    /// Configura a largura da linha em colunas de caracteres (ex: 32 para 58mm, 48 para 80mm).
    #[must_use]
    pub const fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    /// Insere o comando de inicialização da impressora (ESC @).
    #[must_use]
    pub fn init(mut self) -> Self {
        self.buffer.extend_from_slice(&INIT);
        self
    }

    /// Seleciona a tabela de caracteres ativa e insere o comando correspondente (ESC t n).
    #[must_use]
    pub fn code_page(mut self, page: CodePage) -> Self {
        self.current_code_page = page;
        self.buffer.extend_from_slice(&page.command_bytes());
        self
    }

    /// Define o alinhamento horizontal (ESC a n).
    #[must_use]
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.buffer.extend_from_slice(&alignment.to_bytes());
        self
    }

    /// Ativa ou desativa o modo negrito (ESC E n).
    #[must_use]
    pub fn bold(mut self, enable: bool) -> Self {
        if enable {
            self.buffer.extend_from_slice(&BOLD_ON);
        } else {
            self.buffer.extend_from_slice(&BOLD_OFF);
        }
        self
    }

    /// Ativa ou desativa o modo de inversão preto/branco (GS B n).
    #[must_use]
    pub fn invert(mut self, enable: bool) -> Self {
        if enable {
            self.buffer.extend_from_slice(&INVERT_ON);
        } else {
            self.buffer.extend_from_slice(&INVERT_OFF);
        }
        self
    }

    /// Configura o sublinhado do texto.
    #[must_use]
    pub fn underline(mut self, style: Underline) -> Self {
        self.buffer.extend_from_slice(&style.to_bytes());
        self
    }

    /// Define a escala de largura e altura do texto (1 a 8).
    #[must_use]
    pub fn size(mut self, width: u8, height: u8) -> Self {
        let scale = TextScale::new(width, height);
        self.buffer.extend_from_slice(&scale.to_bytes());
        self
    }

    /// Avança `lines` linhas em branco.
    #[must_use]
    pub fn feed(mut self, lines: u8) -> Self {
        if lines == 1 {
            self.buffer.push(LF);
        } else if lines > 1 {
            self.buffer.extend_from_slice(&feed_lines(lines));
        }
        self
    }

    /// Adiciona texto codificado na Code Page ativa sem quebra de linha.
    #[must_use]
    pub fn text(mut self, content: &str) -> Self {
        let encoded = self.current_code_page.encode(content);
        self.buffer.extend(encoded);
        self
    }

    /// Adiciona texto codificado seguido de quebra de linha (LF).
    #[must_use]
    pub fn text_ln(mut self, content: &str) -> Self {
        let encoded = self.current_code_page.encode(content);
        self.buffer.extend(encoded);
        self.buffer.push(LF);
        self
    }

    /// Insere uma linha divisória preenchida pelo caractere especificado (ex: '-' ou '=').
    #[must_use]
    pub fn separator(self, ch: char) -> Self {
        let line: String = std::iter::repeat_n(ch, self.columns).collect();
        self.text_ln(&line)
    }

    /// Formata duas colunas alinhadas nos extremos da linha (ex: "Item" à esquerda, "Preço" à direita).
    #[must_use]
    pub fn two_columns(mut self, left: &str, right: &str) -> Self {
        let left_len = left.chars().count();
        let right_len = right.chars().count();

        if left_len + right_len >= self.columns {
            // Se o texto combinado exceder a largura, quebra em duas linhas
            self = self.text_ln(left);
            let spaces = self.columns.saturating_sub(right_len);
            let padding: String = " ".repeat(spaces);
            self.text_ln(&format!("{padding}{right}"))
        } else {
            let spaces = self.columns - left_len - right_len;
            let padding: String = " ".repeat(spaces);
            self.text_ln(&format!("{left}{padding}{right}"))
        }
    }

    /// Aciona o corte de papel especificado.
    #[must_use]
    pub fn cut(mut self, cut_type: CutType) -> Self {
        self.buffer.extend(cut_type.to_bytes());
        self
    }

    /// Dispara o pulso de abertura de gaveta de dinheiro no pino especificado (0 ou 1).
    #[must_use]
    pub fn drawer(mut self, pin: u8) -> Self {
        self.buffer.extend_from_slice(&open_drawer(pin));
        self
    }

    /// Anexa bytes brutos diretamente ao buffer.
    #[must_use]
    pub fn raw(mut self, bytes: &[u8]) -> Self {
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Conclui a montagem e consome o construtor, retornando o vetor final de bytes ESC/POS.
    #[must_use]
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic_receipt() {
        let bytes = EscPosBuilder::new()
            .init()
            .code_page(CodePage::Cp860)
            .align(Alignment::Center)
            .bold(true)
            .text_ln("CUPOM FISCAL")
            .bold(false)
            .separator('-')
            .two_columns("Café", "R$ 5,00")
            .cut(CutType::FeedAndCut(3))
            .build();

        assert!(!bytes.is_empty());
        // Verifica se inicia com ESC @
        assert_eq!(&bytes[0..2], &[0x1B, 0x40]);
        // Verifica se selecionou CP860
        assert_eq!(&bytes[2..5], &[0x1B, 0x74, 0x03]);
        // Verifica se alinhou ao centro
        assert_eq!(&bytes[5..8], &[0x1B, 0x61, 0x01]);
    }

    #[test]
    fn test_builder_two_columns_spacing() {
        let builder = EscPosBuilder::new().with_columns(32);
        let bytes = builder.two_columns("Agua", "R$ 3,00").build();
        let string_rep = String::from_utf8_lossy(&bytes);

        // "Agua" (4) + 21 espaços + "R$ 3,00" (7) = 32 caracteres + \n
        assert!(string_rep.starts_with("Agua"));
        assert!(string_rep.trim_end().ends_with("R$ 3,00"));
        assert_eq!(string_rep.trim_end().chars().count(), 32);
    }
}
