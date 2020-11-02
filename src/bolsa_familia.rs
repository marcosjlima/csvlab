use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BolsaFamilia {
    mesreferencia: String,
    mescompetencia: String,
    uf: String,
    codigomunicipiosiafi: String,
    nomemunicipio: String,
    cpffavorecido: String,
    nisfavorecido: String,
    nomefavorecido: String,
    valorparcela: String,
}