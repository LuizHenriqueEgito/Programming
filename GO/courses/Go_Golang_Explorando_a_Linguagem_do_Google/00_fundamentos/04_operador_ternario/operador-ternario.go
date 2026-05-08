package main

import "fmt"

// GO não possui operador ternario
func operadorTernario(nota, corte float64) string {
	if nota >= corte {
		return "Aprovado"
	}
	return "Reprovado"
}

func main() {
	nota, corte := 5.0, 7.0
	aprovado := operadorTernario(nota, corte)
	fmt.Println("Aluno:", aprovado)
}
