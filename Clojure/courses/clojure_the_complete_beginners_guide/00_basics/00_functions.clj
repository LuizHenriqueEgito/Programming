(defn -main
  "First function"
  []
  (println "My name is Egito")
  (println "Loving clojure so far")
  (+ 2 5))

(#(println "Hello" %1 "how are you" %2) "Maria" "today")

(def increment (fn [x] (+ x 1)))

(defn increment_set
  [x]
  (map increment x))

(increment_set [1 2 3 4 5 6])