#include <iostream>
#include <string>

const int MAX = 100;
// void é um retorno de função vazio
void print(std::string s) {
    std::cout << s << "\n";
}

int soma_fn(int a, int b) {
    return a + b;
}

// Classe
class ContaBancaria {
private:
    std::string numero_conta;
    double saldo;

public:
    // Construtor
    ContaBancaria(std::string numero, double saldo_inicial) {
        numero_conta = numero;
        saldo = saldo_inicial;
    }

    void depositar(double valor) {
        if (valor > 0) {
            saldo += valor;
        }
    }

    bool sacar(double valor) {
        if (valor > 0 && valor <= saldo) {
            saldo -= valor;
            return true;
        }
        return false;
    }

    double getSaldo() const {
        return saldo;
    }

    void mostraConta() const {
        std::cout << "Conta: " << numero_conta
                  << " | Saldo: " << saldo << std::endl;
    }
};


// Struct
struct Human {
    // construtor
    Human() {
        name = "Human";
        age = 0;
        height = 0.10f;
        weight = 10.f;
    }
    Human(std::string n, int age=0) {
        // this é como o self no python
        this->name = n;
        this->age = age;
        this->height = 0.1f;
        this->weight = 0.5f;
    }
    std::string name;
    int age;
    float height;
    float weight;
};

int main() {
    // Variáveis
    int myInt = 0;
    float my_float = 0.0f;
    double my_double = 0.0;
    bool my_bool = false;
    char my_char = 'c';
    unsigned int my_uint = 1;
    char my_array[] = "0123456789";  // Tamanho automático

    print("Printando como no Python!");
    print(my_array);
    print("\n");

    std::string my_string = "Teste";
    std::cout << "my_float: " << my_float << "\n";
    std::cout << "my_double: " << my_double << "\n";
    std::cout << "my_bool: " << std::boolalpha << my_bool << "\n";
    std::cout << "my_char: " << my_char << "\n";
    std::cout << "my_uint: " << my_uint << "\n";
    std::cout << "my_array: " << my_array << "\n";
    std::cout << "my_string: " << my_string << "\n\n";

    int my_number = soma_fn(2, 3);
    std::cout << "Meu número somado: " << my_number << "\n";
    
    // Primeiro if
    if (myInt >= 0) {
        std::cout << "myInt é maior ou igual a zero\n";
    } else {
        std::cout << "myInt é menor do que zero\n";
    }

    // Entrada do usuário
    int userInt;
    std::cout << "Digite um número inteiro: ";
    std::cin >> userInt;
    
    if (userInt > 0) {
        std::cout << "Maior que zero\n";
    } else if (userInt < 0) {
        std::cout << "Menor que zero\n";
    } else {
        std::cout << "Zero\n";
    }
    
    // Loop
    for (int i = 0; i < 10; i++) {
        std::cout << "Novo valor de i: " << i << "\n";
    }

    Human me;
    std::cout << me.name << "\n";
    std::cout << me.age << "\n";
    std::cout << me.height << "\n";
    std::cout << me.weight << "\n";

    me.name = "Nuna";
    me.age = 28;
    me.height = 1.82f;
    me.weight = 76.f;
    std::cout << "PREENCHENDO OS VALORES:" << "\n";
    std::cout << me.name << "\n";
    std::cout << me.age << "\n";
    std::cout << me.height << "\n";
    std::cout << me.weight << "\n";

    Human luiz("Luiz", 29);
    
    ContaBancaria conta("123-456", 1000.0);
    conta.mostraConta();
    conta.depositar(500.0);
    conta.sacar(200.0);
    std::cout << "Saldo atual: " << conta.getSaldo() << std::endl;
    return 0;
}