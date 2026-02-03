package main

import "fmt"

func main() {
	// com o () ao final estamos chamando a função após sua criação
	func(texto string) {
		fmt.Println("Olá mundo!")
		fmt.Println(texto)
	}("Meu parametro.") // isso é como se fosse uma lambda function

	retornoMnhaLambda := func(texto string) string {
		// Sprintf: consegue concatenar seu texto com objetos
		return fmt.Sprintf("Recebido -> %s", texto)
	}("Meu outro parametro")

	fmt.Println(retornoMnhaLambda)
}
