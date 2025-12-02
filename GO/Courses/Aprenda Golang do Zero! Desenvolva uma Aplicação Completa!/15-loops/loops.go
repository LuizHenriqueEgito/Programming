package main

import (
	"fmt"
	"time"
)

func main() {
	i := 0
	for i < 5 {
		i++ // é como o += no python
		fmt.Println("incrementando i")
		time.Sleep(time.Second)
	}

	for j := 0; j < 5; j++ {
		fmt.Println("incrementando j:", j)
		time.Sleep(time.Second)
	}

	nomes := [3]string{"A", "B", "C"} // array
	// poderiamos fazer assim também:
	nomes2 := []string{"A", "B", "C"} // slice
	for indice, nome := range nomes {
		fmt.Println(indice, nome)
	}

	for _, nome := range nomes2 {
		fmt.Println(nome)
	}

	for indice, letra := range "PALAVRA" {
		// se não colocar string(letra) ele não retorna o caracter, apenas letra retorna o numero ascii do caracter
		fmt.Println(indice, letra)
	}

	for indice, letra := range "PALAVRA" {
		// se não colocar string(letra) ele não retorna o caracter, apenas letra retorna o numero asc do caracter
		fmt.Println(indice, string(letra))
	}

	users := map[string]string{
		"nome":      "Luiz",
		"sobrenome": "Egito",
	}
	for k, v := range users {
		fmt.Println(k, v)
	}
	// type userStruct struct {  // não é possivel iterar sobre uma struct
	//	nome string
	//	sobrenome string
	//}
	for {
		fmt.Println("É como um while True não para")
		time.Sleep(time.Second)
	}
}
