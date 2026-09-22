# Customizações do Agente (.agents)

Esta pasta contém as configurações, regras e habilidades personalizadas para os agentes de IA (Antigravity) que trabalham neste repositório.

## Estrutura

- **[`rules/`](./rules)**: Regras e padrões de projeto aplicados continuamente pelo agente.
  - [`rust-standards.md`](./rules/rust-standards.md): Diretrizes de estilo, boas práticas em Rust e convenções do projeto.
- **[`skills/`](./skills)**: Habilidades modulares carregadas sob demanda quando um assunto específico é abordado.
  - [`escpos-helper/SKILL.md`](./skills/escpos-helper/SKILL.md): Base de conhecimento e cheatsheet de bytes para o protocolo ESC/POS.
- **`plugins/`** (opcional): Pacotes distribuíveis que agrupam regras, skills e servidores MCP.
- **`hooks.json`** (opcional): Gatilhos de automação (execução de scripts pré/pós ferramentas).
- **`mcp_config.json`** (opcional): Configuração de servidores do protocolo MCP.

Para criar novas habilidades ou regras, consulte o formato com cabeçalho YAML nos arquivos existentes.
