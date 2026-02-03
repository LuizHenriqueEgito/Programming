package main

import "fmt"

func funcao1() {
	fmt.Println("Executando a função 1")
}

func funcao2() {
	fmt.Println("Executando a função 2")
}

func alunoEstaAprovado(n1, n2 float32) bool {
	media := (n1 + n2) / 2
	if media >= 6 {
		fmt.Println()
		return true
	}
	return false
}

func main() {
	// defer: significa adiar, ele deixa pra depois
	defer funcao1()
	funcao2()
	fmt.Println(alunoEstaAprovado(7, 8))
}
