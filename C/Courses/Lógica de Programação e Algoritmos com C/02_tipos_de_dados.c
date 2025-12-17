/*
Tipos de dados em C
char: 1 byte, exemplo: char letra = 'A';
int: 4 bytes, exemplo: int idade = 25;
flaot: 4 bytes, exemplo: float pi = 3.1415;
double: 8 bytes, exemplo: double numero = 2.718281828;
_Bool: 1 byte (0: falso | 1: verdadeiro), exemplo: _Bool ligado = 1; 
*/
#include <stdio.h>

int main() {
    int numero = 10;
    float numeroQuebrado = 50.49;  // C usa camelCase
    char letra = 'A';  // apenas 1 aspas
    char nome[6] = "Egito";  // aloca espaço na memória, sempre coloque 1 a mais para a linguagem saber que finalizou

    printf("Número: %d \n", numero);  // usamos %d para inteiro pois vem de decimal
    printf("Flutuante: %.2f \n", numeroQuebrado);  // .2 mostra apenas 2 casas decimais
    printf("Letra: %c \n", letra);
    printf("Nome: %s \n", nome);  //%s de string

    return 0;
}