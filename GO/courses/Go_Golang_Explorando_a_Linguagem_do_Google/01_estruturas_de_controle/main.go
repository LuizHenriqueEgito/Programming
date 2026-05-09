package main

import (
	"fmt"
	"math/rand"
	"time"
)

// if else
func imprimirResultado(nota float64) {
	if nota >= 7 {
		fmt.Println("Aprovado com nota:", nota)
	} else if nota >= 5 && nota < 7 {
		fmt.Println("Recuperação com nota:", nota)
	} else {
		fmt.Println("Reprovado com nota:", nota)
	}
}

// if init
func numeroAleatorio() int {
	s := rand.NewSource(time.Now().UnixNano())
	r := rand.New(s)
	return r.Intn(10)
}

func main() {
	imprimirResultado(7.3)
	imprimirResultado(5.3)
	imprimirResultado(3.1)

	if i := numeroAleatorio(); i > 5 { // Também é suportado no switch
		fmt.Println("Ganhou!!", i)
	} else {
		fmt.Println("Perdeu!", i)
	}
	// fmt.Println(i) Aqui da erro pois i só existe naquele escopo
}
