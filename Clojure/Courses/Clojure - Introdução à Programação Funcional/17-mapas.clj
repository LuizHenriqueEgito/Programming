; Os vetores apresentam limitações os mapas nos ajudam a resolver esse problema
; é como um dicionario do python

; Carrinho de compras
(defn compras
[]
{:tomate {:quantidade 2, :preco 5}
 :arroz {:quantidade 1, :preco 4}
 :feijao {:quantidade 2, :preco 10}})

(println (get (compras) :tomate))
(println ((compras) :tomate))  ;Não é recomendado utilizar essa forma
(println (:tomate (compras)))

(println (:preco (:tomate (compras))))

(println (:batata (compras) {:quantidade 0, :preco 0})) ; colocamos um valor default