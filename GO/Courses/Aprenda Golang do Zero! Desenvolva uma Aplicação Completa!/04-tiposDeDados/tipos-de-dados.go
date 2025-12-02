package main

import (
	"errors"
	"fmt"
)

// int8, int16, int32, int64, int → usa o sistema do seu computar 64 bits ou 32 bits
func main() {
	var num_1 int = 100
	fmt.Println(num_1)
	num_2 := 1000000 // usa a arquitetura do seu computador ou var num int
	fmt.Println(num_2)
	var num_3 uint8 = 8 // uint: int sem sinal ou seja apenas positivos
	fmt.Println(num_3)
	// int32 = rune
	var num_4 rune = 123
	fmt.Println(num_4)

	// uint8 = byte
	var num_5 byte = 123
	fmt.Println(num_5)

	// float32, float64
	numeroReal := 123.456
	var numeroReal_1 float32 = 123.456
	var numeroReal_2 float64 = 123000000.000001
	fmt.Println(numeroReal)
	fmt.Println(numeroReal_1)
	fmt.Println(numeroReal_2)

	// strings
	var texto_1 string = "texto"
	texto_2 := "texto"
	char := "B" // numero desse caracter na tabela ASC, não existe char
	fmt.Println(texto_1, texto_2, char)

	var variavel string // vazio, ou 0 ou nil
	fmt.Println(variavel)

	var booleano_false bool = false
	var booleano_true bool = true
	booleano := false
	var booleano_0 bool
	fmt.Println(booleano_0, booleano_true, booleano_false, booleano)

	var erro error
	fmt.Println(erro) // <nil>

	var erro_1 error = errors.New("Erro interno")
	fmt.Println(erro_1)
}
