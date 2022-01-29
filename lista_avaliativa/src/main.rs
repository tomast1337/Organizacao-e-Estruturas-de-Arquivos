use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::Path;

struct RegNascimento {
    cod_municipio_nasci: [u8; 6], /* Código do Município de Nascimento */
    cod_estabelecimento: [u8; 7], /* Código do Estabelecimento */
    cod_municipio_resi: [u8; 6],  /* Código do Município de Residência */
    data_nasc: [u8; 8],           /* Data de Nascimento no formato DDMMAAAA */
    semanas_gestacao: [u8; 2],    /* Número de Semanas de Gestação */
    sexo: [u8; 1],                /* Sexo 0 não informado, 1 Masculino ou 2 Feminino */
    peso: [u8; 4],                /* Peso em gramas */
    data_nasci_mae: [u8; 8],      /* Data de Nascimento no formato DDMMAAAA */
} // 6 + 7 + 6 + 8 + 2 + 1 + 4 + 8 = 42 bytes

// Converte uma array de u8 para String
fn u8_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}

impl fmt::Display for RegNascimento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cod_municipio_nasci: {}\ncod_estabelecimento: {}\ncod__municipio_resi: {}\ndata_nasc: {}\nsemanas_gestacao: {}\nsexo: {}\npeso: {}\ndata_nasci_mae: {}",
            u8_to_string(&self.cod_municipio_nasci),
            u8_to_string(&self.cod_estabelecimento),
            u8_to_string(&self.cod_municipio_resi),
            u8_to_string(&self.data_nasc),
            u8_to_string(&self.semanas_gestacao),
            u8_to_string(&self.sexo),
            u8_to_string(&self.peso),
            u8_to_string(&self.data_nasci_mae)
        )
    }
}

/*
    4) Copie em um novo arquivo chamado “sinasc-sp-capital-2018.dat”
    os registros dos nascimentos (CODMUNNASC) que ocorreram na capital,
    cujo código é “355030”.  Quantos registros tem esse novo arquivo?
*/
fn questao_4(arquivo_original: &mut File, num_registros: u64) {
    let mut arquivo_copia = match File::create(Path::new("sinasc-sp-capital-2018.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao criar o arquivo: {}", e),
    };
    let mut buffer = BufReader::new(arquivo_original);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    let mut contagem_nasci_capital = 0;

    let mut registro = [0; 42];
    for _ in 0..num_registros {
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };

                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "355030" {
                    contagem_nasci_capital += 1;
                    arquivo_copia.write(&registro).unwrap();
                }
            }
            Err(e) => panic!("Erro ao ler o arquivo: {}", e),
        }
    }

    println!(
        "Quantidade de registros na capital: {}",
        contagem_nasci_capital
    )
}

/*
    5) Quantas meninas nasceram em Santos (354850) no ano de 2018?
*/
fn questao_5(arquivo_original: &mut File) {
    let mut contagem = 0;
    let mut buffer = BufReader::new(arquivo_original);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    loop {
        let mut registro = [0; 42];
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };

                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "354850"
                    && u8_to_string(&ultimo_reg.sexo) == "2"
                {
                    contagem += 1;
                }
            }
            Err(e) => panic!("Erro ao ler o arquivo: {}", e),
        }
    }

    println!(
        "Quantidade de meninas nasceram em Santos (354850) no ano de 2018: {}",
        contagem
    );
}

/*
    6) Quantos bebês nasceram com baixo peso (< 2500) em campinas (350950) no ano de 2018?
*/
fn questao_6(arquivo_original: &mut File) {
    let mut contagem = 0;
    let mut buffer = BufReader::new(arquivo_original);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    loop {
        let mut registro = [0; 42];
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };

                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "350950" {
                    let peso: u32 = u8_to_string(&ultimo_reg.peso).parse().unwrap();
                    if peso < 2500 {
                        contagem += 1;
                    }
                }
            }
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }

    println!(
        "Quantidade de bebês nasceram com baixo peso (< 2500) em campinas (350950) no ano de 2018: {}",
        contagem
    );
}

