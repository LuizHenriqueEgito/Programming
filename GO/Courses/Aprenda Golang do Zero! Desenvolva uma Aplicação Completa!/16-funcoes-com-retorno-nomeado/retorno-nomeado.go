package main

import "fmt"

// isso é o retorno nomeado (soma int, subtracao int)
func calculosMatematicos(n1, n2 int) (soma int, subtracao int) {
	soma = n1 + n2
	subtracao = n1 - n2
	return // ele já sabe o que deve retornar
}

func main() {
	add, sub := calculosMatematicos(10, 20)
	fmt.Println(add, sub)
}
