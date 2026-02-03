; ns: namespace
; banco.logic: banco é a pasta e core é o arquivo
; no clojure tudo que começa com : é uma keyword :require por exemplo
(ns banco.core)

; Nos testes podemos ter algo como
(ns banco.imposto.logic-test
  (:require [clojure.test :refer :all]
            [banco.imposto.logic :as l])) ; é como um alias
           ;[banco.imposto.logic :refer :all]  importa todas as funções
           ;[banco.imposto.logic :refer [lista de funções desse arquivo que você quer]]