/*
    7) Ordene o arquivo pelo código do estabelecimento, gere o arquivo “sinasc-sp-2018-ordenado.dat”.
    Não é para fazer ordenação externa.
*/
fn questao_7(arquivo_original: &mut File) {
    // Mapa guarda o código do estabelecimento,u64 , e o index, u64, no arquivo original
    let mut mapa_estabelecimento: HashMap<u64, u64> = HashMap::new();
    let mut buffer = BufReader::new(arquivo_original);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    loop {
        let mut registro = [0; 42];
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };
                println!("{}", ultimo_reg);
                let cod_estabelecimento = u8_to_string(&ultimo_reg.cod_estabelecimento)
                    .parse()
                    .unwrap();
                let position = buffer.seek(SeekFrom::Current(0)).unwrap();
                mapa_estabelecimento.insert(cod_estabelecimento, position);
            }
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }
    println!(
        "Número de registros a serem ordenados: {}",
        mapa_estabelecimento.len()
    );
    // Ordena o mapa pelo código do estabelecimento
    //Transforma o mapa em uma lista de tuplas que é ordenável
    let mut mapa_ordenado: Vec<(&u64, &u64)> = mapa_estabelecimento.iter().collect();
    // Ordena a lista de tuplas
    mapa_ordenado.sort_by(|a, b| a.0.cmp(b.0));

    // Cria o arquivo para ordenar
    let mut arquivo_ordenado = match File::create(Path::new("sinasc-sp-2018-ordenado.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao criar o arquivo: {}", e),
    };

    // Copia o arquivo original para o arquivo ordenado a partir da lista ordenada
    for (_, index) in mapa_ordenado {
        let mut registro = [0; 42];
        match buffer.seek(SeekFrom::Start(*index)) {
            Ok(_) => {}
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF
                arquivo_ordenado.write(&registro).unwrap();
            }
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }
    println!("Dados organizados com sucesso e armazenados em 'sinasc-sp-2018-ordenado.dat'!");
}

/*
    8) Com o arquivo ordenado, conte o número de nascimentos por estabelecimento.
    Leia o primeiro registro e atribua ao contador
    1. Enquanto não for final do arquivo,
    leia os registros subsequentes sempre guardando o código do estabelecimento do registro anterior.

    Quando o estabelecimento mudar ou quando o final do arquivo for alcançado,
    imprima o contador. Se o registro lido tiver o mesmo código do estabelecimento do anterior,
    apenas acrescente 1 unidade ao contador, sem imprimir.
*/
fn questao_8() {
    let mut arquivo_ordenado = match File::open("sinasc-sp-2018-ordenado.dat") {
        Ok(file) => file,
        Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
    };

    let mut buffer = BufReader::new(&mut arquivo_ordenado);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    // Mapa guarda o código do estabelecimento,u64 , e a quantidade de nascimentos no estabelecimento
    let mut contador: HashMap<u64, u64> = HashMap::new();
    loop {
        let mut registro = [0; 42];
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };
                let cod_estabelecimento = u8_to_string(&ultimo_reg.cod_estabelecimento)
                    .parse()
                    .unwrap();
                if contador.contains_key(&cod_estabelecimento) {
                    let valor = contador.get_mut(&cod_estabelecimento).unwrap();
                    *valor += 1;
                } else {
                    contador.insert(cod_estabelecimento, 1);
                }
            }
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }
}

fn main() {
    let mut arquivo_original = match File::open(Path::new("sinasc-sp-2018.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
    };

    let tam_arquivo = match arquivo_original.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => panic!("Erro ao ler o tamanho do arquivo: {}", e),
    };
    //1) Qual é o tamanho do arquivo em bytes?
    println!("Tamanho do arquivo: {} bytes", tam_arquivo);

    let tamanho_registro = mem::size_of::<RegNascimento>();
    //2) Qual é o tamanho de cada registro?
    println!("Tamanho do registro: {} bytes", tamanho_registro);

    let num_registros = tam_arquivo / tamanho_registro as u64;
    //3) Quantos registros tem o arquivo?
    println!("Número de registros: {}", num_registros);

    questao_4(&mut arquivo_original, num_registros);

    questao_5(&mut arquivo_original);

    questao_6(&mut arquivo_original);

    questao_7(&mut arquivo_original);

    //questao_8();

    /*
    9) Faça uma estimativa de quantos passos seriam gastos para encontrar um estabelecimento no seu arquivo gerado na questão 7.
    Justifique sua resposta. Não é necessário implementação nesse item.
    */
}
