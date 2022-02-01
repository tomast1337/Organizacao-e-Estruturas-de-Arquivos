use std::fmt;
use std::fs::{remove_file, File};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::Path;

#[derive(Copy, Clone)]
struct RegNascimento {
    cod_municipio_nasci: [u8; 6], /* Código do Município de Nascimento */
    cod_estabelecimento: [u8; 7], /* Código do Estabelecimento */
    _cod_municipio_resi: [u8; 6], /* Código do Município de Residência */
    _data_nasc: [u8; 8],          /* Data de Nascimento no formato DDMMAAAA */
    _semanas_gestacao: [u8; 2],   /* Número de Semanas de Gestação */
    sexo: [u8; 1],                /* Sexo 0 não informado, 1 Masculino ou 2 Feminino */
    peso: [u8; 4],                /* Peso em gramas */
    _data_nasci_mae: [u8; 8],     /* Data de Nascimento no formato DDMMAAAA */
} // 6 + 7 + 6 + 8 + 2 + 1 + 4 + 8 = 42 bytes

impl RegNascimento{
    pub fn new() -> Self{
        RegNascimento {
            cod_municipio_nasci: [0; 6],
            cod_estabelecimento: [0; 7],
            _cod_municipio_resi: [0; 6],
            _data_nasc: [0; 8],
            _semanas_gestacao: [0; 2],
            sexo: [0; 1],
            peso: [0; 4],
            _data_nasci_mae: [0; 8],
        }
    }
}

impl fmt::Display for RegNascimento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cod_municipio_nasci: {}\ncod_estabelecimento: {}\ncod__municipio_resi: {}\ndata_nasc: {}\nsemanas_gestacao: {}\nsexo: {}\npeso: {}\ndata_nasci_mae: {}",
            u8_to_string(&self.cod_municipio_nasci),
            u8_to_string(&self.cod_estabelecimento),
            u8_to_string(&self._cod_municipio_resi),
            u8_to_string(&self._data_nasc),
            u8_to_string(&self._semanas_gestacao),
            u8_to_string(&self.sexo),
            u8_to_string(&self.peso),
            u8_to_string(&self._data_nasci_mae)
        )
    }
}

// Converte uma array de u8 para String
fn u8_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}

//1) Qual é o tamanho do arquivo em bytes?
fn questao_1(arquivo_original: &File) -> u64 {
    match arquivo_original.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => panic!("Erro ao ler o tamanho do arquivo: {}", e),
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
        "Quantidade de registros na capital copiados para \"sinasc-sp-capital-2018.dat\": {}",
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
    let quant_registros:usize = match arquivo_original.metadata() {
        Ok(metadata) => (metadata.len() / 42 ) as usize,
        Err(e) => panic!("Erro ao ler o tamanho do arquivo: {}", e),
    };

    let mut arr: Vec<RegNascimento> = Vec::new();    
    let mut buffer = BufReader::new(arquivo_original);
    buffer.seek(SeekFrom::Start(0)).unwrap();

    let mut registro = [0; 42];
    for _ in 0..606146 {
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };
                arr.push(ultimo_reg);
            }
            Err(e) => panic!("Erro ao ler o arquivo: {}", e),
        }
    }

    //Ordenando o arquivo
    arr.sort_by(|a, b| a.cod_estabelecimento.cmp(&b.cod_estabelecimento));
    println!("Ordenado o arquivo pela ordem do estabelecimento");

    let mut arquivo_copia = match File::create(Path::new("sinasc-sp-2018-ordenado.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao criar o arquivo: {}", e),
    };

    // Gravando dado no arquivo
    for reg in arr.iter() {
        let registro:[u8; 42] = unsafe { mem::transmute(*reg) };
        arquivo_copia.write(&registro).unwrap();
    }
    println!("Arquivo ordenado e gravado em \"sinasc-sp-2018-ordenado.dat\"")
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

    let mut contador = 0;
    let mut ultimo_estabelecimento = String::new();

    let mut buffer = BufReader::new(&mut arquivo_ordenado);
    buffer.seek(SeekFrom::Start(0)).unwrap();
    let mut registro = [0; 42];
    loop {
        match buffer.read(&mut registro) {
            Ok(size) => {
                if size == 0 {
                    break;
                } // EOF

                let ultimo_reg: RegNascimento = unsafe { mem::transmute(registro) };

                let cod_estabelecimento = u8_to_string(&ultimo_reg.cod_estabelecimento);

                if ultimo_estabelecimento != cod_estabelecimento {
                    println!("{} - {}", ultimo_estabelecimento, contador);
                    contador = 1;
                    ultimo_estabelecimento = cod_estabelecimento;
                } else {
                    contador += 1;
                }
            }
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }
}
// Exclui os arquivos sinasc-sp-capital-2018.dat, sinasc-sp-2018-ordenado.dat se existirem
fn limpar_arquivo() -> std::io::Result<()> {
    remove_file(Path::new("sinasc-sp-capital-2018.dat"))?;
    remove_file(Path::new("sinasc-sp-2018-ordenado.dat"))?;
    Ok(())
}

fn main() {
    match limpar_arquivo() {
        Ok(_) => println!("Arquivos limpos com sucesso!"),
        Err(e) => println!("Arquivos não existentes ou erro {}", e),
    }

    let mut arquivo_original = match File::open(Path::new("sinasc-sp-2018.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
    };

    //1) Qual é o tamanho do arquivo em bytes?
    let tam_arquivo = questao_1(&arquivo_original);
    println!("Tamanho do arquivo: {} bytes", tam_arquivo);

    //2) Qual é o tamanho de cada registro?
    let tamanho_registro = mem::size_of::<RegNascimento>();
    println!("Tamanho do registro: {} bytes", tamanho_registro);

    //3) Quantos registros tem o arquivo?
    let num_registros = tam_arquivo / tamanho_registro as u64;
    println!("Número de registros: {}", num_registros);

    questao_4(&mut arquivo_original, num_registros);

    questao_5(&mut arquivo_original);

    questao_6(&mut arquivo_original);

    questao_7(&mut arquivo_original);

    questao_8();
    /*
    9) Faça uma estimativa de quantos passos seriam gastos para encontrar um estabelecimento no seu arquivo gerado na questão 7.
    Justifique sua resposta. Não é necessário implementação nesse item.
    */
}
