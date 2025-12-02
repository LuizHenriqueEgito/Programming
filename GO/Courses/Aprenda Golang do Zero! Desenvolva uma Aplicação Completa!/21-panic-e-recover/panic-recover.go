package main

import "fmt"

func recuperarExecucao() {
	if r := recover(); r != nil {
		fmt.Println("Tentativa de recuperar a execução.")
	}
}

func alunoEstaAprovado(n1, n2 float64) bool {
	defer recuperarExecucao()
	media := (n1 + n2) / 2
	fmt.Println("A media é:", media)
	if media > 6 {
		return true
	} else if media < 6 {
		return false
	}

	panic("A MÉDIA É EXATAMENTE 6!") // mata o programa
}

func main() {
	fmt.Println(alunoEstaAprovado(6, 6))
	fmt.Println(alunoEstaAprovado(6, 7))
	fmt.Println("Pós execução!")
}
