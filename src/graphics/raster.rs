//! Processamento e empacotamento de imagens no formato raster ESC/POS GS v 0.

use super::threshold::{ThresholdMethod, apply_floyd_steinberg};
use crate::error::GraphicsError;
use image::{DynamicImage, GenericImageView, imageops::FilterType};

/// Empacota um buffer de pixels em tons de cinza em uma sequência de bytes ESC/POS GS v 0.
///
/// No comando ESC/POS `GS v 0 0 xL xH yL yH [dados...]`:
/// - `xL, xH`: Largura em bytes por linha (`width.div_ceil(8)` em Little-Endian).
/// - `yL, yH`: Altura em pixels em Little-Endian.
/// - `dados`: 1 bit por pixel, onde `1` = ponto queimado (preto), `0` = papel branco.
pub fn raw_gray_to_gs_v0(
    gray_pixels: &[u8],
    width: u32,
    height: u32,
    method: ThresholdMethod,
) -> Result<Vec<u8>, GraphicsError> {
    if width == 0 || height == 0 {
        return Err(GraphicsError::InvalidDimensions(width, height));
    }

    let bytes_per_row = (width as usize).div_ceil(8);
    if bytes_per_row > u16::MAX as usize || height > u16::MAX as u32 {
        return Err(GraphicsError::ImageTooLarge(bytes_per_row, height));
    }

    let mut binarized = gray_pixels.to_vec();
    let threshold_val = match method {
        ThresholdMethod::Simple(t) => t,
        ThresholdMethod::FloydSteinberg => {
            apply_floyd_steinberg(&mut binarized, width as usize, height as usize);
            128
        }
    };

    let total_data_bytes = bytes_per_row * (height as usize);
    let mut payload = Vec::with_capacity(8 + total_data_bytes);

    // Cabeçalho GS v 0 0
    payload.extend_from_slice(&[
        0x1D,
        b'v',
        b'0',
        0x00,
        (bytes_per_row & 0xFF) as u8,
        ((bytes_per_row >> 8) & 0xFF) as u8,
        (height & 0xFF) as u8,
        ((height >> 8) & 0xFF) as u8,
    ]);

    for y in 0..height {
        let row_start = (y * width) as usize;
        for xbyte in 0..bytes_per_row {
            let mut byte_accum = 0u8;
            for bit in 0..8 {
                let x = xbyte * 8 + bit;
                if x < (width as usize) {
                    let pixel_val = binarized[row_start + x];
                    // Se o pixel for mais escuro que o limiar, bit é 1 (ponto térmico ligado)
                    if pixel_val < threshold_val {
                        byte_accum |= 0x80 >> bit;
                    }
                }
            }
            payload.push(byte_accum);
        }
    }

    Ok(payload)
}

/// Redimensiona uma imagem dinâmica para a largura útil do cabeçote (mantendo aspect ratio)
/// e retorna o payload pronto em `GS v 0`.
pub fn image_to_gs_v0(
    img: &DynamicImage,
    target_width_dots: u32,
    method: ThresholdMethod,
) -> Result<Vec<u8>, GraphicsError> {
    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return Err(GraphicsError::InvalidDimensions(orig_w, orig_h));
    }

    // Calcula nova altura proporcional
    let target_height =
        ((orig_h as f64) * (target_width_dots as f64) / (orig_w as f64)).round() as u32;
    let target_height = target_height.max(1);

    // Redimensionamento Lanczos3 de alta fidelidade
    let resized = img.resize_exact(target_width_dots, target_height, FilterType::Lanczos3);
    let gray = resized.to_luma8();

    raw_gray_to_gs_v0(gray.as_raw(), target_width_dots, target_height, method)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_gray_to_gs_v0_square() {
        // Imagem 16x16: pixels pretos na borda (0) e brancos no centro (255)
        let width = 16u32;
        let height = 16u32;
        let mut pixels = vec![255u8; (width * height) as usize];

        // Desenha moldura preta
        for i in 0..16 {
            pixels[i] = 0; // topo
            pixels[15 * 16 + i] = 0; // base
            pixels[i * 16] = 0; // esquerda
            pixels[i * 16 + 15] = 0; // direita
        }

        let payload =
            raw_gray_to_gs_v0(&pixels, width, height, ThresholdMethod::Simple(180)).unwrap();

        // Cabeçalho de 8 bytes
        assert_eq!(&payload[0..4], &[0x1D, b'v', b'0', 0x00]);
        // 16 pixels = 2 bytes por linha
        assert_eq!(&payload[4..6], &[2, 0]);
        // 16 pixels de altura
        assert_eq!(&payload[6..8], &[16, 0]);

        // Total de dados: 8 cabeçalho + (2 bytes * 16 linhas) = 40 bytes
        assert_eq!(payload.len(), 40);

        // Primeira linha inteira preta => 0xFF, 0xFF
        assert_eq!(payload[8], 0xFF);
        assert_eq!(payload[9], 0xFF);
    }
}
