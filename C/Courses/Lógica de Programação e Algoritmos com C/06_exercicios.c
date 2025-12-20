#include <stdio.h>
#include <stdlib.h>
/*
​1 - Apresentação do Jogador
Crie um programa que peça o primeiro nome de um jogador,
sua idade e a quantidade de gols que ele já fez na carreira.
Em seguida, exiba essas informações na tela.
*/

/*
2 - Média de Gols por Partida
Escreva um programa que receba o número total de 
gols de um jogador e a quantidade de partidas jogadas. 
Depois, calcule e exiba a média de gols por partida.
*/

/*
3 - Conversão de Minutos para Tempo de Jogo
Peça ao usuário para digitar o tempo jogado em minutos.
Depois, converta esse valor para horas e minutos e exiba o resultado.
*/

/*
4 - Calculando o Aproveitamento de um Time
Crie um programa que peça o número de vitórias, 
empates e derrotas de um time. Depois, 
calcule o número total de pontos, seguindo a regra:

Vitória = 3 pontos
Empate = 1 ponto
Derrota = 0 pontos
*/

/*
5 - Diferença de Salários entre Jogadores

Peça ao usuário os salários de dois jogadores de futebol e mostre a diferença entre eles.
*/

/*
6 - Verificação de Cartão Vermelho
Um jogador recebe cartão vermelho se receber 2 cartões amarelos no jogo.
Escreva um programa que verifica se um jogador será expulso,
baseado na quantidade de cartões amarelos.
*/

/*
7 – Verificar se um Jogador Pode Jogar
Escreva um programa que solicita ao usuário a idade de um
jogador e verifica se ele pode jogar na categoria Sub-20 
(jogadores com 20 anos ou menos). Se for mais velho, 
ele deve jogar na categoria profissional.
*/

/*
8 – Avaliar o desempenho do jogador
Escreva um programa que pede ao usuário a quantidade de
gols marcados por um jogador em uma temporada e verifica:
Se fez mais de 10 gols, exibir "Excelente temporada!"
Se fez entre 5 e 10 gols (inclusive), exibir "Boa temporada!"
Se fez menos de 5 gols, exibir "Temporada abaixo do esperado."
*/

/*
9 – Classificação do jogador baseada em idade e desempenho
Escreva um programa que solicita ao usuário:
A idade do jogador.
A quantidade de gols marcados.
O programa deve classificar o jogador nas seguintes categorias:

✅ Se tem até 20 anos e fez mais de 10 gols: "Jovem talento promissor!"
✅ Se tem até 20 anos e fez 10 gols ou menos: "Jovem em desenvolvimento."
✅ Se tem mais de 20 anos e fez mais de 15 gols: "Jogador experiente em grande fase!"
✅ Se tem mais de 20 anos e fez 15 gols ou menos: "Estevão"
*/

/*
10 – Classificação do nível da partida com base na quantidade de torcedores
Escreva um programa que solicita ao usuário:
A capacidade total do estádio.
A quantidade de torcedores presentes no jogo.
Com base na taxa de ocupação do estádio, o programa deve classificar a partida da seguinte forma:

✅ Se o estádio estiver com mais de 90% da capacidade ocupada: "Lotado!"
✅ Se estiver entre 70% e 90% (inclusive): "Ótima presença de público!"
✅ Se estiver entre 50% e 70% (inclusive): "Público razoável."
✅ Se estiver abaixo de 50%: "Morumbis"
*/
int main() {
    // EXERCICIO 1
    char nome[10];
    int idade;
    int qtde_gols;
    printf("EXERCICIO I\n");
    printf("Digite seu nome:\n");
    scanf("%9s", nome);
    printf("Digite sua idade:\n");
    scanf("%d", &idade);
    printf("Quantidade de gols feitos:\n");
    scanf("%d", &qtde_gols);
    printf("Jogador: %s | idade: %d | Qtde Gols: %d\n\n", nome, idade, qtde_gols);

    // EXERCICIO 2
    printf("EXERCICIO II\n");
    int n_partidas;
    printf("Quantas partidas você jogou?\n");
    scanf("%d", &n_partidas);
    float media_gols_partida = (float)qtde_gols / n_partidas;
    printf("Média de gols por parida: %.2f\n\n", media_gols_partida);

    printf("EXERCICIO III\n");
    int tempo_jogo;
    printf("Digite o seu tempo de jogo em minutos:");
    scanf("%d", &tempo_jogo);
    int horas = tempo_jogo / 60;
    int minutos = tempo_jogo % 60;
    printf("Horas jogadas: %d | Minutos jogados: %d\n\n", horas, minutos);

    printf("EXERCICIO IV\n");
    int resultados_partida[3];
    printf("Coloque na respectiva ordem, o número de vitórias, de empates e de derrotas do seu time: ");
    scanf(
        "%d, %d, %d",
        &resultados_partida[0],
        &resultados_partida[1],
        &resultados_partida[2]
    );
    int pontos = 3 * resultados_partida[0] + resultados_partida[1];
    printf("Seu time fez %d pontos", pontos);

    printf("\nEXERCICIO V\n");
    int income_jogador_a;
    int income_jogador_b;
    printf("Qual o salário do jogador A:");
    scanf("\n%d", &income_jogador_a);
    printf("Qual o salário do jogador B:");
    scanf("\n%d", &income_jogador_b);
    int income_diff = abs(income_jogador_a - income_jogador_b);
    printf("A diferença salarial entre os jogadores é %d", income_diff);

    printf("\nEXERCICIO VI\n");
    int n_cartoes;
    printf("Quantos cartões o jogador recebeu?");
    scanf("%d", &n_cartoes);
    if (n_cartoes > 2) {
        printf("Este jogador não pode jogar mais!\n");
    } else {
        printf("Este jogador ainda está apto para jogar\n");
    }
    printf("\nEXERCICIO VII\n");
    if (idade <= 20) {
        printf("Pode jogar na sub20\n");
    } else {
        printf("Não pode jogar na sub20\n");
    }

    printf("\nEXERCICIO VIII\n");
    if (qtde_gols >=  10) {
        printf("Excelente temporada!\n");
    } else if (qtde_gols > 5 && qtde_gols < 10) {
        printf("Boa temporada!\n");
    } else {
        printf("Temporada abaixo do esperado.\n");
    }

    printf("\nEXERCICIO IX\n");
    if (idade <= 20 && qtde_gols >  10) {
        printf("Jovem talento promissor!\n");
    } else if (idade <= 20 && qtde_gols <= 10) {
        printf("Jovem em desenvolvimento.\n");
    } else if (idade > 20 && qtde_gols > 15) {
        printf("Jogador experiente em grande fase!\n");
    } else {
        printf("Jogador comum.\n");
    }

    printf("\nEXERCICIO X\n");
    int capacidade;
    int qtde_torcedores;
    printf("Qual é a capacidade do Estádio?\n");
    scanf("%d", &capacidade);
    printf("Qual foi o número de torcedores presente?\n");
    scanf("%d", &qtde_torcedores);

    float razao = (float)qtde_torcedores / capacidade;
    printf("Lotação do estadio: %.2f", razao);
    
    if (razao > 0.9) {
        printf("Lotado!");
    } else if (razao > 0.7 && razao <= 0.9) {
        printf("Ótima presença de público!");
    } else if (razao > 0.5 && razao <= 0.7) {
        printf("Público razoável.");
    } else {
        printf("Jogo do cruzeiro");
    }
    return 0;
}