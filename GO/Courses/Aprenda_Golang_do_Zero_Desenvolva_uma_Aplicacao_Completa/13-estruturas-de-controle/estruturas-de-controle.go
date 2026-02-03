package main

import "fmt"

func main() {
	fmt.Println("Estruturas de Controle")
	numero := 20

	if numero > 15 {
		fmt.Println("numero é maior que 15")
	} else {
		fmt.Println("numero é menor que 15")
	}

	if outroNumero := numero; outroNumero > 0 { // limitado a este campo/bloco outroNumero não existe fora dele
		fmt.Println("Numero é mair que 0")
	} else {
		fmt.Println("numero é menor que zero")
	}

	// fmt.Println(outroNumero) nao funciona pois não existe neste escopo
}
