package main

import "fmt"

type usuario struct {
	nome     string
	idade    uint8
	endereco endereco
}

type endereco struct {
	logradouro string
	numero     uint8
}

func main() {
	fmt.Println("Arquivo structs")
	endereco := endereco{logradouro: "Rua XPTO", numero: 111}
	var user_1 usuario
	user_1.nome = "Nuna"
	user_1.idade = 21
	fmt.Println(user_1)

	user_2 := usuario{"Two", 21, endereco}
	fmt.Println(user_2)

	user_3 := usuario{idade: 21}
	fmt.Println(user_3)
}
