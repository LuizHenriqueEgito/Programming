#include <stdio.h>

int main() {
    int numero;
    printf("Digite um numero inteiro: ");
    // escaneia o que o usuario colocou
    scanf("%d", &numero);  // &numero: referencia o valor passado em numero (grava o valor)
    printf("O número foi o: %d\n", numero);

    return 0;   
}