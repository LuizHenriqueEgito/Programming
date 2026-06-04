;; São como modulos no python
;; a ideia é organizar o código e evitar conflitos de nomes
(ns courses.Namespaces
  (:require [clojure.string :refer [capitalize]]))

(defn -main
  []
  (println (capitalize "hello")))

(-main)