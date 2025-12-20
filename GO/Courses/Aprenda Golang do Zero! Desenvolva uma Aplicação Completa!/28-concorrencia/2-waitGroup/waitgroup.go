package main

import (
	"fmt"
	"sync"
	"time"
)

func escreverHelloWord(texto string) {
	for i := 0; i < 5; i++ {
		fmt.Println(texto)
		time.Sleep(time.Second)
	}
}

// usar *go* na frente da usa função/método ele usa goroutines
func main() {
	var waitGroup sync.WaitGroup
	waitGroup.Add(2) // meu grupo de espera para que o programa termine
	go func() {
		escreverHelloWord("Olá Mundo!")
		waitGroup.Done() // tira um cara do contador -1
	}()

	go func() {
		escreverHelloWord("Programando em GO!")
		waitGroup.Done() // tira um cara do contador -1
	}()

	waitGroup.Wait() // Espera chegar em 0

	// coloque go nos dois e explique o comportamento:
	// Isso acontece pois o programa só passa para frente
}
