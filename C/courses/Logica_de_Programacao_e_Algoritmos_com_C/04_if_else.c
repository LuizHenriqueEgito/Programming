// && -> and
// || -> or
#include <stdio.h>


int main() {
    int idade;
    printf("Digite a sua idade: \n");
    scanf("%d", &idade);

    if (idade >= 18) {
        printf("Mario\n");
    } else {
        printf("Menor\n");
    }
    return 0;
}