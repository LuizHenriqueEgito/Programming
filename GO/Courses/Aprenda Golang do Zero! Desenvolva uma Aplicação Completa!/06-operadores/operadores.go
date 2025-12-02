package main

import "fmt"

func main() {
	// aritméticos
	soma := 1 + 2
	subtracao := 1 - 2
	divisao := 10 / 4
	multiplicacao := 10 * 5
	restoDaDivisao := 10 % 2

	fmt.Println(soma, subtracao, divisao, multiplicacao, restoDaDivisao)
	// go é fortemente tipado você não pode somar int8 com int16

	// atribuição
	var variavel1 string = "string"
	variavel2 := "string2"
	fmt.Println(variavel1, variavel2)

	// operadores relacionais
	fmt.Println(1 > 2)
	fmt.Println(1 >= 2)
	fmt.Println(1 == 1)
	fmt.Println(1 <= 2)
	fmt.Println(1 > 2)
	fmt.Println(1 < 2)
	fmt.Println(1 != 2)

	// operadores logicos
	fmt.Println("----------------------------------")
	verdadeiro, falso := true, false
	fmt.Println(verdadeiro && falso)
	fmt.Println(verdadeiro || falso)
	fmt.Println(!verdadeiro)
	fmt.Println(!falso)

	// operadores unarios
	numero := 10
	numero++ // aumenta 1
	fmt.Println(numero)
	numero += 2 // aumenta 2

	numero--    //diminui 1
	numero -= 1 //diminui 1

	numero *= 3
	numero /= 10
	numero %= 3
	fmt.Println(numero)
	// no GO não existe operador ternario
}
