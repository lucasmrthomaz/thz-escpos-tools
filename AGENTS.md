# Antigravity & Agent Guidelines: thz-escpos-tools

Diretrizes gerais para agentes de IA atuando neste repositório.

## Visão do Projeto
O **`thz-escpos-tools`** é uma suíte e biblioteca modular em Rust (Edition 2024) para manipulação, geração, renderização e envio de comandos ESC/POS diretamente para impressoras térmicas (58mm e 80mm).

O projeto é baseado nas descobertas e validações técnicas do projeto irmão de referência **`thermal-probe`** (`THZ ThermalKit`), focando em:
1. **Comunicação Direta (Driverless / Direct Transport):**
   - No Windows, acesso direto ao hardware via handle do `usbprint.sys` (`\\?\usb#...`) e portas seriais/Bluetooth SPP via `serialport`.
   - Elimina dependência de spooler do Windows ou drivers POS-58 obsoletos.
   - Descoberta de hardware via Windows SetupAPI (`USBPRINT`, `COMPORT`, `USB_DEVICE`).
2. **Protocolo ESC/POS e Codificação:**
   - Suporte nativo a texto acentuado em português via seleção de Code Page 860 (`ESC t 3` / `0x1B, 0x74, 0x03`).
   - Raster bitonal para imagens e gráficos via `GS v 0` (`0x1D, 0x76, 0x30, 0x00, xL, xH, yL, yH, [...]`) com redimensionamento e limiarização.
   - Comandos de controle (inicialização `ESC @`, quebras `LF`, alinhamento `ESC a`, negrito `ESC E`, cortes `GS V`).
3. **Perfis de Impressora (JSON):**
   - Suporte a perfis de configuração física (largura útil em dots ex: 384 ou 576, baud rate, VID/PID, modo de transporte).
4. **Segurança de Hardware:**
   - Ações que enviem dados físicos para impressoras devem exigir confirmação explícita ou proteção contra envios acidentais.

## Diretrizes de Engenharia
- **Linguagem & Edição:** Rust (Edition 2024).
- **Tratamento de Erros:** Não utilize `unwrap()` ou `expect()` em código de biblioteca; utilize tipos de erro descritivos e tipados (`Result<T, E>`).
- **Padrão de Código:** O código deve sempre respeitar o `cargo fmt` e passar sem alertas no `cargo clippy`.
- **Modularidade:** Manter separadas as camadas de:
  - `protocol`: geração pura de bytes ESC/POS e encoders (CP860, etc.).
  - `graphics`: redimensionamento, dithering/thresholding e empacotamento raster.
  - `profile`: estruturas e desserialização de perfis JSON.
  - `transport`: abstração de envio (USBPRINT direct Win32 handle, Serial COM, Mock).
- **Testes:** Priorize testes unitários para a geração correta de bytes de cada comando ESC/POS.

## Fluxo de Versionamento e Git
- **Commits Atômicos:** A cada etapa ou funcionalidade concluída, realize commits atômicos, claros e com escopo bem definido (Conventional Commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`), garantindo rastreabilidade precisa e facilitando eventuais rollbacks ou bisects. Evite acumular muitas alterações em um único commit gigante.
- **Branches para Marcos e Funcionalidades:** Sugira e utilize branches dedicadas (ex: `feature/nova-funcionalidade` ou `exp/teste-hardware`) para desenvolvimentos em andamento ou recursos experimentais, mantendo a branch principal (`master`/`main`) sempre estável e testada.

