#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// clang "questao2.c" -o "questao2" && "./questao2" "test.txt"
// gcc "questao2.c" -o "questao2" && "./questao2" "test.txt"
int main(int arcg, char *argv[])
{
    printf("Questão 2\n");
    printf("Contador de caracteres/bytes de um arquivo de texto\n");
    int counts[256] = {0}; // array de contagem de caracteres
    
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

    int line_size = 256;
    char line[line_size]; // buffer para ler as linhas do arquivo
    // pra cada linha do arquivo de entrada, contar os caracteres
    while (
        fgets(line, line_size, file) != NULL) // lendo uma linha do arquivo de entrada
    {
        int tamanho_linha = strlen(line);
        for (int i = 0; i < tamanho_linha; i++)
        {
            // converter o char para int
            int pos = (int)line[i]; // esse valor é o index do array counts
            counts[pos]++; // incrementando o contador
        }   
    }

    // fechando o arquivo
    fclose(file);

    // prints do resultado
    for (int i = 0; i < 256; i++)
    {
        printf("%c: %d\n", i, counts[i]);
    }
}
