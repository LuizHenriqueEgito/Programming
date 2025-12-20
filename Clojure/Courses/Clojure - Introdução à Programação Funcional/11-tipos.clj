(defn soma
  [x y]
  (+ x y))

(defn imprime-soma
  []
  (println (soma 2 2)))

; (println (class 10)) -> faz a mesma coisa que type
(println (type 10))  ; java.lang.Long
(println (type 10.0))  ; java.lang.Double
; É uma dizima periodica
(println (type (/ 10 3)))  ; clojure.lang.Ratio
(println (type (* 3 (/ 10 3))))  ; retorna 10N -> clojure.lang.BigInt
(println (type 2N))  ; clojure.lang.BigInt
(println (type 2M))  ; java.math.BigDecimal
(println (type "Egito"))  ; java.lang.String
(println (type nil))  ; nil
(println (type []))  ; clojure.lang.PersistentVector
(println (type [1 2 3]))  ; clojure.lang.PersistentVector
(println (type println))  ; clojure.core$println