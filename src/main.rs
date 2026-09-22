//! Utilitário de linha de comando `thz-escpos`.
//! Demonstra o uso do motor thz-escpos-tools para descoberta, teste e geração de recibos.

use std::env;
use thz_escpos::{
    Alignment, CodePage, CutType, EscPosBuilder, FileTransport, MockTransport, Transport,
    discover_printers, generic_58mm, generic_80mm, tech_cla58,
};

fn print_help() {
    println!("thz-escpos-tools CLI (Motor ESC/POS Cross-Platform)");
    println!("Uso: thz-escpos <subcomando> [opções]\n");
    println!("Subcomandos disponíveis:");
    println!(
        "  discover              Lista todas as impressoras térmicas detectadas (USB / COM / Bluetooth)"
    );
    println!(
        "  demo-receipt [arquivo] Gera um cupom de demonstração formatado (imprime ou grava em arquivo .bin)"
    );
    println!("  profiles              Lista os perfis de impressoras pré-configurados no catálogo");
    println!("  help                  Exibe esta mensagem de ajuda");
}

fn handle_discover() {
    println!("--- Buscando impressoras térmicas conectadas... ---");
    let printers = discover_printers();
    if printers.is_empty() {
        println!("Nenhuma impressora térmica detectada no momento.");
        println!("Verifique as conexões USB ou pareamento Bluetooth SPP.");
    } else {
        println!("Encontrada(s) {} impressora(s):", printers.len());
        for (i, p) in printers.iter().enumerate() {
            println!(
                "  [{}] {} | Tipo: {:?} | Caminho: {}",
                i + 1,
                p.name,
                p.transport_type,
                p.path
            );
            if let (Some(vid), Some(pid)) = (&p.vid, &p.pid) {
                println!("      VID: {vid} | PID: {pid}");
            }
            if let Some(port) = &p.port {
                println!("      Porta: {port}");
            }
        }
    }
}

fn handle_demo_receipt(output_file: Option<&str>) {
    println!("--- Gerando recibo de teste com EscPosBuilder (CP860, 32 colunas) ---");

    let builder = EscPosBuilder::new()
        .with_columns(32)
        .init()
        .code_page(CodePage::Cp860)
        .align(Alignment::Center)
        .bold(true)
        .text_ln("THZ THERMALKIT")
        .bold(false)
        .text_ln("Motor Global ESC/POS")
        .text_ln("Validado com thermal-probe")
        .feed(1)
        .separator('=')
        .align(Alignment::Left)
        .two_columns("Café Especial c/ Acento", "R$ 8,50")
        .two_columns("Pão de Queijo Mineiro", "R$ 6,00")
        .two_columns("Água Mineral 500ml", "R$ 4,00")
        .separator('-')
        .bold(true)
        .two_columns("TOTAL", "R$ 18,50")
        .bold(false)
        .feed(1)
        .align(Alignment::Center)
        .text_ln("Obrigado pela preferência!")
        .feed(2)
        .cut(CutType::FeedAndCut(3));

    let payload = builder.build();
    println!("Tamanho do payload gerado: {} bytes", payload.len());

    if let Some(path) = output_file {
        println!("Gravando payload no arquivo: {path}");
        let mut transport = FileTransport::create(path).expect("Falha ao criar arquivo");
        transport
            .write_all(&payload)
            .expect("Falha ao gravar bytes");
        transport.flush().expect("Falha ao descarregar buffer");
        println!("Recibo gravado com sucesso em '{path}'!");
    } else {
        let mut mock = MockTransport::new();
        mock.write_all(&payload).unwrap();
        mock.flush().unwrap();
        println!("Teste em memória (MockTransport) concluído com sucesso!");
        println!("(Para gravar em arquivo: cargo run -- demo-receipt saida.bin)");
    }
}

fn handle_profiles() {
    println!("--- Catálogo de Perfis Integrados ---");
    let profiles = vec![tech_cla58(), generic_58mm(), generic_80mm()];
    for p in profiles {
        println!("• Perfil: {} ({})", p.name, p.id);
        println!(
            "  - Largura útil: {} dots ({} colunas)",
            p.printable_width_dots,
            p.columns_count()
        );
        println!("  - Transporte padrão: {}", p.transport.transport_type);
        println!("  - Baud rate: {} | Code Page: {}", p.baud, p.code_page);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");

    match command {
        "discover" => handle_discover(),
        "demo-receipt" => handle_demo_receipt(args.get(2).map(String::as_str)),
        "profiles" => handle_profiles(),
        "help" | "-h" | "--help" => print_help(),
        other => {
            eprintln!("Comando desconhecido: '{other}'");
            print_help();
            std::process::exit(1);
        }
    }
}
