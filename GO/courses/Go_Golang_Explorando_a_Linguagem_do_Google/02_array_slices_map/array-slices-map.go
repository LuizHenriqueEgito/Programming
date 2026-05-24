package main

import (
	"fmt"
	"reflect"
)

func main() {
	// ARRAYS
	// Array: homogeneo (mesmo tipo) e estatico (fixo)
	fmt.Println("ARRAYS")
	var notas [3]float64
	fmt.Println(notas)
	notas[0], notas[1], notas[2] = 7.8, 4.3, 9.1
	fmt.Println(notas)
	total := 0.0
	for i := 0; i < len(notas); i++ {
		total += notas[i]
	}
	media := total / float64(len(notas))
	fmt.Printf("Média: %.2f\n", media)

	numeros := [...]int{1, 2, 3, 4, 5} // compilador conta, sem o [...] vira um slice
	for i, numero := range numeros {
		fmt.Printf("%d) %d\n", i+1, numero)
	}
	for _, n := range numeros {
		fmt.Println(n)
	}

	// SLICES
	fmt.Println("\n\nSLICES")
	a1 := [3]int{10, 20, 30} // array
	s1 := []int{10, 20, 30}  // slice
	fmt.Println(a1, s1)
	fmt.Println(reflect.TypeOf(a1), reflect.TypeOf(s1))

	a2 := [5]int{1, 2, 3, 4, 5}
	// Slice não é um array! Slice defne um pedaço de um array
	s2 := a2[1:3]
	fmt.Println(a2, s2)

	s3 := a2[:2] // novo slice, mas aponta para o mesmo array
	fmt.Println(a2, s3)
	// Você pode imaginar um slice como: tamanho e um ponteiro para um elemento de um array
	s4 := s2[:1]
	fmt.Println(s2, s4)

	// Construindo slices com make
	s := make([]int, 10)
	s[9] = 12
	fmt.Println(s)
	s = make([]int, 10, 20) // 10 elementos mas espaço para 20
	fmt.Println(s, len(s), cap(s))
	s = append(s, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0)
	fmt.Println(s, len(s), cap(s))

	s = append(s, 1)
	fmt.Println(s, len(s), cap(s)) // slice vai crescendo a cap vai automaticamente para 40

	sa := make([]int, 10, 20)
	sb := append(sa, 1, 2, 3)
	fmt.Println(sa, sb)
	sa[0] = 7
	fmt.Println(sa, sb)

	// Append Copy
	fmt.Println("\nAppend Copy")
	array1 := [3]int{1, 2, 3}
	var slice1 []int

	slice1 = append(slice1, 4, 5, 6, 7)
	fmt.Println(array1, slice1)
	slice2 := make([]int, 5)
	copy(slice2, slice1)
	fmt.Println(slice2)

	// MAP: São como dicionarios (Hashmaps)
	fmt.Println("MAP")
	// var aprovados map[int]string
	aprovados := make(map[int]string)
	aprovados[123456789] = "Maria"
	aprovados[789456123] = "Pedro"
	aprovados[111111111] = "Maria"
	fmt.Println(aprovados)
	for cpf, nome := range aprovados {
		fmt.Printf("%s (CPF: %d)\n", nome, cpf)
	}
	fmt.Println(aprovados[789456123])
	delete(aprovados, 789456123)
	fmt.Println(aprovados[789456123])

	funcsESalarios := map[string]float64{
		"jose":    123456.45,
		"gabriel": 15564.45,
		"pedro":   1200.00,
	}
	funcsESalarios["egito"] = 1350.0
	delete(funcsESalarios, "inexistente")
	for nome, salario := range funcsESalarios {
		fmt.Println(nome, salario)
	}

	funcsPorLetra := map[string]map[string]float64{
		"G": {
			"gabriel": 15456.78,
			"guga":    8456.78,
		},
		"J": {
			"jose": 1234.56,
		},
		"P": {
			"pedro": 1200.00,
		},
	}
	fmt.Println(funcsPorLetra)
	delete(funcsPorLetra, "P")
	fmt.Println(funcsPorLetra)

	for letra, funcs := range funcsPorLetra {
		for nome, salario := range funcs {
			fmt.Println(letra, nome, salario)
		}
		// fmt.Println(letra, funcs)
	}
}
