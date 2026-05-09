package main

import (
	"fmt"
	"math/rand"
	"time"
)

// if else
func imprimirResultado(nota float64) {
	if nota >= 7 {
		fmt.Println("Aprovado com nota:", nota)
	} else if nota >= 5 && nota < 7 {
		fmt.Println("Recuperação com nota:", nota)
	} else {
		fmt.Println("Reprovado com nota:", nota)
	}
}

// if init
func numeroAleatorio() int {
	s := rand.NewSource(time.Now().UnixNano())
	r := rand.New(s)
	return r.Intn(10)
}

func notaParaConceito(n float64) string {
	var nota = int(n)
	switch nota {
	case 10:
		fallthrough // continua executando não sai do switch
	case 9:
		return "A"
	case 8, 7:
		return "B"
	case 6, 5:
		return "C"
	case 4, 3:
		return "D"
	case 2, 1, 0:
		return "E"
	default:
		return "Nota inválida"
	}
}

func tipo(i interface{}) string {
	switch i.(type) {
	case int:
		return "inteiro"
	case float32, float64:
		return "real"
	case string:
		return "string"
	case func():
		return "função"
	default:
		return "Tipo desconhecido"
	}
}

func main() {
	imprimirResultado(7.3)
	imprimirResultado(5.3)
	imprimirResultado(3.1)

	if i := numeroAleatorio(); i > 5 { // Também é suportado no switch
		fmt.Println("Ganhou!!", i)
	} else {
		fmt.Println("Perdeu!", i)
	}
	// fmt.Println(i) Aqui da erro pois i só existe naquele escopo

	// laço for (Em go não existe while)
	i := 1
	for i <= 10 {
		fmt.Println(i)
		i++
	}
	for i := 0; i <= 20; i += 2 {
		fmt.Printf("%d ", i)
	}

	for {
		// isso é como um while
		fmt.Println("Para sempre...")
		time.Sleep(time.Second * 5)
		break
	}

	// switch
	fmt.Println(notaParaConceito(9.8))
	fmt.Println(notaParaConceito(5.8))
	fmt.Println(notaParaConceito(1.8))

	t := time.Now()
	switch { // é como um switch true
	case t.Hour() < 12:
		fmt.Println("Bom dia!")
	case t.Hour() < 18:
		fmt.Println("Boa tarde")
	default:
		fmt.Println("Boa noite")
	}

	fmt.Println(tipo(2.3))
	fmt.Println(tipo(2))
	fmt.Println(tipo("Hello"))
	fmt.Println(tipo(func() {}))
	fmt.Println(tipo(time.Now()))
}
