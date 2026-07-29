#include <stdio.h>

int main() {
    for (int i = 0; i < 5; i++) {  // i++ incremento
        printf("Numero: %d\n", i);
    }

    printf("Contagem regressiva para a bomba explodir:\n");
    for (int i = 10; i >= 1; i--) {  // i-- decremento
        printf("%d...\n", i);
    }
    printf("## BOOM! ##\n");

    return 0;
}