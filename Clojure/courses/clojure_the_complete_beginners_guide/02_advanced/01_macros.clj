;; macros recebem o código de entrada e devolvem código de saida antes da execução do programa.
(macroexpand-1 '(when true (println "hello")))
