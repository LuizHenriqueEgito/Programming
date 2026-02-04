#include <iostream>
#include <string>

int main() {
    int myInt = 0;  // criando variaveis
    float myFloat = 0.0f;
    double myDouble = 0.0;  // Tem mais precisão que o float
    bool myBool = false;
    char myChar = 'c';
    unsigned int myUint = 1;  // é o usize
    char myArray[11] = "0123456789";  // 11 pois o último sempre será o \0
    std::string myString = "Teste";
    
    if (myInt >= 0) {
        std::cout << "myInt é maior ou igual a zero\n";
    } else {
        std::cout << "myInt é menor do que zero\n";
    }

    int userInt;
    std::cin >> userInt;
    if (userInt > 0) {
        std::cout << "Maior que zero";
    } else if (userInt < 0) {
        std::cout << "Menor que zero";
    } else {
        std::cout << "Zero";
    }
    
    return 0;
}