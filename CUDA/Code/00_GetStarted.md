# Diferença entre GPU e CPU

# Host vs Device

# Kernel

# Funções kernel `__global__`, `__device__`, `__host__`

# `threadIdx`, `blockIdx`, `blockDim`, `gridDim`

# Por que quase todo kernel começa com:
```c++
int i = blockIdx.x * blockDim.x + threadIdx.x;
```

# Memory

## 1. Memória básica: **global**

## 2. Global Memory

## 3. Registers

## 4. O que significa acesso coalescido?

# O que é o Triton
> Triton = kernels CUDA escritos em Python, com um compilador focado em workloads de ML.

# Prática em CUDA e no Triton
- Soma de vetores
- Multiplicação de vetores
- ReLU
- Sigmoid
- Softmax
- Convolução simples