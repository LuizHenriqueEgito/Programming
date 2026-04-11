#include <iostream>  // entrada e saída de dados no console
#include <vector>  // array, é o container mais usado em C++
#include <string>  // manipulação de texto
#include <memory>  // gerenciamento inteligente de memória
#include <algorithm>  // algoritmos genéricos
#include <unordered_map>  // HashTables, como dicionários em Python


// Classe base
class Person {

// Protege a classe o que está aqui
// só pode ser acessado dentro da classe ou em classes derivadas (que herdam)
protected:  // se fosse private: Alien NÃO poderia acessar!
    std::string name;  // armazena o nome
    int age;  // armazena a idade

public:
    // construtor da classe
    Person(std::string name, int age) : name(name), age(age) {}
    // virtual: permite que classes derivadas sobrescrevam este método
    // virtual void: não retorna nada
    virtual void apresentar() {
        // imprime o nome e a idade
        std::cout << "Nome: " << name << ", Idade: " << age << std::endl;
    }
    // deleta todo o cache quando o objeto for destruido
    // virtual aqui faz deletar o cache aqui e não de quem herdou
    virtual ~Person() {}
};

// Classe derivada (Herança + Polimorfismo)
class Alien : public Person {
private:
    std::vector<double> scores;

public:
    Alien(std::string name, int age, std::vector<double> scores)
        : Person(name, age), scores(scores) {}

    double media() {
        double soma = 0;
        for (double s : scores) {
            soma += s;
        }
        return soma / scores.size();
    }

    void apresentar() override {
        std::cout << "Alien: " << name
                  << " | Média: " << media() << std::endl;
    }
};

int main() {
    // vector
    std::vector<int> numeros = {5, 2, 9, 1, 7};

    // algorithm (sort)
    std::sort(numeros.begin(), numeros.end());

    std::cout << "Numeros ordenados: ";
    for (auto n : numeros) { // range-based for + auto
        std::cout << n << " ";
    }
    std::cout << std::endl;

    // unordered_map
    std::unordered_map<std::string, int> idade_map;
    idade_map["Luiz"] = 30;
    idade_map["Maria"] = 25;

    std::cout << "Idade do Luiz: " << idade_map["Luiz"] << std::endl;

    // ponteiro (tradicional)
    int x = 10;
    int* ptr = &x;  // * pega o endereço, & acessa o valor

    std::cout << "Valor de x via ponteiro: " << *ptr << std::endl;

    // Smart Pointer
    std::unique_ptr<Person> p1 = std::make_unique<Person>("Carlos", 40);

    // Polimorfismo com smart pointer
    std::unique_ptr<Person> p2 = std::make_unique<Alien>(
        "Luiz", 28, std::vector<double>{8.5, 9.0, 7.5}
    );

    p1->apresentar();
    p2->apresentar(); // chama versão sobrescrita

    // Lambda
    auto soma = [](int a, int b) {
        return a + b;
    };

    std::cout << "Soma lambda: " << soma(3, 4) << std::endl;

    return 0;
}