# Regras de Desenvolvimento em Rust

Diretrizes e restrições a serem seguidas ao criar ou modificar código Rust neste projeto.

## 1. Padrões de Código e Estilo
- Siga estritamente as convenções idiomáticas do Rust (Rust API Guidelines).
- Execute `cargo fmt` para formatação uniforme.
- O código deve compilar sem warnings com `cargo clippy --all-targets --all-features`.

## 2. Tratamento de Erros e Segurança
- **Proibido Panic em Lib:** Evite `panic!`, `unwrap()` e `expect()` em módulos de biblioteca. Utilize `Result<T, E>` propagando erros com o operador `?`.
- Modele erros com `enum` próprios ou via crates como `thiserror` para a biblioteca, e `anyhow` para binários/CLI.

## 3. Comandos ESC/POS e Manipulação de Bytes
- Represente sequências de comandos ESC/POS com tipos fortemente tipados (enums, structs e builders).
- Assegure que geradores de comandos retornem vetores de bytes (`Vec<u8>` ou slices `&[u8]`).
- Todo comando criado deve possuir teste unitário validando a saída exata de bytes contra a especificação ESC/POS.

## 4. Documentação
- Comente todas as structs, enums e métodos públicos utilizando doc-comments (`///`).
- Forneça exemplos simples (`/// # Examples`) nas funções da API pública.
