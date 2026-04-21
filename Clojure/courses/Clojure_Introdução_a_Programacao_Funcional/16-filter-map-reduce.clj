; Imagine que tenhamos um vetor de números onde cada número representa o ano
; que a estudante está. Exemplo: [5 6 6], temos 1 estudante no quinto ano
; e dois no sexto.
;
; DESAFIO 1: crie uma função que recebe este vetor como entrada e retorna
; a quantidade de estudantes que estão no quinto ano.
; [5 5 6 7 8 6 5 5] -> 4

; filter predicado lista (predicato é algo que retorna true ou false)
(println "Desafio 1:")
(defn esta-no-quinto-ano?
  [ano]
  (= ano 5))


(defn quantidade-estudantes-no-quinto-ano-v0
  [estudantes]
  (count (filter esta-no-quinto-ano? estudantes)))

(println (quantidade-estudantes-no-quinto-ano-v0 [5 5 6 7 8 6 5 5]))

; usando uma função anonmima
(defn quantidade-estudantes-no-quinto-ano-v1
  [estudantes]
  (count (filter (fn [ano] (= 5 ano)) estudantes)))

(println (quantidade-estudantes-no-quinto-ano-v1 [5 5 6 7 8 6 5 5]))

; usando uma função anonmima (outra maneira de criar funções anonimas)
(defn quantidade-estudantes-no-quinto-ano-v2 
  [estudantes]
  (count (filter #(= 5 %) estudantes)))

(println (quantidade-estudantes-no-quinto-ano-v1 [5 5 6 7 8 6 5 5]))
(println "")

; DESAFIO 2: crie uma função que recebe um vetor de idades e retorna a soma.
; Ex: (soma-das-idades [5 10 5]); deve retornar 20
(println "Desafio 2:")
(defn soma-das-idades
  [idades]
  (reduce + idades))

(println (soma-das-idades [5 10 5]))
(println "")

; DESAFIO 3: Crie uma função que recebe um vetor de nomes e retorne o 
; tamanho médio dos nomes.
; Ex: (tamanho-medio-dos-nomes ["Marcio" "Joao"]); deve retornar 5
(println "Desafio 3:")
(defn tamanho-medio-dos-nomes
  [nomes]
  (/ (reduce + (map count nomes)) (count nomes)))

(println (tamanho-medio-dos-nomes ["Marcio" "Joao"]))
(println "")

; DESAFIO 4: Crie uma função que recebe um vetor de nomes e retorne o
; tamanho médio dos nomes, mas deve-se ignorar nomes com 3 ou menos caracteres.
(println "Desafio 4:")
(defn tamanho-medio-nomes-desafio-3
  [nomes]
  ; 1º filtrar a lista -> 2º conta -> 3º acha a média
  (/ (reduce + (map count (filter #(> (count %) 3) nomes))) (count nomes)))

(println (tamanho-medio-nomes-desafio-3 ["Marcio" "Joao" "Nessa", "Two", "Rol", "Nuna", "Maria", "Niet", "Lier", "Astolfin"]))