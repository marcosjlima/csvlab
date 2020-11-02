# csvlab
Antes de executar alterar o header do arquivo 202001_BolsaFamilia_Pagamentos.csv para o exemplo abaixo:

  "Mesreferencia";"Mescompetencia";"Uf";"Codigomunicipiosiafi";"Nomemunicipio";"Cpffavorecido";"Nisfavorecido";"Nomefavorecido";"Valorparcela"

Compilar

  $ cargo build && cargo build --release 

Executar

  $ ./target/release/csvlab c:\temp\202001_BolsaFamilia_Pagamentos.csv
