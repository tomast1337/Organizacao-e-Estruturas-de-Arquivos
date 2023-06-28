#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// clang "questao1.c" -o "questao1" && "./questao1" "test.txt"
// gcc "questao1.c" -o "questao1" && "./questao1" "test.txt"
int main(int arcg, char *argv[])
{
    printf("Questão 1\n");
    printf("Conversor de arquivos de texto de Unix para Windows\n");

    // Leia o arquivo de entrada
    const char *filename = argv[1]; // pegando o nome do arquivo dos argumentos
    printf("Arquivo de entrada: %s\n", filename);

    // abrindo o arquivo
    const FILE *file = fopen(filename, "r");
    if (file == NULL)
    {
        printf("Erro ao abrir o arquivo!\n");
        return 1;
    }
    printf("Arquivo aberto com sucesso!\n");

    // Criando o arquivo de Saida
    const char *saídaName = strcat(filename, ".out"); // concatenando o nome do arquivo de entrada com .out
    const FILE *output = fopen(saídaName, "w");
    if (output == NULL)
    {
        printf("Erro ao criar o arquivo de saída!\n");
        return 1;
    }
    printf("Arquivo de saída: %s\n", saídaName);

    int line_size = 256;
    char line[line_size]; // buffer para ler as linhas do arquivo
    // Pra cada linha do arquivo de entrada, troar o \n por \n\r e escrever no arquivo de saída
    while (
        fgets(line, line_size, file) != NULL) // lendo uma linha do arquivo de entrada
    {
        // removendo o \n do final da linha
        char *pos;
        if ((pos = strchr(line, '\n')) != NULL)
            *pos = '\0';
        // concatenando o \n\r no final da linha
        char *newLine = strcat(line, "\r\n");
        // escrevendo a linha no terminal, só pra debugar
        printf("%s", line);
        // escrevendo a linha no arquivo de saída
        fputs(newLine, output);
    }

    // fechando os arquivos
    fclose(file);
    fclose(output);

    printf("Arquivo convertido com sucesso!\n");

    return 0;
}
