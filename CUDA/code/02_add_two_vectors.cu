// PARA COMPILAR: nvcc 00_adding_two_vectors.cu -o 00_adding_two_vectors
// PARA RODAR: ./00_adding_two_vectors

// para imprimir no terminal
#include <iostream>

// Kernel (__global__ faz rodar na GPU)
// essa função sempre retorna void
__global__ void addVectors(int *v, int *u, int *w, int n) {
    // cada thread calcula um índice diferente
    // i: é a posição que a thread vai calcular
    // é aqui que o paralelismo começa, estamos mapeando cada thread
    // para elas saberem onde vão trabalhar
    // blockIdx.x: Qual bloco
    // blockDim.x: Quantas threads por bloco
    // threadIdx.x: Qual thread dentro do bloco
    /*
    Por exemplo:
    blockIdx.x = 1
    blockDim.x = 5
    threadIdx.x = 2

    i = 1 * 5 + 2 = 7, então essa thread vai cuidar do indice 7 do nosso array
    */
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    // Evitando acessar memória invalida
    if (i < n) {
            // cada trhread faz 1 soma, cada uma em paralelo
            w[i] = v[i] + u[i];
    }
}

int main() {
    // tamanho do vetor
    int n = 10;
    // tamanho do vetor em bytes
    int size = n * sizeof(int);

    // cria os vetores na CPU
    int v[n] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int u[n] = {2, 2, 2, 2, 2, 2, 2, 2, 2, 2};
    int w[n];
    // Cria os ponteiros para a GPU
    int *d_v, *d_u, *d_w;

    // reserva memoria na GPU
    cudaMalloc((void**)&d_v, size);
    cudaMalloc((void**)&d_u, size);
    cudaMalloc((void**)&d_w, size);
    // agora ele manda para a GPU pois a memoria já está alocada
    cudaMemcpy(d_v, v, size, cudaMemcpyHostToDevice);
    cudaMemcpy(d_u, u, size, cudaMemcpyHostToDevice);

    // Executando
    // Cada block tem 5 threads
    int threadsBlock = 5;
    int blocks = (n + threadsBlock - 1) / threadsBlock;
    // roda a função de soma na GPU
    addVectors<<<blocks, threadsBlock>>>(d_v, d_u, d_w, n);
    // o resultado volta da GPU para a CPU
    cudaMemcpy(w, d_w, size, cudaMemcpyDeviceToHost);

    // Print vetor v
    std::cout << "Vetor v: ";
    for (int i = 0; i < n; i++) {
        std::cout << v[i] << " ";
    }
    std::cout << std::endl;

    // Print vetor u
    std::cout << "Vetor u: ";
    for (int i = 0; i < n; i++) {
        std::cout << u[i] << " ";
    }
    std::cout << std::endl;

    // Resultado final, vetor w
    std::cout << "\nResultado final (w = v + u): ";
    for (int i = 0; i < n; i++) {
        std::cout << w[i] << " ";
    }
    std::cout << std::endl;

    // Limpa a GPU
    cudaFree(d_v);
    cudaFree(d_u);
    cudaFree(d_w);

    return 0;
}