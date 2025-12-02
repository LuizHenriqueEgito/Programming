package main

import "fmt"

func main() {
	fmt.Println("Ponteiros")

	var var_1 int = 10
	var var_2 int = var_1

	fmt.Println(var_1, var_2)
	var_1++
	fmt.Println(var_1, var_2)

	// PONTEIRO É UMA REFERÊNCIA DE MEMÓRIA
	var var_3 int
	var ponteiro *int // guarda o endereço de memoria
	fmt.Println(var_3, ponteiro)

	var_3 = 42
	// ponteiro = variavel3 não pode acontecer
	ponteiro = &var_3             // referencia ao valor da var_3 em memoria
	fmt.Println(var_3, ponteiro)  // mostra em memoria onde está
	fmt.Println(var_3, *ponteiro) // pega o valor (desreferenciação)
}
