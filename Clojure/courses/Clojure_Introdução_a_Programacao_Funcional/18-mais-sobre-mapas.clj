(defn compras
[]
{:tomate {:quantidade 2, :preco 5}
 :arroz {:quantidade 1, :preco 4}
 :feijao {:quantidade 2, :preco 10}})

; podemos criar um novo mapa com conj
(println (conj (compras) {:alface {:quantidade 2, :preco 8}}))

; podemos fazer isso também com a função assoc
(println (assoc (compras) :alface {:quantidade 2, :preco 8}))
; podemos "atualizar" não é atualização poisserá uma lista nova
(println (assoc (compras) :tomate {:quantidade 5, :preco 111}))

; podemos remover uma chave com dissoc
(println (dissoc (compras) :tomate))
; podemos remover mais de um elemento
(println (dissoc (compras) :tomate :arroz))

; usando update
(println (update {:nome "Luiz Egito", :idade 28} :idade inc))

; update de um mapa dentro de um mapa
(println (update-in (compras) [:tomate :preco] inc))

(println (update-in (compras) [:tomate :preco] * 2))