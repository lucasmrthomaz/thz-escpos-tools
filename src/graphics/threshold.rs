//! Algoritmos de binarização e limiarização de imagens em tons de cinza para impressão térmica.

/// Método de conversão de tons de cinza (0-255) em preto e branco puro (1 bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdMethod {
    /// Limiar simples: pixels com valor < limite tornam-se pretos (1). Padrão validado no thermal-probe: 180.
    Simple(u8),
    /// Difusão de erro Floyd-Steinberg: excelente para fotos, degradês e tons contínuos.
    FloydSteinberg,
}

impl Default for ThresholdMethod {
    fn default() -> Self {
        Self::Simple(180)
    }
}

/// Aplica a binarização com difusão de erro Floyd-Steinberg em um buffer de escala de cinza mutável.
/// Modifica o buffer diretamente: 0 = preto (impresso), 255 = branco.
pub fn apply_floyd_steinberg(pixels: &mut [u8], width: usize, height: usize) {
    let mut buffer: Vec<i32> = pixels.iter().map(|&p| i32::from(p)).collect();

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_val = buffer[idx];
            let new_val = if old_val < 128 { 0 } else { 255 };
            buffer[idx] = new_val;
            let quant_error = old_val - new_val;

            // Difusão do erro de quantização para vizinhos
            // Pixel à direita (7/16)
            if x + 1 < width {
                buffer[idx + 1] += (quant_error * 7) / 16;
            }
            // Pixel inferior esquerdo (3/16)
            if y + 1 < height && x > 0 {
                buffer[idx + width - 1] += (quant_error * 3) / 16;
            }
            // Pixel abaixo (5/16)
            if y + 1 < height {
                buffer[idx + width] += (quant_error * 5) / 16;
            }
            // Pixel inferior direito (1/16)
            if y + 1 < height && x + 1 < width {
                buffer[idx + width + 1] += quant_error / 16;
            }
        }
    }

    for (out, &b) in pixels.iter_mut().zip(buffer.iter()) {
        *out = b.clamp(0, 255) as u8;
    }
}
