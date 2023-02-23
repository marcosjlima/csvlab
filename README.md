# csvlab
Antes de executar alterar o header do arquivo 202001_BolsaFamilia_Pagamentos.csv para o exemplo abaixo:

  "Mesreferencia";"Mescompetencia";"Uf";"Codigomunicipiosiafi";"Nomemunicipio";"Cpffavorecido";"Nisfavorecido";"Nomefavorecido";"Valorparcela"

Compilar

  $ cargo build && cargo build --release 

Executar

  $ ./target/release/csvlab c:\temp\202001_BolsaFamilia_Pagamentos.csv


https://www.regular-expressions.info/unicode.html#prop

String valor = "[\\p{M} \\p{Z}\\p{Zl}\\p{Zs}\\p{Zp}\\p{Lm}\\p{Sm}\\p{Sc}\\p{Sk}\\p{So}\\p{C}\\p{Cc}\\p{Cf}\\p{Pd}@,;'\"]";

String teste = "=A@\rOla	Mundao\r+,-; Teste 12344 10.2 çã 0x0D ' * $ . \\ / \"";
