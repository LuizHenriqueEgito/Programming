(println "Criando vetores:")
[1 2 3 4 5]
["A" "B"]
["A" "B" 3 3.14]  ; ela permite isso
(count [1 2 3 4])  ; é como o len do python
(conj [1 2 3 4] 5)  ; é como o append do python conj vem de conjoin (juntar)
(pop [1 2 3 4 5])  ; é como o pop
(peek [1 2 3 4])  ; mostra o ultimo valor
(peek (conj [1 2 3 4] 5))  ; retorna 5