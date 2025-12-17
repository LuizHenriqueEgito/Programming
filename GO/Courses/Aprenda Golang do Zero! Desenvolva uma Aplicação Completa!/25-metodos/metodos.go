package main

import "fmt"

type usuario struct {
	nome  string
	idade uint8
}

// O usuario vai ter o método salvar
func (user usuario) salvar() {
	fmt.Printf("Salvando os dados do usuário %s no banco de dados.\n", user.nome)
}

func (user usuario) maiorIdade() bool {
	return user.idade >= 18
}

// Como vamos mexer nele usamos o ponteiro!
func (user *usuario) fazerAniversario() {
	user.idade++
}

func main() {
	user_1 := usuario{"Usuário_A", 20}
	fmt.Println(user_1)
	user_1.salvar()
	maiorDeIdade := user_1.maiorIdade()
	fmt.Println(maiorDeIdade)
	fmt.Printf("idade do usuario %s é: %d\n", user_1.nome, user_1.idade)
	user_1.fazerAniversario()
	fmt.Printf("Após seu aniversário sua idade agora é: %d", user_1.idade)
}
