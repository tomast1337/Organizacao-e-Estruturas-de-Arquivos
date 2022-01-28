use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom,Write};
use std::mem;
use std::path::Path;
use std::collections::HashMap;

struct RegNascimento {
    cod_municipio_nasci: [u8; 6], /* Código do Município de Nascimento */
    cod_estabelecimento: [u8; 7], /* Código do Estabelecimento */
    cod_municipio_resi: [u8; 6], /* Código do Município de Residência */
    data_nasc: [u8; 8], /* Data de Nascimento no formato DDMMAAAA */
    semanas_gestacao: [u8; 2], /* Número de Semanas de Gestação */ 
    sexo: [u8; 1], /* Sexo 0 não informado, 1 Masculino ou 2 Feminino */
    peso: [u8; 4], /* Peso em gramas */
    data_nasci_mae: [u8; 8], /* Data de Nascimento no formato DDMMAAAA */
} // 6 + 7 + 6 + 8 + 2 + 1 + 4 + 8 = 42 bytes

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

fn main() {
    let arquivo_original = match File::open(Path::new("sinasc-sp-2018.dat")) {
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

    /*
    4) Copie em um novo arquivo chamado “sinasc-sp-capital-2018.dat”
    os registros dos nascimentos (CODMUNNASC) que ocorreram na capital,
    cujo código é “355030”.  Quantos registros tem esse novo arquivo?
    */

    let mut arquivo_copia = match File::create(Path::new("sinasc-sp-capital-2018.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao criar o arquivo: {}", e),
    };

    let mut buffer = BufReader::new(arquivo_original);
    let mut registro = [0; 42];
    let mut contador = 0;
    
    for _ in 0..num_registros {
        match buffer.read(&mut registro) {
            Ok(42) => {
                let ultimo_reg:RegNascimento = unsafe { mem::transmute(registro) };
                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "355030" {
                    contador += 1;
                    arquivo_copia.write(&registro).unwrap();
                }
            },
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }

    print!("Quantidade de registros na capital: {}", contador);
    
    /*
    5) Quantas meninas nasceram em Santos (354850) no ano de 2018?
    */
    
    //Mover o cursor para o início do arquivo
    buffer.seek(SeekFrom::Start(0)).unwrap();
    //Zerar o contador
    contador = 0;

    for _ in 0..num_registros {
        match buffer.read(&mut registro) {
            Ok(42) => {
                let ultimo_reg:RegNascimento = unsafe { mem::transmute(registro) };
                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "354850" 
                && u8_to_string(&ultimo_reg.sexo) == "2" {
                    contador += 1;
                }
            },
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }

    println!("\nQuantidade de meninas nasceram em Santos (354850) no ano de 2018: {}", contador);

    /*
    6) Quantos bebês nasceram com baixo peso (< 2500) em campinas (350950) no ano de 2018?
    */

    //Mover o cursor para o início do arquivo
    buffer.seek(SeekFrom::Start(0)).unwrap();
    //Zerar o contador
    contador = 0;
    for _ in 0..num_registros {
        match buffer.read(&mut registro) {
            Ok(42) => {
                let ultimo_reg:RegNascimento = unsafe { mem::transmute(registro) };
                if u8_to_string(&ultimo_reg.cod_municipio_nasci) == "350950"{
                    let peso: u32 = u8_to_string(&ultimo_reg.peso).parse().unwrap();
                    if peso < 2500 {
                        contador += 1;
                    }
                }
            },
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }


    /*
    7) Ordene o arquivo pelo código do estabelecimento, gere o arquivo “sinasc-sp-2018-ordenado.dat”.
    Não é para fazer ordenação externa.
    */

    //Mover o cursor para o início do arquivo
    buffer.seek(SeekFrom::Start(0)).unwrap();
    //Ler todos os código do estabelecimento em uma mapa com o código do estabelecimento e o indice do registro
    let mut mapa_estabelecimento: HashMap<String, u64> = HashMap::new();
    for i in 0..num_registros {
        match buffer.read(&mut registro) {
            Ok(42) => {
                let ultimo_reg:RegNascimento = unsafe { mem::transmute(registro) };
                mapa_estabelecimento.insert(u8_to_string(&ultimo_reg.cod_estabelecimento), i);
            },
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
    }

    //Ordenar o mapa pelo código do estabelecimento

    let mut mapa_ordenado: Vec<(String, u64)> = mapa_estabelecimento.into_iter().collect();
    mapa_ordenado.sort_by(|a, b| a.0.cmp(&b.0));

    //Criar um novo arquivo movendo os registros para o novo arquivo a partir do mapa ordenado
    let mut arquivo_ordenado = match File::create(Path::new("sinasc-sp-2018-ordenado.dat")) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao criar o arquivo: {}", e),
    };

    for (cod_estabelecimento, indice) in mapa_ordenado {
        buffer.seek(SeekFrom::Start(indice * 42)).unwrap();
        let mut registro_ordenado = [0; 42];
        match buffer.read(&mut registro_ordenado) {
            Ok(42) => {
                arquivo_ordenado.write(&registro_ordenado).unwrap();
            },
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Erro ao ler o registro: {}", e),
        }
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

    /*
    9) Faça uma estimativa de quantos passos seriam gastos para encontrar um estabelecimento no seu arquivo gerado na questão 7.
    Justifique sua resposta. Não é necessário implementação nesse item. 
    */
}
