use thz_escpos::{PrinterProfile, tech_cla58};

#[test]
fn test_profile_json_roundtrip_compatibility() {
    let original_json = r#"{
  "id": "generic-tech-cla58-raster",
  "name": "TECH CLA58",
  "transport": {
    "type": "usb-printer-class",
    "vid": "6868",
    "pid": "0200",
    "port": null
  },
  "protocol": "escpos",
  "raster_mode": "gs-v-0",
  "printable_width_dots": 384,
  "baud": 9600,
  "code_page": 3
}"#;

    let profile = PrinterProfile::from_json_str(original_json).expect("Falha ao analisar JSON");

    assert_eq!(profile.id, "generic-tech-cla58-raster");
    assert_eq!(profile.name, "TECH CLA58");
    assert_eq!(profile.printable_width_dots, 384);
    assert_eq!(profile.columns_count(), 32);
    assert_eq!(profile.transport.transport_type, "usb-printer-class");
    assert_eq!(profile.transport.vid.as_deref(), Some("6868"));
    assert_eq!(profile.transport.pid.as_deref(), Some("0200"));
    assert_eq!(profile.code_page, 3);

    let serialized = profile.to_json_string().expect("Falha ao serializar JSON");
    let reloaded = PrinterProfile::from_json_str(&serialized).expect("Falha ao recarregar JSON");
    assert_eq!(profile, reloaded);
}

#[test]
fn test_builtin_catalog_profiles() {
    let cla58 = tech_cla58();
    assert_eq!(cla58.printable_width_dots, 384);
    assert_eq!(cla58.columns_count(), 32);
}
