use thz_escpos::CodePage;

#[test]
fn test_all_portuguese_accented_characters_match_thermal_probe() {
    let lower = "áéíóúàâêôãõç";
    let encoded_lower = CodePage::Cp860.encode(lower);

    let expected_lower = [
        0xA0, // á
        0x82, // é
        0xA1, // í
        0xA2, // ó
        0xA3, // ú
        0x85, // à
        0x83, // â
        0x88, // ê
        0x93, // ô
        0x84, // ã
        0x94, // õ
        0x87, // ç
    ];
    assert_eq!(encoded_lower, expected_lower);

    let upper = "ÁÉÍÓÚÀÂÊÔÃÕÇ";
    let encoded_upper = CodePage::Cp860.encode(upper);

    let expected_upper = [
        0x86, // Á
        0x90, // É
        0x8B, // Í
        0x9F, // Ó
        0x96, // Ú
        0x91, // À
        0x8F, // Â
        0x89, // Ê
        0x8C, // Ô
        0x8E, // Ã
        0x99, // Õ
        0x80, // Ç
    ];
    assert_eq!(encoded_upper, expected_upper);
}

#[test]
fn test_symbols_cp860() {
    let symbols = "Nº 1ª §";
    let encoded = CodePage::Cp860.encode(symbols);
    assert_eq!(encoded, vec![b'N', 0xA7, b' ', b'1', 0xA6, b' ', 0x15]);
}
