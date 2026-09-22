use thz_escpos::{Alignment, CutType, EscPosBuilder, TextScale, Underline};

#[test]
fn test_init_command_bytes() {
    let bytes = EscPosBuilder::new().init().build();
    assert_eq!(bytes, vec![0x1B, 0x40]);
}

#[test]
fn test_alignment_command_bytes() {
    let left = EscPosBuilder::new().align(Alignment::Left).build();
    assert_eq!(left, vec![0x1B, 0x61, 0x00]);

    let center = EscPosBuilder::new().align(Alignment::Center).build();
    assert_eq!(center, vec![0x1B, 0x61, 0x01]);

    let right = EscPosBuilder::new().align(Alignment::Right).build();
    assert_eq!(right, vec![0x1B, 0x61, 0x02]);
}

#[test]
fn test_bold_command_bytes() {
    let on = EscPosBuilder::new().bold(true).build();
    assert_eq!(on, vec![0x1B, 0x45, 0x01]);

    let off = EscPosBuilder::new().bold(false).build();
    assert_eq!(off, vec![0x1B, 0x45, 0x00]);
}

#[test]
fn test_invert_command_bytes() {
    let on = EscPosBuilder::new().invert(true).build();
    assert_eq!(on, vec![0x1D, 0x42, 0x01]);

    let off = EscPosBuilder::new().invert(false).build();
    assert_eq!(off, vec![0x1D, 0x42, 0x00]);
}

#[test]
fn test_underline_command_bytes() {
    let single = EscPosBuilder::new().underline(Underline::Single).build();
    assert_eq!(single, vec![0x1B, 0x2D, 0x01]);

    let double = EscPosBuilder::new().underline(Underline::Double).build();
    assert_eq!(double, vec![0x1B, 0x2D, 0x02]);
}

#[test]
fn test_text_scale_command_bytes() {
    let scale_1x1 = TextScale::new(1, 1).to_bytes();
    assert_eq!(scale_1x1, [0x1D, 0x21, 0x00]);

    // 2x largura (bit 4 = 1 << 4 = 0x10), 2x altura (bit 0 = 1 => 0x11)
    let scale_2x2 = TextScale::new(2, 2).to_bytes();
    assert_eq!(scale_2x2, [0x1D, 0x21, 0x11]);
}

#[test]
fn test_cut_command_bytes() {
    let full = CutType::Full.to_bytes();
    assert_eq!(full, vec![0x1D, 0x56, 0x00]);

    let partial = CutType::Partial.to_bytes();
    assert_eq!(partial, vec![0x1D, 0x56, 0x01]);

    let feed_cut = CutType::FeedAndCut(4).to_bytes();
    assert_eq!(feed_cut, vec![0x1D, 0x56, 0x42, 0x04]);
}

#[test]
fn test_separator_generation() {
    let bytes = EscPosBuilder::new().with_columns(10).separator('-').build();
    // 10 vezes '-' + \n (0x0A)
    assert_eq!(bytes, b"----------\n");
}
