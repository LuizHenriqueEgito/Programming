(defn cond-if
  []
  (println "Cond IF:")
  (if (= 5 6)
    (println "Equal")
    (println "Not Equal")))

(cond-if)

(defn cond-if-do
  []
  (println "Cond IF DO:")
  (if (= 7 7)
    (do (println "Equal first statement")
        (println "Second statement"))
    (do (println "Not equal first statement")
        (println "Second statement"))))
(cond-if-do)

(defn cond-nested-if
  []
  (println "Nested IF")
  (if (and (= 5 5) (or (= 2 2) (not true)))
    (println "True")
    (println "False")))
(cond-nested-if)

(defn cond-case
  []
  (println "Cond CASE")
  (def pet "dog")
  (case pet
    "cat" (println "I have a cat")
    "dog" (println "I have a dog")
    "fish" (println "I have a goldfish")))

(cond-case "dog")

(defn cond-cond
  [amount]
  (println "Cond COND")
  (cond 
    (<= amount 2) (println "Few")
    (<= amount 10) (println "Several")
    (<= amount 100) (println "Many")
    :else (println "Loads")))

(cond-cond 5)