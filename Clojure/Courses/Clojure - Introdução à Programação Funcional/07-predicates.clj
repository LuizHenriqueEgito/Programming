; PREDICATE
; Predicate: São funcões que que devolvem sempre true ou false
; Por convenção utilize ? ao final de uma função predicate
(even? 3)
(odd? 3)
(neg? 3)
(defn isento-imposto?
  [valor]
  (< valor 1000))
