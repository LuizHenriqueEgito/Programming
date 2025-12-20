package main

import (
	"fmt"
	"time"
)

func escreverHelloWord(texto string) {
	for {
		fmt.Println(texto)
		time.Sleep(time.Second)
	}
}

// usar *go* na frente da usa função/método ele usa goroutines
func main() {
	go escreverHelloWord("Olá Mundo!")
	escreverHelloWord("Programando em GO!")
	// coloque go nos dois e explique o comportamento:
	// Isso acontece pois o programa só passa para frente
}
