package main

import "fmt"

func closure() func() { // retorna uma função
	texto := "Dentro da função closure"

	funcao := func() {
		fmt.Println(texto)
	}

	return funcao
}

func main() {
	texto := "Dentro da função main"
	fmt.Println(texto)

	funcaoNova := closure() // faz o print no texto "Dentro da função closure"
	funcaoNova()
}
