use crate::bolsa_familia::BolsaFamilia;
use std::error::Error;

pub fn gerar(data: String) -> Result<Vec<BolsaFamilia>, Box<dyn Error>> {
    let bolsa_familia: Vec<BolsaFamilia> = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes())
        .into_deserialize()
        .collect::<Result<Vec<BolsaFamilia>, csv::Error>>()?;

    Ok(bolsa_familia)
}

pub fn gerar_01(data: String) -> Result<Vec<BolsaFamilia>, Box<dyn Error>> {
    let mut bolsa_familia: Vec<BolsaFamilia> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    for registro in rdr.deserialize() {
        let beneficio: BolsaFamilia = registro?;
        bolsa_familia.push(beneficio);
    }

    Ok(bolsa_familia)
}

pub fn gerar_02(data: String) -> Result<Vec<BolsaFamilia>, Box<dyn Error>> {
    let mut bolsa_familia: Vec<BolsaFamilia> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    let mut raw_record = csv::ByteRecord::new();
    let headers = rdr.byte_headers()?.clone();

    while rdr.read_byte_record(&mut raw_record)? {
        let beneficio: BolsaFamilia = raw_record.deserialize(Some(&headers))?;
        bolsa_familia.push(beneficio);
    }

    Ok(bolsa_familia)
}