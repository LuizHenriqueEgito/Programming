package main

import "fmt"

func main() {
	fmt.Println("Entendendo Maps")
	// é como um dicionario no Python
	// map[tipo chave]tipo valor
	user := map[string]string{
		"nome":      "Agent",
		"sobrenome": "Smith",
	}
	fmt.Println(user["nome"])
	user2 := map[string]map[string]string{
		"nome": {
			"primeiro": "Agent",
			"ultimo":   "Smith",
		},
		"curso": {
			"nome":   "Engenharia",
			"campus": "Campus 1",
		},
	}
	fmt.Println(user2)
	// deletando
	delete(user2, "nome")
	fmt.Println(user2)
	// adiciona
	user2["nacionalidade"] = map[string]string{
		"pais":   "Brazil",
		"cidade": "Minas Gerais",
		"estado": "BH",
	}
	fmt.Println(user2)
}
