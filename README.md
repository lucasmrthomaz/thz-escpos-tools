# thz-escpos-tools

Motor e suíte de ferramentas de baixo nível em **Rust (Edition 2024)** para manipulação, geração, renderização e envio direto de comandos **ESC/POS** para impressoras térmicas (58mm e 80mm).

Projetado para operar em **modo driverless / direct transport**, comunicando-se diretamente com o hardware no **Windows** e no **Linux / WSL2** sem intermediação do spooler ou necessidade de drivers proprietários instáveis.

---

## 🚀 Principais Recursos

- **Comunicação Direta de Hardware (Driverless):**
  - **Windows:** Acesso direto via handle nativo de dispositivo `usbprint.sys` (`\\?\usb#...`) e portas seriais/Bluetooth SPP via `serialport`.
  - **Linux / WSL2:** Envio direto para nós de impressora USB (`/dev/usb/lp*`) e seriais (`/dev/ttyUSB*`, `/dev/rfcomm*`).
  - **Zero Spooler:** Elimina travamentos de fila de impressão do sistema operacional.
- **Suporte Nativo a Português (CP860):**
  - Seleção automática de Code Page 860 (`ESC t 3`).
  - Mapeamento robusto de acentuação (`á`, `é`, `í`, `ó`, `ú`, `ã`, `õ`, `ç`, etc.).
- **Pipeline Gráfico Raster (`GS v 0`):**
  - Conversão de imagens bitonais para bobinas térmicas (384 dots para 58mm / 576 dots para 80mm).
  - Dois métodos de binarização: Limiar simples (threshold 180) e difusão de erro (*Floyd-Steinberg Dithering*).
- **Perfis Físicos Desacoplados (JSON):**
  - Schema de perfis compatível com o ecossistema `thermal-probe` (`tech-cla58.json`), desacoplando a lógica de impressão das propriedades do hardware.
- **Abstração de Transporte (`Transport` Trait):**
  - Interface unificada para `Win32UsbTransport`, `LinuxUsbTransport`, `SerialTransport`, `FileTransport` e `MockTransport` (para testes em memória).
- **Descoberta de Dispositivos:**
  - Varredura automática de impressoras térmicas conectadas (via Windows `SetupAPI` ou Linux `sysfs`).

---

## 📦 Estrutura Modular (Features no `Cargo.toml`)

O motor foi construído com feature flags para permitir consumo mínimo e desacoplado:

| Feature | Descrição | Dependências |
| :--- | :--- | :--- |
| `protocol` *(padrão)* | Gerador puro de comandos ESC/POS e codificador CP860 em memória. | *Nenhuma (zero I/O)* |
| `graphics` *(padrão)* | Processamento de imagens e conversão raster `GS v 0`. | `image` |
| `profile` *(padrão)* | Modelagem, parsing e catálogo de perfis JSON de impressoras. | `serde`, `serde_json` |
| `transport-usb` *(padrão)* | Acesso direto a impressoras USB (`usbprint.sys` no Windows / `/dev/usb/lp*` no Linux). | *APIs nativas do SO* |
| `transport-serial` *(padrão)* | Portas seriais RS232 e Bluetooth SPP. | `serialport` |
| `discovery` *(padrão)* | Enumeração e detecção de impressoras térmicas conectadas. | *APIs nativas do SO* |

---

## 🛠️ Como Utilizar como Biblioteca

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
thz-escpos-tools = { path = "../thz-escpos-tools" }
```

### Exemplo: Gerando e Enviando um Cupom Formatado

```rust
use thz_escpos::{
    Alignment, CodePage, CutType, EscPosBuilder, MockTransport, Transport
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Monta o cupom de forma fluente e declarativa
    let payload = EscPosBuilder::new()
        .with_columns(32) // 32 colunas para bobinas de 58mm (ou 48 para 80mm)
        .init()
        .code_page(CodePage::Cp860)
        .align(Alignment::Center)
        .bold(true)
        .text_ln("MEU RESTAURANTE")
        .bold(false)
        .text_ln("CNPJ: 00.000.000/0001-00")
        .separator('=')
        .align(Alignment::Left)
        .two_columns("Café Espresso", "R$ 6,00")
        .two_columns("Pão de Queijo", "R$ 7,50")
        .separator('-')
        .bold(true)
        .two_columns("TOTAL", "R$ 13,50")
        .bold(false)
        .feed(2)
        .cut(CutType::FeedAndCut(3))
        .build();

    // 2. Envia para o transporte desejado
    let mut transport = MockTransport::new();
    transport.write_all(&payload)?;
    transport.flush()?;

    println!("Recibo gerado: {} bytes transmitidos com sucesso!", transport.bytes().len());
    Ok(())
}
```

---

## 💻 Utilitário de Linha de Comando (CLI)

O repositório inclui um binário utilitário para testes imediatos:

```powershell
# 1. Detectar impressoras conectadas (USB / Bluetooth SPP)
cargo run -- discover

# 2. Gerar cupom de demonstração (teste em memória)
cargo run -- demo-receipt

# 3. Gerar cupom gravando em arquivo binário
cargo run -- demo-receipt cupom_teste.bin

# 4. Listar catálogo de perfis suportados
cargo run -- profiles
```

---

## 🧪 Testes e Validação Multiplataforma

A suíte conta com 21 testes unitários e de integração cobrindo cada byte de comando ESC/POS, mapeamento CP860, renderização gráfica e transporte.

```powershell
# No Windows
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check

# No Linux (WSL2)
wsl bash -c "cargo test"
```

---

## 📄 Licença

Distribuído sob a licença **MIT**. Consulte `LICENSE` para mais detalhes.
