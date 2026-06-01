# Criando uma Biblioteca com Rust
## 1º Código
```bash
cargo init --lib
```
Cria o arquivo `lib.rs`
- Uma `lib` é como um modulo (por isso coloque *pub* no que precisar)

## 2º Criando conta no crates.io
```bash
cargo login  # se autentique
```

## 3º Arquivo Manisto
No arquivo `cargo.toml`:
```toml
[packages]
name = "nome do pacote"
version = "0.1.0"
edition = "2021"
authors = ["Luiz <luiz@gmail.com>", "Egito <egito@gmail.com>"]
description = "Biblioteca XPTO"
homepage = "link"
repository = "link"
documentation = "link"
licence = "MIT"

keywords = ["lib", "xpto", "funcs"]
categories = ["functions"]

[dependencies]
```
Depois rode:
```bash
cargo publish
```

## 4º Usando a Biblioteca
Rode no terminal:
```bash
cargo add nome_da_lib
```
ou adicione a linha no seu `Cargo.toml`:
```toml
[dependencies]
nome_da_lib = "0.1.0"
```
