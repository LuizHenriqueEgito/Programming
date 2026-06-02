use std::fs;
use std::io::{Read, Write};
use std::fs::{metadata, File};

mod arquivo {
    use super::*;

    pub fn obter_caminho_usuario() -> String {
        "./".to_string()
    }

    pub fn criar(caminho: &str, nome_arquivo: &str) {
        println!("Criando arquivo no caminho: {}", caminho);

        let caminho_completo = format!("{}/{}", caminho, nome_arquivo);

        match File::create(&caminho_completo) {
            Ok(mut arquivo) => {
                let conteudo = "Olá você!";

                match arquivo.write_all(conteudo.as_bytes()) {
                    Ok(_) => {
                        println!(
                            "Arquivo criado com sucesso no caminho: {}",
                            caminho_completo
                        );
                    }
                    Err(e) => {
                        println!("Erro ao escrever no arquivo: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Erro ao criar o arquivo: {}", e);
            }
        }
    }

    pub fn ler(caminho_completo: &str) {
        if existe(caminho_completo).is_ok() {
            match File::open(caminho_completo) {
                Ok(mut arquivo) => {
                    let mut conteudo = String::new();

                    match arquivo.read_to_string(&mut conteudo) {
                        Ok(_) => println!("Arquivo aberto: {}", conteudo),
                        Err(e) => println!("Erro ao ler conteúdo: {}", e),
                    }
                }
                Err(e) => {
                    println!("Erro ao abrir o arquivo: {}", e);
                }
            }
        }
    }

    pub fn existe(caminho_completo: &str) -> Result<(), &'static str> {
        if metadata(caminho_completo).is_ok() {
            Ok(())
        } else {
            Err("O arquivo não existe.")
        }
    }

    pub fn ler_diretorio(caminho: &str) -> Result<(), std::io::Error> {
        let items = fs::read_dir(caminho)?;

        for item in items {
            let item = item?;
            let item_caminho = item.path();

            if item_caminho.is_dir() {
                println!("Diretório: {}", item_caminho.display());
            } else {
                println!("Arquivo: {}", item_caminho.display());
            }
        }

        Ok(())
    }
}

fn main() {
    let caminho = arquivo::obter_caminho_usuario();

    let nome_arquivo = "egito.txt";
    let caminho_completo = format!("{}/{}", caminho, nome_arquivo);

    arquivo::criar(&caminho, nome_arquivo);

    match arquivo::existe(&caminho_completo) {
        Ok(_) => println!("Arquivo existe"),
        Err(msg) => println!("{}", msg),
    }

    arquivo::ler(&caminho_completo);

    match arquivo::ler_diretorio(&caminho) {
        Ok(_) => println!("Leitura OK"),
        Err(e) => println!("Falha na leitura: {}", e),
    }
}