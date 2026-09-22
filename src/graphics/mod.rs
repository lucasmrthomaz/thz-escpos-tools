//! Pipeline gráfico e de renderização raster bitonal para ESC/POS.

pub mod raster;
pub mod threshold;

pub use raster::{image_to_gs_v0, raw_gray_to_gs_v0};
pub use threshold::ThresholdMethod;
