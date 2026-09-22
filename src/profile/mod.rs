//! Módulo de gerenciamento e persistência de perfis JSON de impressoras térmicas.
//!

pub mod catalog;
pub mod model;

pub use catalog::{generic_58mm, generic_80mm, tech_cla58};
pub use model::{PrinterProfile, TransportProfile};
