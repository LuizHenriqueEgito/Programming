package main

import "fmt"

func main() {
	fmt.Print("Mesma ")
	fmt.Print("Linha.")
	fmt.Println("")
	fmt.Println("Nova Linha.")

	x := 3.1415
	fmt.Printf("O valor de x é %.2f\n", x)

	a := 1 
	b := 1.999
	c := false
	d := "oi"
	/*
	d: int
	f: float
	t: bool
	s: string
	*/
	fmt.Printf("a=%d | b=%f | c=%t | d=%s\n", a, b, c ,d)
}