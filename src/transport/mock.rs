//! Transporte de teste em memória (Mock).
//! Captura todos os bytes transmitidos sem necessidade de hardware físico.

use super::Transport;
use crate::error::TransportError;

/// Implementação de `Transport` que acumula os bytes enviados em um vetor em memória.
#[derive(Debug, Default, Clone)]
pub struct MockTransport {
    buffer: Vec<u8>,
}

impl MockTransport {
    /// Cria uma nova instância de `MockTransport` com buffer vazio.
    #[must_use]
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Retorna uma referência imutável aos bytes capturados até o momento.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Limpa o buffer de bytes capturados.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Consome o `MockTransport` e retorna o vetor de bytes capturados.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }
}

impl Transport for MockTransport {
    fn write_all(&mut self, data: &[u8]) -> Result<(), TransportError> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_transport_writes() {
        let mut transport = MockTransport::new();
        transport.write_all(b"Hello").unwrap();
        transport.write_all(b" World").unwrap();
        transport.flush().unwrap();

        assert_eq!(transport.bytes(), b"Hello World");
        assert_eq!(transport.into_bytes(), b"Hello World".to_vec());
    }
}
