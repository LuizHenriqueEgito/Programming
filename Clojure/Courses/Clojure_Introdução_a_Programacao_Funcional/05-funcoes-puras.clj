; uma função para ser considerada pura ela não deve causar NENHUM efeito colateral
; funções puras NÃO podem depender do estado do universo
(defn soma
  [x y]
  (+ x y))
