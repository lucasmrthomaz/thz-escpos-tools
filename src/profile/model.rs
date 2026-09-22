//! Estruturas de modelagem e serialização de perfis de impressora JSON.
//! Compatível com o formato do ecossistema `thermal-probe`.

use crate::error::ProfileError;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// Configuração do método de transporte físico no perfil.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportProfile {
    /// Tipo de transporte: "usb-printer-class", "com", "bluetooth", "net", etc.
    #[serde(rename = "type")]
    pub transport_type: String,
    /// Vendor ID (ex: "6868") se aplicável.
    pub vid: Option<String>,
    /// Product ID (ex: "0200") se aplicável.
    pub pid: Option<String>,
    /// Nome da porta serial (ex: "COM3" no Windows, "/dev/ttyUSB0" no Linux).
    pub port: Option<String>,
}

/// Perfil de características e capacidades de uma impressora térmica.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterProfile {
    /// Identificador único (slug) do perfil.
    pub id: String,
    /// Nome amigável de exibição da impressora.
    pub name: String,
    /// Informações de transporte associadas ao perfil.
    pub transport: TransportProfile,
    /// Dialeto de protocolo (normalmente "escpos").
    pub protocol: String,
    /// Modo de geração raster (normalmente "gs-v-0").
    pub raster_mode: String,
    /// Largura útil imprimível em pontos (dots), ex: 384 (58mm) ou 576 (80mm).
    pub printable_width_dots: u32,
    /// Baud rate para conexões seriais/Bluetooth (padrão: 9600).
    #[serde(default = "default_baud")]
    pub baud: u32,
    /// Código da tabela de caracteres ativa (padrão: 3 para CP860).
    #[serde(default = "default_code_page")]
    pub code_page: u8,
}

const fn default_baud() -> u32 {
    9600
}

const fn default_code_page() -> u8 {
    3
}

impl PrinterProfile {
    /// Retorna a quantidade de colunas de texto para Fonte A com base na largura útil em dots.
    #[must_use]
    pub const fn columns_count(&self) -> usize {
        if self.printable_width_dots >= 576 {
            48 // 80mm
        } else {
            32 // 58mm
        }
    }

    /// Carrega e valida um perfil a partir de uma string JSON.
    pub fn from_json_str(json: &str) -> Result<Self, ProfileError> {
        serde_json::from_str(json).map_err(ProfileError::Json)
    }

    /// Carrega um perfil a partir de um arquivo no disco.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ProfileError> {
        let content = fs::read_to_string(path)?;
        Self::from_json_str(&content)
    }

    /// Converte o perfil para uma string JSON formatada (com indentação).
    pub fn to_json_string(&self) -> Result<String, ProfileError> {
        serde_json::to_string_pretty(self).map_err(ProfileError::Json)
    }

    /// Salva o perfil em um arquivo JSON no disco.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ProfileError> {
        let json = self.to_json_string()?;
        fs::write(path, json)?;
        Ok(())
    }
}
