# RUST

## Como dar nome as variáveis:
- **Rust** usa `snake_case` para *variáveis/funções* `camelCase` para *structs/enums* e `UPPER_SNAKE_CASE` para *constantes*  (é como o `Python`).

## Como criar um novo projeto com **cargo**?
```bash
cargo new nome_projeto
```
Explicação dos arquivos criados:
- `cargo.toml`: Semelhante ao `pyproject.toml` onde você encontra os metadados do projeto e as bibliotecas;
- `src/main.rs`: Arquivo principal do projeto. Por padrão ele vem com um println!;

## Como rodar um projeto em Rust?
```bash
cargp run  # dentro do src do seu projeto
```

## Como *compilar* meu script Rust?
```bash
cargo build
```