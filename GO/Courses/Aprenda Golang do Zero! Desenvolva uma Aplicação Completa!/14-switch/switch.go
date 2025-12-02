package main

import "fmt"

func diaDaSemana(numero int) string {
	switch numero {
	case 1:
		return "Domingo"
	case 2:
		return "Segunda Feira"
	case 3:
		return "Terça Feira"
	case 4:
		return "Quarta Feira"
	case 5:
		return "Quinta Feira"
	case 6:
		return "Sexta Feira"
	case 7:
		return "Sabado"
	default:
		return "Número invalido"
	}
}

// outra forma
func diaDaSemana2(num int) string {
	switch {
	case num == 1:
		return "Domingo"
		// fallthrough // se cair nessa condição ele faz cair sem validar nada na proxima condição
	case num == 2:
		return "Segunda-Feira"
	case num == 3:
		return "Terça-Feira"
	case num == 4:
		return "Quarta-Feira"
	case num == 5:
		return "Quinta-Feira"
	case num == 6:
		return "Sexta-Feira"
	case num == 7:
		return "Sabado"
	default:
		return "Número Invalido"
	}
}

func main() {
	fmt.Println("Switch")
	dia := diaDaSemana(200)
	fmt.Println(dia)

	dia1 := diaDaSemana(1)
	dia2 := diaDaSemana2(2)

	fmt.Println(dia1)
	fmt.Println(dia2)
}
