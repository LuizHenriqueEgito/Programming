// Go não possui heranças mas isso se "aproxima" de uma
package main

import "fmt"

type person struct {
	nome      string
	sobrenome string
	idade     uint8
	altura    uint8
}

// O estudante é uma pessoa, e aqui pessoa já é um tipo por isso não é preciso passar
type student struct {
	person
	curso  string
	school string
}

func main() {
	fmt.Println("Quase Heranças")
	person_1 := person{"João", "Pedro", 20, 178}
	fmt.Println(person_1)

	student_1 := student{person_1, "Eng", "USP"}
	fmt.Println(student_1)
	fmt.Println(student_1.nome)
}
