
; IF: é uma forma especial
; É como se fosse um parâmetro com 3 parametros
;; (if (condicao)
;;   (expressao s true)
;;   (expressao se false))

; Statements Vs expressoions
; Statements: Abordagem imperativa if else normal
; Expressions: Abordagem declarativa sempre um retorne como um operador ternario
(> 10 20)  ; retorna: false
(< 10 20)  ; retorna: true
(<= 10 20)  ; retorna: false
(= 10 20)  ; retorna: false

(defn imposto-retido
  "Se salario abaixo ou igual a 1000 reais não tem imposto. Acima de 1000
  e abaixo de 2000 reais tem 10% de imposto e acima ou igual a 2000, imposto
  deve ser de 20%"
  [salario]
  (if (<= salario 1000)
    0
    (if (< salario 2000)
      (* salario 0.1)
      (* salario 0.2))))

(imposto-retido 1500)

(defn imposto-retido-ajustado
  [salario]
  (let [imposto-primeira-faixa 0
        imposto-segunda-faixa 0.1
        imposto-terceira-faixa 0.2
        salario-inferior 1000
        salario-superior 2000]
    (if (<= salario salario-inferior)
        (* salario imposto-primeira-faixa)
        (if (< salario salario-superior)
          (* salario imposto-segunda-faixa)
          (* salario imposto-terceira-faixa)))))

(defn isento-de-imposto? ; isso é um predicate
  [valor]
  (if (<= valor 1000)
    true
    false)) ; você pode não passar nada e ele irá retornar nil pois em cloojure ele sempre retorna

(defn isento-de-imposto?
  [valor]
  (<= valor 1000)) ; faz a mesma coisa mas bem mais simples

(defn imprimir-mensagem-boas-vindas
  [idade]
  (when (>= idade 18)
    (println "Essa mensagem será exibida para pessoas maiores de idade")
  ))