package main

import "fmt"

// aceita n argumentos é como um *args no python
func soma(numeros ...int) int {
	fmt.Println(numeros)
	total := 0
	for _, numero := range numeros {
		total += numero
	}
	return total
}

// só podemos ter um parametro variatico por função
// e ele sempre deve ser o último parametro
func escrever(texto string, numeros ...int) {
	for _, numero := range numeros {
		fmt.Println(texto, numero)
	}
}

func main() {
	totalSoma := soma(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
	fmt.Println("O total da soma foi:", totalSoma)

	escrever("Olá mundo", 1, 2, 3, 4, 5)
}
