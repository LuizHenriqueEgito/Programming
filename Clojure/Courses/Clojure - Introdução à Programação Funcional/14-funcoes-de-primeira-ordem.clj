; Funções de primeira ordem
;; Permite:
;;  - Passar funções como argumentos de outras funções
;;  - Retornar uma função como resultado da execução de outra função
; Ou seja: Funções também são valores
(defn consulta-taxa-padrao-por-http
  "imagine que este código faz uma requisição http para obter a taxa-padrão."
  []
  0.30)

(defn imposto-retido
  "Se salario abaixo de 1000 não tem imposto. Acima ou igual a 1000 deve ser passado uma taxa de tributação"
  [consulta-taxa-padrao salario]
  (if (< salario 1000)
      0
      (* salario (consulta-taxa-padrao))))

(println (imposto-retido consulta-taxa-padrao-por-http 2000))

(defn consulta-taxa-padrao-fixa
  []
  0.10)

(defn escolhe-consulta-taxa-padrao
  [ambiente]
  (if (= ambiente :test)   ; criamos essa keyword (:test)
    consulta-taxa-padrao-fixa
    consulta-taxa-padrao-por-http))  ; faz uma referencia para minha função criada na linha 20

(println (escolhe-consulta-taxa-padrao :producao))
(println (escolhe-consulta-taxa-padrao :test))
(println (imposto-retido (escolhe-consulta-taxa-padrao :producao) 2000))
(println (imposto-retido (escolhe-consulta-taxa-padrao :test) 2000))

(println (imposto-retido (fn [] 0.27) 2000)) ; função anonima
(println (imposto-retido (constantly 0.27) 2000))