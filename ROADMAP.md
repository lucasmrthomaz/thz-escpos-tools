# Roadmap & Árvores de Desenvolvimento (Branches)

Este documento registra as branches dedicadas criadas no repositório para isolar experimentos e novos recursos em relação à branch estável (`master`).

---

## Mapa de Branches

```text
master (Branch estável - Produção)
│
├── 🔬 exp/wsl-hardware-passthrough    (Validação de impressora física no Linux via WSL2)
├── 🚀 feature/barcodes-qrcode         (Suporte a QR Code nativo e Códigos de Barras 1D)
├── 🚀 feature/network-transport       (Impressoras de rede Ethernet / Wi-Fi via TCP 9100)
└── 🔌 feature/csharp-node-examples    (Exemplos de consumo da DLL/SO em Node.js e C#)
```

---

## 1. `exp/wsl-hardware-passthrough` (Experimento)

* **Objetivo:** Permitir o envio de comandos ESC/POS para a impressora térmica física diretamente de dentro do Linux (WSL2), utilizando redirecionamento USB.
* **Pré-requisitos no Windows:**
  ```powershell
  # Instalar ferramenta oficial da Microsoft para passthrough USB
  winget install --exact dorssel.usbipd-win
  ```
* **Passo a Passo de Execução:**
  1. No Windows (PowerShell Administrador):
     ```powershell
     # Listar dispositivos USB e identificar o BUSID da impressora
     usbipd list
     
     # Compartilhar e anexar ao WSL2
     usbipd bind --busid <BUSID>
     usbipd attach --wsl --busid <BUSID>
     ```
  2. No Linux (WSL2):
     ```bash
     # Verificar se o nó /dev/usb/lp0 foi criado pelo driver usblp
     ls -la /dev/usb/lp*
     dmesg | grep usblp
     ```
  3. Executar o CLI ou teste do `thz-escpos-tools` dentro do WSL2 apontando para `/dev/usb/lp0`.

---

## 2. `feature/barcodes-qrcode` (Nova Funcionalidade)

* **Objetivo:** Adicionar suporte a códigos de barras 1D e 2D diretamente no `EscPosBuilder`.
* **Escopo Técnico:**
  - **QR Code ESC/POS nativo:** Sequências padrão `GS ( k` (Function 165 - dados, 167 - tamanho do módulo, 169 - nível de correção de erro L/M/Q/H, 181 - imprimir buffer).
  - **Códigos de barras 1D:** Comando `GS k` para EAN-13, EAN-8 e Code 128 (comum em comandas de restaurante e boletos).
  - **Métodos no Builder:**
    - `.qrcode(data: &str, size: u8)`
    - `.barcode(symbology: BarcodeType, data: &str)`
  - **C-ABI (FFI):**
    - `escpos_builder_qrcode(builder, data, size)`
    - `escpos_builder_barcode(builder, type, data)`

---

## 3. `feature/network-transport` (Nova Funcionalidade)

* **Objetivo:** Suportar impressoras térmicas conectadas em rede local (Ethernet cabo RJ45 ou Wi-Fi), comuns em cozinhas de restaurantes e depósitos.
* **Escopo Técnico:**
  - Criação de `TcpTransport` em `src/transport/net.rs` conectando via `std::net::TcpStream` com timeout de conexão e escrita.
  - Porta padrão de mercado: **9100** (JetDirect / RAW TCP).
  - Feature flag opcional no `Cargo.toml`: `transport-net`.
  - Subcomando no CLI:
    ```powershell
    cargo run -- print-net --ip 192.168.1.200 --port 9100 recibo.bin
    ```

---

## 4. `feature/csharp-node-examples` (Integração & Documentação)

* **Objetivo:** Fornecer templates prontos para desenvolvedores que forem consumir a `thz_escpos.dll` ou `libthz_escpos.so`.
* **Escopo Técnico:**
  - `examples/nodejs/`: Exemplo funcional usando a biblioteca [`koffi`](https://koffi.dev/) ou `ffi-napi` carregando a DLL/SO e imprimindo recibo.
  - `examples/csharp/`: Projeto de console .NET 8 demonstrando importação via `[DllImport("thz_escpos")]` com structs e ponteiros gerenciados.

---

## Como Alternar entre as Branches

```powershell
# Para trabalhar no passthrough WSL:
git checkout exp/wsl-hardware-passthrough

# Para trabalhar nos QR Codes / Barcodes:
git checkout feature/barcodes-qrcode

# Para trabalhar no transporte de rede:
git checkout feature/network-transport

# Para trabalhar nos exemplos Node / C#:
git checkout feature/csharp-node-examples

# Para retornar à versão estável de produção:
git checkout master
```
