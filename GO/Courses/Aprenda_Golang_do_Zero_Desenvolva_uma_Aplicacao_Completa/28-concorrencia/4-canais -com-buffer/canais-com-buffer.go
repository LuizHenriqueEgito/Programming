package main

import "fmt"

func main() {
	canal := make(chan string, 2) // canal com buffer só vai bloquear o trafego de dados quando o canal atingir a sua capacidade maxima (nesse caso 2)
	canal <- "Olá mundo!"
	canal <- "Programando em GO!"

	msg1 := <-canal
	msg2 := <-canal
	fmt.Println(msg1)
	fmt.Println(msg2)

	canal <- "Msg 3"
	msg3 := <-canal
	fmt.Println(msg3)
}
