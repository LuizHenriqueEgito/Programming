(conj nil "banana")
(conj ["batata", "arroz", "feijão"] "banana")
(pop ["batata", "arroz", "feijão"])
(defn desistir-ultima-compra
  [compras]
  (pop compras))
(desistir-ultima-compra ["batata", "arroz", "feijão"])

; Listas em clojure são parenteses () por isso Lisp
(conj (conj nil "banana") "arroz")
(conj [1 2 3] nil)
(pop nil) ; retorna nil
; nulo no clojure tem um comportamento diferente
(defn imprime-primeiro-nome
  [nomes]
  (println (nomes 0))) ; pega o indice 0 desse vetor

(imprime-primeiro-nome ["Luiz", "Egito"])
(defn imprime-terceiro-nome
  [nome]
  (println (nomes 2)))

(imprime-terceiro-nome ["Luiz", "Henrique", "Egito"]) ; retorna Egito

(defn imprime-primeiro-nome
  [nomes]
  (println (get nomes 0))) ; não retorna Exception


(defn substituir-primeiro-nome
  [nomes novo-primeiro-nome]
  (assoc nomes 0 novo-primeiro-nome))  ; assoc faz essa associação para criação de um novo vetor alterado