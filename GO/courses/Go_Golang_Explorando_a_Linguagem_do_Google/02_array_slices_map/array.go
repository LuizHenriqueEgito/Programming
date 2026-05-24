package main

import "fmt"

func main() {
	// Array: homogeneo (mesmo tipo) e estatico (fixo)
	var notas [3]float64
	fmt.Println(notas)
	notas[0], notas[1], notas[2] = 7.8, 4.3, 9.1
	fmt.Println(notas)
	total := 0.0
	for i := 0; i < len(notas); i++ {
		total += notas[i]
	}
	media := total / float64(len(notas))
	fmt.Printf("Média: %.2f\n", media)

	numeros := [...]int{1, 2, 3, 4, 5} // compilador conta, sem o [...] vira um slice
	for i, numero := range numeros {
		fmt.Printf("%d) %d\n", i+1, numero)
	}
	for _, n := range numeros {
		fmt.Println(n)
	}
}
