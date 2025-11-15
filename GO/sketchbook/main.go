package main

import "fmt"

func soma(a int, b int) int {
	return a + b
}

func main() {
	var a int = 40
	b := 2
	resultado := soma(a, b)
	fmt.Println("Resultado:", resultado)
}
