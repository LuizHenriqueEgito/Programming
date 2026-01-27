# 1º Instale o Windows Subystem for Linux (WSL)

# 2º Installe o Ubunto
```bash
wsl --install -d Ubuntu
```
Para abrir o terminal no `windowns` rode:
```bash
wsl
# ou para entrar como root
wsl -d Ubuntu -u root
```
Com isso ele já entra no `Linux`.

# 3º Atualize o Sistema
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install wget curl
sudo apt install python3-pip
```
# 4º Installe o CUDA Toolkit
```bash
wget https://developer.download.nvidia.com/compute/cuda/13.1.1/local_installers/cuda_13.1.1_590.48.01_linux.run

# ao dar o run vai abrir um menu
sudo sh cuda_13.1.1_590.48.01_linux.run
# vá na opção de install e em seguida adicione ao PATH
export PATH=/usr/local/cuda-13.1/bin:$PATH
export LD_LIBRARY_PATH=/usr/local/cuda-13.1/lib64:$LD_LIBRARY_PATH
source ~/.bashrc
```

# 5º Confira se está tudo ok
```bash
nvcc --version
nvidia-smi
```

# 6º Escreva o código no arquivo .cu
```c
#include <iostream>
using namespace std;

int main() {
    cout << "hello world" << endl;
    return 0;
}
```

# 7º Compile
```bash
nvcc -o main main.cu
```
E por fim rode:
```bash
./ main
```
