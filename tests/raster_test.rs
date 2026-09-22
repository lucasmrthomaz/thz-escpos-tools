use thz_escpos::{ThresholdMethod, raw_gray_to_gs_v0};

#[test]
fn test_raster_gs_v0_dimensions_and_header() {
    // 384 dots (largura padrão 58mm) = 48 bytes por linha
    // 10 linhas de altura
    let width = 384u32;
    let height = 10u32;
    let pixels = vec![0u8; (width * height) as usize]; // tudo preto

    let payload = raw_gray_to_gs_v0(&pixels, width, height, ThresholdMethod::Simple(180)).unwrap();

    // Cabeçalho: 1D 76 30 00 xL xH yL yH
    assert_eq!(&payload[0..4], &[0x1D, b'v', b'0', 0x00]);
    // 48 bytes por linha: xL=48, xH=0
    assert_eq!(&payload[4..6], &[48, 0]);
    // 10 pixels de altura: yL=10, yH=0
    assert_eq!(&payload[6..8], &[10, 0]);

    // Tamanho total: 8 bytes de cabeçalho + (48 * 10 = 480 bytes) = 488 bytes
    assert_eq!(payload.len(), 488);

    // Como todos os pixels são pretos (0 < 180), todos os bytes devem ser 0xFF
    for &b in &payload[8..] {
        assert_eq!(b, 0xFF);
    }
}
