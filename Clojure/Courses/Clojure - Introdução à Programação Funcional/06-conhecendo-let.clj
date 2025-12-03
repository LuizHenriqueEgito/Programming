(defn valor-multa
  "Calcular valor total incluindo multa"  ; é como uma docString no python
  [valor]
  (let [percentual-multa 0.1] ; se eu tirar ele sai do escopo e ele não encontra
    (* valor percentual-multa)))

(defn valor-total
  "Calcula o valor total"
  [valor]
  (let [percentual-multa 0.1
        valor-multa (* valor percentual-multa)]
    (+ valor valor-multa)))

(println (valor-multa 1000))
(println (valor-total 1000))