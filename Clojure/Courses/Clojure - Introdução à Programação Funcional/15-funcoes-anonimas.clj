(defn dobro [x] (* 2 x))

(fn [x] (* 2 x)) ; é como uma lambda function
(println ((fn [x] (* 2 x)) 3)) ; retorna 6

#(* 2 %) ; isso é a mesma coisa que (fn [x] (* 2 x)) mas bem mais simplificado
(println (#(* 2 %) 5))
(println (#(+ % %) 2))

(println (#(+ %1 %2) 2 3))  ; agora ela espera 2 parametros

