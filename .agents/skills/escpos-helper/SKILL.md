---
name: escpos-helper
description: >-
  Use esta habilidade ao projetar, implementar ou testar sequências de comandos ESC/POS,
  formatação de recibos térmicos, codificação de acentos CP860, imagens raster GS v 0 ou cortes de papel.
---

# Guia de Referência de Protocolo ESC/POS (Baseado em THZ ThermalKit / thermal-probe)

Este guia documenta o padrão de bytes e comportamento validado no projeto de referência `thermal-probe` para impressoras térmicas ESC/POS de 58mm (384 dots) e 80mm (576 dots).

---

## 1. Comandos Essenciais de Controle

| Ação | Mnemônico | Bytes Hexadecimais | Bytes Decimais | Observações |
| :--- | :--- | :--- | :--- | :--- |
| **Inicializar Impressora** | `ESC @` | `1B 40` | `27, 64` | Limpa buffer, reseta modos de texto |
| **Pular Linha (LF)** | `LF` | `0A` | `10` | Avança uma linha de texto |
| **Selecionar Code Page 860**| `ESC t 3` | `1B 74 03` | `27, 116, 3` | Tabela CP860 (Português de Portugal/Brasil) |
| **Alinhamento à Esquerda** | `ESC a 0` | `1B 61 00` | `27, 97, 0` | Padrão |
| **Alinhamento Centralizado**| `ESC a 1` | `1B 61 01` | `27, 97, 1` | Títulos e cabeçalhos |
| **Alinhamento à Direita** | `ESC a 2` | `1B 61 02` | `27, 97, 2` | Valores e totais |
| **Ativar Negrito** | `ESC E 1` | `1B 45 01` | `27, 69, 1` | |
| **Desativar Negrito** | `ESC E 0` | `1B 45 00` | `27, 69, 0` | |
| **Corte Parcial de Papel** | `GS V 1` | `1D 56 01` | `29, 86, 1` | Deixa pequeno ponto de junção |
| **Corte Total de Papel** | `GS V 0` | `1D 56 00` | `29, 86, 0` | Corta completamente |
| **Avançar N Linhas e Cortar**| `GS V 66 n` | `1D 56 42 n` | `29, 86, 66, n` | Evita cortar em cima do texto impresso |

---

## 2. Codificação de Texto em Português (CP860)

Impressoras térmicas genéricas não suportam UTF-8 puro. O fluxo validado requer:
1. Enviar `ESC t 3` antes do texto.
2. Mapear caracteres acentuados para os bytes da tabela CP860:
   - `á`: `0xA0`, `é`: `0x82`, `í`: `0xA1`, `ó`: `0xA2`, `ú`: `0xA3`
   - `à`: `0x85`, `â`: `0x83`, `ê`: `0x88`, `ô`: `0x93`
   - `ã`: `0x84`, `õ`: `0x94`, `ç`: `0x87`
   - `Á`: `0x86`, `É`: `0x90`, `Í`: `0x8B`, `Ó`: `0x9F`, `Ú`: `0x96`
   - `À`: `0x91`, `Â`: `0x8F`, `Ê`: `0x89`, `Ô`: `0x8C`
   - `Ã`: `0x8E`, `Õ`: `0x99`, `Ç`: `0x80`
   - Caracteres não mapeados: substituir por `?` (`0x3F`).

---

## 3. Modo Gráfico Raster (`GS v 0`)

O comando `GS v 0` envia imagens bitonais (1 bit por pixel, onde bit `1` = ponto queimado/preto):

### Estrutura do Cabeçalho:
```text
[0x1D, 0x76, 0x30, m, xL, xH, yL, yH, ...dados...]
```
- `m`: Modo de densidade (`0x00` = normal / 203 DPI).
- `xL, xH`: Largura em bytes por linha (`width_pixels.div_ceil(8)` como `u16` Little-Endian: `(xL = bytes & 0xFF, xH = bytes >> 8)`).
- `yL, yH`: Altura em pixels (`height` como `u16` Little-Endian: `(yL = height & 0xFF, yH = height >> 8)`).
- `dados`: Sequência de bytes, cada byte contendo 8 pixels horizontais (MSB na esquerda).

### Binarização:
- No `thermal-probe`, o limiar validado é: `luma < 180` torna o pixel preto (`1`), caso contrário branco (`0`).
- Largura padrão de bobina de 58mm: **384 dots** (48 bytes por linha).
- Largura padrão de bobina de 80mm: **576 dots** (72 bytes por linha).

---

## 4. Transporte Direto de Hardware (Windows Driverless)

Conforme validado no `thermal-probe`:
- **USB:** Identificar via SetupAPI classe `USBPRINT` (`{28D78FAD-5A12-11D1-AE5B-00F803A8C2}`) e abrir diretamente o caminho do dispositivo (`OpenOptions::new().write(true).open(&device_path)`).
- **COM / Bluetooth SPP:** Abrir porta serial via `serialport` com timeout de 10s e flush síncrono.

---

## 5. Uso do Motor em Rust (`thz_escpos`)

A biblioteca expõe o `EscPosBuilder` e a trait `Transport` para construção declarativa:

```rust
use thz_escpos::{EscPosBuilder, CodePage, Alignment, CutType, MockTransport, Transport};

let payload = EscPosBuilder::new()
    .with_columns(32)
    .init()
    .code_page(CodePage::Cp860)
    .align(Alignment::Center)
    .bold(true)
    .text_ln("CUPOM FISCAL")
    .bold(false)
    .separator('-')
    .two_columns("Item", "R$ 10,00")
    .feed(2)
    .cut(CutType::FeedAndCut(3))
    .build();

// Envio para qualquer transporte (Mock, Win32UsbTransport, LinuxUsbTransport, SerialTransport)
let mut transport = MockTransport::new();
transport.write_all(&payload)?;
transport.flush()?;
```

### Validação da Suíte de Testes (Cross-Platform)
```powershell
# No Windows
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check

# No Linux (WSL2)
wsl bash -c "cargo test"
```
