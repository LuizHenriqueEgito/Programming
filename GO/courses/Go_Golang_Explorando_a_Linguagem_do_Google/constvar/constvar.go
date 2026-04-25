package main

import (
	"fmt"
	m "math" // fazer isso é como um as no python
)

func main() {
	const PI float64 = 3.1415
	var raio = 3.2
	// a = 1  Isso não pode
	// fmt.Println(a)
	area := PI * m.Pow(raio, 2)
	fmt.Println("area =", area)

	const (
		A = 1
		B = 2
	)
	var (
		c = 3
		d = 4
	)
	fmt.Println(A, B, c, d)

	var e, f bool = true, false
	g, h, i := 2, false, "oi"
	fmt.Println(e, f, g, h, i)

}
