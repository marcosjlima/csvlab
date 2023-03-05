# csvlab
Antes de executar alterar o header do arquivo 202001_BolsaFamilia_Pagamentos.csv para o exemplo abaixo:

  "Mesreferencia";"Mescompetencia";"Uf";"Codigomunicipiosiafi";"Nomemunicipio";"Cpffavorecido";"Nisfavorecido";"Nomefavorecido";"Valorparcela"

Compilar

  $ cargo build && cargo build --release 

Executar

  $ ./target/release/csvlab c:\temp\202001_BolsaFamilia_Pagamentos.csv



		String eventos = "1234;4321;2528";
		String horarioExecucao = "18:00;07:59";      
		String[] horarios = horarioExecucao.split(";");
		LocalDateTime currentTime = LocalDateTime.now();
		
		LocalDateTime datainicial = LocalDateTime.of(currentTime.getYear(), 
			currentTime.getMonth(), 
			currentTime.getDayOfMonth(), 
			Integer.valueOf(horarios[0].split(":")[0]), 
			Integer.valueOf(horarios[0].split(":")[1]),
			0);

		LocalDateTime dataFinal = datainicial.plusDays(1);

		dataFinal = LocalDateTime.of(dataFinal.getYear(), 
			dataFinal.getMonth(), dataFinal.getDayOfMonth(), 
			Integer.valueOf(horarios[1].split(":")[0]), 
			Integer.valueOf(horarios[1].split(":")[1]), 
			0);

		boolean horarioValido = currentTime.isAfter(datainicial) && currentTime.isBefore(dataFinal);
		boolean contemEvento = eventos.contains("2528");

		System.out.println(String.format("%s > %s < %s %s %s", datainicial, currentTime, dataFinal, horarioValido, contemEvento));
