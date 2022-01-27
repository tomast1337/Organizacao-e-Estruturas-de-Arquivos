typedef struct _sinasc sinasc;

struct _sinasc {
    char CODMUNNASC[6]; /* Código do Município de Nascimento */
    char CODESTAB[7]; /* Código do Estabelecimento */
    char CODMUNRES[6]; /* Código do Município de Residência */
    char DTNASC[8]; /* Data de Nascimento no formato DDMMAAAA */
    char SEMAGESTAC[2]; /* Número de Semanas de Gestação */
    char SEXO[1]; /* Sexo 0 não informado, 1 Masculino ou 2 Feminino */
    char PESO[4]; /* Peso em gramas */
    char DTNASCMAE[8]; /* Data de Nascimento no formato DDMMAAAA */
};