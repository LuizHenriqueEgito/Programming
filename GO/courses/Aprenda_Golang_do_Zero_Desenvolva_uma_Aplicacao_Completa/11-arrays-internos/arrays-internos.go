package main

import "fmt"

func main() {
	fmt.Println("Arrays e Slices")
	var array1 [5]int // troque int por string e você terá 5 espaços em branco
	array1[0] = 1
	fmt.Println(array1)

	array2 := [5]string{"pos1", "pos2", "pos3", "pos4", "pos5"}
	fmt.Println(array2)

	array3 := [...]int{1, 2, 3, 4, 5, 6, 7} // os ... faz com que você não precise falar quantos itens seu array terá
	fmt.Println(array3)

	slice := []int{10, 12, 14, 16} //slices são mais usados do que arrays
	fmt.Println(slice)

	slice = append(slice, 18)
	fmt.Println(slice)

	slice2 := array2[1:3]
	fmt.Println(slice2)

	array2[1] = "pos alterada"
	fmt.Println(array2)
	fmt.Println(slice2) // continua apontando para o array2

	// arrays internos
	fmt.Println("=======================")
	slice3 := make([]float32, 10, 15)
	fmt.Println(slice3)

	fmt.Println(len(slice3)) // tamanho
	fmt.Println(cap(slice3)) // capacidade
}
