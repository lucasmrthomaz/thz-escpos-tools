# Regras de Git e Versionamento

Diretrizes para gerenciamento de código e histórico no repositório `thz-escpos-tools`.

## 1. Commits Atômicos e Incrementais
- Cada commit deve representar **uma única unidade lógica de alteração** (ex: adição de um comando de protocolo, ajuste no driver USB, novo teste unitário).
- **Não acumule múltiplas fases ou reescritas amplas em um único commit gigante.**
- Mensagens devem seguir o padrão **Conventional Commits**:
  - `feat(...)`: Nova funcionalidade ou módulo.
  - `fix(...)`: Correção de bug ou ajuste de comportamento.
  - `test(...)`: Adição ou modificação de testes unitários/integração.
  - `refactor(...)`: Refatoração sem alteração de funcionalidade externa.
  - `docs(...)`: Documentação, README, SKILLs ou regras.

## 2. Estratégia de Branches
- A branch `master` (ou `main`) deve ser mantida sempre em estado estável e com testes verdes.
- Para qualquer nova funcionalidade de médio ou grande porte, ou testes com hardware físico experimental:
  - Sugerir a criação de branch: `feature/<nome-da-feature>` ou `exp/<nome-do-experimento>`.
  - Desenvolver e validar incrementalmente na branch com commits atômicos.
  - Mesclar (merge/PR) na branch principal após validação e testes aprovados.
