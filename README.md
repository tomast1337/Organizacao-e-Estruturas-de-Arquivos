# Organizacao e Estruturas de Arquivos

## Busca binaria cep

compilar:
`cargo build --release`


Executar:
Da pasta do projeto `busca_binaria_cep`:
`./target/release/busca_binaria_cep <cep> <caminho para o arquivo cep_ordenado.dat>`

exemplo:
```
$ target/release/busca_binaria_cep 20770160 ../../cep_ordenado.dat
Buscando o cep 20770160 no arquivo ../cep_ordenado.dat
Tamanho do arquivo: 209792100 bytes
Iniciando a busca binária...
Cep encontrado!
logradouro: RUA CONSELHEIRO AGOSTINHO                                               
bairro: TODOS OS SANTOS                                                         
cidade: RIO DE JANEIRO                                                          
uf: RIO DE JANEIRO                                                          
sigla: RJ
cep: 20770160 
```
