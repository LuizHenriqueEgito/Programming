package main

import "fmt"

func func_1() {
	fmt.Println("Primeira função")
}

func func_2(p1 string, p2 string) {
	fmt.Printf("Func 2: %s %s\n", p1, p2)
}

func func_3() string {
	return "Func 3"
}

func func_4(p1, p2 string) string {
	return fmt.Sprintf("Func 4: %s %s", p1, p2)
}

func func_5() (string, string) {
	return "Retorno 1", "Retorno 2"
}

func main() {
	func_1()
	func_2("Olá", "mundo")
	fmt.Println(func_3())
	fmt.Println(func_4("Olá", "mundo"))
	r5_1, r5_2 := func_5()
	fmt.Println(r5_1, r5_2)
}
