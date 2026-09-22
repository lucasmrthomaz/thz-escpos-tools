//! Módulo do protocolo ESC/POS: gerador puro de comandos, estilos e codificação.

pub mod builder;
pub mod code_page;
pub mod commands;
pub mod style;

pub use builder::EscPosBuilder;
pub use code_page::CodePage;
pub use style::{Alignment, CutType, TextScale, Underline};
