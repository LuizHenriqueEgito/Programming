use std::fs::File;
use std::io::{self, Read};


fn main() {
    let resultado = ler_arquivo("/home/smith/Documentos/Dev/Programming/Rust/courses/rust_alem_do_basico/05_tratamento_de_erro/00_panic.rs");
    match resultado {
        Ok(conteudo) => {
            println!("Conteudo do arquivo: \n{}", conteudo);
        },
        Err(erro) => {
            println!("Erro ao ler o arquivo:\n {}", erro);
        }

    }
}


fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    // o ? faz o match direto se for Ok continua se for Err já retorna o Erro
    let mut arquivo = File::open(caminho)?;
    let mut conteudo = String::new();
    let _ = arquivo.read_to_string(&mut conteudo);
    Ok(conteudo)
}