 // qual espaço de memoria vamos alocar
 /*
 # Static
 Memoria Estatica:
 - Binario do programa
 - Static Variables
 - String Literals

 Lifetime:
 - Todo programa

 Cleanup:
 - Quando o programa termina

 # Stack
Memoria da Stack:
- Funções e Argumentos
- Local Variaveis
- Cada threading tem uma stack isolada

Lifetime:
- Funções

Cleanup:
- Quando a função retorna

# Heap
Memoria do Heap:
- Valores que precisam viver depois da função
- Compartilhado entre Threads
- Valores grandes
- Tamanhos Dinamicos

Lifetime:
- Definido pelo programador

Cleanup:
- Via RAII
O tamanho da HEAP depende da maquina em que o programa está rodando 
*/

// Static
static _Y: u32 = 13;

fn main() {
    // Stack
    let x = 5;
    let x = true;
    let numbers = [1, 2, 3];

    // Heap
    let users = ...;  // não sabemos de antemão o tamanho pois é dinamico então precisa ir para a HEAP
}