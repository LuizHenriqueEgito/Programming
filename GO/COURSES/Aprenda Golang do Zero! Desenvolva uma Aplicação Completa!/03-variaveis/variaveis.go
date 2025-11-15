package main

import "fmt"

func main() {
	var variavel_1 string = "variavel 1" // deixa explicito o tipo
	variavel_2 := "variavel 2"           // aqui ele conclui o tipo

	fmt.Println(variavel_1)
	fmt.Println(variavel_2)

	variavel_5, variavel_6 := "variavel 5", "variavel 6"
	fmt.Println(variavel_5)
	fmt.Println(variavel_6)

	const constante string = "CONSTANTE"
	fmt.Println(constante)
}
