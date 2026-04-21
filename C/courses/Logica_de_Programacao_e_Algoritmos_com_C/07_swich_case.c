#include <stdio.h>

int main() {
    int option = 1;
    if (option == 1) {
        printf("Novo Jogo\n");
    } else if (option == 2) {
        printf("Carregar jogo\n");
    } else if (option == 3) {
        printf("Sair\n");
    }
    

    // usando switch case
    switch (option) {
        case 1:
            printf("Fase numero 1\n");
            break;
        case 2:
            printf("Fase numero 2\n");
            break;
        case 3:
            printf("Fase numero 3\n");
            break;
        default:
            printf("Opcao invalida");
    }

    printf("Exemplo:");
    int day;
    printf("Digite um numero de 1 a 7: ");
    scanf("%d", &day);

    switch (day) {
        case 1: printf("Domingo\n"); break;
        case 2: printf("Segunda\n"); break;
        case 3: printf("Terça\n"); break;
        case 4: printf("Quarta\n"); break;
        case 5: printf("Quinta\n"); break;
        case 6: printf("Sexta\n"); break;
        case 7: printf("Sabado\n"); break;
        default: printf("Numero invalido!\n");
    }


    return 0;
}