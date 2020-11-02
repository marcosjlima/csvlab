use chrono::Local;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub mod bolsa_familia;
pub mod gerar_beneficios;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    println!("Inicio: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    match readfile(&args[1]) {
        Ok(content) => tratar_beneficios(content),
        Err(err) => {
            println!("Erro: {:?}", err);
            process::exit(1);
        }
    }

    println!("Fim: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

fn tratar_beneficios(content: String) {
    match gerar_beneficios::gerar(content) {
        Ok(beneficios) => {
            println!("Total de beneficios: {}", beneficios.len());
        }
        Err(err) => {
            println!("Erro: {:?}", err);
            process::exit(1);
        }
    }
}

fn readfile(filename: &String) -> Result<String, Box<dyn Error>> {
    println!("File Name {}", filename);

    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}