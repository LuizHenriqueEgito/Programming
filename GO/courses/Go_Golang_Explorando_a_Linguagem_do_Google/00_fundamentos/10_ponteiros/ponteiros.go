package main

import "fmt"

func main() {
	i := 1

	// GO não tem aritmética de ponteiros
	// Ponteiro é uma referencia de memória
	var p *int = nil
	p = &i // pegando o endereço da variável i
	*p++   // com * eu acesso o valor
	i++
	fmt.Println(p, &i, *p, i)

}
