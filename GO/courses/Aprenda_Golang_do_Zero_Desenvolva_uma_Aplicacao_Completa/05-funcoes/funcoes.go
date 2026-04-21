package main

import "fmt"

func somar(a int8, b int8) int8 {
	return a + b
}

// ao fazer int 8 só ao final ele infere que a também é int8
func calculosMatematicos(a, b int8) (int8, int8) {
	add := somar(a, b)
	sub := a - b

	return add, sub
}

func main() {
	resultadoSoma_0, resultadoSub := calculosMatematicos(10, 20)
	fmt.Println(resultadoSoma_0, resultadoSub)

	resultadoSoma_1, _ := calculosMatematicos(10, 20)
	fmt.Println(resultadoSoma_1)
}
