package main

import (
	"fmt"
	"time"
)

func main() {
	canal := make(chan string)

	go escrever("Olá mundo!", canal)
	// mensagem, aberto := <-canal // espero que cehgue um valor
	for message := range canal {
		fmt.Println(message)
	}
}

func escrever(texto string, canal chan string) {
	for i := 0; i < 5; i++ {
		canal <- texto // enviando um valor para o canal
		time.Sleep(time.Second)
	}
	close(canal)
}
