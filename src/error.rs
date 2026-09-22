//! Hierarquia de erros tipados para a biblioteca `thz-escpos-tools`.

use std::io;
use thiserror::Error;

/// Erro raiz que encapsula falhas em qualquer subsistema do motor ESC/POS.
#[derive(Debug, Error)]
pub enum EscPosError {
    #[error("Erro de protocolo: {0}")]
    Protocol(#[from] ProtocolError),

    #[cfg(feature = "graphics")]
    #[error("Erro gráfico: {0}")]
    Graphics(#[from] GraphicsError),

    #[cfg(feature = "profile")]
    #[error("Erro de perfil: {0}")]
    Profile(#[from] ProfileError),

    #[error("Erro de transporte: {0}")]
    Transport(#[from] TransportError),

    #[cfg(feature = "discovery")]
    #[error("Erro de descoberta: {0}")]
    Discovery(#[from] DiscoveryError),

    #[error("Erro de E/S genérico: {0}")]
    Io(#[from] io::Error),
}

/// Erros relacionados à construção e codificação de sequências de comandos ESC/POS.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("Texto inválido para a tabela de caracteres '{code_page}': '{details}'")]
    EncodingError {
        code_page: &'static str,
        details: String,
    },

    #[error("Parâmetro de comando fora dos limites: {0}")]
    ParameterOutOfBounds(String),

    #[error("Linha de colunas excede a largura máxima permitida ({max_cols} colunas)")]
    LineOverflow { max_cols: usize },
}

/// Erros relacionados ao processamento de imagens e renderização raster bitonal.
#[cfg(feature = "graphics")]
#[derive(Debug, Error)]
pub enum GraphicsError {
    #[error("Imagem vazia ou com dimensões inválidas: {0}x{1}")]
    InvalidDimensions(u32, u32),

    #[error("Dimensões excedem o limite do comando GS v 0 (largura_bytes: {0}, altura: {1})")]
    ImageTooLarge(usize, u32),

    #[error("Falha ao carregar ou decodificar imagem: {0}")]
    ImageProcessing(String),
}

/// Erros relacionados ao carregamento, validação e persistência de perfis JSON.
#[cfg(feature = "profile")]
#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Falha de E/S ao ler/gravar perfil: {0}")]
    Io(#[from] io::Error),

    #[error("Falha ao analisar JSON do perfil: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Perfil de impressora inválido ou incompleto: {0}")]
    InvalidProfile(String),

    #[error("Perfil '{0}' não encontrado no catálogo")]
    ProfileNotFound(String),
}

/// Erros relacionados à comunicação e envio físico (USB, Serial, Rede, Arquivo).
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Dispositivo não encontrado: {0}")]
    DeviceNotFound(String),

    #[error("Acesso negado ao abrir o dispositivo '{path}': {source}")]
    AccessDenied {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Tempo limite (timeout) esgotado durante a operação de transporte")]
    Timeout,

    #[error("Falha na porta serial: {0}")]
    Serial(String),

    #[error("Falha na transmissão de dados via E/S: {0}")]
    Io(#[from] io::Error),
}

/// Erros durante a descoberta de impressoras conectadas (SetupAPI no Windows ou Sysfs no Linux).
#[cfg(feature = "discovery")]
#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("Falha ao enumerar dispositivos da classe SetupAPI: código {0}")]
    SetupApiFailed(u32),

    #[error("Falha ao inspecionar dispositivos do sistema Linux: {0}")]
    LinuxSysfsFailed(#[from] io::Error),
}
