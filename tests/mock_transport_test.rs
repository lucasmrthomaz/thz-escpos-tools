use thz_escpos::{Alignment, CodePage, CutType, EscPosBuilder, MockTransport, Transport};

#[test]
fn test_end_to_end_receipt_via_mock_transport() {
    let mut transport = MockTransport::new();

    let receipt = EscPosBuilder::new()
        .with_columns(32)
        .init()
        .code_page(CodePage::Cp860)
        .align(Alignment::Center)
        .bold(true)
        .text_ln("TESTE E2E")
        .bold(false)
        .separator('-')
        .two_columns("Item A", "R$ 10,00")
        .feed(2)
        .cut(CutType::Partial)
        .build();

    transport
        .write_all(&receipt)
        .expect("Falha ao gravar no mock");
    transport.flush().expect("Falha ao descarregar");

    let captured = transport.bytes();
    assert!(!captured.is_empty());
    assert_eq!(&captured[0..2], &[0x1B, 0x40]); // ESC @
    assert_eq!(&captured[captured.len() - 3..], &[0x1D, 0x56, 0x01]); // Partial cut
}
