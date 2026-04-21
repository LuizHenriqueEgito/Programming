package main

import "fmt"

// a ordem de quem vem primeiro não interfere em anda
func init() {
	fmt.Println("Função init sendo executada")
}

func main() {
	fmt.Println("Função main sendo executada")
}
