; clojure não trabalha com 0 e 1 como false e true
; if só cairá no false se você passar false ou nil
(defn parametros-preenchidos
  [nome idade]
  (if (and nome idade)  ; retorna o último valor se ambos forem truthy e nil se algum for nil ou false
    (println "Estão preenchidos")
    (println "Não estão preenchidos")))

(parametros-preenchidos "Maria" 25)