use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

// Converter os bytes para string latin1
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}

struct Cep {
    logradouro: [u8; 72],
    bairro: [u8; 72],
    cidade: [u8; 72],
    uf: [u8; 72],
    sigla: [u8; 2],
    cep: [u8; 8],
    lixo: [u8; 2],
}

impl Cep {
    fn get_cep_value(&self) -> i32 {
        latin1_to_string(&self.cep).trim().parse::<i32>().unwrap()
    }
}

impl fmt::Display for Cep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Imprimir informações do CEP
        write!(
            f,
            "logradouro: {}\nbairro: {}\ncidade: {}\nuf: {}\nsigla: {}\ncep: {} ",
            latin1_to_string(&self.logradouro),
            latin1_to_string(&self.bairro),
            latin1_to_string(&self.cidade),
            latin1_to_string(&self.uf),
            latin1_to_string(&self.sigla),
            self.get_cep_value()
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Nenhum argumento foi passado!
        \nExemplo de uso:
        \n\t$ {} <cep> <arquivo_ordenado>",
            args.get(0).unwrap()
        );
    }

    // Verifica se o argumento passado é um numero
    let cep: i32 = match args.get(1).unwrap().parse() {
        Ok(cep) => cep,
        Err(_) => {
            println!("O CEP deve ser um número inteiro!");
            return;
        }
    };

    let arquivo = args.get(2).unwrap();

    println!("Buscando o cep {} no arquivo {}", cep, arquivo);

    let arquivo = match File::open(Path::new(&arquivo)) {
        Ok(file) => file,
        Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
    };

    let file_size = match arquivo.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => panic!("Erro ao obter o tamanho do arquivo: {}", e),
    };

    println!("Tamanho do arquivo: {} bytes", file_size);
    println!("Iniciando a busca binária...");

    let mut buf_reader = BufReader::new(arquivo);

    let tamanho_linha = 300;
    let mut inicio: u64 = 0; // Inicio da sessão de busca
    let mut fim: u64 = file_size / tamanho_linha; // Fim da sessão de busca

    while inicio < fim {
        let meio: u64 = (inicio + fim) / 2; // meio da sessão de busca

        let mut cep_atual = Cep {
            logradouro: [0; 72],
            bairro: [0; 72],
            cidade: [0; 72],
            uf: [0; 72],
            sigla: [0; 2],
            cep: [0; 8],
            lixo: [0; 2],
        };

        buf_reader
            .seek(SeekFrom::Start(meio * tamanho_linha))
            .unwrap();

        buf_reader.read_exact(&mut cep_atual.logradouro).unwrap();
        buf_reader.read_exact(&mut cep_atual.bairro).unwrap();
        buf_reader.read_exact(&mut cep_atual.cidade).unwrap();
        buf_reader.read_exact(&mut cep_atual.uf).unwrap();
        buf_reader.read_exact(&mut cep_atual.sigla).unwrap();
        buf_reader.read_exact(&mut cep_atual.cep).unwrap();
        buf_reader.read_exact(&mut cep_atual.lixo).unwrap();

        let valor = cep_atual.get_cep_value();

        if valor == cep {
            println!("Cep encontrado!");
            println!("{}", cep_atual);
            return;
        } else if valor > cep {
            fim = meio;
        } else {
            inicio = meio + 1;
        }
    }
    println!("Cep não encontrado!");
}
