package main

import (
	"fmt"
	"math"
	"reflect"
)

func main() {
	// números inteiros
	fmt.Println(1, 2, 1000)
	fmt.Println("Literal inteiro é", reflect.TypeOf(1))

	// sem sinal usigned (só positivos), uint8, ..., uint64
	var bb byte = 255
	fmt.Println("O byte é", reflect.TypeOf(bb))

	// com sinal, int8, ..., int64
	i1 := math.MaxInt64
	fmt.Println("O valor máximo do int é", i1)
	fmt.Println("O tipo de i1 é", reflect.TypeOf(i1))

	var i2 rune = 'a' // representa um mapeamento da tabela unicode (int32)
	fmt.Println("O rune é", reflect.TypeOf(i2))
	fmt.Println(i2)
	// números reais (float32, float64)
	var x float64 = 49.99
	fmt.Println("O tipo de x é", reflect.TypeOf(x))
	fmt.Println(i2)

	bo := true
	fmt.Println("O tipo de bo é", reflect.TypeOf(bo))
	fmt.Println(!bo)

	// string
	s1 := "Olá meu nome é Luiz"
	fmt.Println(s1 + "!")
	fmt.Println("O tamanho da string é", len(s1))

	// string com multiplas linhas
	s2 := `Olá
	meu
	nome
	é
	Luiz
	`
	fmt.Println("O tamanho da string é", len(s2))

	// char (é um int32)
	char := 'a'
	fmt.Println("O tipo de char é", reflect.TypeOf(char))
	fmt.Println(char)

	// valores zeros (vazios)
	var a int
	var b float64
	var c bool
	var d string
	var e *int
	fmt.Printf("a:%v b:%v c:%v d:%q e:%v", a, b, c, d, e)
}
